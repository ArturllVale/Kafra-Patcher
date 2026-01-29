use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use super::get_patcher_name;
use anyhow::{Context, Result};
use flate2::read::GzDecoder;
use serde::Deserialize;

/// Marcador para identificar config embutido no final do EXE
const CONFIG_MARKER: &[u8; 4] = b"KCFG";

#[derive(Deserialize, Clone)]
pub struct PatcherConfiguration {
    pub window: WindowConfiguration,
    pub play: PlayConfiguration,
    pub setup: SetupConfiguration,
    pub web: WebConfiguration,
    pub client: ClientConfiguration,
    pub patching: PatchingConfiguration,
}

#[derive(Deserialize, Clone)]
pub struct WindowConfiguration {
    pub title: String,
    pub width: i32,
    pub height: i32,
    pub resizable: bool,
    pub frameless: Option<bool>,
    #[cfg_attr(not(windows), allow(dead_code))]
    pub border_radius: Option<i32>,
}

#[derive(Deserialize, Clone)]
pub struct PlayConfiguration {
    pub path: String,
    pub arguments: Vec<String>,
    pub exit_on_success: Option<bool>,
    pub play_with_error: Option<bool>,
}

#[derive(Deserialize, Clone)]
pub struct SetupConfiguration {
    pub path: String,
    pub arguments: Vec<String>,
    pub exit_on_success: Option<bool>,
}

#[derive(Deserialize, Clone)]
pub struct WebConfiguration {
    pub index_url: String, // URL of the index file implementing the UI
    pub preferred_patch_server: Option<String>, // Name of the patch server to use in priority
    pub patch_servers: Vec<PatchServerInfo>,
}

#[derive(Deserialize, Clone)]
pub struct PatchServerInfo {
    pub name: String,      // Name of that identifies the patch server
    pub plist_url: String, // URL of the plist.txt file
    pub patch_url: String, // URL of the directory containing .thor files
}

#[derive(Deserialize, Clone)]
pub struct ClientConfiguration {
    pub default_grf_name: String, // GRF file to patch by default
}

#[derive(Deserialize, Clone)]
pub struct PatchingConfiguration {
    pub in_place: bool,        // In-place GRF patching
    pub check_integrity: bool, // Check THOR archives' integrity
    pub create_grf: bool,      // Create new GRFs if they don't exist
}

pub fn retrieve_patcher_configuration(
    config_file_path: Option<PathBuf>,
) -> Result<PatcherConfiguration> {
    // Primeiro, tentar extrair config embutido do próprio executável
    if let Ok(config) = extract_embedded_config() {
        log::info!("Using embedded configuration");
        return Ok(config);
    }

    // Fallback: carregar de arquivo externo
    let patcher_name = get_patcher_name()?;
    let config_file_path =
        config_file_path.unwrap_or_else(|| PathBuf::from(patcher_name).with_extension("yml"));
    log::info!("Loading configuration from: {}", config_file_path.display());
    parse_configuration(config_file_path)
}

/// Tenta extrair configuração embutida no final do executável.
///
/// Formato esperado: [EXE] + [YAML gzip] + [tamanho u32 LE] + [marcador "KCFG"]
fn extract_embedded_config() -> Result<PatcherConfiguration> {
    let exe_path = env::current_exe().context("Failed to get current executable path")?;
    let mut file = File::open(&exe_path).context("Failed to open executable")?;

    let file_size = file.metadata()?.len();
    if file_size < 8 {
        anyhow::bail!("File too small to contain embedded config");
    }

    // Ler os últimos 8 bytes (tamanho + marcador)
    file.seek(SeekFrom::End(-8))?;
    let mut footer = [0u8; 8];
    file.read_exact(&mut footer)?;

    // Verificar marcador
    if &footer[4..8] != CONFIG_MARKER {
        anyhow::bail!("No embedded config marker found");
    }

    // Extrair tamanho do config comprimido
    let compressed_size = u32::from_le_bytes([footer[0], footer[1], footer[2], footer[3]]) as u64;

    // Validar tamanho
    if compressed_size == 0 || compressed_size > file_size - 8 {
        anyhow::bail!("Invalid embedded config size");
    }

    // Posicionar no início do config comprimido
    let config_start = file_size - 8 - compressed_size;
    file.seek(SeekFrom::Start(config_start))?;

    // Ler o config comprimido
    let mut compressed_data = vec![0u8; compressed_size as usize];
    file.read_exact(&mut compressed_data)?;

    // Descomprimir
    let mut decoder = GzDecoder::new(&compressed_data[..]);
    let mut yaml_content = String::new();
    decoder
        .read_to_string(&mut yaml_content)
        .context("Failed to decompress embedded config")?;

    // Parse YAML
    serde_yaml::from_str(&yaml_content).context("Invalid embedded configuration")
}

fn parse_configuration(config_file_path: impl AsRef<Path>) -> Result<PatcherConfiguration> {
    let config_file = File::open(config_file_path)?;
    let config_reader = BufReader::new(config_file);
    serde_yaml::from_reader(config_reader).context("Invalid configuration")
}
