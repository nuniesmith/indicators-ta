# indicators-ta — TODO / Roadmap

> Technical-analysis indicators + market-regime detection. Published on
> crates.io (**0.2.1**), imported as `indicators`. A standalone library —
> downstream consumers (e.g. janus and trading bots) depend on it from
> crates.io.

## Where things stand (2026-06, v0.2.1)

A broad, tested indicator library: `trend` (EMA/SMA/WMA/MACD/linear-regression/
parabolic-SAR), `momentum` (RSI/Stochastic/StochasticRSI/Williams %R/STC),
`volatility` (Bollinger/Keltner/Choppiness/Elder-Ray/market-cycle), `volume`
(ADL/CMF/VZO), an 11-layer `signal` engine (VWAP/confluence/liquidity/structure/
CVD/…), and `regime` detection (indicator + HMM + ensemble + strategy router).
Batch functions over slices **plus** incremental streaming structs with a
unified `update → Option<T>` warm-up contract (EMA/ATR/RSI/MACD/Bollinger).
NaN-hardened hot paths (no panicking float comparisons), 394 tests incl.
property tests + NaN-robustness regression suites; CI gates fmt/clippy/test/
docs/MSRV (1.94.1) + cargo-deny + cargo-semver-checks. Lean dependency tree
(21 crates resolved).

**0.2.x migration for consumers** (0.1.5 → 0.2.1; 0.2.0 was never published): `IncrementalEma::update` and
`IncrementalMacd::update` now return `Option` (always `Some` — wrap call
sites in `.unwrap()` or `let Some(v) = …`); `IndicatorConfig` gained a
`#[serde(default)]` `engine` field (tuned JSON loads unchanged).

## Recommended next steps (priority order)

1. **✅ `CHANGELOG.md` added** (this PR) — backfilled 0.1.0 → 0.1.5 from git
   history in Keep-a-Changelog format. The backfill corrected a loose
   attribution: **0.1.4** is the incremental EMA/ATR release, and **0.1.5** was
   a test-suite repair + MSRV-CI alignment patch (no API change). Keep it
   current on each release. Residual: the `v0.1.4` tag is still missing (the
   other `v0.1.x` tags exist), so that one `compare` link 404s — note that
   pushing any `v*` tag triggers the publish workflow, so cutting it
   retroactively will produce a failed (already-published) publish run.

2. **Consumer-driven API requests.** As janus's
   live loop starts feeding the regime detector and emitting regime/fear
   (janus TODO P0), expect requests for:
   - [x] A streaming/incremental wrapper for the **regime ensemble** — confirmed
         present: `EnsembleRegimeDetector::update(h,l,c)` + `is_ready()` gating
         (`src/regime/ensemble.rs`) are a clean per-candle API for the live loop.
   - [x] Incremental variants of the **signal-engine** layers — already present:
         LiquidityProfile, ConfluenceEngine, MarketStructure, CVDTracker,
         VolatilityPercentile, SignalStreak each expose an O(1) `update(&candle)`
         (the batch `compute_signal` path remains for backtests). A single wrapper
         bundling all layers is the only residual, if the live loop wants one.
   - [x] Confirmed — `tests/consumer_parity.rs` pins the numerics of the exact
         surface janus consumes (`ema`/`sma`/`rsi`/`macd`/`atr` + the `EMA`/`ATR`
         incremental structs + `IndicatorCalculator`) with spec-derived golden
         values + definitional invariants. `jflow-indicators` is retired, so this
         regression-locks current behaviour rather than cross-checking the old
         impl — keeping the consolidation stable on future changes.

3. **✅ Quality / supply-chain CI** — `cargo-deny` (advisories + licenses +
   sources, `deny.toml`) and `cargo-semver-checks` (vs. last published
   version) gate every PR and the publish job. Residual: optional coverage
   badge.

4. **`docs.rs` polish.** `package.metadata.docs.rs` with `all-features = true`
   so the full surface builds on docs.rs; per-module doc examples for the
   signal engine + regime detectors (the README covers batch indicators well,
   but the signal/regime APIs are the least-documented).

## Backlog / nice-to-have

- [x] **Property/fuzz tests** for the numerically-sensitive paths — present:
      `tests/property_tests.rs` (proptest) covers HMM state-prob
      normalisation, parabolic SAR flips, signal aggregation, and the
      batch/incremental bound invariants.
- [x] **Benchmarks** (`criterion`) for the incremental structs + the signal
      engine — `benches/indicators.rs` covers the engine (replay/hot-bar),
      the full signal pipeline, the per-bar streaming loop, and the
      `incremental_10k_ticks` group for the streaming structs.
- [x] **More incremental indicators** — `IncrementalRsi`, `IncrementalMacd`, and
      `IncrementalBollinger` added (mirror the batch `rsi`/`macd`/Bollinger). The
      streaming set now covers EMA / ATR / RSI / MACD / Bollinger.
- [ ] Consider a `no_std`-friendly core for the pure-math functions (only if a
      consumer ever needs it — not speculative work).

## Done

- v0.1.5 — test-suite repair (CI green) + MSRV-CI alignment to 1.94.1
  (maintenance patch; no API change).
- v0.1.4 — `IncrementalEma` / `IncrementalAtr` (streaming O(1), return value on
  each update), re-exported at the crate root. Made the crate a strict superset
  of janus's retired `jflow-indicators`, unblocking that consolidation.
- v0.1.0–0.1.3 — the indicator library + signal engine + regime detection +
  CI gates. (Now backfilled into `CHANGELOG.md`.)
