//! Schaff Trend Cycle (STC).
//!
//! Python source: `indicators/other/schaff_trend_cycle.py :: class SchaffTrendCycle`
//!
//! # Python algorithm (to port)
//! ```python
//! short_ema  = df["Close"].ewm(span=self.short_ema).mean()
//! long_ema   = df["Close"].ewm(span=self.long_ema).mean()
//! macd       = short_ema - long_ema
//! macd_sig   = macd.ewm(span=9).mean()
//! macd_diff  = macd - macd_sig
//!
//! lowest  = macd_diff.rolling(self.stoch_period).min()
//! highest = macd_diff.rolling(self.stoch_period).max()
//! stc     = 100 * (macd_diff - lowest) / (highest - lowest)
//!
//! if self.signal_period > 0:
//!     stc = stc.ewm(span=self.signal_period).mean()
//! ```
//!
//! Readings above 75 → overbought; below 25 → oversold.
//! Oscillates 0–100.
//!
//! Output column: `"STC"`.

use std::collections::HashMap;

use crate::error::IndicatorError;
use crate::functions::{self};
use crate::indicator::{Indicator, IndicatorOutput};
use crate::registry::param_usize;
use crate::types::Candle;

#[derive(Debug, Clone)]
pub struct StcParams {
    pub short_ema: usize,
    pub long_ema: usize,
    pub stoch_period: usize,
    pub signal_period: usize,
}
impl Default for StcParams {
    fn default() -> Self {
        Self {
            short_ema: 12,
            long_ema: 26,
            stoch_period: 10,
            signal_period: 3,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SchaffTrendCycle {
    pub params: StcParams,
}

impl SchaffTrendCycle {
    pub fn new(params: StcParams) -> Self {
        Self { params }
    }
    pub fn default() -> Self {
        Self::new(StcParams::default())
    }
}

impl Indicator for SchaffTrendCycle {
    fn name(&self) -> &str {
        "SchaffTrendCycle"
    }

    fn required_len(&self) -> usize {
        self.params.long_ema + self.params.stoch_period + self.params.signal_period
    }

    fn required_columns(&self) -> &[&'static str] {
        &["close"]
    }

    /// TODO: port Python MACD-then-Stochastic-then-EMA pipeline.
    fn calculate(&self, candles: &[Candle]) -> Result<IndicatorOutput, IndicatorError> {
        self.check_len(candles)?;

        let close: Vec<f64> = candles.iter().map(|c| c.close).collect();
        let n = close.len();

        // Step 1: MACD components.
        let short_e = functions::ema(&close, self.params.short_ema)?;
        let long_e = functions::ema(&close, self.params.long_ema)?;
        let macd_line: Vec<f64> = (0..n)
            .map(|i| {
                if short_e[i].is_nan() || long_e[i].is_nan() {
                    f64::NAN
                } else {
                    short_e[i] - long_e[i]
                }
            })
            .collect();

        // Signal of MACD (span=9).
        let macd_sig = functions::ema(&macd_line, 9)?;
        let macd_diff: Vec<f64> = (0..n)
            .map(|i| {
                if macd_line[i].is_nan() || macd_sig[i].is_nan() {
                    f64::NAN
                } else {
                    macd_line[i] - macd_sig[i]
                }
            })
            .collect();

        // Step 2: Stochastic of MACD diff.
        let sp = self.params.stoch_period;
        let mut stc = vec![f64::NAN; n];
        for i in (sp - 1)..n {
            let window = &macd_diff[(i + 1 - sp)..=i];
            let min_d = window.iter().cloned().fold(f64::INFINITY, f64::min);
            let max_d = window.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let range = max_d - min_d;
            if macd_diff[i].is_nan() || range == 0.0 {
                stc[i] = f64::NAN;
            } else {
                stc[i] = 100.0 * (macd_diff[i] - min_d) / range;
            }
        }

        // Step 3: optional EMA smoothing.
        let values = if self.params.signal_period > 0 {
            functions::ema(&stc, self.params.signal_period)?
        } else {
            stc
        };

        Ok(IndicatorOutput::from_pairs([("STC".to_string(), values)]))
    }
}

pub fn factory(params: &HashMap<String, String>) -> Result<Box<dyn Indicator>, IndicatorError> {
    Ok(Box::new(SchaffTrendCycle::new(StcParams {
        short_ema: param_usize(params, "short_ema", 12)?,
        long_ema: param_usize(params, "long_ema", 26)?,
        stoch_period: param_usize(params, "stoch_period", 10)?,
        signal_period: param_usize(params, "signal_period", 3)?,
    })))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn candles(n: usize) -> Vec<Candle> {
        (0..n)
            .map(|i| Candle {
                time: i as i64,
                open: 10.0,
                high: 10.0 + (i % 5) as f64,
                low: 10.0 - (i % 3) as f64,
                close: 10.0 + (i as f64).sin(),
                volume: 100.0,
            })
            .collect()
    }

    #[test]
    fn stc_output_column() {
        let p = StcParams::default();
        let needed = p.long_ema + p.stoch_period + p.signal_period + 5;
        let out = SchaffTrendCycle::default()
            .calculate(&candles(needed))
            .unwrap();
        assert!(out.get("STC").is_some());
    }

    #[test]
    fn factory_creates_stc() {
        assert_eq!(factory(&HashMap::new()).unwrap().name(), "SchaffTrendCycle");
    }
}
