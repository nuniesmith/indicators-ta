//! Momentum indicators: RSI, Stochastic, StochasticRSI.

pub mod rsi;
pub mod stochastic;
pub mod stochastic_rsi;

pub use rsi::Rsi;
pub use stochastic::Stochastic;
pub use stochastic_rsi::StochasticRsi;

use crate::registry::IndicatorRegistry;

pub fn register_all(reg: &IndicatorRegistry) {
    reg.register("rsi",        rsi::factory);
    reg.register("stochastic", stochastic::factory);
    reg.register("stochrsi",   stochastic_rsi::factory);
}
