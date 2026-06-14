# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

This file was backfilled from git history and the crates.io release record at
v0.1.5; entries from before it was introduced are reconstructed from commits
and may be coarser than going-forward entries.

## [Unreleased]

## [0.2.2] - 2026-06-14

### Fixed
- **Batch `SignalIndicator` adapter ignored its tuned `engine_cfg`.**
  `SignalIndicator::calculate` constructed the engine from `self.engine_cfg`
  but then built a fresh `IndicatorConfig::default()` and passed *that* to
  `compute_signal`, so a tuned `signal_mode`, `conf_min_score`,
  `hurst_threshold`, etc. were silently dropped in the aggregation step. It now
  passes `self.engine_cfg`, so the adapter honours the configured signal mode
  and thresholds. (Per-bar `compute_signal` callers were never affected.)

### Documentation
- Noted on `functions::atr` and the `Atr` indicator that their smoothing is
  EMA/SMA (Python-parity), **not** Wilder's RMA, so consumers expecting a
  Wilder-style ATR aren't surprised by the differing values.

## [0.2.1] - 2026-06-12

> Version **0.2.0 was never published**: its tag was accidentally cut on the
> v0.1.5 commit, so the publish job's tag/version guard rejected it. The
> release shipped as 0.2.1 with identical content (plus the version bump).

### Changed (breaking)
- **Unified the incremental warm-up contract**: every `Incremental*` struct's
  `update` now returns `Option<T>` — `None` strictly means "no value defined
  yet". `IncrementalEma::update` changed from `f64` to `Option<f64>` and
  `IncrementalMacd::update` from `(f64, f64, f64)` to
  `Option<(f64, f64, f64)>` (both always `Some`, since their EMAs seed from
  the first tick — the change is signature-level so all five structs share one
  consumer pattern, and leaves room to gate warm-up later without another
  break). Numerics are unchanged.
- **`IndicatorConfig` gained an `engine: SignalEngineConfig` field** holding
  the previously hard-coded signal-engine internals: KMeans recompute cadence
  (10 bars), KMeans iteration cap (100), and Hurst recompute cadence (10
  bars). The field is `#[serde(default)]`, so existing tuned JSON files load
  unchanged; only exhaustive struct-literal construction breaks. Defaults
  reproduce the old behaviour exactly.
- Dependency removals below (`polars`/`anyhow`) are also part of this bump.

### Added
- Property-based tests (`tests/property_tests.rs`, `proptest` dev-dependency)
  for the numerically sensitive paths: HMM state-probability normalisation,
  parabolic-SAR hull bounds across flips, signal-engine aggregation vote
  domain, and bound/ordering/hull invariants for the batch functions and all
  incremental structs — driven by arbitrary well-formed market data.
- Criterion benchmark group `incremental_10k_ticks` covering the per-tick cost
  of `IncrementalEma` / `IncrementalRsi` / `IncrementalMacd` /
  `IncrementalBollinger` / `IncrementalAtr`, so the streaming path stays fast
  as it grows.
- Documented the incremental warm-up contract (a per-struct table in the
  `functions` module docs): which structs return `Option` vs plain values,
  when the first defined value appears, and that NaN inputs poison state
  without panicking.
- **`IncrementalRsi` / `IncrementalMacd` / `IncrementalBollinger`** — streaming
  structs (re-exported at the crate root) mirroring the batch `rsi` / `macd` /
  Bollinger formulas, completing the incremental indicator set alongside
  `IncrementalEma` / `IncrementalAtr`. RSI/MACD compose from `IncrementalEma`;
  Bollinger keeps a rolling window and uses the sample stddev (ddof = 1) to
  match the batch.
- NaN-robustness regression tests (`tests/nan_robustness.rs`) feeding poisoned
  candles through the signal engine, liquidity profile, market structure, HMM,
  regime detector, and ensemble — locking in the panic fixes below.
- Supply-chain CI: `cargo-deny` (advisories / licenses / sources, see
  `deny.toml`) and `cargo-semver-checks` (breaking-change detection against the
  last published release) now gate every PR and the publish job.
- `[package.metadata.docs.rs] all-features = true` so docs.rs always builds the
  full surface.

### Fixed
- **NaN inputs no longer panic the streaming engines.** All
  `partial_cmp().unwrap()` comparisons (signal-engine KMeans, liquidity POC,
  HMM state argmax) now use the NaN-safe `f64::total_cmp`; a stale tick or
  zero-volume bar producing NaN degrades gracefully instead of crashing a live
  loop. Remaining guarded `unwrap()`s in the regime detector / aggregator /
  engine were replaced with destructuring that cannot panic, and
  `compute_kmeans_centroids` no longer indexes into an empty buffer.
- **`MarketStructure` ATR used swapped Wilder coefficients** — it weighted the
  current true range at 13/14 and the previous ATR at 1/14 (the reverse of
  Wilder smoothing, and of the engine's own `rma_step`), making the swing
  significance filter track raw bar range instead of a smoothed ATR. Now
  `tr/14 + prev*13/14` as the "Wilder 1/14" comment always intended.
- Registry no longer panics on a poisoned `RwLock` — it recovers the guard
  (entries are plain fn-pointer inserts, so no torn state is possible).

### Removed
- Dropped unused `polars` and `anyhow` dependencies. Neither was referenced
  anywhere in `src/`; removing them shrinks the resolved dependency graph from
  154 crates to 21 for every downstream consumer.

## [0.1.5] - 2026-05-31

### Fixed
- Repaired pre-existing test-suite breakage so CI is green again (updates across
  `trend`, `volume`, `signal_pipeline`, `macd_atr_stc`, and `registry_fuzz`
  tests). No library API change — a maintenance/release-hygiene patch.

### Changed
- Aligned the MSRV CI job to the declared `rust-version = "1.94.1"` (the job was
  still pinned to 1.92.0), so CI now actually enforces the documented minimum.

## [0.1.4] - 2026-05-31

### Added
- **`IncrementalEma` / `IncrementalAtr`** — streaming O(1) variants that return
  the updated value on each `update` (seeding from the first sample),
  complementing the existing `EMA` / `ATR` structs whose
  `update` / `value` / `is_ready` API suits batch warm-up. Re-exported at the
  crate root (3 unit tests added).
- This made `indicators-ta` a strict superset of janus's internal
  `jflow-indicators` (its `IncrementalEma` / `IncrementalAtr`), unblocking
  janus's TA consolidation onto this crate.

## [0.1.1] - 2026-04-11 — [0.1.3] - 2026-04-12

Early iteration on the indicator suite, the signal engine, and regime detection
after the initial publish. Granular per-patch history for the 0.1.1–0.1.3 line
predates this changelog (see git log); the cumulative result is the library
described under 0.1.0 below, hardened and extended.

## [0.1.0] - 2026-04-09

### Added
- Initial public release. A broad, tested technical-analysis library imported as
  `indicators`:
  - **`trend`** — EMA / SMA / WMA / MACD / linear-regression / parabolic-SAR.
  - **`momentum`** — RSI / Stochastic / StochasticRSI / Williams %R / STC.
  - **`volatility`** — Bollinger / Keltner / Choppiness / Elder-Ray / market-cycle.
  - **`volume`** — ADL / CMF / VZO.
  - **`signal`** — an 11-layer signal engine (VWAP / confluence / liquidity /
    structure / CVD / …).
  - **`regime`** — market-regime detection (indicator-based + HMM + ensemble +
    strategy router).
- Batch functions over slices **plus** incremental O(1) structs (`EMA` / `ATR`)
  for streaming, a typed `IndicatorError`, and an indicator registry.
- CI gates: `fmt`, `clippy`, `test`, `docs`, and an MSRV check.

[Unreleased]: https://github.com/nuniesmith/indicators-ta/compare/v0.2.2...HEAD
[0.2.2]: https://github.com/nuniesmith/indicators-ta/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/nuniesmith/indicators-ta/compare/v0.1.5...v0.2.1
[0.1.5]: https://github.com/nuniesmith/indicators-ta/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/nuniesmith/indicators-ta/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/nuniesmith/indicators-ta/compare/v0.1.0...v0.1.3
[0.1.0]: https://github.com/nuniesmith/indicators-ta/releases/tag/v0.1.0
