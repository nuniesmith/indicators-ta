//! Trend indicators.
//!
//! Python equivalents:
//! - `indicators/trend/moving_average.py`  → `sma`, `ema`, `wma`, `vwap`
//! - `indicators/trend/macd.py`            → `macd`
//! - `indicators/trend/accumulation_distribution_line.py` → `adl`
//! - `indicators/trend/volatility/`        → `volatility::{atr, bollinger, keltner}`

pub mod adl;
pub mod ema;
pub mod macd;
pub mod sma;
pub mod vwap;
pub mod volatility;
pub mod wma;

pub use adl::Adl;
pub use ema::Ema;
pub use macd::Macd;
pub use sma::Sma;
pub use vwap::Vwap;
pub use wma::Wma;
pub use volatility::{Atr, BollingerBands, KeltnerChannels};

use crate::registry::IndicatorRegistry;

/// Register all trend indicators into the global registry.
///
/// Called once from `crate::registry::registry()` at startup.
pub fn register_all(reg: &IndicatorRegistry) {
    reg.register("sma", sma::factory);
    reg.register("ema", ema::factory);
    reg.register("wma", wma::factory);
    reg.register("vwap", vwap::factory);
    reg.register("macd", macd::factory);
    reg.register("adl", adl::factory);
    reg.register("atr", volatility::atr::factory);
    reg.register("bollingerbands", volatility::bollinger::factory);
    reg.register("keltnerchannel", volatility::keltner::factory);
}
