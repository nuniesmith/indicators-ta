Now I have a thorough picture of both repos. Let me build the full mirrored structure.Now let me update `IndicatorError` in `functions.rs` to add the `UnknownIndicator` variant, and fix `indicator.rs` to match:Now fix `indicator.rs` to use the existing error shape:Now I need to move `IndicatorError` to `types.rs` since `indicator.rs` imports from there. Let me check the current `types.rs`:Now create all the indicator modules. Starting with trend:---

## Review & Proposed Structure

Here's what I found after deep-reading both repos, and the full plan for mirroring Python's clean structure into Rust.

---

### Current State of the Rust Repo — Problems

| Issue | Detail |
|---|---|
| **No `Indicator` trait** | Python has an abstract `Indicator` base class; Rust has nothing analogous — every indicator is a one-off struct with no shared contract |
| **No registry** | Python has `@register_indicator` + `IndicatorRegistry`; Rust has no way to create indicators by name at runtime |
| **No category modules** | Everything is flat in `src/`; Python is cleanly split into `trend/`, `momentum/`, `volume/`, `other/` |
| **No tests** | No `#[cfg(test)]` blocks anywhere; Python has validated math |
| **`IndicatorError` too narrow** | Only 2 variants — missing `UnknownIndicator`, `InvalidParam` needed for registry + new indicators |
| **Duplication** | `EMA` struct exists in both `primitives.rs` and `functions.rs` with different semantics |

---

### New Structure Created / Planned

```
src/
  indicator.rs          ✅ NEW — Indicator + StreamingIndicator traits (mirrors base.py)
  registry.rs           ✅ NEW — IndicatorRegistry + DynIndicator (mirrors registry.py)
  functions.rs          ✅ UPDATED — added UnknownIndicator, InvalidParam error variants

  trend/
    mod.rs              ✅ NEW
    sma.rs              🔲 stub + tests
    ema.rs              🔲 stub + tests
    wma.rs              🔲 stub + tests
    macd.rs             🔲 stub + tests
    vwap.rs             🔲 stub + tests
    adl.rs              🔲 stub + tests
    volatility/
      mod.rs            🔲
      atr.rs            🔲 stub + tests
      bollinger.rs      🔲 stub + tests
      keltner.rs        🔲 stub + tests

  momentum/
    mod.rs              🔲
    rsi.rs              🔲 stub + tests
    stochastic.rs       🔲 stub + tests

  volume/
    mod.rs              🔲
    vzo.rs              🔲 stub + tests
    vwap.rs             🔲 stub + tests

  other/
    mod.rs              🔲
    chaikin_money_flow.rs, choppiness_index.rs,
    elder_ray_index.rs, linear_regression.rs,
    market_cycle.rs, parabolic_sar.rs,
    schaff_trend_cycle.rs, williams_r.rs  🔲 all stubs
```