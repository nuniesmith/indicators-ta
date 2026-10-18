//! Simple-moving-average band with hysteresis: a long-term trend regime.
//!
//! The regime turns bull once the close rises more than `band` above its
//! `period`-bar simple moving average, turns bear once it falls more than
//! `band` below it, and holds while the close stays inside the band. On the
//! first bar that has an average there is no earlier state to hold, so the
//! reading is the plain comparison: bull exactly when the close is above the
//! average.
//!
//! With the defaults (`period = 200`, `band = 0.05`) on daily closes this is
//! the long-term rule the nuniesmith/crypto bot trades: a coin holds its full
//! weight in a bull and half in a bear, and trades only when the regime
//! flips. The band is what keeps a close wandering around its average from
//! flipping the regime back and forth.
//!
//! The average is the same window mean as [`crate::functions::sma`], so the
//! `SMA_BAND_{period}` column equals `SMA_{period}`.
//!
//! Output columns, for `period = 200`:
//! - `SMA_BAND_200`: the moving average
//! - `SMA_BAND_200_upper`: `average * (1 + band)`; a close above it turns the regime bull
//! - `SMA_BAND_200_lower`: `average * (1 - band)`; a close below it turns the regime bear
//! - `SMA_BAND_200_bull`: 1.0 bull, 0.0 bear, `NaN` until `period` closes exist

use std::collections::HashMap;

use crate::error::IndicatorError;
use crate::indicator::{Indicator, IndicatorOutput, PriceColumn};
use crate::registry::{param_f64, param_str, param_usize};
use crate::types::Candle;

/// One bar's reading of the band.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BandReading {
    pub close: f64,
    pub sma: f64,
    pub bull: bool,
}

impl BandReading {
    /// How far the close sits from its average, e.g. 0.278 for 27.8% above.
    pub fn distance(&self) -> f64 {
        self.close / self.sma - 1.0
    }
}

/// The band's reading on every bar of `closes`, oldest first.
///
/// `None` until `period` closes exist, and on a bar whose close or average
/// is not a number; the regime carries across such a bar unchanged. Fewer
/// than `period` closes is not an error: every reading is `None`.
///
/// # Errors
/// [`IndicatorError::InvalidParameter`] for a zero `period`, or a `band`
/// that is negative or not finite.
pub fn sma_band(
    closes: &[f64],
    period: usize,
    band: f64,
) -> Result<Vec<Option<BandReading>>, IndicatorError> {
    if period == 0 {
        return Err(IndicatorError::InvalidParameter {
            name: "period".into(),
            value: 0.0,
        });
    }
    if !(band.is_finite() && band >= 0.0) {
        return Err(IndicatorError::InvalidParameter {
            name: "band".into(),
            value: band,
        });
    }
    let mut out = vec![None; closes.len()];
    let mut state: Option<bool> = None;
    for i in (period - 1)..closes.len() {
        let sum: f64 = closes[(i + 1 - period)..=i].iter().sum();
        let sma = sum / period as f64;
        let close = closes[i];
        if close.is_nan() || sma.is_nan() {
            continue;
        }
        let bull = match state {
            None => close > sma,
            Some(_) if close > sma * (1.0 + band) => true,
            Some(_) if close < sma * (1.0 - band) => false,
            Some(held) => held,
        };
        state = Some(bull);
        out[i] = Some(BandReading { close, sma, bull });
    }
    Ok(out)
}

/// Parameters for [`SmaBand`].
#[derive(Debug, Clone)]
pub struct SmaBandParams {
    /// Moving-average length in bars. Default: 200.
    pub period: usize,
    /// Half-width of the band as a fraction of the average. Default: 0.05.
    pub band: f64,
    /// Which OHLCV field the band reads. Default: close.
    pub column: PriceColumn,
}

impl Default for SmaBandParams {
    fn default() -> Self {
        Self {
            period: 200,
            band: 0.05,
            column: PriceColumn::Close,
        }
    }
}

/// The moving-average band as an [`Indicator`]; see the module docs.
///
/// # Example
/// ```rust,ignore
/// let band = SmaBand::new(SmaBandParams::default());
/// let output = band.calculate(&daily_candles)?;
/// let bull = output.latest("SMA_BAND_200_bull"); // Some(1.0) in a bull
/// ```
#[derive(Debug, Clone)]
pub struct SmaBand {
    pub params: SmaBandParams,
}

impl SmaBand {
    pub fn new(params: SmaBandParams) -> Self {
        Self { params }
    }

    fn key(&self) -> String {
        format!("SMA_BAND_{}", self.params.period)
    }
}

impl Indicator for SmaBand {
    fn name(&self) -> &'static str {
        "SMA_BAND"
    }

    fn required_len(&self) -> usize {
        self.params.period
    }

    fn required_columns(&self) -> &[&'static str] {
        &["close"]
    }

    fn calculate(&self, candles: &[Candle]) -> Result<IndicatorOutput, IndicatorError> {
        self.check_len(candles)?;
        let prices = self.params.column.extract(candles);
        let readings = sma_band(&prices, self.params.period, self.params.band)?;
        let n = readings.len();
        let (mut sma, mut upper, mut lower, mut bull) = (
            vec![f64::NAN; n],
            vec![f64::NAN; n],
            vec![f64::NAN; n],
            vec![f64::NAN; n],
        );
        for (i, r) in readings.iter().enumerate() {
            if let Some(r) = r {
                sma[i] = r.sma;
                upper[i] = r.sma * (1.0 + self.params.band);
                lower[i] = r.sma * (1.0 - self.params.band);
                bull[i] = if r.bull { 1.0 } else { 0.0 };
            }
        }
        let key = self.key();
        Ok(IndicatorOutput::from_pairs([
            (format!("{key}_upper"), upper),
            (format!("{key}_lower"), lower),
            (format!("{key}_bull"), bull),
            (key, sma),
        ]))
    }
}

/// Factory registered under `"smaband"` in the global registry.
pub fn factory<S: ::std::hash::BuildHasher>(
    params: &HashMap<String, String, S>,
) -> Result<Box<dyn Indicator>, IndicatorError> {
    let period = param_usize(params, "period", 200)?;
    let band = param_f64(params, "band", 0.05)?;
    if period == 0 {
        return Err(IndicatorError::InvalidParameter {
            name: "period".into(),
            value: 0.0,
        });
    }
    if !(band.is_finite() && band >= 0.0) {
        return Err(IndicatorError::InvalidParameter {
            name: "band".into(),
            value: band,
        });
    }
    let column = match param_str(params, "column", "close") {
        "open" => PriceColumn::Open,
        "high" => PriceColumn::High,
        "low" => PriceColumn::Low,
        "volume" => PriceColumn::Volume,
        _ => PriceColumn::Close,
    };
    Ok(Box::new(SmaBand::new(SmaBandParams {
        period,
        band,
        column,
    })))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn flat(n: usize, px: f64) -> Vec<f64> {
        vec![px; n]
    }

    fn last(closes: &[f64]) -> Option<BandReading> {
        sma_band(closes, 200, 0.05)
            .unwrap()
            .last()
            .copied()
            .flatten()
    }

    #[test]
    fn needs_a_full_window() {
        assert_eq!(last(&flat(199, 100.0)), None);
        assert!(last(&flat(200, 100.0)).is_some());
        assert!(sma_band(&[], 200, 0.05).unwrap().is_empty());
    }

    #[test]
    fn starts_from_the_plain_comparison_with_the_average() {
        assert!(
            !last(&flat(200, 100.0)).unwrap().bull,
            "equal to its average is not above it"
        );
        let mut up = flat(199, 100.0);
        up.push(101.0);
        assert!(
            last(&up).unwrap().bull,
            "1% above is bull on the first reading"
        );
    }

    #[test]
    fn inside_the_band_the_regime_holds() {
        // Bull from the first reading. A close 3% under the ~100 average is
        // inside the band, so still bull; 10% under crosses it.
        let mut c = flat(199, 100.0);
        c.push(101.0);
        let mut dip = c.clone();
        dip.push(97.0);
        assert!(last(&dip).unwrap().bull);
        let mut crash = c.clone();
        crash.push(90.0);
        assert!(!last(&crash).unwrap().bull);
    }

    #[test]
    fn a_bear_needs_a_close_above_the_upper_edge_to_turn_bull() {
        let mut c = flat(199, 100.0);
        c.push(99.0); // first reading: below the average, bear
        c.push(103.0); // ~3% above: inside the band, still bear
        assert!(!last(&c).unwrap().bull);
        c.push(106.0); // above 1.05x the average
        assert!(last(&c).unwrap().bull);
    }

    #[test]
    fn a_missing_close_carries_the_regime() {
        let mut c = flat(199, 100.0);
        c.push(101.0);
        c.push(f64::NAN);
        let r = sma_band(&c, 200, 0.05).unwrap();
        assert_eq!(r[200], None);
        assert!(r[199].unwrap().bull);
    }

    #[test]
    fn the_average_equals_the_sma_function() {
        let closes: Vec<f64> = (0..400)
            .map(|i| 100.0 + (f64::from(i) * 0.37).sin() * 9.0)
            .collect();
        let sma = crate::functions::sma(&closes, 200).unwrap();
        let band = sma_band(&closes, 200, 0.05).unwrap();
        for i in 199..400 {
            assert_eq!(band[i].unwrap().sma.to_bits(), sma[i].to_bits(), "bar {i}");
        }
    }

    #[test]
    fn bad_parameters_are_refused() {
        assert!(sma_band(&[1.0], 0, 0.05).is_err());
        assert!(sma_band(&[1.0], 200, -0.01).is_err());
        assert!(sma_band(&[1.0], 200, f64::NAN).is_err());
    }

    #[test]
    fn the_indicator_emits_the_band_and_the_regime() {
        let mut closes = flat(199, 100.0);
        closes.push(101.0);
        let candles: Vec<Candle> = closes
            .iter()
            .enumerate()
            .map(|(i, &c)| Candle {
                time: i64::try_from(i).expect("fits"),
                open: c,
                high: c,
                low: c,
                close: c,
                volume: 1.0,
            })
            .collect();
        let out = SmaBand::new(SmaBandParams::default())
            .calculate(&candles)
            .unwrap();
        let sma = out.latest("SMA_BAND_200").unwrap();
        assert!((sma - (199.0 * 100.0 + 101.0) / 200.0).abs() < 1e-12);
        assert!((out.latest("SMA_BAND_200_upper").unwrap() - sma * 1.05).abs() < 1e-12);
        assert!((out.latest("SMA_BAND_200_lower").unwrap() - sma * 0.95).abs() < 1e-12);
        assert_eq!(out.latest("SMA_BAND_200_bull"), Some(1.0));
        assert!(out.get("SMA_BAND_200_bull").unwrap()[198].is_nan());
    }

    #[test]
    fn the_factory_reads_period_and_band() {
        let params: HashMap<String, String> = [
            ("period".to_string(), "50".to_string()),
            ("band".to_string(), "0.1".to_string()),
        ]
        .into_iter()
        .collect();
        let ind = factory(&params).unwrap();
        assert_eq!(ind.name(), "SMA_BAND");
        assert_eq!(ind.required_len(), 50);
        let bad: HashMap<String, String> =
            std::iter::once(("band".to_string(), "-1".to_string())).collect();
        assert!(factory(&bad).is_err());
    }
}
