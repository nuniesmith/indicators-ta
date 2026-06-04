//! Standalone batch indicator functions and incremental structs.
//!
//! Ported from the original `indicators` crate lib. These work on slices
//! (batch mode) or as incremental O(1)-per-tick structs.

use std::collections::VecDeque;

use crate::error::IndicatorError;
use crate::types::MacdResult;

// ── Batch functions ───────────────────────────────────────────────────────────

/// Exponential Moving Average over a price slice.
/// Returns a Vec of the same length; leading values are `NaN` until warm-up.
pub fn ema(prices: &[f64], period: usize) -> Result<Vec<f64>, IndicatorError> {
    if period == 0 {
        return Err(IndicatorError::InvalidParameter {
            name: "period".into(),
            value: 0.0,
        });
    }
    if prices.len() < period {
        return Err(IndicatorError::InsufficientData {
            required: period,
            available: prices.len(),
        });
    }
    let mut result = vec![f64::NAN; prices.len()];
    let alpha = 2.0 / (period as f64 + 1.0);
    let first_sma: f64 = prices.iter().take(period).sum::<f64>() / period as f64;
    result[period - 1] = first_sma;
    for i in period..prices.len() {
        result[i] = prices[i] * alpha + result[i - 1] * (1.0 - alpha);
    }
    Ok(result)
}

/// EMA that handles leading NaN values, matching Python's `ewm(adjust=False)` behaviour.
///
/// Unlike [`ema`], which seeds from the arithmetic mean of the first `period`
/// values, this function seeds from the **first non-NaN value** and applies
/// the recursive formula from that point on.  All positions before the first
/// non-NaN value are left as `NaN`.
///
/// This is needed wherever EMA is applied to a derived series (e.g. the MACD
/// line) that already has a leading NaN warm-up period.  Using the standard
/// [`ema`] on such a series would propagate NaN through the SMA seed and
/// produce an all-NaN output.
pub fn ema_nan_aware(prices: &[f64], period: usize) -> Result<Vec<f64>, IndicatorError> {
    if period == 0 {
        return Err(IndicatorError::InvalidParameter {
            name: "period".into(),
            value: 0.0,
        });
    }
    let mut result = vec![f64::NAN; prices.len()];
    let alpha = 2.0 / (period as f64 + 1.0);

    // Seed from the first non-NaN value (adjust=False, no SMA warm-up).
    let Some(start) = prices.iter().position(|v| !v.is_nan()) else {
        return Ok(result); // all NaN — nothing to compute
    };

    result[start] = prices[start];
    for i in (start + 1)..prices.len() {
        result[i] = if prices[i].is_nan() {
            f64::NAN
        } else {
            prices[i] * alpha + result[i - 1] * (1.0 - alpha)
        };
    }
    Ok(result)
}

/// Simple Moving Average over a price slice.
pub fn sma(prices: &[f64], period: usize) -> Result<Vec<f64>, IndicatorError> {
    if period == 0 {
        return Err(IndicatorError::InvalidParameter {
            name: "period".into(),
            value: 0.0,
        });
    }
    if prices.len() < period {
        return Err(IndicatorError::InsufficientData {
            required: period,
            available: prices.len(),
        });
    }
    let mut result = vec![f64::NAN; prices.len()];
    for i in (period - 1)..prices.len() {
        let sum: f64 = prices[(i + 1 - period)..=i].iter().sum();
        result[i] = sum / period as f64;
    }
    Ok(result)
}

/// True Range = max(H-L, |H-prevC|, |L-prevC|).
pub fn true_range(high: &[f64], low: &[f64], close: &[f64]) -> Result<Vec<f64>, IndicatorError> {
    if high.len() != low.len() || high.len() != close.len() {
        return Err(IndicatorError::InsufficientData {
            required: high.len(),
            available: low.len().min(close.len()),
        });
    }
    let mut result = vec![f64::NAN; high.len()];
    if !high.is_empty() {
        result[0] = high[0] - low[0];
    }
    for i in 1..high.len() {
        let tr1 = high[i] - low[i];
        let tr2 = (high[i] - close[i - 1]).abs();
        let tr3 = (low[i] - close[i - 1]).abs();
        result[i] = tr1.max(tr2).max(tr3);
    }
    Ok(result)
}

/// Average True Range (EMA-smoothed).
pub fn atr(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    period: usize,
) -> Result<Vec<f64>, IndicatorError> {
    let tr = true_range(high, low, close)?;
    ema(&tr, period)
}

/// Relative Strength Index.
pub fn rsi(prices: &[f64], period: usize) -> Result<Vec<f64>, IndicatorError> {
    if prices.len() < period + 1 {
        return Err(IndicatorError::InsufficientData {
            required: period + 1,
            available: prices.len(),
        });
    }
    let mut result = vec![f64::NAN; prices.len()];
    let mut gains = vec![0.0; prices.len()];
    let mut losses = vec![0.0; prices.len()];
    for i in 1..prices.len() {
        let change = prices[i] - prices[i - 1];
        if change > 0.0 {
            gains[i] = change;
        } else {
            losses[i] = -change;
        }
    }
    let avg_gains = ema(&gains, period)?;
    let avg_losses = ema(&losses, period)?;
    for i in period..prices.len() {
        if avg_losses[i] == 0.0 {
            result[i] = 100.0;
        } else {
            let rs = avg_gains[i] / avg_losses[i];
            result[i] = 100.0 - (100.0 / (1.0 + rs));
        }
    }
    Ok(result)
}

/// MACD — returns (macd_line, signal_line, histogram).
pub fn macd(
    prices: &[f64],
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
) -> MacdResult {
    // Use ema_nan_aware to match Python's ewm(span=X, adjust=False), which
    // seeds from the first value rather than an SMA of the first `period` bars.
    let fast_ema = ema_nan_aware(prices, fast_period)?;
    let slow_ema = ema_nan_aware(prices, slow_period)?;
    let mut macd_line = vec![f64::NAN; prices.len()];
    for i in 0..prices.len() {
        if !fast_ema[i].is_nan() && !slow_ema[i].is_nan() {
            macd_line[i] = fast_ema[i] - slow_ema[i];
        }
    }
    // The macd_line has leading NaN (warm-up from the slow EMA); use the
    // NaN-aware variant so the signal seeds from the first valid MACD value
    // rather than an all-NaN SMA, matching Python's ewm(adjust=False).
    let signal_line = ema_nan_aware(&macd_line, signal_period)?;
    let mut histogram = vec![f64::NAN; prices.len()];
    for i in 0..prices.len() {
        if !macd_line[i].is_nan() && !signal_line[i].is_nan() {
            histogram[i] = macd_line[i] - signal_line[i];
        }
    }
    Ok((macd_line, signal_line, histogram))
}

// ── Incremental structs ───────────────────────────────────────────────────────

/// Incremental EMA — O(1) update, SMA warm-up.
///
/// Unlike the batch [`ema`] function (which initialises from an SMA over the
/// first `period` prices), this struct emits its first value *after* it has
/// accumulated exactly `period` samples and seeds itself from their average.
/// Both approaches are correct; this one is more natural for streaming use.
#[derive(Debug, Clone)]
pub struct EMA {
    period: usize,
    alpha: f64,
    value: f64,
    initialized: bool,
    warmup: VecDeque<f64>,
}

impl EMA {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            alpha: 2.0 / (period as f64 + 1.0),
            value: 0.0,
            initialized: false,
            warmup: VecDeque::with_capacity(period),
        }
    }

    pub fn update(&mut self, price: f64) {
        if !self.initialized {
            self.warmup.push_back(price);
            if self.warmup.len() >= self.period {
                self.value = self.warmup.iter().sum::<f64>() / self.period as f64;
                self.initialized = true;
                self.warmup.clear();
            }
        } else {
            self.value = price * self.alpha + self.value * (1.0 - self.alpha);
        }
    }

    pub fn value(&self) -> f64 {
        if self.initialized {
            self.value
        } else {
            f64::NAN
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn reset(&mut self) {
        self.value = 0.0;
        self.initialized = false;
        self.warmup.clear();
    }
}

/// Incremental Wilder ATR.
#[derive(Debug, Clone)]
pub struct ATR {
    #[allow(dead_code)]
    period: usize,
    ema: EMA,
    prev_close: Option<f64>,
}

impl ATR {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            ema: EMA::new(period),
            prev_close: None,
        }
    }

    pub fn update(&mut self, high: f64, low: f64, close: f64) {
        let tr = if let Some(prev) = self.prev_close {
            (high - low)
                .max((high - prev).abs())
                .max((low - prev).abs())
        } else {
            high - low
        };
        self.ema.update(tr);
        self.prev_close = Some(close);
    }

    pub fn value(&self) -> f64 {
        self.ema.value()
    }

    pub fn is_ready(&self) -> bool {
        self.ema.is_ready()
    }
}

/// Bundle of per-strategy indicator series.
#[derive(Debug, Clone)]
pub struct StrategyIndicators {
    pub ema_fast: Vec<f64>,
    pub ema_slow: Vec<f64>,
    pub atr: Vec<f64>,
}

/// Multi-period indicator calculator (batch mode).
#[derive(Debug, Clone)]
pub struct IndicatorCalculator {
    pub fast_ema_period: usize,
    pub slow_ema_period: usize,
    pub atr_period: usize,
}

impl Default for IndicatorCalculator {
    fn default() -> Self {
        Self {
            fast_ema_period: 8,
            slow_ema_period: 21,
            atr_period: 14,
        }
    }
}

impl IndicatorCalculator {
    pub fn new(fast_ema: usize, slow_ema: usize, atr_period: usize) -> Self {
        Self {
            fast_ema_period: fast_ema,
            slow_ema_period: slow_ema,
            atr_period,
        }
    }

    pub fn calculate_all(
        &self,
        close: &[f64],
        high: &[f64],
        low: &[f64],
    ) -> Result<StrategyIndicators, IndicatorError> {
        Ok(StrategyIndicators {
            ema_fast: ema(close, self.fast_ema_period)?,
            ema_slow: ema(close, self.slow_ema_period)?,
            atr: atr(high, low, close, self.atr_period)?,
        })
    }
}

/// Incremental EMA — O(1) update per tick that returns the new value each call.
///
/// Unlike [`EMA`] (which separates `update` from `value`/`is_ready`), this
/// seeds from the first sample and returns the EMA on every `update`, which
/// suits streaming pipelines that consume the value inline.
#[derive(Debug, Clone)]
pub struct IncrementalEma {
    alpha: f64,
    state: f64,
    initialized: bool,
}

impl IncrementalEma {
    /// Create an incremental EMA for the given period.
    pub fn new(period: usize) -> Self {
        Self {
            alpha: 2.0 / (period as f64 + 1.0),
            state: 0.0,
            initialized: false,
        }
    }

    /// Feed the next price; returns the updated EMA (seeds from the first price).
    pub fn update(&mut self, price: f64) -> f64 {
        if !self.initialized {
            self.state = price;
            self.initialized = true;
        } else {
            self.state = self.alpha * price + (1.0 - self.alpha) * self.state;
        }
        self.state
    }

    /// Current EMA value, or `None` before the first `update`.
    pub fn current(&self) -> Option<f64> {
        if self.initialized {
            Some(self.state)
        } else {
            None
        }
    }
}

/// Incremental ATR — O(1) per-tick true-range EMA.
///
/// Wraps an [`IncrementalEma`] over the true range and returns the smoothed
/// ATR on each `update`. The first sample's true range is `high - low`.
pub struct IncrementalAtr {
    ema: IncrementalEma,
    prev_close: Option<f64>,
}

impl IncrementalAtr {
    /// Create an incremental ATR for the given period.
    pub fn new(period: usize) -> Self {
        Self {
            ema: IncrementalEma::new(period),
            prev_close: None,
        }
    }

    /// Feed the next high/low/close; returns the updated ATR.
    pub fn update(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        let tr = if let Some(prev) = self.prev_close {
            let tr1 = high - low;
            let tr2 = (high - prev).abs();
            let tr3 = (low - prev).abs();
            tr1.max(tr2).max(tr3)
        } else {
            high - low
        };

        self.prev_close = Some(close);
        Some(self.ema.update(tr))
    }
}

/// Incremental RSI — O(1) per-tick, EMA-smoothed gains/losses.
///
/// Mirrors the batch [`rsi`]: the average gain and loss are tracked with an
/// [`IncrementalEma`] of the given period and combined as
/// `100 - 100 / (1 + avg_gain / avg_loss)` (RSI is 100 when the average loss is
/// zero). Like the other incremental structs it seeds from the first sample
/// (vs. the batch SMA warm-up), so warm-up values differ slightly from [`rsi`]
/// but converge — both are valid.
#[derive(Debug, Clone)]
pub struct IncrementalRsi {
    gain_ema: IncrementalEma,
    loss_ema: IncrementalEma,
    prev_price: Option<f64>,
}

impl IncrementalRsi {
    /// Create an incremental RSI for the given period.
    pub fn new(period: usize) -> Self {
        Self {
            gain_ema: IncrementalEma::new(period),
            loss_ema: IncrementalEma::new(period),
            prev_price: None,
        }
    }

    /// Feed the next price; returns the updated RSI, or `None` for the very
    /// first sample (RSI needs a prior price to form the first change).
    pub fn update(&mut self, price: f64) -> Option<f64> {
        let prev = self.prev_price.replace(price)?;
        let change = price - prev;
        let (gain, loss) = if change > 0.0 {
            (change, 0.0)
        } else {
            (0.0, -change)
        };
        let avg_gain = self.gain_ema.update(gain);
        let avg_loss = self.loss_ema.update(loss);
        let rsi = if avg_loss == 0.0 {
            100.0
        } else {
            let rs = avg_gain / avg_loss;
            100.0 - 100.0 / (1.0 + rs)
        };
        Some(rsi)
    }
}

/// Incremental MACD — O(1) per-tick MACD line, signal, and histogram.
///
/// Mirrors the batch [`macd`]: a fast and slow [`IncrementalEma`] give the MACD
/// line (`fast - slow`), a third EMA over that line is the signal, and the
/// histogram is `macd - signal`. The EMAs seed from the first sample (matching
/// the batch's `adjust=false` / first-value seeding via `ema_nan_aware`), so
/// values are emitted from the first tick rather than after a NaN warm-up.
#[derive(Debug, Clone)]
pub struct IncrementalMacd {
    fast: IncrementalEma,
    slow: IncrementalEma,
    signal: IncrementalEma,
}

impl IncrementalMacd {
    /// Create an incremental MACD from the fast, slow, and signal periods.
    pub fn new(fast_period: usize, slow_period: usize, signal_period: usize) -> Self {
        Self {
            fast: IncrementalEma::new(fast_period),
            slow: IncrementalEma::new(slow_period),
            signal: IncrementalEma::new(signal_period),
        }
    }

    /// Feed the next price; returns `(macd, signal, histogram)`.
    pub fn update(&mut self, price: f64) -> (f64, f64, f64) {
        let macd = self.fast.update(price) - self.slow.update(price);
        let signal = self.signal.update(macd);
        (macd, signal, macd - signal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_incremental_rsi_first_sample_is_none() {
        let mut rsi = IncrementalRsi::new(14);
        assert_eq!(rsi.update(10.0), None);
        assert!(rsi.update(11.0).is_some());
    }

    #[test]
    fn test_incremental_rsi_all_gains_saturates_at_100() {
        let mut rsi = IncrementalRsi::new(14);
        let mut last = None;
        for p in [10.0, 11.0, 12.0, 13.0, 14.0, 15.0] {
            last = rsi.update(p);
        }
        // Monotonically rising → average loss is zero → RSI saturates at 100.
        assert!((last.unwrap() - 100.0).abs() < 1e-9);
    }

    #[test]
    fn test_incremental_rsi_stays_in_bounds() {
        let mut rsi = IncrementalRsi::new(5);
        let prices = [44.0, 44.3, 44.1, 44.2, 43.6, 44.3, 44.8, 45.0, 44.7, 44.9];
        let mut produced = 0;
        for p in prices {
            if let Some(v) = rsi.update(p) {
                assert!((0.0..=100.0).contains(&v), "RSI out of bounds: {v}");
                produced += 1;
            }
        }
        assert_eq!(produced, prices.len() - 1);
    }

    #[test]
    fn test_incremental_macd_composes_like_batch() {
        let mut m = IncrementalMacd::new(12, 26, 9);
        let mut fast = IncrementalEma::new(12);
        let mut slow = IncrementalEma::new(26);
        let mut sig = IncrementalEma::new(9);
        for p in [10.0, 11.0, 10.5, 12.0, 13.0, 12.5, 11.0, 11.5] {
            let (macd, signal, hist) = m.update(p);
            let expect_macd = fast.update(p) - slow.update(p);
            let expect_sig = sig.update(expect_macd);
            assert!((macd - expect_macd).abs() < 1e-12);
            assert!((signal - expect_sig).abs() < 1e-12);
            assert!((hist - (expect_macd - expect_sig)).abs() < 1e-12);
        }
    }

    #[test]
    fn test_ema_sma_seed() {
        let prices = vec![22.27, 22.19, 22.08, 22.17, 22.18];
        let result = ema(&prices, 5).unwrap();
        let expected = (22.27 + 22.19 + 22.08 + 22.17 + 22.18) / 5.0;
        assert!((result[4] - expected).abs() < 1e-9);
    }

    #[test]
    fn test_true_range_first() {
        let h = vec![50.0, 52.0];
        let l = vec![48.0, 49.0];
        let c = vec![49.0, 51.0];
        let tr = true_range(&h, &l, &c).unwrap();
        assert_eq!(tr[0], 2.0);
        assert_eq!(tr[1], 3.0);
    }

    #[test]
    fn test_ema_incremental() {
        let mut e = EMA::new(3);
        e.update(10.0);
        assert!(!e.is_ready());
        e.update(20.0);
        assert!(!e.is_ready());
        e.update(30.0);
        assert!(e.is_ready());
        assert!((e.value() - 20.0).abs() < 1e-9);
    }

    #[test]
    fn test_incremental_ema_returns_value() {
        let mut e = IncrementalEma::new(3); // alpha = 0.5
        assert_eq!(e.current(), None);
        assert_eq!(e.update(10.0), 10.0); // seeds from first sample
        assert_eq!(e.current(), Some(10.0));
        let v = e.update(20.0); // 0.5*20 + 0.5*10
        assert!((v - 15.0).abs() < 1e-9);
    }

    #[test]
    fn test_incremental_atr_first_is_range() {
        let mut a = IncrementalAtr::new(3);
        // First sample: TR = high - low, EMA seeds to it.
        assert_eq!(a.update(12.0, 10.0, 11.0), Some(2.0));
    }
}
