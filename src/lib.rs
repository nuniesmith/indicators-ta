//! # indicators — Technical Indicators + Market Regime Detection
//!
//! Unified crate combining:
//! - **Indicator categories**: [`trend`], [`momentum`], [`volume`], [`volatility`]
//! - **Signal pipeline**: [`signal`] — VWAP, EMA, ML SuperTrend, Confluence, Liquidity, Structure, CVD
//! - **Regime detection**: [`regime`] — Indicator-based, HMM, and Ensemble detectors
//! - **Standalone batch functions**: [`functions`] — `ema()`, `sma()`, `atr()`, `rsi()`, `macd()`
//!
//! ## Quick start — batch indicators
//! ```rust,ignore
//! use indicators::{IndicatorRegistry, trend, momentum};
//!
//! // Build a registry with all indicators
//! let reg = IndicatorRegistry::default();
//! trend::register_all(&reg);
//! momentum::register_all(&reg);
//!
//! // Or use typed structs directly
//! use indicators::trend::Sma;
//! let sma = Sma::with_period(20);
//! let output = sma.calculate(&candles)?;
//! println!("{:?}", output.latest("SMA_20"));
//! ```
//!
//! ## Quick start — signal engine
//! ```rust,ignore
//! use indicators::{Indicators, ConfluenceEngine, LiquidityProfile, MarketStructure,
//!                  CVDTracker, VolatilityPercentile, SignalStreak, compute_signal};
//!
//! let mut ind    = Indicators::new(&cfg);
//! let mut liq    = LiquidityProfile::new(cfg.liq_period, cfg.liq_bins);
//! let mut conf   = ConfluenceEngine::new(cfg.conf_ema_fast, cfg.conf_ema_slow,
//!                                        cfg.conf_ema_trend, cfg.conf_rsi_len, cfg.conf_adx_len);
//! let mut ms     = MarketStructure::new(cfg.struct_swing_len, cfg.struct_atr_mult);
//! let mut cvd    = CVDTracker::new(cfg.cvd_slope_bars, cfg.cvd_div_lookback);
//! let mut vol    = VolatilityPercentile::new(200);
//! let mut streak = SignalStreak::new(cfg.signal_confirm_bars);
//!
//! // per-candle update
//! ind.update(&candle);
//! liq.update(&candle);
//! conf.update(&candle);
//! ms.update(&candle);
//! cvd.update(&candle);
//! vol.update(ind.atr);
//! let (raw, _) = compute_signal(candle.close, &ind, &liq, &conf, &ms, &cfg, Some(&cvd), Some(&vol));
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

// ── Core types & traits ───────────────────────────────────────────────────────
pub mod functions;
pub mod indicator;
pub mod indicator_config;
pub mod registry;
pub mod types;

// ── Indicator categories ──────────────────────────────────────────────────────
pub mod momentum;
pub mod trend;
pub mod volatility;
pub mod volume;

// ── Signal pipeline ───────────────────────────────────────────────────────────
pub mod signal;

// ── Regime detection ─────────────────────────────────────────────────────────
pub mod regime;

// ── Re-exports: core ─────────────────────────────────────────────────────────
pub use functions::{ATR, EMA, IndicatorCalculator, StrategyIndicators};
pub use functions::{IndicatorError, atr, ema, macd, rsi, sma, true_range};
pub use indicator::{Indicator, IndicatorOutput, PriceColumn};
pub use indicator_config::IndicatorConfig;
pub use registry::IndicatorRegistry;
pub use types::{
    Candle, MarketRegime, RecommendedStrategy, RegimeConfidence, RegimeConfig, TrendDirection,
};

// ── Re-exports: momentum ─────────────────────────────────────────────────────
pub use momentum::{Rsi, Stochastic, StochasticRsi};

// ── Re-exports: signal pipeline ──────────────────────────────────────────────
pub use signal::CVDTracker;
pub use signal::ConfluenceEngine;
pub use signal::Indicators;
pub use signal::LiquidityProfile;
pub use signal::MarketStructure;
pub use signal::{MarketRegimeTracker, PercentileTracker, VolatilityPercentile};
pub use signal::{SignalComponents, SignalStreak, compute_signal};

// ── Re-exports: regime detection ─────────────────────────────────────────────
pub use regime::RegimeDetector;
/// Internal Bollinger Bands used by the regime detector (incremental, not batch).
/// For the batch `Indicator` impl see [`volatility::BollingerBands`].
pub use regime::{ADX, BollingerBands, BollingerBandsValues, RSI};
pub use regime::{
    ActiveStrategy, AssetSummary, DetectionMethod, EnhancedRouter, EnhancedRouterConfig,
    RoutedSignal,
};
pub use regime::{EnsembleConfig, EnsembleRegimeDetector, EnsembleResult, EnsembleStatus};
pub use regime::{HMMConfig, HMMRegimeDetector};
