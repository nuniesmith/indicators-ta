//! `BotSettings` — typed replacement for Python's `DEFAULT_SETTINGS_*` dicts.
//!
//! Every field maps 1-to-1 to a key in the Python `SETTINGS` dict so
//! Optuna-tuned JSON files can be loaded with zero field renaming.

use serde::{Deserialize, Serialize};

/// Per-symbol bot configuration — mirrors Python `DEFAULT_SETTINGS_BTC`.
///
/// Load from JSON with `serde_json::from_str`, or use a `BotSettings::btc()` constructor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotSettings {
    pub symbol: String,

    // ── Execution ────────────────────────────────────────────────────────────
    pub leverage: u32,
    pub contracts: u32,
    pub use_market_orders: bool,
    pub taker_fee: f64,
    pub maker_fee: f64,
    pub risk_fraction: f64,
    pub max_contracts: u32,
    pub sim_balance: f64,

    // ── Kline fetch ──────────────────────────────────────────────────────────
    pub kline_interval: String,
    pub rest_kline_type: String,
    pub history_candles: usize,
    pub backtest_candles: usize,

    // ── Layer 1-2: VWAP & EMA ────────────────────────────────────────────────
    pub ema_len: usize,

    // ── Layer 3: ML SuperTrend ───────────────────────────────────────────────
    pub atr_len: usize,
    pub st_factor: f64,
    pub training_period: usize,
    pub highvol_pct: f64,
    pub midvol_pct: f64,
    pub lowvol_pct: f64,

    // ── Layer 4: Trend Speed ─────────────────────────────────────────────────
    pub ts_max_length: usize,
    pub ts_accel_mult: f64,
    pub ts_rma_len: usize,
    pub ts_hma_len: usize,
    pub ts_collen: usize,
    pub ts_lookback: usize,
    pub ts_speed_exit_threshold: Option<f64>,

    // ── Layer 5: Liquidity Profile ───────────────────────────────────────────
    pub liq_period: usize,
    pub liq_bins: usize,

    // ── Layer 6: Confluence ──────────────────────────────────────────────────
    pub conf_ema_fast: usize,
    pub conf_ema_slow: usize,
    pub conf_ema_trend: usize,
    pub conf_rsi_len: usize,
    pub conf_adx_len: usize,
    pub conf_min_score: f64,

    // ── Layer 7: Market Structure + Fibonacci ────────────────────────────────
    pub struct_swing_len: usize,
    pub struct_atr_mult: f64,
    pub fib_zone_enabled: bool,

    // ── Signal mode ──────────────────────────────────────────────────────────
    /// `"majority"` | `"strict"` | `"any"`
    pub signal_mode: String,
    pub signal_confirm_bars: usize,

    // ── Layer 8: CVD ─────────────────────────────────────────────────────────
    pub cvd_slope_bars: usize,
    pub cvd_div_lookback: usize,

    // ── Layer 9: AO + Percentile gates ──────────────────────────────────────
    pub wave_pct_l: f64,
    pub wave_pct_s: f64,
    pub mom_pct_min: f64,
    pub vol_pct_window: usize,

    // ── Layer 10: Hurst ──────────────────────────────────────────────────────
    pub hurst_threshold: f64,
    pub hurst_lookback: usize,

    // ── Layer 11: Price Acceleration ─────────────────────────────────────────
    pub stop_atr_mult: f64,

    // ── Entry gates ──────────────────────────────────────────────────────────
    pub min_vol_pct: f64,
    pub min_hold_candles: usize,

    // ── Circuit breaker ──────────────────────────────────────────────────────
    pub breaker_loss_limit: u32,
    pub breaker_cooldown_sec: u32,

    // ── Live loop ────────────────────────────────────────────────────────────
    pub heartbeat_interval_sec: u32,
    pub param_watch_interval_sec: u32,
}

impl BotSettings {
    fn base() -> Self {
        Self {
            symbol: String::new(),
            leverage: 5,
            contracts: 1,
            use_market_orders: true,
            taker_fee: 0.0006,
            maker_fee: 0.0002,
            risk_fraction: 0.95,
            max_contracts: 50,
            sim_balance: 10_000.0,
            kline_interval: "1min".into(),
            rest_kline_type: "1".into(),
            history_candles: 200,
            backtest_candles: 8000,
            ema_len: 9,
            atr_len: 10,
            st_factor: 3.0,
            training_period: 100,
            highvol_pct: 0.75,
            midvol_pct: 0.50,
            lowvol_pct: 0.25,
            ts_max_length: 50,
            ts_accel_mult: 5.0,
            ts_rma_len: 10,
            ts_hma_len: 5,
            ts_collen: 100,
            ts_lookback: 50,
            ts_speed_exit_threshold: None,
            liq_period: 100,
            liq_bins: 31,
            conf_ema_fast: 9,
            conf_ema_slow: 21,
            conf_ema_trend: 55,
            conf_rsi_len: 13,
            conf_adx_len: 14,
            conf_min_score: 5.0,
            struct_swing_len: 10,
            struct_atr_mult: 0.5,
            fib_zone_enabled: true,
            signal_mode: "majority".into(),
            signal_confirm_bars: 2,
            cvd_slope_bars: 10,
            cvd_div_lookback: 30,
            wave_pct_l: 0.25,
            wave_pct_s: 0.75,
            mom_pct_min: 0.30,
            vol_pct_window: 200,
            hurst_threshold: 0.52,
            hurst_lookback: 20,
            stop_atr_mult: 1.5,
            min_vol_pct: 0.20,
            min_hold_candles: 2,
            breaker_loss_limit: 3,
            breaker_cooldown_sec: 900,
            heartbeat_interval_sec: 30,
            param_watch_interval_sec: 300,
        }
    }

    pub fn btc() -> Self {
        Self {
            symbol: "XBTUSDTM".into(),
            ..Self::base()
        }
    }

    pub fn eth() -> Self {
        Self {
            symbol: "ETHUSDTM".into(),
            ..Self::base()
        }
    }

    pub fn sol() -> Self {
        Self {
            symbol: "SOLUSDTM".into(),
            ..Self::base()
        }
    }

    pub fn by_symbol(symbol: &str) -> Self {
        match symbol {
            "XBTUSDTM" => Self::btc(),
            "ETHUSDTM" => Self::eth(),
            "SOLUSDTM" => Self::sol(),
            _ => Self {
                symbol: symbol.into(),
                ..Self::base()
            },
        }
    }
}

/// Contract multiplier (base asset per contract).
pub fn contract_value(symbol: &str) -> f64 {
    match symbol {
        "XBTUSDTM" => 0.001,
        "ETHUSDTM" => 0.01,
        "SOLUSDTM" => 1.0,
        _ => 0.001,
    }
}
