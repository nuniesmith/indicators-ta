# Rust Lint Report

| | |
|---|---|
| **Generated** | 2026-04-10 00:54:51 |
| **Workspace** | `/home/jordan/github/indicators-ta` |
| **Overall** | ❌ One or more checks failed |

---

## Summary

| Check | Status | Errors | Warnings | Time |
|-------|--------|--------|----------|------|
| `cargo fmt --check` | ✅ Pass | 0 | 0 | 0.09s |
| `cargo clippy` | ✅ Pass | 0 | 0 | 1.32s |
| `cargo test` | ❌ 1 error(s) | 1 | 0 | 0.57s |
| `cargo doc` | ✅ Pass | 0 | 0 | 0.49s |

---

## cargo fmt

> Checks that all source files match `rustfmt` formatting rules.
> Fix with: `cargo fmt --all`

```
No output.
```

---

## cargo clippy

> Lints for correctness, style, and performance issues.
> Fix with: `cargo clippy --fix`

```
Checking indicators-ta v0.1.0 (/home/jordan/github/indicators-ta)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.28s
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
test detector::tests::test_ranging_detection ... ok
test detector::tests::test_warmup_returns_uncertain ... ok
test ensemble::tests::test_agreement_rate_empty ... ok
test ensemble::tests::test_balanced_creation ... ok
test detector::tests::test_metrics_populated_after_warmup ... ok
test ensemble::tests::test_combine_results_agreement_boosts_confidence ... ok
test ensemble::tests::test_agreement_rate_tracked ... ok
test ensemble::tests::test_combine_results_disagreement_returns_uncertain_at_low_conf ... ok
test ensemble::tests::test_detector_accessors ... ok
test detector::tests::test_recommended_strategy ... ok
test ensemble::tests::test_ensemble_creation ... ok
test detector::tests::test_stability_filter_prevents_whipsaw ... ok
test detector::tests::test_set_config_resets_state ... ok
test ensemble::tests::test_ensemble_result_disagreement_display ... ok
test detector::tests::test_trending_bullish_direction ... ok
test ensemble::tests::test_ensemble_result_display ... ok
test detector::tests::test_trending_bearish_direction ... ok
test detector::tests::test_trending_detection ... ok
test ensemble::tests::test_hmm_focused_creation ... ok
test detector::tests::test_regime_history_tracking ... ok
test ensemble::tests::test_ensemble_to_regime_confidence ... ok
test ensemble::tests::test_indicator_focused_creation ... ok
test ensemble::tests::test_expected_regime_duration ... ok
test ensemble::tests::test_regimes_agree_direction ... ok
test ensemble::tests::test_regimes_agree_same_category ... ok
test ensemble::tests::test_regimes_disagree_different_category ... ok
test ensemble::tests::test_status_display ... ok
test functions::tests::test_ema_incremental ... ok
test functions::tests::test_ema_sma_seed ... ok
test functions::tests::test_true_range_first ... ok
test hmm::tests::test_expected_regime_duration ... ok
test hmm::tests::test_hmm_conservative_config ... ok
test ensemble::tests::test_hmm_state_probabilities_accessible ... ok
test hmm::tests::test_hmm_initialization ... ok
test hmm::tests::test_hmm_crypto_config ... ok
test hmm::tests::test_hmm_becomes_ready ... ok
test hmm::tests::test_state_parameters ... ok
test hmm::tests::test_n_observations_tracking ... ok
test hmm::tests::test_transition_matrix_rows_sum_to_one ... ok
test hmm::tests::test_hmm_warmup ... ok
test primitives::tests::test_adx_creation ... ok
test primitives::tests::test_adx_di_values ... ok
test primitives::tests::test_adx_reset ... ok
test hmm::tests::test_predict_next_state ... ok
test hmm::tests::test_state_probabilities_sum_to_one ... ok
test primitives::tests::test_adx_trend_direction ... ok
test primitives::tests::test_adx_trending_detection ... ok
test primitives::tests::test_atr_creation ... ok
test primitives::tests::test_atr_increases_with_volatility ... ok
test primitives::tests::test_atr_reset ... ok
test primitives::tests::test_atr_warmup ... ok
test primitives::tests::test_bb_band_ordering ... ok
test primitives::tests::test_bb_creation ... ok
test hmm::tests::test_confidence_range ... ok
test primitives::tests::test_bb_overbought_oversold ... ok
test primitives::tests::test_bb_percent_b ... ok
test primitives::tests::test_bb_reset ... ok
test primitives::tests::test_bb_warmup ... ok
test hmm::tests::test_bull_market_detection ... ok
test hmm::tests::test_update_ohlc_uses_close ... ok
test primitives::tests::test_calculate_sma ... ok
test primitives::tests::test_calculate_sma_precision ... ok
test primitives::tests::test_bb_squeeze_detection ... ok
test primitives::tests::test_ema_calculation ... ok
test primitives::tests::test_ema_reset ... ok
test primitives::tests::test_ema_tracks_trend ... ok
test primitives::tests::test_ema_warmup ... ok
test primitives::tests::test_rsi_bearish_market ... ok
test primitives::tests::test_rsi_bullish_market ... ok
test primitives::tests::test_rsi_creation ... ok
test primitives::tests::test_rsi_range ... ok
test primitives::tests::test_rsi_reset_clears_value ... ok
test primitives::tests::test_rsi_value_cached ... ok
test router::tests::test_active_strategy_display ... ok
test hmm::tests::test_volatile_market_detection ... ok
test router::tests::test_asset_registration ... ok
test primitives::tests::test_ema_creation ... ok
test router::tests::test_asset_summary_display ... FAILED
test router::tests::test_auto_registration ... ok
test router::tests::test_asset_unregistration ... ok
test router::tests::test_compute_strategy_low_confidence ... ok
test router::tests::test_compute_strategy_mean_reverting ... ok
test router::tests::test_compute_strategy_trending ... ok
test router::tests::test_compute_strategy_uncertain ... ok
test router::tests::test_compute_strategy_volatile ... ok
test router::tests::test_detection_method_display ... ok
test router::tests::test_ensemble_signal_has_agreement ... ok
test router::tests::test_hmm_signal_has_state_probs ... ok
test router::tests::test_initial_regime_is_uncertain ... ok
test router::tests::test_is_ready_unknown_asset ... ok
test router::tests::test_method_switching ... ok
test router::tests::test_not_ready_before_warmup ... ok
test router::tests::test_duplicate_registration_noop ... ok
test ensemble::tests::test_bull_market_agreement ... ok
test ensemble::tests::test_ready_state ... ok
test router::tests::test_registered_assets ... ok
test router::tests::test_routed_signal_display ... ok
test router::tests::test_routed_signal_fields ... ok
test router::tests::test_router_creation_ensemble ... ok
test router::tests::test_router_creation_hmm ... ok
test router::tests::test_router_creation_indicators ... ok
test router::tests::test_summary ... ok
test router::tests::test_regime_changes_counted ... ok

failures:

---- router::tests::test_asset_summary_display stdout ----

thread 'router::tests::test_asset_summary_display' (479301) panicked at src/router.rs:912:9:
assertion failed: display.contains("{regime_changes}")
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    router::tests::test_asset_summary_display

test result: FAILED. 109 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
   Compiling indicators-ta v0.1.0 (/home/jordan/github/indicators-ta)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.55s
     Running unittests src/lib.rs (target/debug/deps/indicators-50faea5bca6d0575)
error: test failed, to rerun pass `--lib`
```

---

## cargo doc

> Verifies documentation compiles without warnings.

```
Documenting indicators-ta v0.1.0 (/home/jordan/github/indicators-ta)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.47s
   Generated /home/jordan/github/indicators-ta/target/doc/indicators/index.html
```

---

*Report generated by `scripts/lint_report`*
