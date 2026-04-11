//! Statistical market regime detection.
//!
//! Three detection approaches, from fastest to most robust:
//!
//! | Detector | Module | Description |
//! |----------|--------|-------------|
//! | Indicator-based | [`detector`] | ADX + Bollinger Bands + ATR — immediate, rule-based |
//! | HMM | [`hmm`] | Hidden Markov Model — learns regime distributions from returns |
//! | Ensemble | [`ensemble`] | Combines both — recommended for production use |
//!
//! The [`router`] module routes to strategies based on the detected regime.
//!
//! Internal indicator primitives (EMA, ATR, ADX, Bollinger Bands) used by the
//! detectors live in [`primitives`]. These are intentionally separate from the
//! batch `Indicator` implementations in `crate::volatility` — they are
//! incremental accumulators optimised for the regime detection loop.

pub mod detector;
pub mod ensemble;
pub mod hmm;
pub mod primitives;
pub mod router;
/// Re-exports of crate-level types consumed by regime detector internals.
pub(crate) mod types;

pub use detector::RegimeDetector;
pub use ensemble::{EnsembleConfig, EnsembleRegimeDetector, EnsembleResult, EnsembleStatus};
pub use hmm::{HMMConfig, HMMRegimeDetector};
pub use primitives::{ADX, BollingerBands, BollingerBandsValues, RSI};
pub use router::{
    ActiveStrategy, AssetSummary, DetectionMethod, EnhancedRouter, EnhancedRouterConfig,
    RoutedSignal,
};
