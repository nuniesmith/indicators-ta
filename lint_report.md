# Rust Lint Report

| | |
|---|---|
| **Generated** | 2026-04-10 00:39:29 |
| **Workspace** | `/home/jordan/github/indicators-ta` |
| **Overall** | ❌ One or more checks failed |

---

## Summary

| Check | Status | Errors | Warnings | Time |
|-------|--------|--------|----------|------|
| `cargo fmt --check` | ❌ Fail | 0 | 0 | 0.10s |
| `cargo clippy` | ❌ 4 error(s) | 4 | 1 | 4.64s |
| `cargo test` | ❌ 1 error(s) | 1 | 0 | 1.07s |
| `cargo doc` | ✅ Pass | 0 | 0 | 0.50s |

---

## cargo fmt

> Checks that all source files match `rustfmt` formatting rules.
> Fix with: `cargo fmt --all`

```
Diff in /home/jordan/github/indicators-ta/src/cvd.rs:43:
     }
 
     pub fn update(&mut self, candle: &Candle) {
[31m-        let dt: DateTime<Utc> = Utc.timestamp_millis_opt(candle.time).single()
(B[m[32m+        let dt: DateTime<Utc> = Utc
(B[m[32m+            .timestamp_millis_opt(candle.time)
(B[m[32m+            .single()
(B[m             .unwrap_or_else(Utc::now);
         let date = dt.date_naive();
 
Diff in /home/jordan/github/indicators-ta/src/cvd.rs:63:
         self.cvd = self.day_cvd;
 
         let cap = self.cvd_hist.capacity();
[31m-        if self.cvd_hist.len() == cap { self.cvd_hist.pop_front(); }
(B[m[31m-        if self.price_hist.len() == cap { self.price_hist.pop_front(); }
(B[m[32m+        if self.cvd_hist.len() == cap {
(B[m[32m+            self.cvd_hist.pop_front();
(B[m[32m+        }
(B[m[32m+        if self.price_hist.len() == cap {
(B[m[32m+            self.price_hist.pop_front();
(B[m[32m+        }
(B[m         self.cvd_hist.push_back(self.cvd);
         self.price_hist.push_back(candle.close);
 
Diff in /home/jordan/github/indicators-ta/src/cvd.rs:78:
 
     fn check_divergence(&self) -> i8 {
         let n = self.cvd_hist.len().min(self.div_lookback);
[31m-        if n < 10 { return 0; }
(B[m[32m+        if n < 10 {
(B[m[32m+            return 0;
(B[m[32m+        }
(B[m         let prices: Vec<f64> = self.price_hist.iter().rev().take(n).copied().collect();
[31m-        let cvds:   Vec<f64> = self.cvd_hist.iter().rev().take(n).copied().collect();
(B[m[32m+        let cvds: Vec<f64> = self.cvd_hist.iter().rev().take(n).copied().collect();
(B[m 
         let last_p = prices[0];
         let last_c = cvds[0];
Diff in /home/jordan/github/indicators-ta/src/cvd.rs:88:
         // Bullish divergence: price at new low but CVD is not
         let min_p = prices[1..].iter().copied().fold(f64::INFINITY, f64::min);
         let min_c = cvds[1..].iter().copied().fold(f64::INFINITY, f64::min);
[31m-        if last_p < min_p && last_c > min_c { return 1; }
(B[m[32m+        if last_p < min_p && last_c > min_c {
(B[m[32m+            return 1;
(B[m[32m+        }
(B[m 
         // Bearish divergence: price at new high but CVD is not
[31m-        let max_p = prices[1..].iter().copied().fold(f64::NEG_INFINITY, f64::max);
(B[m[32m+        let max_p = prices[1..]
(B[m[32m+            .iter()
(B[m[32m+            .copied()
(B[m[32m+            .fold(f64::NEG_INFINITY, f64::max);
(B[m         let max_c = cvds[1..].iter().copied().fold(f64::NEG_INFINITY, f64::max);
[31m-        if last_p > max_p && last_c < max_c { return -1; }
(B[m[32m+        if last_p > max_p && last_c < max_c {
(B[m[32m+            return -1;
(B[m[32m+        }
(B[m 
         0
     }
Diff in /home/jordan/github/indicators-ta/src/detector.rs:147:
             stable_regime,
             confidence,
             adx_value.unwrap(),
[31m-            bb_values
(B[m[31m-                .as_ref()
(B[m[31m-                .map_or(50.0, |b| b.width_percentile),
(B[m[32m+            bb_values.as_ref().map_or(50.0, |b| b.width_percentile),
(B[m             Self::calculate_trend_strength(ema_short.unwrap(), ema_long.unwrap(), close),
         )
     }
Diff in /home/jordan/github/indicators-ta/src/liquidity.rs:47:
             return;
         }
 
[31m-        let h: f64 = self.candles.iter().map(|c| c.high).fold(f64::NEG_INFINITY, f64::max);
(B[m[31m-        let l: f64 = self.candles.iter().map(|c| c.low).fold(f64::INFINITY, f64::min);
(B[m[32m+        let h: f64 = self
(B[m[32m+            .candles
(B[m[32m+            .iter()
(B[m[32m+            .map(|c| c.high)
(B[m[32m+            .fold(f64::NEG_INFINITY, f64::max);
(B[m[32m+        let l: f64 = self
(B[m[32m+            .candles
(B[m[32m+            .iter()
(B[m[32m+            .map(|c| c.low)
(B[m[32m+            .fold(f64::INFINITY, f64::min);
(B[m         let rng = h - l;
[31m-        if rng <= 0.0 { return; }
(B[m[32m+        if rng <= 0.0 {
(B[m[32m+            return;
(B[m[32m+        }
(B[m 
         let step = rng / self.n_bins as f64;
         let mut bins = vec![0.0_f64; self.n_bins];
Diff in /home/jordan/github/indicators-ta/src/liquidity.rs:57:
 
         for c in &self.candles {
             let bar_rng = c.high - c.low;
[31m-            if bar_rng <= 0.0 || c.volume <= 0.0 { continue; }
(B[m[32m+            if bar_rng <= 0.0 || c.volume <= 0.0 {
(B[m[32m+                continue;
(B[m[32m+            }
(B[m             #[allow(clippy::needless_range_loop)]
             for i in 0..self.n_bins {
                 let bin_lo = l + step * i as f64;
Diff in /home/jordan/github/indicators-ta/src/liquidity.rs:70:
         }
 
         // Point of Control
[31m-        let poc_idx = bins.iter().enumerate()
(B[m[32m+        let poc_idx = bins
(B[m[32m+            .iter()
(B[m[32m+            .enumerate()
(B[m             .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
             .map_or(0, |(i, _)| i);
         self.poc_price = Some(l + step * poc_idx as f64 + step / 2.0);
Diff in /home/jordan/github/indicators-ta/src/liquidity.rs:83:
         let mut lower = poc_idx;
 
         while area_vol < target {
[31m-            let can_up   = upper + 1 < self.n_bins;
(B[m[32m+            let can_up = upper + 1 < self.n_bins;
(B[m             let can_down = lower > 0;
[31m-            if !can_up && !can_down { break; }
(B[m[31m-            let vol_up   = if can_up   { bins[upper + 1] } else { -1.0 };
(B[m[32m+            if !can_up && !can_down {
(B[m[32m+                break;
(B[m[32m+            }
(B[m[32m+            let vol_up = if can_up { bins[upper + 1] } else { -1.0 };
(B[m             let vol_down = if can_down { bins[lower - 1] } else { -1.0 };
             if vol_up >= vol_down {
                 upper += 1;
Diff in /home/jordan/github/indicators-ta/src/liquidity.rs:102:
 
         // Buy / sell liquidity split around close
         let cl = candle.close;
[31m-        self.buy_liq  = (0..self.n_bins).map(|i| {
(B[m[31m-            if l + step * i as f64 + step / 2.0 < cl { bins[i] } else { 0.0 }
(B[m[31m-        }).sum();
(B[m[31m-        self.sell_liq = (0..self.n_bins).map(|i| {
(B[m[31m-            if l + step * i as f64 + step / 2.0 >= cl { bins[i] } else { 0.0 }
(B[m[31m-        }).sum();
(B[m[32m+        self.buy_liq = (0..self.n_bins)
(B[m[32m+            .map(|i| {
(B[m[32m+                if l + step * i as f64 + step / 2.0 < cl {
(B[m[32m+                    bins[i]
(B[m[32m+                } else {
(B[m[32m+                    0.0
(B[m[32m+                }
(B[m[32m+            })
(B[m[32m+            .sum();
(B[m[32m+        self.sell_liq = (0..self.n_bins)
(B[m[32m+            .map(|i| {
(B[m[32m+                if l + step * i as f64 + step / 2.0 >= cl {
(B[m[32m+                    bins[i]
(B[m[32m+                } else {
(B[m[32m+                    0.0
(B[m[32m+                }
(B[m[32m+            })
(B[m[32m+            .sum();
(B[m 
         let total = self.buy_liq + self.sell_liq;
[31m-        self.buy_pct   = if total > 0.0 { self.buy_liq / total } else { 0.5 };
(B[m[32m+        self.buy_pct = if total > 0.0 {
(B[m[32m+            self.buy_liq / total
(B[m[32m+        } else {
(B[m[32m+            0.5
(B[m[32m+        };
(B[m         self.imbalance = self.buy_liq - self.sell_liq;
     }
 
Diff in /home/jordan/github/indicators-ta/src/primitives.rs:1031:
     #[test]
     fn test_rsi_value_cached() {
         let mut rsi = RSI::new(14);
[31m-        assert!(rsi.value().is_none(), "value() should be None before warmup");
(B[m[32m+        assert!(
(B[m[32m+            rsi.value().is_none(),
(B[m[32m+            "value() should be None before warmup"
(B[m[32m+        );
(B[m 
         let mut last_from_update = None;
         for i in 0..30 {
Diff in /home/jordan/github/indicators-ta/src/primitives.rs:1059:
         rsi.reset();
         assert!(rsi.value().is_none(), "value() should be None after reset");
     }
[31m-
(B[m[31m-
(B[m 
     // --- SMA Helper Test ---
 
Diff in /home/jordan/github/indicators-ta/src/signal.rs:3:
 //! Port of the Python `compute_signal` function and `SignalStreak` class.
 
 use crate::confluence::ConfluenceEngine;
[31m-use crate::engine::Indicators;
(B[m use crate::cvd::CVDTracker;
[32m+use crate::engine::Indicators;
(B[m use crate::liquidity::LiquidityProfile;
 use crate::settings::BotSettings;
 use crate::structure::MarketStructure;
Diff in /home/jordan/github/indicators-ta/src/signal.rs:16:
 #[derive(Debug, Clone)]
 pub struct SignalComponents {
     // Votes: +1 = bull, -1 = bear, 0 = neutral
[31m-    pub v_vwap:      i8,
(B[m[31m-    pub v_ema:       i8,
(B[m[31m-    pub v_st:        i8,
(B[m[31m-    pub v_ts:        i8,
(B[m[31m-    pub v_liq:       i8,
(B[m[32m+    pub v_vwap: i8,
(B[m[32m+    pub v_ema: i8,
(B[m[32m+    pub v_st: i8,
(B[m[32m+    pub v_ts: i8,
(B[m[32m+    pub v_liq: i8,
(B[m     pub v_conf_bull: i8,
     pub v_conf_bear: i8,
[31m-    pub v_struct:    i8,
(B[m[31m-    pub v_cvd:       i8,
(B[m[31m-    pub v_ao:        i8,
(B[m[31m-    pub v_hurst:     i8,
(B[m[32m+    pub v_struct: i8,
(B[m[32m+    pub v_cvd: i8,
(B[m[32m+    pub v_ao: i8,
(B[m[32m+    pub v_hurst: i8,
(B[m     pub v_accel_bull: i8,
     pub v_accel_bear: i8,
 
Diff in /home/jordan/github/indicators-ta/src/signal.rs:33:
     // Supporting values
[31m-    pub hurst:        f64,
(B[m[31m-    pub price_accel:  f64,
(B[m[31m-    pub bull_score:   f64,
(B[m[31m-    pub bear_score:   f64,
(B[m[32m+    pub hurst: f64,
(B[m[32m+    pub price_accel: f64,
(B[m[32m+    pub bull_score: f64,
(B[m[32m+    pub bear_score: f64,
(B[m     pub conf_min_adj: f64,
     pub liq_imbalance: f64,
[31m-    pub liq_buy_pct:   f64,
(B[m[31m-    pub poc:           Option<f64>,
(B[m[31m-    pub struct_bias:   i8,
(B[m[31m-    pub fib618:        Option<f64>,
(B[m[31m-    pub fib_zone:      &'static str,
(B[m[31m-    pub fib_ok:        bool,
(B[m[31m-    pub bos:           bool,
(B[m[31m-    pub choch:         bool,
(B[m[31m-    pub ts_norm:       f64,
(B[m[31m-    pub dominance:     f64,
(B[m[31m-    pub cvd_slope:     Option<f64>,
(B[m[31m-    pub cvd_div:       i8,
(B[m[31m-    pub ao:            f64,
(B[m[31m-    pub ao_rising:     bool,
(B[m[31m-    pub wr_pct:        f64,
(B[m[31m-    pub mom_pct:       f64,
(B[m[31m-    pub wave_ok_long:  bool,
(B[m[32m+    pub liq_buy_pct: f64,
(B[m[32m+    pub poc: Option<f64>,
(B[m[32m+    pub struct_bias: i8,
(B[m[32m+    pub fib618: Option<f64>,
(B[m[32m+    pub fib_zone: &'static str,
(B[m[32m+    pub fib_ok: bool,
(B[m[32m+    pub bos: bool,
(B[m[32m+    pub choch: bool,
(B[m[32m+    pub ts_norm: f64,
(B[m[32m+    pub dominance: f64,
(B[m[32m+    pub cvd_slope: Option<f64>,
(B[m[32m+    pub cvd_div: i8,
(B[m[32m+    pub ao: f64,
(B[m[32m+    pub ao_rising: bool,
(B[m[32m+    pub wr_pct: f64,
(B[m[32m+    pub mom_pct: f64,
(B[m[32m+    pub wave_ok_long: bool,
(B[m     pub wave_ok_short: bool,
[31m-    pub mom_ok_long:   bool,
(B[m[31m-    pub mom_ok_short:  bool,
(B[m[31m-    pub vol_pct:       Option<f64>,
(B[m[31m-    pub vol_regime:    Option<&'static str>,
(B[m[32m+    pub mom_ok_long: bool,
(B[m[32m+    pub mom_ok_short: bool,
(B[m[32m+    pub vol_pct: Option<f64>,
(B[m[32m+    pub vol_regime: Option<&'static str>,
(B[m }
 
 // ── compute_signal ────────────────────────────────────────────────────────────
Diff in /home/jordan/github/indicators-ta/src/signal.rs:71:
 /// - `0`  → neutral / no trade
 pub fn compute_signal(
     close: f64,
[31m-    ind:    &Indicators,
(B[m[31m-    liq:    &LiquidityProfile,
(B[m[31m-    conf:   &ConfluenceEngine,
(B[m[31m-    ms:     &MarketStructure,
(B[m[31m-    s:      &BotSettings,
(B[m[31m-    cvd:    Option<&CVDTracker>,
(B[m[31m-    vol:    Option<&VolatilityPercentile>,
(B[m[32m+    ind: &Indicators,
(B[m[32m+    liq: &LiquidityProfile,
(B[m[32m+    conf: &ConfluenceEngine,
(B[m[32m+    ms: &MarketStructure,
(B[m[32m+    s: &BotSettings,
(B[m[32m+    cvd: Option<&CVDTracker>,
(B[m[32m+    vol: Option<&VolatilityPercentile>,
(B[m ) -> (i32, SignalComponents) {
     // Not ready yet
     if ind.vwap.is_none() || ind.ema.is_none() || ind.st.is_none() {
Diff in /home/jordan/github/indicators-ta/src/signal.rs:86:
     }
 
     let vwap = ind.vwap.unwrap();
[31m-    let ema  = ind.ema.unwrap();
(B[m[32m+    let ema = ind.ema.unwrap();
(B[m 
     // ── Layer votes ───────────────────────────────────────────────────────────
[31m-    let v1 = if close > vwap { 1_i8 } else { -1 };          // L1 VWAP
(B[m[31m-    let v2 = if close > ema  { 1 } else { -1 };             // L2 EMA
(B[m[31m-    let v3 = if ind.st_dir_pub == -1 { -1 } else { 1 };     // L3 SuperTrend (-1=bullish)
(B[m[31m-    let v4 = if ind.ts_bullish { 1 } else { -1 };           // L4 TrendSpeed
(B[m[31m-    let v5 = if liq.bullish() { 1 } else { -1 };            // L5 Liquidity
(B[m[32m+    let v1 = if close > vwap { 1_i8 } else { -1 }; // L1 VWAP
(B[m[32m+    let v2 = if close > ema { 1 } else { -1 }; // L2 EMA
(B[m[32m+    let v3 = if ind.st_dir_pub == -1 { -1 } else { 1 }; // L3 SuperTrend (-1=bullish)
(B[m[32m+    let v4 = if ind.ts_bullish { 1 } else { -1 }; // L4 TrendSpeed
(B[m[32m+    let v5 = if liq.bullish() { 1 } else { -1 }; // L5 Liquidity
(B[m 
     let conf_adj = vol.map_or(1.0, |v| v.conf_adj);
     let adj_min = s.conf_min_score * conf_adj;
Diff in /home/jordan/github/indicators-ta/src/signal.rs:100:
     let v6_bull = if conf.bull_score >= adj_min { 1_i8 } else { -1 }; // L6 bull
     let v6_bear = if conf.bear_score >= adj_min { 1_i8 } else { -1 }; // L6 bear
 
[31m-    let v7 = ms.bias;                                        // L7 Market Structure
(B[m[32m+    let v7 = ms.bias; // L7 Market Structure
(B[m 
     let v8: i8 = cvd.map_or(0, |c| {
[31m-        if c.divergence != 0 { c.divergence } else if c.bullish { 1 } else { -1 }
(B[m[31m-    });                                                      // L8 CVD
(B[m[32m+        if c.divergence != 0 {
(B[m[32m+            c.divergence
(B[m[32m+        } else if c.bullish {
(B[m[32m+            1
(B[m[32m+        } else {
(B[m[32m+            -1
(B[m[32m+        }
(B[m[32m+    }); // L8 CVD
(B[m 
     let v9: i8 = if ind.highs.len() >= 34 {
         if ind.ao_rising { 1 } else { -1 }
Diff in /home/jordan/github/indicators-ta/src/signal.rs:111:
[31m-    } else { 0 };                                            // L9 AO
(B[m[32m+    } else {
(B[m[32m+        0
(B[m[32m+    }; // L9 AO
(B[m 
[31m-    let v10: i8 = if (ind.hurst - 0.5).abs() < 0.005 { 0 }
(B[m[31m-    else if ind.hurst >= s.hurst_threshold { 1 } else { -1 }; // L10 Hurst
(B[m[32m+    let v10: i8 = if (ind.hurst - 0.5).abs() < 0.005 {
(B[m[32m+        0
(B[m[32m+    } else if ind.hurst >= s.hurst_threshold {
(B[m[32m+        1
(B[m[32m+    } else {
(B[m[32m+        -1
(B[m[32m+    }; // L10 Hurst
(B[m 
     let (v11_bull, v11_bear): (i8, i8) = if ind.price_accel.abs() < 0.005 {
         (0, 0)
Diff in /home/jordan/github/indicators-ta/src/signal.rs:118:
     } else {
[31m-        (if ind.price_accel > 0.0 { 1 } else { -1 },
(B[m[31m-         if ind.price_accel < 0.0 { 1 } else { -1 })
(B[m[31m-    };                                                       // L11 PriceAccel
(B[m[32m+        (
(B[m[32m+            if ind.price_accel > 0.0 { 1 } else { -1 },
(B[m[32m+            if ind.price_accel < 0.0 { 1 } else { -1 },
(B[m[32m+        )
(B[m[32m+    }; // L11 PriceAccel
(B[m 
     // Fibonacci zone gates
[31m-    let fib_ok_long  = !s.fib_zone_enabled || ms.in_discount || ms.fib500.is_none();
(B[m[31m-    let fib_ok_short = !s.fib_zone_enabled || ms.in_premium  || ms.fib500.is_none();
(B[m[32m+    let fib_ok_long = !s.fib_zone_enabled || ms.in_discount || ms.fib500.is_none();
(B[m[32m+    let fib_ok_short = !s.fib_zone_enabled || ms.in_premium || ms.fib500.is_none();
(B[m 
     // ── Signal logic ──────────────────────────────────────────────────────────
     let (bull, bear) = match s.signal_mode.as_str() {
Diff in /home/jordan/github/indicators-ta/src/signal.rs:129:
         "strict" => {
[31m-            let bull = v1==1 && v2==1 && v3==-1 && v4==1 && v5==1
(B[m[31m-                    && v6_bull==1 && v7==1 && fib_ok_long && (v8==1 || v8==0);
(B[m[31m-            let bear = v1==-1 && v2==-1 && v3==1 && v4==-1 && v5==-1
(B[m[31m-                    && v6_bear==1 && v7==-1 && fib_ok_short && (v8==-1 || v8==0);
(B[m[32m+            let bull = v1 == 1
(B[m[32m+                && v2 == 1
(B[m[32m+                && v3 == -1
(B[m[32m+                && v4 == 1
(B[m[32m+                && v5 == 1
(B[m[32m+                && v6_bull == 1
(B[m[32m+                && v7 == 1
(B[m[32m+                && fib_ok_long
(B[m[32m+                && (v8 == 1 || v8 == 0);
(B[m[32m+            let bear = v1 == -1
(B[m[32m+                && v2 == -1
(B[m[32m+                && v3 == 1
(B[m[32m+                && v4 == -1
(B[m[32m+                && v5 == -1
(B[m[32m+                && v6_bear == 1
(B[m[32m+                && v7 == -1
(B[m[32m+                && fib_ok_short
(B[m[32m+                && (v8 == -1 || v8 == 0);
(B[m             (bull, bear)
         }
         "majority" => {
Diff in /home/jordan/github/indicators-ta/src/signal.rs:137:
[31m-            let core_bull = v1==1 && v2==1 && v3==-1 && v4==1;
(B[m[31m-            let core_bear = v1==-1 && v2==-1 && v3==1 && v4==-1;
(B[m[32m+            let core_bull = v1 == 1 && v2 == 1 && v3 == -1 && v4 == 1;
(B[m[32m+            let core_bear = v1 == -1 && v2 == -1 && v3 == 1 && v4 == -1;
(B[m 
             let ext_bull_count = [
[31m-                v5==1, v6_bull==1, v7==1, fib_ok_long,
(B[m[31m-                v8==1, v9==1, ind.wave_ok_long, ind.mom_ok_long, v10==1, v11_bull==1,
(B[m[31m-            ].iter().filter(|&&b| b).count();
(B[m[32m+                v5 == 1,
(B[m[32m+                v6_bull == 1,
(B[m[32m+                v7 == 1,
(B[m[32m+                fib_ok_long,
(B[m[32m+                v8 == 1,
(B[m[32m+                v9 == 1,
(B[m[32m+                ind.wave_ok_long,
(B[m[32m+                ind.mom_ok_long,
(B[m[32m+                v10 == 1,
(B[m[32m+                v11_bull == 1,
(B[m[32m+            ]
(B[m[32m+            .iter()
(B[m[32m+            .filter(|&&b| b)
(B[m[32m+            .count();
(B[m 
             let ext_bear_count = [
[31m-                v5==-1, v6_bear==1, v7==-1, fib_ok_short,
(B[m[31m-                v8==-1, v9==-1, ind.wave_ok_short, ind.mom_ok_short, v10==1, v11_bear==1,
(B[m[31m-            ].iter().filter(|&&b| b).count();
(B[m[32m+                v5 == -1,
(B[m[32m+                v6_bear == 1,
(B[m[32m+                v7 == -1,
(B[m[32m+                fib_ok_short,
(B[m[32m+                v8 == -1,
(B[m[32m+                v9 == -1,
(B[m[32m+                ind.wave_ok_short,
(B[m[32m+                ind.mom_ok_short,
(B[m[32m+                v10 == 1,
(B[m[32m+                v11_bear == 1,
(B[m[32m+            ]
(B[m[32m+            .iter()
(B[m[32m+            .filter(|&&b| b)
(B[m[32m+            .count();
(B[m 
[31m-            (core_bull && ext_bull_count >= 2, core_bear && ext_bear_count >= 2)
(B[m[32m+            (
(B[m[32m+                core_bull && ext_bull_count >= 2,
(B[m[32m+                core_bear && ext_bear_count >= 2,
(B[m[32m+            )
(B[m         }
         _ => {
             // "any" / default — core layers only
Diff in /home/jordan/github/indicators-ta/src/signal.rs:154:
[31m-            let bull = v1==1 && v2==1 && v3==-1 && v4==1;
(B[m[31m-            let bear = v1==-1 && v2==-1 && v3==1 && v4==-1;
(B[m[32m+            let bull = v1 == 1 && v2 == 1 && v3 == -1 && v4 == 1;
(B[m[32m+            let bear = v1 == -1 && v2 == -1 && v3 == 1 && v4 == -1;
(B[m             (bull, bear)
         }
     };
Diff in /home/jordan/github/indicators-ta/src/signal.rs:159:
 
[31m-    let fib_zone = if ms.in_discount { "discount" } else if ms.in_premium { "premium" } else { "mid" };
(B[m[32m+    let fib_zone = if ms.in_discount {
(B[m[32m+        "discount"
(B[m[32m+    } else if ms.in_premium {
(B[m[32m+        "premium"
(B[m[32m+    } else {
(B[m[32m+        "mid"
(B[m[32m+    };
(B[m 
     let comps = SignalComponents {
[31m-        v_vwap: v1, v_ema: v2, v_st: v3, v_ts: v4, v_liq: v5,
(B[m[31m-        v_conf_bull: v6_bull, v_conf_bear: v6_bear, v_struct: v7,
(B[m[31m-        v_cvd: v8, v_ao: v9, v_hurst: v10,
(B[m[31m-        v_accel_bull: v11_bull, v_accel_bear: v11_bear,
(B[m[31m-        hurst: ind.hurst, price_accel: ind.price_accel,
(B[m[31m-        bull_score: conf.bull_score, bear_score: conf.bear_score,
(B[m[32m+        v_vwap: v1,
(B[m[32m+        v_ema: v2,
(B[m[32m+        v_st: v3,
(B[m[32m+        v_ts: v4,
(B[m[32m+        v_liq: v5,
(B[m[32m+        v_conf_bull: v6_bull,
(B[m[32m+        v_conf_bear: v6_bear,
(B[m[32m+        v_struct: v7,
(B[m[32m+        v_cvd: v8,
(B[m[32m+        v_ao: v9,
(B[m[32m+        v_hurst: v10,
(B[m[32m+        v_accel_bull: v11_bull,
(B[m[32m+        v_accel_bear: v11_bear,
(B[m[32m+        hurst: ind.hurst,
(B[m[32m+        price_accel: ind.price_accel,
(B[m[32m+        bull_score: conf.bull_score,
(B[m[32m+        bear_score: conf.bear_score,
(B[m         conf_min_adj: adj_min,
[31m-        liq_imbalance: liq.imbalance, liq_buy_pct: liq.buy_pct * 100.0,
(B[m[32m+        liq_imbalance: liq.imbalance,
(B[m[32m+        liq_buy_pct: liq.buy_pct * 100.0,
(B[m         poc: liq.poc_price,
         struct_bias: ms.bias,
         fib618: ms.fib618,
Diff in /home/jordan/github/indicators-ta/src/signal.rs:174:
         fib_zone,
         fib_ok: if bull { fib_ok_long } else { fib_ok_short },
[31m-        bos: ms.bos, choch: ms.choch,
(B[m[31m-        ts_norm: ind.ts_norm, dominance: ind.dominance,
(B[m[32m+        bos: ms.bos,
(B[m[32m+        choch: ms.choch,
(B[m[32m+        ts_norm: ind.ts_norm,
(B[m[32m+        dominance: ind.dominance,
(B[m         cvd_slope: cvd.map(|c| c.cvd_slope),
         cvd_div: cvd.map_or(0, |c| c.divergence),
[31m-        ao: ind.ao, ao_rising: ind.ao_rising,
(B[m[31m-        wr_pct: ind.wr_pct, mom_pct: ind.mom_pct,
(B[m[31m-        wave_ok_long: ind.wave_ok_long, wave_ok_short: ind.wave_ok_short,
(B[m[31m-        mom_ok_long: ind.mom_ok_long, mom_ok_short: ind.mom_ok_short,
(B[m[31m-        vol_pct:    vol.map(|v| v.vol_pct),
(B[m[32m+        ao: ind.ao,
(B[m[32m+        ao_rising: ind.ao_rising,
(B[m[32m+        wr_pct: ind.wr_pct,
(B[m[32m+        mom_pct: ind.mom_pct,
(B[m[32m+        wave_ok_long: ind.wave_ok_long,
(B[m[32m+        wave_ok_short: ind.wave_ok_short,
(B[m[32m+        mom_ok_long: ind.mom_ok_long,
(B[m[32m+        mom_ok_short: ind.mom_ok_short,
(B[m[32m+        vol_pct: vol.map(|v| v.vol_pct),
(B[m         vol_regime: vol.map(|v| v.vol_regime),
     };
 
Diff in /home/jordan/github/indicators-ta/src/signal.rs:188:
[31m-    if bull  { return (1, comps);  }
(B[m[31m-    if bear  { return (-1, comps); }
(B[m[32m+    if bull {
(B[m[32m+        return (1, comps);
(B[m[32m+    }
(B[m[32m+    if bear {
(B[m[32m+        return (-1, comps);
(B[m[32m+    }
(B[m     (0, comps)
 }
 
Diff in /home/jordan/github/indicators-ta/src/signal.rs:193:
 fn empty_components(
[31m-    ind:  &Indicators,
(B[m[31m-    liq:  &LiquidityProfile,
(B[m[32m+    ind: &Indicators,
(B[m[32m+    liq: &LiquidityProfile,
(B[m     conf: &ConfluenceEngine,
[31m-    ms:   &MarketStructure,
(B[m[31m-    cvd:  Option<&CVDTracker>,
(B[m[31m-    vol:  Option<&VolatilityPercentile>,
(B[m[32m+    ms: &MarketStructure,
(B[m[32m+    cvd: Option<&CVDTracker>,
(B[m[32m+    vol: Option<&VolatilityPercentile>,
(B[m ) -> SignalComponents {
     SignalComponents {
[31m-        v_vwap: 0, v_ema: 0, v_st: 0, v_ts: 0, v_liq: 0,
(B[m[31m-        v_conf_bull: 0, v_conf_bear: 0, v_struct: 0,
(B[m[31m-        v_cvd: 0, v_ao: 0, v_hurst: 0, v_accel_bull: 0, v_accel_bear: 0,
(B[m[31m-        hurst: ind.hurst, price_accel: ind.price_accel,
(B[m[31m-        bull_score: conf.bull_score, bear_score: conf.bear_score,
(B[m[32m+        v_vwap: 0,
(B[m[32m+        v_ema: 0,
(B[m[32m+        v_st: 0,
(B[m[32m+        v_ts: 0,
(B[m[32m+        v_liq: 0,
(B[m[32m+        v_conf_bull: 0,
(B[m[32m+        v_conf_bear: 0,
(B[m[32m+        v_struct: 0,
(B[m[32m+        v_cvd: 0,
(B[m[32m+        v_ao: 0,
(B[m[32m+        v_hurst: 0,
(B[m[32m+        v_accel_bull: 0,
(B[m[32m+        v_accel_bear: 0,
(B[m[32m+        hurst: ind.hurst,
(B[m[32m+        price_accel: ind.price_accel,
(B[m[32m+        bull_score: conf.bull_score,
(B[m[32m+        bear_score: conf.bear_score,
(B[m         conf_min_adj: 0.0,
[31m-        liq_imbalance: liq.imbalance, liq_buy_pct: liq.buy_pct * 100.0,
(B[m[31m-        poc: liq.poc_price, struct_bias: ms.bias,
(B[m[31m-        fib618: ms.fib618, fib_zone: "mid", fib_ok: false,
(B[m[31m-        bos: false, choch: false, ts_norm: 0.5, dominance: 0.0,
(B[m[31m-        cvd_slope: cvd.map(|c| c.cvd_slope), cvd_div: 0,
(B[m[31m-        ao: ind.ao, ao_rising: false,
(B[m[31m-        wr_pct: 0.5, mom_pct: 0.5,
(B[m[31m-        wave_ok_long: false, wave_ok_short: false,
(B[m[31m-        mom_ok_long: false, mom_ok_short: false,
(B[m[31m-        vol_pct: vol.map(|v| v.vol_pct), vol_regime: vol.map(|v| v.vol_regime),
(B[m[32m+        liq_imbalance: liq.imbalance,
(B[m[32m+        liq_buy_pct: liq.buy_pct * 100.0,
(B[m[32m+        poc: liq.poc_price,
(B[m[32m+        struct_bias: ms.bias,
(B[m[32m+        fib618: ms.fib618,
(B[m[32m+        fib_zone: "mid",
(B[m[32m+        fib_ok: false,
(B[m[32m+        bos: false,
(B[m[32m+        choch: false,
(B[m[32m+        ts_norm: 0.5,
(B[m[32m+        dominance: 0.0,
(B[m[32m+        cvd_slope: cvd.map(|c| c.cvd_slope),
(B[m[32m+        cvd_div: 0,
(B[m[32m+        ao: ind.ao,
(B[m[32m+        ao_rising: false,
(B[m[32m+        wr_pct: 0.5,
(B[m[32m+        mom_pct: 0.5,
(B[m[32m+        wave_ok_long: false,
(B[m[32m+        wave_ok_short: false,
(B[m[32m+        mom_ok_long: false,
(B[m[32m+        mom_ok_short: false,
(B[m[32m+        vol_pct: vol.map(|v| v.vol_pct),
(B[m[32m+        vol_regime: vol.map(|v| v.vol_regime),
(B[m     }
 }
 
Diff in /home/jordan/github/indicators-ta/src/signal.rs:229:
 
 impl SignalStreak {
     pub fn new(required: usize) -> Self {
[31m-        Self { required, direction: 0, count: 0 }
(B[m[32m+        Self {
(B[m[32m+            required,
(B[m[32m+            direction: 0,
(B[m[32m+            count: 0,
(B[m[32m+        }
(B[m     }
 
     /// Feed a raw signal (`+1`, `-1`, or `0`).
Diff in /home/jordan/github/indicators-ta/src/signal.rs:249:
         self.count = 0;
     }
 
[31m-    pub fn current_direction(&self) -> i32 { self.direction }
(B[m[31m-    pub fn current_count(&self)     -> usize { self.count }
(B[m[32m+    pub fn current_direction(&self) -> i32 {
(B[m[32m+        self.direction
(B[m[32m+    }
(B[m[32m+    pub fn current_count(&self) -> usize {
(B[m[32m+        self.count
(B[m[32m+    }
(B[m }
 
Diff in /home/jordan/github/indicators-ta/src/structure.rs:11:
     atr_mult: f64,
     maxlen: usize,
 
[31m-    highs:  VecDeque<f64>,
(B[m[31m-    lows:   VecDeque<f64>,
(B[m[32m+    highs: VecDeque<f64>,
(B[m[32m+    lows: VecDeque<f64>,
(B[m     closes: VecDeque<f64>,
 
[31m-    swing_hi:      Option<f64>,
(B[m[31m-    swing_lo:      Option<f64>,
(B[m[32m+    swing_hi: Option<f64>,
(B[m[32m+    swing_lo: Option<f64>,
(B[m     prev_swing_hi: Option<f64>,
     prev_swing_lo: Option<f64>,
[31m-    atr:           Option<f64>,
(B[m[32m+    atr: Option<f64>,
(B[m     bias_internal: i8,
[31m-    fib_hi:  Option<f64>,
(B[m[31m-    fib_lo:  Option<f64>,
(B[m[32m+    fib_hi: Option<f64>,
(B[m[32m+    fib_lo: Option<f64>,
(B[m     fib_dir: i8,
     last_broken_hi: Option<f64>,
     last_broken_lo: Option<f64>,
Diff in /home/jordan/github/indicators-ta/src/structure.rs:49:
             swing_len,
             atr_mult: atr_mult_min,
             maxlen,
[31m-            highs:  VecDeque::with_capacity(maxlen),
(B[m[31m-            lows:   VecDeque::with_capacity(maxlen),
(B[m[32m+            highs: VecDeque::with_capacity(maxlen),
(B[m[32m+            lows: VecDeque::with_capacity(maxlen),
(B[m             closes: VecDeque::with_capacity(maxlen),
             swing_hi: None,
             swing_lo: None,
Diff in /home/jordan/github/indicators-ta/src/structure.rs:78:
     }
 
     pub fn update(&mut self, candle: &Candle) {
[31m-        if self.highs.len() == self.maxlen { self.highs.pop_front(); }
(B[m[31m-        if self.lows.len()  == self.maxlen { self.lows.pop_front(); }
(B[m[31m-        if self.closes.len()== self.maxlen { self.closes.pop_front(); }
(B[m[32m+        if self.highs.len() == self.maxlen {
(B[m[32m+            self.highs.pop_front();
(B[m[32m+        }
(B[m[32m+        if self.lows.len() == self.maxlen {
(B[m[32m+            self.lows.pop_front();
(B[m[32m+        }
(B[m[32m+        if self.closes.len() == self.maxlen {
(B[m[32m+            self.closes.pop_front();
(B[m[32m+        }
(B[m         self.highs.push_back(candle.high);
         self.lows.push_back(candle.low);
         self.closes.push_back(candle.close);
Diff in /home/jordan/github/indicators-ta/src/structure.rs:88:
         // ATR (Wilder 1/14)
         let prev_c = if self.closes.len() >= 2 {
             *self.closes.iter().rev().nth(1).unwrap()
[31m-        } else { candle.close };
(B[m[32m+        } else {
(B[m[32m+            candle.close
(B[m[32m+        };
(B[m         let tr = (candle.high - candle.low)
             .max((candle.high - prev_c).abs())
[31m-            .max((candle.low  - prev_c).abs());
(B[m[32m+            .max((candle.low - prev_c).abs());
(B[m         self.atr = Some(match self.atr {
             None => tr,
             Some(prev) => prev / 14.0 + tr * (1.0 - 1.0 / 14.0),
Diff in /home/jordan/github/indicators-ta/src/structure.rs:101:
         let ph = self.pivot_high();
         let pl = self.pivot_low();
 
[31m-        self.bos   = false;
(B[m[32m+        self.bos = false;
(B[m         self.choch = false;
         self.choch_dir = 0;
 
Diff in /home/jordan/github/indicators-ta/src/structure.rs:108:
         if let Some(ph_val) = ph {
[31m-            let atr_ok = self.swing_lo.is_none_or(|slo| (ph_val - slo) >= atr * self.atr_mult);
(B[m[32m+            let atr_ok = self
(B[m[32m+                .swing_lo
(B[m[32m+                .is_none_or(|slo| (ph_val - slo) >= atr * self.atr_mult);
(B[m             if atr_ok {
                 self.prev_swing_hi = self.swing_hi;
                 self.swing_hi = Some(ph_val);
Diff in /home/jordan/github/indicators-ta/src/structure.rs:113:
             }
         }
         if let Some(pl_val) = pl {
[31m-            let atr_ok = self.swing_hi.is_none_or(|shi| (shi - pl_val) >= atr * self.atr_mult);
(B[m[32m+            let atr_ok = self
(B[m[32m+                .swing_hi
(B[m[32m+                .is_none_or(|shi| (shi - pl_val) >= atr * self.atr_mult);
(B[m             if atr_ok {
                 self.prev_swing_lo = self.swing_lo;
                 self.swing_lo = Some(pl_val);
Diff in /home/jordan/github/indicators-ta/src/structure.rs:123:
         let cl = candle.close;
 
         if let Some(shi) = self.swing_hi
[31m-            && cl > shi && self.last_broken_hi != Some(shi) {
(B[m[31m-                if self.bias_internal <= 0 {
(B[m[31m-                    self.choch = true;
(B[m[31m-                    self.choch_dir = 1;
(B[m[31m-                    self.fib_dir = 1;
(B[m[31m-                    self.fib_hi = Some(candle.high);
(B[m[31m-                    self.fib_lo = self.swing_lo;
(B[m[31m-                } else {
(B[m[31m-                    self.bos = true;
(B[m[31m-                    self.fib_hi = Some(candle.high);
(B[m[31m-                    self.fib_lo = self.swing_lo;
(B[m[31m-                    self.fib_dir = 1;
(B[m[31m-                }
(B[m[31m-                self.bias_internal = 1;
(B[m[31m-                self.last_broken_hi = Some(shi);
(B[m[32m+            && cl > shi
(B[m[32m+            && self.last_broken_hi != Some(shi)
(B[m[32m+        {
(B[m[32m+            if self.bias_internal <= 0 {
(B[m[32m+                self.choch = true;
(B[m[32m+                self.choch_dir = 1;
(B[m[32m+                self.fib_dir = 1;
(B[m[32m+                self.fib_hi = Some(candle.high);
(B[m[32m+                self.fib_lo = self.swing_lo;
(B[m[32m+            } else {
(B[m[32m+                self.bos = true;
(B[m[32m+                self.fib_hi = Some(candle.high);
(B[m[32m+                self.fib_lo = self.swing_lo;
(B[m[32m+                self.fib_dir = 1;
(B[m             }
[32m+            self.bias_internal = 1;
(B[m[32m+            self.last_broken_hi = Some(shi);
(B[m[32m+        }
(B[m         if let Some(slo) = self.swing_lo
[31m-            && cl < slo && self.last_broken_lo != Some(slo) {
(B[m[31m-                if self.bias_internal >= 0 {
(B[m[31m-                    self.choch = true;
(B[m[31m-                    self.choch_dir = -1;
(B[m[31m-                    self.fib_dir = -1;
(B[m[31m-                    self.fib_lo = Some(candle.low);
(B[m[31m-                    self.fib_hi = self.swing_hi;
(B[m[31m-                } else {
(B[m[31m-                    self.bos = true;
(B[m[31m-                    self.fib_lo = Some(candle.low);
(B[m[31m-                    self.fib_hi = self.swing_hi;
(B[m[31m-                    self.fib_dir = -1;
(B[m[31m-                }
(B[m[31m-                self.bias_internal = -1;
(B[m[31m-                self.last_broken_lo = Some(slo);
(B[m[32m+            && cl < slo
(B[m[32m+            && self.last_broken_lo != Some(slo)
(B[m[32m+        {
(B[m[32m+            if self.bias_internal >= 0 {
(B[m[32m+                self.choch = true;
(B[m[32m+                self.choch_dir = -1;
(B[m[32m+                self.fib_dir = -1;
(B[m[32m+                self.fib_lo = Some(candle.low);
(B[m[32m+                self.fib_hi = self.swing_hi;
(B[m[32m+            } else {
(B[m[32m+                self.bos = true;
(B[m[32m+                self.fib_lo = Some(candle.low);
(B[m[32m+                self.fib_hi = self.swing_hi;
(B[m[32m+                self.fib_dir = -1;
(B[m             }
[32m+            self.bias_internal = -1;
(B[m[32m+            self.last_broken_lo = Some(slo);
(B[m[32m+        }
(B[m 
         self.bias = self.bias_internal;
 
Diff in /home/jordan/github/indicators-ta/src/structure.rs:162:
         if let (Some(fh), Some(fl)) = (self.fib_hi, self.fib_lo)
[31m-            && self.fib_dir != 0 {
(B[m[31m-                self.compute_fibs(fh, fl, self.fib_dir);
(B[m[31m-            }
(B[m[32m+            && self.fib_dir != 0
(B[m[32m+        {
(B[m[32m+            self.compute_fibs(fh, fl, self.fib_dir);
(B[m[32m+        }
(B[m 
         if let (Some(f5), dir) = (self.fib500, self.fib_dir) {
             if dir != 0 {
Diff in /home/jordan/github/indicators-ta/src/structure.rs:169:
                 if dir == 1 {
                     self.in_discount = cl <= f5;
[31m-                    self.in_premium  = cl >  f5;
(B[m[32m+                    self.in_premium = cl > f5;
(B[m                 } else {
[31m-                    self.in_premium  = cl >= f5;
(B[m[31m-                    self.in_discount = cl <  f5;
(B[m[32m+                    self.in_premium = cl >= f5;
(B[m[32m+                    self.in_discount = cl < f5;
(B[m                 }
             }
         } else {
Diff in /home/jordan/github/indicators-ta/src/structure.rs:178:
             self.in_discount = false;
[31m-            self.in_premium  = false;
(B[m[32m+            self.in_premium = false;
(B[m         }
 
         // Fibonacci confluence score
Diff in /home/jordan/github/indicators-ta/src/structure.rs:183:
         let tol = atr * 0.3;
         let mut score = 0.0_f64;
[31m-        if self.fib382.is_some_and(|f| (cl - f).abs() < tol) { score += 1.5; }
(B[m[31m-        if self.fib500.is_some_and(|f| (cl - f).abs() < tol) { score += 2.0; }
(B[m[31m-        if self.fib618.is_some_and(|f| (cl - f).abs() < tol) { score += 2.5; }
(B[m[31m-        if self.fib786.is_some_and(|f| (cl - f).abs() < tol) { score += 1.5; }
(B[m[32m+        if self.fib382.is_some_and(|f| (cl - f).abs() < tol) {
(B[m[32m+            score += 1.5;
(B[m[32m+        }
(B[m[32m+        if self.fib500.is_some_and(|f| (cl - f).abs() < tol) {
(B[m[32m+            score += 2.0;
(B[m[32m+        }
(B[m[32m+        if self.fib618.is_some_and(|f| (cl - f).abs() < tol) {
(B[m[32m+            score += 2.5;
(B[m[32m+        }
(B[m[32m+        if self.fib786.is_some_and(|f| (cl - f).abs() < tol) {
(B[m[32m+            score += 1.5;
(B[m[32m+        }
(B[m         self.confluence = (score * 10.0).min(100.0);
     }
 
Diff in /home/jordan/github/indicators-ta/src/structure.rs:192:
     fn pivot_high(&self) -> Option<f64> {
         let arr: Vec<f64> = self.highs.iter().copied().collect();
         let n = self.swing_len;
[31m-        if arr.len() < 2 * n + 1 { return None; }
(B[m[32m+        if arr.len() < 2 * n + 1 {
(B[m[32m+            return None;
(B[m[32m+        }
(B[m         let mid = arr[arr.len() - n - 1];
[31m-        let left_ok  = (1..=n).all(|i| mid >= arr[arr.len() - n - 1 - i]);
(B[m[32m+        let left_ok = (1..=n).all(|i| mid >= arr[arr.len() - n - 1 - i]);
(B[m         let right_ok = (1..=n).all(|i| mid >= arr[arr.len() - n - 1 + i]);
         if left_ok && right_ok { Some(mid) } else { None }
     }
Diff in /home/jordan/github/indicators-ta/src/structure.rs:202:
     fn pivot_low(&self) -> Option<f64> {
         let arr: Vec<f64> = self.lows.iter().copied().collect();
         let n = self.swing_len;
[31m-        if arr.len() < 2 * n + 1 { return None; }
(B[m[32m+        if arr.len() < 2 * n + 1 {
(B[m[32m+            return None;
(B[m[32m+        }
(B[m         let mid = arr[arr.len() - n - 1];
[31m-        let left_ok  = (1..=n).all(|i| mid <= arr[arr.len() - n - 1 - i]);
(B[m[32m+        let left_ok = (1..=n).all(|i| mid <= arr[arr.len() - n - 1 - i]);
(B[m         let right_ok = (1..=n).all(|i| mid <= arr[arr.len() - n - 1 + i]);
         if left_ok && right_ok { Some(mid) } else { None }
     }
Diff in /home/jordan/github/indicators-ta/src/structure.rs:211:
 
     fn compute_fibs(&mut self, hi: f64, lo: f64, direction: i8) {
         let rng = hi - lo;
[31m-        if rng <= 0.0 { return; }
(B[m[32m+        if rng <= 0.0 {
(B[m[32m+            return;
(B[m[32m+        }
(B[m         if direction == 1 {
             self.fib382 = Some(hi - rng * 0.382);
             self.fib500 = Some(hi - rng * 0.500);
```

---

## cargo clippy

> Lints for correctness, style, and performance issues.
> Fix with: `cargo clippy --fix`

```
Compiling proc-macro2 v1.0.106
   Compiling quote v1.0.45
   Compiling unicode-ident v1.0.24
   Compiling serde_core v1.0.228
   Compiling autocfg v1.5.0
   Compiling zmij v1.0.21
   Compiling libc v0.2.184
   Compiling serde v1.0.228
    Checking cfg-if v1.0.4
    Checking rand_core v0.10.0
   Compiling getrandom v0.4.2
   Compiling serde_json v1.0.149
    Checking cpufeatures v0.3.0
    Checking iana-time-zone v0.1.65
    Checking memchr v2.8.0
    Checking itoa v1.0.18
    Checking chacha20 v0.10.0
   Compiling num-traits v0.2.19
   Compiling syn v2.0.117
    Checking rand v0.10.0
   Compiling serde_derive v1.0.228
    Checking chrono v0.4.44
    Checking indicators-ta v0.1.0 (/home/jordan/github/indicators-ta)
error: binding's name is too similar to existing binding
   --> src/primitives.rs:263:17
    |
263 |             let plus_di = (plus_dm_smooth / atr_val) * 100.0;
    |                 ^^^^^^^
    |
note: existing binding defined here
   --> src/primitives.rs:226:14
    |
226 |         let (plus_dm, minus_dm) = match (self.prev_high, self.prev_low) {
    |              ^^^^^^^
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#similar_names
    = note: `-D clippy::similar-names` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::similar_names)]`

error: binding's name is too similar to existing binding
   --> src/primitives.rs:264:17
    |
264 |             let minus_di = (minus_dm_smooth / atr_val) * 100.0;
    |                 ^^^^^^^^
    |
note: existing binding defined here
   --> src/primitives.rs:226:23
    |
226 |         let (plus_dm, minus_dm) = match (self.prev_high, self.prev_low) {
    |                       ^^^^^^^^
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#similar_names

error: could not compile `indicators-ta` (lib) due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `indicators-ta` (lib test) due to 2 previous errors
```

---

## cargo test

> Runs the full test suite including doc-tests.

```
running 110 tests
test detector::tests::test_conservative_creation ... ok
test detector::tests::test_crypto_optimized_creation ... ok
test detector::tests::test_detector_creation ... ok
test detector::tests::test_last_close_tracking ... ok
test detector::tests::test_bars_in_regime_increments ... ok
test detector::tests::test_adx_atr_accessors ... ok
test detector::tests::test_confidence_range ... ok
test detector::tests::test_warmup_returns_uncertain ... ok
test detector::tests::test_ranging_detection ... ok
test detector::tests::test_metrics_populated_after_warmup ... ok
test ensemble::tests::test_agreement_rate_empty ... ok
test detector::tests::test_recommended_strategy ... ok
test detector::tests::test_stability_filter_prevents_whipsaw ... ok
test ensemble::tests::test_balanced_creation ... ok
test ensemble::tests::test_combine_results_agreement_boosts_confidence ... ok
test detector::tests::test_set_config_resets_state ... ok
test ensemble::tests::test_combine_results_disagreement_returns_uncertain_at_low_conf ... ok
test detector::tests::test_trending_bullish_direction ... ok
test ensemble::tests::test_agreement_rate_tracked ... ok
test ensemble::tests::test_detector_accessors ... ok
test detector::tests::test_trending_bearish_direction ... ok
test ensemble::tests::test_ensemble_creation ... ok
test ensemble::tests::test_ensemble_result_display ... ok
test ensemble::tests::test_ensemble_result_disagreement_display ... ok
test ensemble::tests::test_ensemble_to_regime_confidence ... ok
test ensemble::tests::test_expected_regime_duration ... ok
test detector::tests::test_regime_history_tracking ... ok
test ensemble::tests::test_hmm_focused_creation ... ok
test detector::tests::test_trending_detection ... ok
test ensemble::tests::test_indicator_focused_creation ... ok
test ensemble::tests::test_regimes_agree_direction ... ok
test ensemble::tests::test_regimes_agree_same_category ... ok
test ensemble::tests::test_regimes_disagree_different_category ... ok
test ensemble::tests::test_status_display ... ok
test functions::tests::test_ema_incremental ... ok
test functions::tests::test_ema_sma_seed ... ok
test functions::tests::test_true_range_first ... ok
test hmm::tests::test_expected_regime_duration ... ok
test ensemble::tests::test_hmm_state_probabilities_accessible ... ok
test hmm::tests::test_hmm_conservative_config ... ok
test hmm::tests::test_hmm_crypto_config ... ok
test hmm::tests::test_hmm_initialization ... ok
test hmm::tests::test_hmm_warmup ... ok
test hmm::tests::test_n_observations_tracking ... ok
test hmm::tests::test_state_parameters ... ok
test hmm::tests::test_hmm_becomes_ready ... ok
test hmm::tests::test_transition_matrix_rows_sum_to_one ... ok
test primitives::tests::test_adx_creation ... ok
test primitives::tests::test_adx_di_values ... ok
test primitives::tests::test_adx_reset ... ok
test hmm::tests::test_predict_next_state ... ok
test primitives::tests::test_adx_trend_direction ... ok
test primitives::tests::test_adx_trending_detection ... ok
test hmm::tests::test_state_probabilities_sum_to_one ... ok
test primitives::tests::test_atr_increases_with_volatility ... ok
test primitives::tests::test_atr_reset ... ok
test primitives::tests::test_atr_warmup ... ok
test primitives::tests::test_bb_band_ordering ... ok
test primitives::tests::test_bb_creation ... ok
test primitives::tests::test_bb_overbought_oversold ... ok
test primitives::tests::test_bb_percent_b ... ok
test hmm::tests::test_update_ohlc_uses_close ... ok
test primitives::tests::test_bb_reset ... ok
test hmm::tests::test_bull_market_detection ... ok
test hmm::tests::test_confidence_range ... ok
test primitives::tests::test_bb_warmup ... ok
test primitives::tests::test_calculate_sma ... ok
test primitives::tests::test_calculate_sma_precision ... ok
test primitives::tests::test_atr_creation ... ok
test primitives::tests::test_ema_calculation ... ok
test primitives::tests::test_ema_creation ... ok
test primitives::tests::test_ema_reset ... ok
test primitives::tests::test_bb_squeeze_detection ... ok
test primitives::tests::test_ema_tracks_trend ... ok
test primitives::tests::test_ema_warmup ... ok
test primitives::tests::test_rsi_bearish_market ... ok
test primitives::tests::test_rsi_bullish_market ... ok
test primitives::tests::test_rsi_creation ... ok
test hmm::tests::test_volatile_market_detection ... ok
test primitives::tests::test_rsi_range ... ok
test primitives::tests::test_rsi_reset_clears_value ... ok
test primitives::tests::test_rsi_value_cached ... ok
test router::tests::test_active_strategy_display ... ok
test router::tests::test_asset_registration ... ok
test router::tests::test_asset_unregistration ... ok
test router::tests::test_asset_summary_display ... FAILED
test router::tests::test_auto_registration ... ok
test router::tests::test_compute_strategy_low_confidence ... ok
test router::tests::test_compute_strategy_mean_reverting ... ok
test router::tests::test_compute_strategy_trending ... ok
test router::tests::test_compute_strategy_uncertain ... ok
test router::tests::test_compute_strategy_volatile ... ok
test router::tests::test_detection_method_display ... ok
test ensemble::tests::test_bull_market_agreement ... ok
test router::tests::test_ensemble_signal_has_agreement ... ok
test router::tests::test_hmm_signal_has_state_probs ... ok
test router::tests::test_initial_regime_is_uncertain ... ok
test router::tests::test_is_ready_unknown_asset ... ok
test router::tests::test_method_switching ... ok
test router::tests::test_duplicate_registration_noop ... ok
test router::tests::test_not_ready_before_warmup ... ok
test router::tests::test_registered_assets ... ok
test router::tests::test_routed_signal_display ... ok
test router::tests::test_routed_signal_fields ... ok
test router::tests::test_router_creation_ensemble ... ok
test router::tests::test_router_creation_hmm ... ok
test router::tests::test_router_creation_indicators ... ok
test router::tests::test_summary ... ok
test ensemble::tests::test_ready_state ... ok
test router::tests::test_regime_changes_counted ... ok

failures:

---- router::tests::test_asset_summary_display stdout ----

thread 'router::tests::test_asset_summary_display' (477022) panicked at src/router.rs:912:9:
assertion failed: display.contains("{regime_changes}")
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    router::tests::test_asset_summary_display

test result: FAILED. 109 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
   Compiling indicators-ta v0.1.0 (/home/jordan/github/indicators-ta)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.05s
     Running unittests src/lib.rs (target/debug/deps/indicators-50faea5bca6d0575)
error: test failed, to rerun pass `--lib`
```

---

## cargo doc

> Verifies documentation compiles without warnings.

```
Documenting indicators-ta v0.1.0 (/home/jordan/github/indicators-ta)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.48s
   Generated /home/jordan/github/indicators-ta/target/doc/indicators/index.html
```

---

*Report generated by `scripts/lint_report`*
