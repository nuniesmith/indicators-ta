//! NaN-robustness regression tests.
//!
//! A live feed can produce NaN (stale tick, zero-volume bar, upstream
//! division by zero). The streaming engines must degrade gracefully —
//! never panic — when NaN reaches them. These tests lock in the
//! `total_cmp`/destructuring fixes in the signal engine, liquidity
//! profile, HMM, and regime detector.

use indicators::{
    EnsembleConfig, EnsembleRegimeDetector, HMMConfig, HMMRegimeDetector, IndicatorConfig,
    Indicators, LiquidityProfile, MarketStructure, RegimeConfig, RegimeDetector, types::Candle,
};

/// A normal candle followed by NaN-poisoned candles, then normal again
/// (recovery path).
fn poisoned_candles(n: usize) -> Vec<Candle> {
    (0..n)
        .map(|i| {
            let c = 100.0 + i as f64 * 0.5;
            let poison = i % 7 == 3;
            Candle {
                time: i64::try_from(i).unwrap() * 60_000,
                open: if poison { f64::NAN } else { c - 0.2 },
                high: if poison { f64::NAN } else { c + 0.3 },
                low: if poison { f64::NAN } else { c - 0.3 },
                close: if poison { f64::NAN } else { c },
                volume: if poison { f64::NAN } else { 1_000.0 },
            }
        })
        .collect()
}

#[test]
fn signal_engine_survives_nan_candles() {
    let mut ind = Indicators::new(IndicatorConfig::default());
    for c in poisoned_candles(400) {
        ind.update(&c); // must not panic (KMeans centroid assignment)
    }
}

#[test]
fn liquidity_profile_survives_nan_candles() {
    let mut liq = LiquidityProfile::new(50, 25);
    for c in poisoned_candles(200) {
        liq.update(&c); // must not panic (POC max_by)
    }
}

#[test]
fn market_structure_survives_nan_candles() {
    let mut ms = MarketStructure::new(5, 0.5);
    for c in poisoned_candles(200) {
        ms.update(&c);
    }
}

#[test]
fn hmm_survives_nan_closes() {
    let mut hmm = HMMRegimeDetector::new(HMMConfig::default());
    for i in 0..300 {
        let close = if i % 11 == 5 {
            f64::NAN
        } else {
            100.0 + (i as f64 * 0.1).sin() * 5.0
        };
        let _ = hmm.update(close); // must not panic (state-prob argmax)
    }
}

#[test]
fn regime_detector_survives_nan_bars() {
    let mut det = RegimeDetector::new(RegimeConfig::default());
    for i in 0..300 {
        let (h, l, c) = if i % 13 == 4 {
            (f64::NAN, f64::NAN, f64::NAN)
        } else {
            let p = 100.0 + i as f64 * 0.2;
            (p + 1.0, p - 1.0, p)
        };
        let _ = det.update(h, l, c);
    }
}

#[test]
fn ensemble_survives_nan_bars() {
    let mut ens = EnsembleRegimeDetector::new(EnsembleConfig::default(), RegimeConfig::default());
    for i in 0..300 {
        let (h, l, c) = if i % 17 == 6 {
            (f64::NAN, f64::NAN, f64::NAN)
        } else {
            let p = 100.0 + i as f64 * 0.2;
            (p + 1.0, p - 1.0, p)
        };
        let _ = ens.update(h, l, c);
    }
}
