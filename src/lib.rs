//! # indicators — Technical Indicators + Market Regime Detection
//!
//! Unified crate combining:
//! - **Signal layers** (ported from `indicators.py`): VWAP, EMA, ML SuperTrend, Trend Speed,
//!   Liquidity Profile, Confluence Engine, Market Structure, CVD, AO, Hurst, Price Acceleration.
//! - **Regime detection**: Indicator-based, HMM, and Ensemble detectors.
//! - **Standalone batch functions**: `ema()`, `sma()`, `atr()`, `rsi()`, `macd()`.
//!
//! ## Quick start — signal engine
//! ```rust,ignore
//! use indicators::{Indicators, ConfluenceEngine, LiquidityProfile, MarketStructure,
//!                  CVDTracker, VolatilityPercentile, SignalStreak, compute_signal};
//!
//! let mut ind  = Indicators::new(&s);
//! let mut liq  = LiquidityProfile::new(s.liq_period, s.liq_bins);
//! let mut conf = ConfluenceEngine::new(s.conf_ema_fast, s.conf_ema_slow,
//!                                      s.conf_ema_trend, s.conf_rsi_len, s.conf_adx_len);
//! let mut ms   = MarketStructure::new(s.struct_swing_len, s.struct_atr_mult);
//! let mut cvd  = CVDTracker::new(s.cvd_slope_bars, s.cvd_div_lookback);
//! let mut vol  = VolatilityPercentile::new(200);
//! let mut streak = SignalStreak::new(s.signal_confirm_bars);
//!
//! // per-candle update
//! ind.update(&candle);
//! liq.update(&candle);
//! conf.update(&candle);
//! ms.update(&candle);
//! cvd.update(&candle);
//! vol.update(ind.atr);
//! let (raw, _) = compute_signal(candle.close, &ind, &liq, &conf, &ms, &s, Some(&cvd), Some(&vol));
//! if streak.update(raw) { /* confirmed signal */ }
//! ```
//!
//! ## Quick start — regime detection
//! ```rust,ignore
//! use indicators::EnsembleRegimeDetector;
//! let mut det = EnsembleRegimeDetector::default_config();
//! det.update(high, low, close);
//! if det.is_ready() { println!("{}", det.regime()); }
//! ```

// ── Standalone batch indicator functions ─────────────────────────────────────
pub mod functions;

// ── Indicator trait system ────────────────────────────────────────────────────
pub mod indicator;
pub mod indicator_config;
pub mod registry;

// ── Grouped indicator implementations ────────────────────────────────────────
pub mod momentum;
pub mod trend;
pub mod volume;
pub mod other;

// ── Signal aggregator (moved from kucoin-futures) ─────────────────────────────
pub mod signal;

// ── Python-ported signal engine ───────────────────────────────────────────────
pub mod confluence; // ConfluenceEngine (Layer 6)
pub mod cvd; // CVDTracker (Layer 8)
pub mod engine; // Indicators: VWAP, EMA, SuperTrend, TrendSpeed, Hurst, Accel
pub mod liquidity; // LiquidityProfile (Layer 5)
pub mod structure; // MarketStructure + Fibonacci (Layer 7)
pub mod vol_regime; // PercentileTracker, VolatilityPercentile, MarketRegimeTracker // compute_signal, SignalStreak

// ── Regime detection system ───────────────────────────────────────────────────
mod detector;
mod ensemble;
mod hmm;
pub mod primitives; // ADX, BB, EMA, ATR, RSI used by regime detectors
pub mod router;
pub mod types; // MarketRegime enum, RegimeConfidence, RegimeConfig, etc.

// ── Re-exports: indicator trait + config ────────────────────────────────────
pub use indicator::{Indicator, IndicatorOutput, PriceColumn};
pub use indicator_config::IndicatorConfig;
pub use registry::IndicatorRegistry;

// ── Re-exports: momentum ─────────────────────────────────────────────────────
pub use momentum::{Rsi, Stochastic, StochasticRsi};

// ── Re-exports: signal ───────────────────────────────────────────────────────
pub use signal::{SignalComponents, SignalStreak, compute_signal};
pub use confluence::ConfluenceEngine;
pub use cvd::CVDTracker;
pub use engine::Indicators;
pub use liquidity::LiquidityProfile;
pub use structure::MarketStructure;
pub use vol_regime::{MarketRegimeTracker, PercentileTracker, VolatilityPercentile};

// ── Re-exports: batch functions ──────────────────────────────────────────────
pub use functions::{IndicatorError, atr, ema, macd, rsi, sma, true_range};

// ── Re-exports: incremental structs ─────────────────────────────────────────
pub use functions::{ATR, EMA, IndicatorCalculator, StrategyIndicators};

// ── Re-exports: regime detection ────────────────────────────────────────────
pub use detector::RegimeDetector;
pub use ensemble::{EnsembleConfig, EnsembleRegimeDetector, EnsembleResult, EnsembleStatus};
pub use hmm::{HMMConfig, HMMRegimeDetector};
pub use primitives::{ADX, BollingerBands, BollingerBandsValues, RSI};
pub use router::{
    ActiveStrategy, AssetSummary, DetectionMethod, EnhancedRouter, EnhancedRouterConfig,
    RoutedSignal,
};
pub use types::{
    MarketRegime, RecommendedStrategy, RegimeConfidence, RegimeConfig, TrendDirection,
};
