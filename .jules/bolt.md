## 2024-05-22 - Pre-allocating collections in GRF parsing
**Learning:** `nom` parsers in `gruf` were using `HashMap::new()` in `fold_many_m_n`, causing repeated reallocations during file table parsing.
**Action:** Always check `fold_many` operations if the count is known, and use `with_capacity` to pre-allocate collections.
