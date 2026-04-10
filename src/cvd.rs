//! Layer 8 — Cumulative Volume Delta (OHLCV heuristic).
//!
//! Estimates buy/sell volume from OHLCV bars and tracks cumulative delta,
//! slope, and price-CVD divergence.

use crate::types::Candle;
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use std::collections::VecDeque;

pub struct CVDTracker {
    slope_bars: usize,
    div_lookback: usize,

    day_cvd: f64,
    last_date: Option<NaiveDate>,
    cvd_hist: VecDeque<f64>,
    price_hist: VecDeque<f64>,

    pub cvd: f64,
    pub delta: f64,
    pub cvd_slope: f64,
    pub bullish: bool,
    /// `+1` = bullish divergence, `-1` = bearish divergence, `0` = none.
    pub divergence: i8,
}

impl CVDTracker {
    pub fn new(slope_bars: usize, div_lookback: usize) -> Self {
        let cap = (div_lookback + 10).max(50);
        Self {
            slope_bars,
            div_lookback,
            day_cvd: 0.0,
            last_date: None,
            cvd_hist: VecDeque::with_capacity(cap),
            price_hist: VecDeque::with_capacity(cap),
            cvd: 0.0,
            delta: 0.0,
            cvd_slope: 0.0,
            bullish: false,
            divergence: 0,
        }
    }

    pub fn update(&mut self, candle: &Candle) {
        let dt: DateTime<Utc> = Utc
            .timestamp_millis_opt(candle.time)
            .single()
            .unwrap_or_else(Utc::now);
        let date = dt.date_naive();

        if Some(date) != self.last_date {
            self.day_cvd = 0.0;
            self.last_date = Some(date);
        }

        let bar_rng = candle.high - candle.low;
        let buy_vol = if bar_rng > 0.0 {
            candle.volume * (candle.close - candle.low) / bar_rng
        } else {
            candle.volume * 0.5
        };
        self.delta = buy_vol - (candle.volume - buy_vol);
        self.day_cvd += self.delta;
        self.cvd = self.day_cvd;

        let cap = self.cvd_hist.capacity();
        if self.cvd_hist.len() == cap {
            self.cvd_hist.pop_front();
        }
        if self.price_hist.len() == cap {
            self.price_hist.pop_front();
        }
        self.cvd_hist.push_back(self.cvd);
        self.price_hist.push_back(candle.close);

        if self.cvd_hist.len() >= self.slope_bars {
            let arr: Vec<f64> = self.cvd_hist.iter().copied().collect();
            self.cvd_slope = arr[arr.len() - 1] - arr[arr.len() - self.slope_bars];
        }
        self.bullish = self.cvd_slope > 0.0;
        self.divergence = self.check_divergence();
    }

    fn check_divergence(&self) -> i8 {
        let n = self.cvd_hist.len().min(self.div_lookback);
        if n < 10 {
            return 0;
        }
        let prices: Vec<f64> = self.price_hist.iter().rev().take(n).copied().collect();
        let cvds: Vec<f64> = self.cvd_hist.iter().rev().take(n).copied().collect();

        let last_p = prices[0];
        let last_c = cvds[0];

        // Bullish divergence: price at new low but CVD is not
        let min_p = prices[1..].iter().cloned().fold(f64::INFINITY, f64::min);
        let min_c = cvds[1..].iter().cloned().fold(f64::INFINITY, f64::min);
        if last_p < min_p && last_c > min_c {
            return 1;
        }

        // Bearish divergence: price at new high but CVD is not
        let max_p = prices[1..]
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);
        let max_c = cvds[1..].iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        if last_p > max_p && last_c < max_c {
            return -1;
        }

        0
    }
}
