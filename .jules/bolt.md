## 2024-05-22 - Pre-allocating collections in GRF parsing
**Learning:** `nom` parsers in `gruf` were using `HashMap::new()` in `fold_many_m_n`, causing repeated reallocations during file table parsing.
**Action:** Always check `fold_many` operations if the count is known, and use `with_capacity` to pre-allocate collections.

## 2024-05-23 - Offloading blocking integrity checks
**Learning:** `is_archive_valid` in `kpatcher/src/patcher/core.rs` performs blocking I/O and CRC32 calculation, taking ~876ms for a 50MB file. This blocks the async executor when called directly.
**Action:** Wrapped the call in `tokio::task::spawn_blocking` to execute it on a thread pool dedicated for blocking operations, keeping the async runtime responsive.
