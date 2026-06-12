//! Property-based tests for the numerically sensitive paths.
//!
//! Each test feeds arbitrary (but finite, well-formed) inputs and asserts
//! domain invariants that must hold for *any* market data — bounds, ordering,
//! normalisation — rather than golden values. Targets the backlog item:
//! HMM forward/backward, signal-engine aggregation, parabolic SAR flips,
//! plus the incremental structs.

use indicators::{
    HMMConfig, HMMRegimeDetector, IncrementalBollinger, IncrementalEma, IncrementalMacd,
    IncrementalRsi, IndicatorConfig, Indicators, compute_signal, ema,
    indicator::Indicator,
    macd, rsi,
    signal::{
        confluence::ConfluenceEngine, cvd::CVDTracker, liquidity::LiquidityProfile,
        structure::MarketStructure, vol_regime::VolatilityPercentile,
    },
    sma,
    trend::parabolic_sar::ParabolicSar,
    types::Candle,
};
use proptest::prelude::*;

// ── Strategies ────────────────────────────────────────────────────────────────

/// Finite positive prices spanning several orders of magnitude.
fn prices(min_len: usize, max_len: usize) -> impl Strategy<Value = Vec<f64>> {
    prop::collection::vec(0.01f64..1.0e6, min_len..=max_len)
}

/// Well-formed candles: `low <= open, close <= high`, positive volume.
fn candles(min_len: usize, max_len: usize) -> impl Strategy<Value = Vec<Candle>> {
    prop::collection::vec(
        (
            0.01f64..1.0e6,
            0.0f64..0.1,
            0.0f64..0.1,
            0.0f64..1.0,
            0.0f64..1.0e6,
        ),
        min_len..=max_len,
    )
    .prop_map(|rows| {
        rows.into_iter()
            .enumerate()
            .map(|(i, (p, up, down, body_pos, vol))| {
                let high = p * (1.0 + up);
                let low = p * (1.0 - down);
                let open = low + (high - low) * body_pos;
                Candle {
                    time: i64::try_from(i).unwrap() * 60_000,
                    open,
                    high,
                    low,
                    close: p,
                    volume: vol,
                }
            })
            .collect()
    })
}

// ── Batch functions ───────────────────────────────────────────────────────────

proptest! {
    #[test]
    fn ema_finite_after_warmup(ps in prices(5, 200), period in 1usize..=5) {
        let out = ema(&ps, period).unwrap();
        prop_assert_eq!(out.len(), ps.len());
        for (i, v) in out.iter().enumerate().skip(period - 1) {
            prop_assert!(v.is_finite(), "ema[{i}] not finite: {v}");
        }
    }

    #[test]
    fn sma_within_window_hull(ps in prices(5, 200), period in 1usize..=5) {
        let out = sma(&ps, period).unwrap();
        for i in (period - 1)..ps.len() {
            let win = &ps[(i + 1 - period)..=i];
            let lo = win.iter().copied().fold(f64::INFINITY, f64::min);
            let hi = win.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            prop_assert!(out[i] >= lo - 1e-9 && out[i] <= hi + 1e-9,
                "sma[{}]={} outside window hull [{}, {}]", i, out[i], lo, hi);
        }
    }

    #[test]
    fn rsi_bounded_0_100(ps in prices(10, 200), period in 2usize..=8) {
        let out = rsi(&ps, period).unwrap();
        for (i, v) in out.iter().enumerate().skip(period) {
            prop_assert!(v.is_finite(), "rsi[{i}] not finite: {v}");
            prop_assert!((0.0..=100.0).contains(v), "rsi[{i}] out of bounds: {v}");
        }
    }

    #[test]
    fn macd_finite_everywhere(ps in prices(5, 200)) {
        let (line, signal, hist) = macd(&ps, 12, 26, 9).unwrap();
        // ema_nan_aware seeds from the first value, so every bar is defined.
        for i in 0..ps.len() {
            prop_assert!(line[i].is_finite() && signal[i].is_finite() && hist[i].is_finite());
            prop_assert!((hist[i] - (line[i] - signal[i])).abs() <= 1e-6 * hist[i].abs().max(1.0));
        }
    }
}

// ── Incremental structs ───────────────────────────────────────────────────────

proptest! {
    #[test]
    fn incremental_ema_within_input_hull(ps in prices(1, 300), period in 1usize..=50) {
        // EMA is a convex combination of the inputs seen so far, so it can
        // never leave their hull.
        let mut e = IncrementalEma::new(period);
        let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
        for &p in &ps {
            lo = lo.min(p);
            hi = hi.max(p);
            let v = e.update(p).unwrap();
            prop_assert!(v >= lo - 1e-9 && v <= hi + 1e-9,
                "ema {v} escaped input hull [{lo}, {hi}]");
        }
    }

    #[test]
    fn incremental_rsi_bounded(ps in prices(2, 300), period in 2usize..=30) {
        let mut r = IncrementalRsi::new(period);
        for &p in &ps {
            if let Some(v) = r.update(p) {
                prop_assert!((0.0..=100.0).contains(&v), "RSI out of bounds: {v}");
            }
        }
    }

    #[test]
    fn incremental_macd_finite_and_consistent(ps in prices(1, 300)) {
        let mut m = IncrementalMacd::new(12, 26, 9);
        for &p in &ps {
            let (line, signal, hist) = m.update(p).unwrap();
            prop_assert!(line.is_finite() && signal.is_finite() && hist.is_finite());
            prop_assert!((hist - (line - signal)).abs() <= 1e-6 * hist.abs().max(1.0));
        }
    }

    #[test]
    fn incremental_bollinger_band_ordering(ps in prices(2, 300), period in 2usize..=30) {
        let mut bb = IncrementalBollinger::new(period, 2.0);
        for &p in &ps {
            if let Some(v) = bb.update(p) {
                prop_assert!(v.lower <= v.middle + 1e-9 && v.middle <= v.upper + 1e-9,
                    "band ordering violated: {} / {} / {}", v.lower, v.middle, v.upper);
                prop_assert!(v.middle.is_finite());
            }
        }
    }
}

// ── Parabolic SAR flips ───────────────────────────────────────────────────────

proptest! {
    #[test]
    fn parabolic_sar_finite_and_bounded(cs in candles(2, 200)) {
        let out = ParabolicSar::default().calculate(&cs).unwrap();
        let sar = out.get("PSAR").unwrap();
        prop_assert_eq!(sar.len(), cs.len());

        // Each step is a convex move toward an extreme point (a candle high or
        // low), so the SAR can never leave the hull of {0} ∪ highs ∪ lows.
        let hi = cs.iter().map(|c| c.high).fold(0.0f64, f64::max);
        let lo = cs.iter().map(|c| c.low).fold(0.0f64, f64::min);
        for (i, v) in sar.iter().enumerate() {
            prop_assert!(v.is_finite(), "PSAR[{i}] not finite: {v}");
            prop_assert!(*v >= lo - 1e-9 && *v <= hi + 1e-9,
                "PSAR[{i}]={v} outside hull [{lo}, {hi}]");
        }
    }
}

// ── HMM forward/backward ──────────────────────────────────────────────────────

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]
    #[test]
    fn hmm_state_probabilities_stay_normalised(ps in prices(10, 400)) {
        let mut hmm = HMMRegimeDetector::new(HMMConfig::default());
        for &p in &ps {
            let rc = hmm.update(p);
            prop_assert!((0.0..=1.0).contains(&rc.confidence),
                "confidence out of bounds: {}", rc.confidence);
            let probs = hmm.state_probabilities();
            let sum: f64 = probs.iter().sum();
            prop_assert!((sum - 1.0).abs() < 1e-6, "state probs sum to {sum}");
            for q in probs {
                prop_assert!((0.0..=1.0 + 1e-9).contains(q), "state prob out of bounds: {q}");
            }
        }
        let (next_state, next_prob) = hmm.predict_next_state();
        prop_assert!(next_state < HMMConfig::default().n_states);
        prop_assert!((0.0..=1.0 + 1e-9).contains(&next_prob));
    }
}

// ── Signal-engine aggregation ─────────────────────────────────────────────────

proptest! {
    // The full streaming stack is heavy (KMeans + Hurst recomputes); fewer
    // cases keep CI fast while still exploring the space.
    #![proptest_config(ProptestConfig::with_cases(24))]
    #[test]
    fn compute_signal_emits_only_valid_votes(cs in candles(10, 250)) {
        let cfg = IndicatorConfig::default();
        let mut ind = Indicators::new(cfg.clone());
        let mut liq = LiquidityProfile::new(50, 20);
        let mut conf = ConfluenceEngine::new(8, 21, 50, 14, 14);
        let mut ms = MarketStructure::new(5, 0.5);
        let mut cvd = CVDTracker::new(10, 20);
        let mut vol = VolatilityPercentile::new(100);

        for c in &cs {
            ind.update(c);
            liq.update(c);
            conf.update(c);
            ms.update(c);
            cvd.update(c);
            vol.update(ind.atr);

            let (signal, _components) = compute_signal(
                c.close, &ind, &liq, &conf, &ms, &cfg, Some(&cvd), Some(&vol),
            );
            prop_assert!((-1..=1).contains(&signal), "invalid signal: {signal}");
        }
    }
}
