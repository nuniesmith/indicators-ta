use anyhow::Result;
use polars::prelude::*;

pub struct VolumeZoneOscillator {
    pub name: String,
    pub period: usize,
}

impl VolumeZoneOscillator {
    pub fn new(period: usize) -> Self {
        Self {
            name: "VZO".to_string(),
            period,
        }
    }
}

impl Indicator for VolumeZoneOscillator {
    fn calculate(&self, data: &DataFrame) -> Result<DataFrame> {
        // Retrieve close and volume columns.
        // Note: Using lowercase "close" and "volume" for consistency with prior examples.
        let close = data.column("close")?.f64()?;
        let volume = data.column("volume")?.f64()?;

        // 1. Calculate price change: close.diff()
        let diff = close.diff(1, NullPolicy::Pad)?;

        // 2. Assign volume based on price direction
        // positive_volume = volume if close_diff > 0 else 0
        let pos_vol = volume.zip_with(&diff, |v, d| match (v, d) {
            (Some(vol), Some(change)) => {
                if change > 0.0 {
                    Some(vol)
                } else {
                    Some(0.0)
                }
            }
            _ => None,
        })?;

        // negative_volume = volume if close_diff < 0 else 0
        let neg_vol = volume.zip_with(&diff, |v, d| match (v, d) {
            (Some(vol), Some(change)) => {
                if change < 0.0 {
                    Some(vol)
                } else {
                    Some(0.0)
                }
            }
            _ => None,
        })?;

        // 3. Define rolling sum options
        let options = RollingOptionsImpl {
            window_size: self.period,
            min_periods: self.period,
            ..Default::default()
        };

        // Calculate rolling sums for positive, negative, and total volume
        let sum_pos = pos_vol.rolling_sum(options.clone())?;
        let sum_neg = neg_vol.rolling_sum(options.clone())?;
        let sum_total = volume.rolling_sum(options)?;

        // 4. Calculate VZO: 100 * (sum_pos - sum_neg) / sum_total
        // Polars handles division by zero by producing NaN/Inf, mimicking np.nan behavior
        let vzo = (((&sum_pos - &sum_neg)? / sum_total)? * 100.0)?;

        let col_name = format!("{}_{}", self.name, self.period);
        Ok(Series::new(&col_name, vzo).into_frame())
    }
}
