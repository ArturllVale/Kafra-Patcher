//! Módulo para embutir configuração YAML dentro de um executável.
//! 
//! Formato do EXE com config embutido:
//! [EXE original] + [Nonce (12)] + [YAML cifrado (gzip + AES-256-GCM)] + [tamanho total u32 LE] + [marcador "KCFG"]

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result};
use flate2::write::GzEncoder;
use flate2::Compression;

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce // Or `Key`
};
use rand::Rng;

/// Marcador para identificar config embutido no final do EXE
pub const CONFIG_MARKER: &[u8; 4] = b"KCFG";

/// Chave de criptografia compartilhada (32 bytes)
/// IMPORTANTE: Manter sincronizado com kpatcher/src/patcher/config.rs
const ENCRYPTION_KEY: &[u8; 32] = b"kpatcher_secret_key_32_bytes!!!!";

/// Embute o conteúdo de um arquivo YAML dentro de um executável.
/// 
/// O resultado é um novo arquivo que contém:
/// - O executável original
/// - O Nonce (12 bytes)
/// - O YAML comprimido e cifrado
/// - 4 bytes com o tamanho total (Nonce + Cifrado) (little-endian)
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
    
    // Criptografar
    let cipher = Aes256Gcm::new(ENCRYPTION_KEY.into());
    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    
    let ciphertext = cipher.encrypt(nonce, compressed_config.as_ref())
        .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

    // Tamanho total = Nonce + Ciphertext
    let total_size = (nonce_bytes.len() + ciphertext.len()) as u32;

    // Criar arquivo de saída
    let mut output_file = File::create(output_path)
        .with_context(|| format!("Failed to create output: {}", output_path.display()))?;

    // Escrever: EXE + Nonce + Ciphertext + Tamanho + Marcador
    output_file.write_all(&exe_data)?;
    output_file.write_all(&nonce_bytes)?;
    output_file.write_all(&ciphertext)?;
    output_file.write_all(&total_size.to_le_bytes())?;
    output_file.write_all(CONFIG_MARKER)?;

    log::info!(
        "Config embedded successfully. Original: {} bytes, Encrypted Bundle: {} bytes",
        exe_data.len(),
        total_size
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
    fn test_compress_encrypt_decrypt_decompress() {
        let original = b"Hello, World! This is a test YAML config.";
        
        // 1. Compress
        let compressed = compress_data(original).unwrap();
        
        // 2. Encrypt
        let cipher = Aes256Gcm::new(ENCRYPTION_KEY.into());
        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = cipher.encrypt(nonce, compressed.as_ref()).unwrap();
        
        // 3. Decrypt
        let plaintext_compressed = cipher.decrypt(nonce, ciphertext.as_ref()).unwrap();
        
        // 4. Decompress
        let mut decoder = GzDecoder::new(Cursor::new(plaintext_compressed));
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed).unwrap();
        
        assert_eq!(original.as_slice(), decompressed.as_slice());
    }
}
