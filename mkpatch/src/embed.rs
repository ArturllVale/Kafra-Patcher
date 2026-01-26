//! Módulo para embutir configuração YAML dentro de um executável.
//! 
//! Formato do EXE com config embutido:
//! [EXE original] + [YAML comprimido gzip] + [tamanho u32 LE] + [marcador "KCFG"]

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result};
use flate2::write::GzEncoder;
use flate2::Compression;

/// Marcador para identificar config embutido no final do EXE
pub const CONFIG_MARKER: &[u8; 4] = b"KCFG";

/// Embute o conteúdo de um arquivo YAML dentro de um executável.
/// 
/// O resultado é um novo arquivo que contém:
/// - O executável original
/// - O YAML comprimido com gzip
/// - 4 bytes com o tamanho do YAML comprimido (little-endian)
/// - 4 bytes com o marcador "KCFG"
pub fn embed_config_in_exe(
    exe_path: &Path,
    config_path: &Path,
    output_path: &Path,
) -> Result<()> {
    // Ler o executável original
    let exe_data = fs::read(exe_path)
        .with_context(|| format!("Failed to read executable: {}", exe_path.display()))?;

    // Ler e comprimir o config YAML
    let config_data = fs::read(config_path)
        .with_context(|| format!("Failed to read config: {}", config_path.display()))?;
    
    let compressed_config = compress_data(&config_data)?;
    let compressed_size = compressed_config.len() as u32;

    // Criar arquivo de saída
    let mut output_file = File::create(output_path)
        .with_context(|| format!("Failed to create output: {}", output_path.display()))?;

    // Escrever: EXE + config comprimido + tamanho + marcador
    output_file.write_all(&exe_data)?;
    output_file.write_all(&compressed_config)?;
    output_file.write_all(&compressed_size.to_le_bytes())?;
    output_file.write_all(CONFIG_MARKER)?;

    log::info!(
        "Config embedded successfully. Original: {} bytes, Config: {} bytes (compressed: {} bytes)",
        exe_data.len(),
        config_data.len(),
        compressed_size
    );

    Ok(())
}

/// Comprime dados usando gzip
fn compress_data(data: &[u8]) -> Result<Vec<u8>> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(data)?;
    encoder.finish().context("Failed to compress data")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Read};
    use flate2::read::GzDecoder;

    #[test]
    fn test_compress_decompress() {
        let original = b"Hello, World! This is a test YAML config.";
        let compressed = compress_data(original).unwrap();
        
        let mut decoder = GzDecoder::new(Cursor::new(compressed));
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed).unwrap();
        
        assert_eq!(original.as_slice(), decompressed.as_slice());
    }
}
