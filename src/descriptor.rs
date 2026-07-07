//! Indicator metadata — the descriptor layer for chart-page auto-discovery.
//!
//! This module is *purely additive metadata*. It does not touch the compute
//! API (`ema()`, `rsi()`, the `Indicator` trait, the runtime `IndicatorRegistry`)
//! — existing consumers keep working unchanged. Its job is to describe, in a
//! machine-readable and JSON-serializable form, *what parameters each indicator
//! takes* so a front-end (janus catalog API → chart page) can render controls
//! for every indicator without hard-coding them.
//!
//! # The one-obvious-place pattern
//!
//! [`catalog()`] is a **hand-maintained registry**: one entry per indicator,
//! co-located in this file. When you add a new Rust indicator to the crate, you
//! add exactly one thing here — a [`IndicatorDescriptor`] with honest
//! [`ParamSpec`]s that mirror its `factory()` constructor params (the same keys
//! and defaults the factory reads from the params map). That single edit is what
//! makes the new indicator show up in the UI. There is no codegen and no derive
//! magic on purpose: the descriptor is the source of truth for the UI, and
//! keeping it explicit keeps it honest.
//!
//! Rule of thumb: the `default`/`min`/`max` of each [`ParamSpec`] must match the
//! `param_usize`/`param_f64` default the corresponding `factory()` uses, and the
//! `id` must match the lowercased name the indicator is registered under in the
//! runtime [`crate::registry::IndicatorRegistry`].

use serde::{Deserialize, Serialize};

/// Where an indicator is drawn on a chart.
///
/// - `Overlay` — plotted on the price pane, sharing the price scale
///   (moving averages, bands, VWAP, parabolic SAR).
/// - `Oscillator` — plotted in a separate pane with its own scale
///   (RSI, MACD, stochastics, volatility/volume oscillators).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IndicatorCategory {
    /// Drawn on the price pane, on the price scale.
    Overlay,
    /// Drawn in its own pane, on its own scale.
    Oscillator,
}

/// The numeric type of a tunable parameter.
///
/// Both variants carry their bounds as `f64` in [`ParamSpec`]; `Integer` simply
/// tells the UI to render a stepper/whole-number control and round on input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParamKind {
    /// A whole-number parameter (a period, a smoothing length, ...).
    Integer,
    /// A real-valued parameter (a multiplier, a standard-deviation count, ...).
    Float,
}

/// A single tunable parameter of an indicator.
///
/// `default`, `min` and `max` are all `f64` regardless of [`ParamKind`] so the
/// serialized JSON has a uniform numeric shape; for `Integer` params the values
/// are whole numbers stored as `f64` (e.g. `14.0`).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ParamSpec {
    /// Parameter key — matches the key the indicator's `factory()` reads
    /// (e.g. `"period"`, `"fast_period"`, `"std_dev"`).
    pub name: &'static str,
    /// Whether the UI should treat this as an integer or a float control.
    pub kind: ParamKind,
    /// Default value — must equal the factory's default and lie in `[min, max]`.
    pub default: f64,
    /// Inclusive lower bound for the UI control.
    pub min: f64,
    /// Inclusive upper bound for the UI control.
    pub max: f64,
}

impl ParamSpec {
    /// Construct an [`ParamKind::Integer`] parameter spec.
    #[must_use]
    pub const fn int(name: &'static str, default: f64, min: f64, max: f64) -> Self {
        Self {
            name,
            kind: ParamKind::Integer,
            default,
            min,
            max,
        }
    }

    /// Construct a [`ParamKind::Float`] parameter spec.
    #[must_use]
    pub const fn float(name: &'static str, default: f64, min: f64, max: f64) -> Self {
        Self {
            name,
            kind: ParamKind::Float,
            default,
            min,
            max,
        }
    }
}

/// Machine-readable description of one indicator, for UI auto-discovery.
///
/// This is the JSON payload the janus catalog API emits. Example (RSI):
///
/// ```json
/// {
///   "id": "rsi",
///   "display_name": "RSI",
///   "category": "Oscillator",
///   "params": [
///     { "name": "period", "kind": "Integer", "default": 14.0, "min": 2.0, "max": 500.0 }
///   ]
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndicatorDescriptor {
    /// Stable identifier — the lowercased name the indicator is registered
    /// under in the runtime registry (e.g. `"rsi"`, `"bollingerbands"`).
    pub id: &'static str,
    /// Human-readable name for the UI (e.g. `"RSI"`, `"Bollinger Bands"`).
    pub display_name: &'static str,
    /// Which chart pane the indicator belongs to.
    pub category: IndicatorCategory,
    /// The indicator's tunable parameters, in display order.
    pub params: Vec<ParamSpec>,
}

impl IndicatorDescriptor {
    /// Convenience constructor keeping [`catalog()`] entries terse.
    fn new(
        id: &'static str,
        display_name: &'static str,
        category: IndicatorCategory,
        params: Vec<ParamSpec>,
    ) -> Self {
        Self {
            id,
            display_name,
            category,
            params,
        }
    }
}

/// The hand-maintained catalog of every indicator in this crate.
///
/// **This is the one place to edit when adding an indicator.** Each entry's
/// `id` matches the runtime registry name, `display_name` matches the
/// indicator's `name()`, and the [`ParamSpec`]s mirror the defaults its
/// `factory()` reads. See the module docs for the full contract.
///
/// Ordering: trend → momentum → volatility → volume, matching
/// `registry::registry()`'s `register_all` call order.
#[must_use]
pub fn catalog() -> Vec<IndicatorDescriptor> {
    use IndicatorCategory::{Oscillator, Overlay};
    use ParamSpec as P;

    // Shared bound: periods are whole numbers in a sane charting range.
    const PMIN: f64 = 1.0;
    const PMAX: f64 = 500.0;

    vec![
        // ── trend ────────────────────────────────────────────────────────────
        IndicatorDescriptor::new(
            "ema",
            "EMA",
            Overlay,
            vec![P::int("period", 20.0, PMIN, PMAX)],
        ),
        IndicatorDescriptor::new(
            "sma",
            "SMA",
            Overlay,
            vec![P::int("period", 20.0, PMIN, PMAX)],
        ),
        IndicatorDescriptor::new(
            "wma",
            "WMA",
            Overlay,
            vec![P::int("period", 14.0, PMIN, PMAX)],
        ),
        IndicatorDescriptor::new(
            "macd",
            "MACD",
            Oscillator,
            vec![
                P::int("fast_period", 12.0, PMIN, PMAX),
                P::int("slow_period", 26.0, PMIN, PMAX),
                P::int("signal_period", 9.0, PMIN, PMAX),
            ],
        ),
        IndicatorDescriptor::new(
            "atr",
            "ATR",
            Oscillator,
            vec![P::int("period", 14.0, PMIN, PMAX)],
        ),
        IndicatorDescriptor::new(
            "linearregression",
            "LinearRegression",
            Overlay,
            vec![P::int("period", 14.0, PMIN, PMAX)],
        ),
        IndicatorDescriptor::new(
            "parabolicsar",
            "ParabolicSAR",
            Overlay,
            vec![
                P::float("step", 0.02, 0.001, 0.5),
                P::float("max_step", 0.2, 0.01, 1.0),
            ],
        ),
        // ── momentum ─────────────────────────────────────────────────────────
        IndicatorDescriptor::new(
            "rsi",
            "RSI",
            Oscillator,
            vec![P::int("period", 14.0, 2.0, PMAX)],
        ),
        IndicatorDescriptor::new(
            "schafftrendcycle",
            "SchaffTrendCycle",
            Oscillator,
            vec![
                P::int("short_ema", 12.0, PMIN, PMAX),
                P::int("long_ema", 26.0, PMIN, PMAX),
                P::int("stoch_period", 10.0, PMIN, PMAX),
                P::int("signal_period", 3.0, PMIN, PMAX),
            ],
        ),
        IndicatorDescriptor::new(
            "stochastic",
            "Stochastic",
            Oscillator,
            vec![
                P::int("k_period", 14.0, PMIN, PMAX),
                P::int("smooth_k", 3.0, PMIN, PMAX),
                P::int("d_period", 3.0, PMIN, PMAX),
            ],
        ),
        IndicatorDescriptor::new(
            "stochasticrsi",
            "StochasticRSI",
            Oscillator,
            vec![
                P::int("rsi_period", 14.0, 2.0, PMAX),
                P::int("stoch_period", 14.0, PMIN, PMAX),
                P::int("k_smooth", 3.0, PMIN, PMAX),
                P::int("d_period", 3.0, PMIN, PMAX),
            ],
        ),
        IndicatorDescriptor::new(
            "williamsr",
            "WilliamsR",
            Oscillator,
            vec![P::int("period", 14.0, PMIN, PMAX)],
        ),
        // ── volatility ───────────────────────────────────────────────────────
        IndicatorDescriptor::new(
            "bollingerbands",
            "BollingerBands",
            Overlay,
            vec![
                P::int("period", 20.0, PMIN, PMAX),
                P::float("std_dev", 2.0, 0.1, 10.0),
            ],
        ),
        IndicatorDescriptor::new(
            "choppinessindex",
            "ChoppinessIndex",
            Oscillator,
            vec![P::int("period", 14.0, PMIN, PMAX)],
        ),
        IndicatorDescriptor::new(
            "elderrayindex",
            "ElderRayIndex",
            Oscillator,
            vec![P::int("fast_period", 14.0, PMIN, PMAX)],
        ),
        IndicatorDescriptor::new(
            "keltnerchannels",
            "KeltnerChannels",
            Overlay,
            vec![
                P::int("period", 20.0, PMIN, PMAX),
                P::float("multiplier", 2.0, 0.1, 10.0),
            ],
        ),
        IndicatorDescriptor::new(
            "marketcycle",
            "MarketCycle",
            Oscillator,
            vec![P::int("momentum_period", 1.0, PMIN, PMAX)],
        ),
        // ── volume ───────────────────────────────────────────────────────────
        IndicatorDescriptor::new("adl", "ADL", Oscillator, vec![]),
        IndicatorDescriptor::new(
            "chaikinmoneyflow",
            "ChaikinMoneyFlow",
            Oscillator,
            vec![P::int("period", 20.0, PMIN, PMAX)],
        ),
        // VWAP: period 0 means cumulative (session) VWAP; > 0 is a rolling window.
        IndicatorDescriptor::new(
            "vwap",
            "VWAP",
            Overlay,
            vec![P::int("period", 0.0, 0.0, PMAX)],
        ),
        IndicatorDescriptor::new(
            "vzo",
            "VZO",
            Oscillator,
            vec![P::int("period", 14.0, PMIN, PMAX)],
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn catalog_is_non_empty() {
        assert!(!catalog().is_empty());
    }

    #[test]
    fn ids_are_unique() {
        let cat = catalog();
        let mut seen = HashSet::new();
        for d in &cat {
            assert!(seen.insert(d.id), "duplicate indicator id: {}", d.id);
        }
    }

    #[test]
    fn defaults_within_bounds() {
        for d in catalog() {
            for p in &d.params {
                assert!(
                    p.min <= p.max,
                    "{}::{} has min {} > max {}",
                    d.id,
                    p.name,
                    p.min,
                    p.max
                );
                assert!(
                    p.default >= p.min && p.default <= p.max,
                    "{}::{} default {} out of [{}, {}]",
                    d.id,
                    p.name,
                    p.default,
                    p.min,
                    p.max
                );
            }
        }
    }

    #[test]
    fn ids_match_runtime_registry() {
        // The descriptor catalog must not drift from the runtime registry:
        // every descriptor id should be a creatable indicator name.
        let reg = crate::registry::registry();
        for d in catalog() {
            assert!(
                reg.contains(d.id),
                "descriptor id `{}` is not registered in the runtime registry",
                d.id
            );
        }
    }

    #[test]
    fn serializes_to_expected_json_shape() {
        let rsi = catalog()
            .into_iter()
            .find(|d| d.id == "rsi")
            .expect("rsi in catalog");
        let json = serde_json::to_value(&rsi).unwrap();
        assert_eq!(json["id"], "rsi");
        assert_eq!(json["display_name"], "RSI");
        assert_eq!(json["category"], "Oscillator");
        assert_eq!(json["params"][0]["name"], "period");
        assert_eq!(json["params"][0]["kind"], "Integer");
        assert_eq!(json["params"][0]["default"], 14.0);
    }
}
