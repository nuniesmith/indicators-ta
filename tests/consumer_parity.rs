//! Consumer-parity regression tests for janus's signal / strategy suite.
//!
//! janus's forward `IndicatorAnalyzer` consumes
//! `janus_indicators::{ATR, EMA, ema, macd, rsi, sma}` and `crates/strategies`
//! uses `ATR` + `IndicatorCalculator` (all of which is *this* crate — janus
//! renames the dependency key to `janus-indicators`). The `jflow-indicators`
//! implementation these were consolidated onto is retired, so rather than
//! cross-check against it, these tests **pin the numerics of the exact surface
//! janus relies on** — with spec-derived golden values (a linear ramp gives
//! EMA/SMA closed forms) and definitional invariants — so a future change can't
//! silently alter what the live signal path sees.

use indicators::{ATR, EMA, IndicatorCalculator, atr, ema, macd, rsi, sma};

/// A clean linear ramp `1.0..=n`. EMA/SMA over a ramp have exact closed forms,
/// so the golden values below are spec-derived, not implementation snapshots.
fn ramp(n: usize) -> Vec<f64> {
    (1..=n).map(|i| i as f64).collect()
}

/// Compare two indicator series, treating `NaN == NaN` (leading warm-up).
fn assert_series_eq(got: &[f64], want: &[f64]) {
    assert_eq!(got.len(), want.len(), "series length");
    for (i, (g, w)) in got.iter().zip(want).enumerate() {
        match (g.is_nan(), w.is_nan()) {
            (true, true) => {}
            _ => assert!((g - w).abs() < 1e-12, "index {i}: {g} != {w}"),
        }
    }
}

#[test]
fn sma_pins_window_mean() {
    let s = sma(&ramp(6), 3).unwrap();
    assert!(s[0].is_nan() && s[1].is_nan(), "leading warm-up is NaN");
    assert_eq!(s[2], 2.0, "mean(1,2,3)");
    assert_eq!(s[5], 5.0, "mean(4,5,6)");
}

#[test]
fn ema_batch_pins_seed_and_steady_state() {
    // Period 5 over a ramp: seed = mean(1..=5) = 3, and the EMA's steady-state
    // lag on a linear ramp is (period-1)/2 = 2, so ema[i] = price[i] - 2 once
    // warmed up. Both are closed-form, independent of the implementation.
    let e = ema(&ramp(10), 5).unwrap();
    for v in e.iter().take(4) {
        assert!(v.is_nan(), "leading warm-up is NaN");
    }
    let want = [3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    for (got, w) in e[4..].iter().zip(want) {
        assert!((got - w).abs() < 1e-9, "ema warmed-up value {got} != {w}");
    }
}

#[test]
fn rsi_is_bounded_and_saturates_on_all_gains() {
    let r = rsi(&ramp(20), 14).unwrap();
    let mut defined = 0;
    for v in r.iter().filter(|v| !v.is_nan()) {
        assert!((0.0..=100.0).contains(v), "RSI {v} out of [0,100]");
        assert!(
            (v - 100.0).abs() < 1e-9,
            "monotone-up ramp → RSI 100, got {v}"
        );
        defined += 1;
    }
    assert!(defined > 0, "RSI produced some defined values");
}

#[test]
fn macd_histogram_equals_macd_minus_signal() {
    let (m, s, h) = macd(&ramp(40), 12, 26, 9).unwrap();
    assert_eq!(m.len(), 40);
    let mut checked = 0;
    for i in 0..m.len() {
        if !m[i].is_nan() && !s[i].is_nan() {
            assert!(
                (h[i] - (m[i] - s[i])).abs() < 1e-12,
                "histogram = macd - signal at {i}"
            );
            checked += 1;
        }
    }
    assert!(checked > 0, "some MACD/signal points are defined");
}

#[test]
fn atr_batch_is_nonnegative() {
    let close = ramp(20);
    let high: Vec<f64> = close.iter().map(|c| c + 1.0).collect();
    let low: Vec<f64> = close.iter().map(|c| c - 1.0).collect();
    let a = atr(&high, &low, &close, 14).unwrap();
    for v in a.iter().filter(|v| !v.is_nan()) {
        assert!(*v >= 0.0, "ATR is non-negative, got {v}");
    }
}

#[test]
fn incremental_ema_seed_equals_sma_of_first_period() {
    // The forward IndicatorAnalyzer builds `EMA::new(period)` and feeds ticks;
    // its first ready value is the SMA of the first `period` samples.
    let mut e = EMA::new(5);
    for p in ramp(5) {
        e.update(p);
    }
    assert!(e.is_ready(), "ready after `period` samples");
    assert!((e.value() - 3.0).abs() < 1e-12, "seed = mean(1..=5) = 3");
}

#[test]
fn incremental_atr_warms_up_and_is_nonnegative() {
    let mut a = ATR::new(5);
    assert!(!a.is_ready(), "not ready before `period` samples");
    for i in 1..=6 {
        a.update(i as f64 + 1.0, i as f64 - 1.0, i as f64);
    }
    assert!(a.is_ready());
    assert!(a.value() >= 0.0, "ATR is non-negative");
}

#[test]
fn indicator_calculator_matches_standalone_functions() {
    // The strategy suite's `IndicatorCalculator::calculate_all` bundle must
    // equal the standalone functions it wraps — so the bundle and the direct
    // calls can't drift apart.
    let close = ramp(30);
    let high: Vec<f64> = close.iter().map(|c| c + 1.0).collect();
    let low: Vec<f64> = close.iter().map(|c| c - 1.0).collect();

    let calc = IndicatorCalculator::new(8, 21, 14);
    let bundle = calc.calculate_all(&close, &high, &low).unwrap();

    assert_series_eq(&bundle.ema_fast, &ema(&close, 8).unwrap());
    assert_series_eq(&bundle.ema_slow, &ema(&close, 21).unwrap());
    assert_series_eq(&bundle.atr, &atr(&high, &low, &close, 14).unwrap());
}
