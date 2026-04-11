//! Exponential Moving Average (EMA).
//!
//! Python source: `indicators/trend/moving_average.py :: class EMA`
//!
//! # Python algorithm (to port)
//! ```python
//! ema = data[self.column].ewm(span=self.period, adjust=False, alpha=self.alpha).mean()
//! ```
//!
//! Note: `self.alpha = params.get("alpha", 2 / (period + 1))`
//!
//! Output column: `"EMA_{period}"`.
//!
//! See also: `crate::functions::ema()` for the existing batch implementation
//! and `crate::functions::EMA` for the existing incremental struct — both
//! can serve as the porting target for the `calculate()` body here.

use std::collections::HashMap;

use crate::functions::{self};
use crate::error::IndicatorError;
use crate::indicator::{Indicator, IndicatorOutput, PriceColumn};
use crate::registry::{param_f64, param_usize, param_str};
use crate::types::Candle;

// ── Params ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct EmaParams {
    /// Lookback period (span).  Python default: 20.
    pub period: usize,
    /// Smoothing factor.  Python default: `2 / (period + 1)`.
    /// Pass `None` to use the standard formula.
    pub alpha: Option<f64>,
    /// Price field.  Python default: `"close"`.
    pub column: PriceColumn,
}

impl Default for EmaParams {
    fn default() -> Self {
        Self {
            period: 20,
            alpha: None,
            column: PriceColumn::Close,
        }
    }
}

impl EmaParams {
    fn effective_alpha(&self) -> f64 {
        self.alpha
            .unwrap_or_else(|| 2.0 / (self.period as f64 + 1.0))
    }
}

// ── Indicator struct ──────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Ema {
    pub params: EmaParams,
}

impl Ema {
    pub fn new(params: EmaParams) -> Self {
        Self { params }
    }

    pub fn with_period(period: usize) -> Self {
        Self::new(EmaParams { period, ..Default::default() })
    }

    fn output_key(&self) -> String {
        format!("EMA_{}", self.params.period)
    }
}

impl Indicator for Ema {
    fn name(&self) -> &str {
        "EMA"
    }

    fn required_len(&self) -> usize {
        self.params.period
    }

    fn required_columns(&self) -> &[&'static str] {
        &["close"]
    }

    /// TODO: port Python ewm logic.
    ///
    /// Delegate to `crate::functions::ema()` which already implements SMA-seeded EMA.
    /// The alpha override from Python's `ewm(alpha=...)` is the only delta to handle.
    fn calculate(&self, candles: &[Candle]) -> Result<IndicatorOutput, IndicatorError> {
        self.check_len(candles)?;

        let prices = self.params.column.extract(candles);
        let alpha = self.params.effective_alpha();
        let n = prices.len();
        let period = self.params.period;

        // TODO: honour custom alpha; functions::ema() uses 2/(period+1) only.
        // If alpha matches the default, we can delegate directly:
        let values = functions::ema(&prices, period)?;

        // If a custom alpha is set, recompute:
        // let values = ema_with_alpha(&prices, period, alpha);

        Ok(IndicatorOutput::from_pairs([(self.output_key(), values)]))
    }
}

// ── Registry factory ──────────────────────────────────────────────────────────

pub fn factory(params: &HashMap<String, String>) -> Result<Box<dyn Indicator>, IndicatorError> {
    let period = param_usize(params, "period", 20)?;
    let alpha = if params.contains_key("alpha") {
        Some(param_f64(params, "alpha", 2.0 / (period as f64 + 1.0))?)
    } else {
        None
    };
    let column = match param_str(params, "column", "close") {
        "open" => PriceColumn::Open,
        "high" => PriceColumn::High,
        "low" => PriceColumn::Low,
        _ => PriceColumn::Close,
    };
    Ok(Box::new(Ema::new(EmaParams { period, alpha, column })))
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn candles(closes: &[f64]) -> Vec<Candle> {
        closes.iter().enumerate().map(|(i, &c)| Candle {
            time: i as i64, open: c, high: c, low: c, close: c, volume: 1.0,
        }).collect()
    }

    #[test]
    fn ema_insufficient_data() {
        let ema = Ema::with_period(5);
        assert!(ema.calculate(&candles(&[1.0, 2.0])).is_err());
    }

    #[test]
    fn ema_output_column_named_correctly() {
        let ema = Ema::with_period(3);
        let out = ema.calculate(&candles(&[10.0, 20.0, 30.0])).unwrap();
        assert!(out.get("EMA_3").is_some());
    }

    #[test]
    fn ema_seed_equals_sma() {
        // EMA at index `period-1` should equal the SMA of first `period` values.
        let closes = vec![10.0, 20.0, 30.0];
        let ema = Ema::with_period(3);
        let out = ema.calculate(&candles(&closes)).unwrap();
        let vals = out.get("EMA_3").unwrap();
        let expected_seed = (10.0 + 20.0 + 30.0) / 3.0;
        assert!((vals[2] - expected_seed).abs() < 1e-9, "got {}", vals[2]);
    }

    #[test]
    fn ema_subsequent_value() {
        // alpha = 2/(3+1) = 0.5; EMA[3] = 40*0.5 + 20*0.5 = 30
        let closes = vec![10.0, 20.0, 30.0, 40.0];
        let ema = Ema::with_period(3);
        let out = ema.calculate(&candles(&closes)).unwrap();
        let vals = out.get("EMA_3").unwrap();
        let expected = 40.0 * 0.5 + 20.0 * 0.5;
        assert!((vals[3] - expected).abs() < 1e-6, "got {}", vals[3]);
    }

    #[test]
    fn factory_creates_ema() {
        let params = [("period".into(), "12".into())].into();
        let ind = factory(&params).unwrap();
        assert_eq!(ind.name(), "EMA");
    }
}
