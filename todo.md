8.1 indicators-ta
✓  Strengths
• Comprehensive unit tests on every indicator with known-value assertions
• Python parity tests (MACD, ATR, WMA, SMA verified against Python reference)
• NaN-handling is consistent — leading NaN during warm-up, then correct values
• ema_nan_aware correctly seeds from first non-NaN value (matches ewm(adjust=False))
• Registry pattern allows runtime indicator creation by name
• Indicator trait is Send + Sync — safe to share across async tasks


⚠  Gaps
• No tests for the full compute_signal pipeline end-to-end
• No benchmark tests — hot path runs KMeans every 10 bars (O(N×K×100 iterations))
• hurst_scalar uses R/S analysis which is O(N log N) per recompute every 10 bars
• No fuzz tests on the registry factory functions
