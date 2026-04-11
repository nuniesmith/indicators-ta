//! Miscellaneous indicators that don't fit cleanly into trend/momentum/volume.
//!
//! Python equivalents: `indicators/other/`

pub mod chaikin_money_flow;
pub mod choppiness_index;
pub mod elder_ray_index;
pub mod linear_regression;
pub mod market_cycle;
pub mod parabolic_sar;
pub mod schaff_trend_cycle;
pub mod williams_r;

pub use chaikin_money_flow::ChaikinMoneyFlow;
pub use choppiness_index::ChoppinessIndex;
pub use elder_ray_index::ElderRayIndex;
pub use linear_regression::LinearRegression;
pub use market_cycle::MarketCycle;
pub use parabolic_sar::ParabolicSar;
pub use schaff_trend_cycle::SchaffTrendCycle;
pub use williams_r::WilliamsR;

use crate::registry::IndicatorRegistry;

pub fn register_all(reg: &IndicatorRegistry) {
    reg.register("chaikinmoneyflow",  chaikin_money_flow::factory);
    reg.register("choppinessindex",   choppiness_index::factory);
    reg.register("elderrayindex",     elder_ray_index::factory);
    reg.register("linearregression",  linear_regression::factory);
    reg.register("marketcycle",       market_cycle::factory);
    reg.register("parabolicsar",      parabolic_sar::factory);
    reg.register("schafftrendcycle",  schaff_trend_cycle::factory);
    reg.register("williamsr",         williams_r::factory);
}
