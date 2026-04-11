//! Signal pipeline — all 11 indicator layers and final signal aggregation.
//!
//! The layers are organised as follows:
//!
//! | Layer | Module | Type |
//! |-------|--------|------|
//! | L1 VWAP + L2 EMA + L3 SuperTrend + L4 Trend Speed + L9 AO + L10 Hurst + L11 Accel | [`engine`] | [`Indicators`] |
//! | L5 Liquidity Profile | [`liquidity`] | [`LiquidityProfile`] |
//! | L6 Confluence | [`confluence`] | [`ConfluenceEngine`] |
//! | L7 Market Structure + Fibonacci | [`structure`] | [`MarketStructure`] |
//! | L8 CVD | [`cvd`] | [`CVDTracker`] |
//! | Aggregator | [`signal`] | [`compute_signal`] · [`SignalStreak`] |
//! | Regime helpers | [`vol_regime`] | [`VolatilityPercentile`] · [`PercentileTracker`] |

pub mod confluence;
pub mod cvd;
pub mod engine;
pub mod liquidity;
pub mod signal;
pub mod structure;
pub mod vol_regime;

pub use confluence::ConfluenceEngine;
pub use cvd::CVDTracker;
pub use engine::Indicators;
pub use liquidity::LiquidityProfile;
pub use signal::{SignalComponents, SignalStreak, compute_signal};
pub use structure::MarketStructure;
pub use vol_regime::{MarketRegimeTracker, PercentileTracker, VolatilityPercentile};
