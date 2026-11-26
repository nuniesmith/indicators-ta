# AGENTS.md — indicators-ta

Technical analysis indicators and market regime detection for algorithmic trading, in Rust, published on crates.io. Two API shapes for most indicators: **batch functions** over price slices (`ema()`, `sma()`, `atr()`, `rsi()`, `macd()`, …) and **incremental structs** with O(1) per-tick updates and SMA warm-up.

## Shared context

Read the shared agent context first: `soul.md` and `memory.md` (especially §9) in the [agents repo](https://github.com/nuniesmith/agents).

## Rules

- **Batch and incremental must agree.** For any indicator that exists in both shapes, the incremental result after warm-up must match the batch result on the same series — that's the property the tests exist to pin. Changing one shape without the other is a bug, even if tests pass.
- Warm-up behaviour is part of the contract: document how many bars an incremental indicator needs before its output is meaningful, and don't silently change it.
- Downstream (rustrade, the FKS stack, the crypto bot's regime rule) depends on exact numeric behaviour. A "cleanup" that changes floating-point results is a breaking change — flag it as one.
- Publishing runs through the reusable crates.io lane in `nuniesmith/actions`; per-repo `CRATES_IO_TOKEN`.
- Verify with `cargo test` (benches compile too), clippy and fmt; a PR is ready only when its own CI is green on its own head.
