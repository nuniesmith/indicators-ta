# indicators-ta — TODO / Roadmap

> Technical-analysis indicators + market-regime detection. Published on
> crates.io (**0.1.5**), imported as `indicators`. Consumed by **janus**
> (its TA crate, post-consolidation) and by the **fks-full** bots
> (`crypto-demo`). Part of the FKS stack — see
> [`fks-full/docs/MULTI_ASSET_BRAIN_ROADMAP.md`](https://github.com/nuniesmith/fks-full/blob/main/docs/MULTI_ASSET_BRAIN_ROADMAP.md).

## Where things stand (2026-06, v0.1.5)

A broad, tested indicator library: `trend` (EMA/SMA/WMA/MACD/linear-regression/
parabolic-SAR), `momentum` (RSI/Stochastic/StochasticRSI/Williams %R/STC),
`volatility` (Bollinger/Keltner/Choppiness/Elder-Ray/market-cycle), `volume`
(ADL/CMF/VZO), an 11-layer `signal` engine (VWAP/confluence/liquidity/structure/
CVD/…), and `regime` detection (indicator + HMM + ensemble + strategy router).
Batch functions over slices **plus** incremental O(1) structs (`EMA`/`ATR`/
`IncrementalEma`/`IncrementalAtr`) for streaming. ~357 tests; CI gates
fmt/clippy/test/docs/MSRV (1.94.1).

## Recommended next steps (priority order)

1. **✅ `CHANGELOG.md` added** (this PR) — backfilled 0.1.0 → 0.1.5 from git
   history in Keep-a-Changelog format. The backfill corrected a loose
   attribution: **0.1.4** is the incremental EMA/ATR release, and **0.1.5** was
   a test-suite repair + MSRV-CI alignment patch (no API change). Keep it
   current on each release. Residual: cut git tags (`v0.1.0`…`v0.1.5`) so the
   `compare` links resolve, and pair with docs.rs/crates.io hygiene.

2. **Consumer-driven API requests (from the FKS brain roadmap).** As janus's
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
   - [ ] Confirm every indicator the janus strategy suite uses is present and
         matches janus's previous `jflow-indicators` numerics (the consolidation
         relied on name+behaviour parity — keep it that way on future changes).

3. **Quality / supply-chain CI** (mirror exchange-apiws): `cargo-deny`
   (advisories + licenses), `cargo-semver-checks` against the last published
   version (catch accidental breaking changes — this crate is a published dep
   of janus + the bots), optional coverage badge.

4. **`docs.rs` polish.** `package.metadata.docs.rs` with `all-features = true`
   so the full surface builds on docs.rs; per-module doc examples for the
   signal engine + regime detectors (the README covers batch indicators well,
   but the signal/regime APIs are the least-documented).

## Backlog / nice-to-have

- [ ] **Property/fuzz tests** for the numerically-sensitive paths (HMM
      forward/backward, the signal-engine aggregation, parabolic SAR flips).
- [ ] **Benchmarks** (`criterion`) for the incremental structs + the signal
      engine, so the streaming path stays fast as it grows.
- [ ] **More incremental indicators** as consumers need them (incremental
      RSI/MACD/Bollinger) — today only EMA/ATR have O(1) streaming structs.
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
