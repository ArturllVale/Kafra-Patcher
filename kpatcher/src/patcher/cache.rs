use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PatcherCache {
    pub last_patch_index: usize,
}

pub async fn read_cache_file(cache_file_path: impl AsRef<Path>) -> Result<PatcherCache> {
    let content = tokio::fs::read(cache_file_path).await?;
    serde_json::from_slice(&content).context("Failed to deserialize patcher cache")
}

pub async fn write_cache_file(
    cache_file_path: impl AsRef<Path>,
    new_cache: PatcherCache,
) -> Result<()> {
    let content = serde_json::to_vec(&new_cache).context("Failed to serialize patcher cache")?;
    tokio::fs::write(cache_file_path, content).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_read_write_cache() {
        let tmp_file = NamedTempFile::new().unwrap();
        let cache_path = tmp_file.path();

        let cache = PatcherCache {
            last_patch_index: 42,
        };

        write_cache_file(cache_path, cache).await.unwrap();

        let read_cache = read_cache_file(cache_path).await.unwrap();

        assert_eq!(read_cache.last_patch_index, 42);
    }
}
