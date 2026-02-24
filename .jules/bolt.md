# Performance Journal

## Offloading Blocking IO in apply_patch

- **Issue**: `apply_patch` performs blocking file I/O and decompression on the async executor thread.
- **Fix**: Wrapped `apply_patch` in `tokio::task::spawn_blocking`.
- **Result**: Benchmarking with a concurrent ticker showed significant improvement in responsiveness. Sync version blocked ticker (0 ticks), Async version allowed ticker to run (41 ticks) during patch application.
