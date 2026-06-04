# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

This file was backfilled from git history and the crates.io release record at
v0.1.5; entries from before it was introduced are reconstructed from commits
and may be coarser than going-forward entries.

## [Unreleased]

### Added
- **`IncrementalRsi` / `IncrementalMacd`** — O(1) streaming structs (re-exported
  at the crate root) composed from `IncrementalEma`, mirroring the batch `rsi` /
  `macd` formulas. Extends the streaming set beyond `IncrementalEma` /
  `IncrementalAtr`. (`IncrementalBollinger` is still open.)

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

[Unreleased]: https://github.com/nuniesmith/indicators-ta/compare/v0.1.5...HEAD
[0.1.5]: https://github.com/nuniesmith/indicators-ta/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/nuniesmith/indicators-ta/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/nuniesmith/indicators-ta/compare/v0.1.0...v0.1.3
[0.1.0]: https://github.com/nuniesmith/indicators-ta/releases/tag/v0.1.0
