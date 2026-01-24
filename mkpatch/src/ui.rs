use std::path::{Path, PathBuf};
use tinyfiledialogs as tfd;
use web_view::*;
use serde::Deserialize;

use crate::generator::generate_patch_from_definition;
use crate::patch_definition::{PatchDefinition, PatchEntry};

pub fn run_ui() {
    let html_content = include_str!("assets/index.html");

    web_view::builder()
        .title("MKPatch UI")
        .content(Content::Html(html_content))
        .size(600, 650)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|webview, arg| {
            if arg == "select_files" {
                handle_select_files(webview);
            } else if arg.starts_with("generate:") {
                let json_str = &arg[9..];
                handle_generate(webview, json_str);
            }
            Ok(())
        })
        .run()
        .unwrap();
}

fn handle_select_files(webview: &mut WebView<()>) {
    let files = tfd::open_file_dialog_multi(
        "Select Files to Patch",
        "",
        None,
    );

    if let Some(files) = files {
        // format as json array string
        let files_json = serde_json::to_string(&files).unwrap_or("[]".to_string());
        let js = format!("filesSelected({})", files_json);
        webview.eval(&js).ok();
    }
}

#[derive(Deserialize)]
struct UIInput {
    target_grf: String,
    output_filename: String,
    merge_grf: bool,
    files: Vec<String>,
}

fn handle_generate(webview: &mut WebView<()>, json_str: &str) {
    let input: UIInput = match serde_json::from_str(json_str) {
        Ok(i) => i,
        Err(e) => {
            webview.eval(&format!("logMessage('Error parsing input: {}')", e)).ok();
            return;
        }
    };

    if input.files.is_empty() {
        webview.eval("logMessage('No files selected!')").ok();
        return;
    }


    
    // We need to write the helper logic to actually read files from their absolute paths
    // The current `generate_patch_from_definition` expects `native_path` to be `patch_data_directory + relative_path`.
    // But here we have absolute paths.
    // We can trick `generate_patch_from_definition` by setting `patch_data_directory` to root and `relative_path` to absolute?
    // Windows absolute paths have drive letters.
    // Let's modify logic or handle it.
    
    // Easier: Copy `generate_patch_from_definition` logic but adapted for absolute paths?
    // Or, actually, `mkpatch` logic `native_path = patch_data_directory.join(relative_path)`.
    // If I set `patch_data_directory` to empty/cwd, and `relative_path` to absolute path, verify if `join` works.
    // Path::new("").join("C:/foo") -> "C:/foo". Yes.
    
    // BUT we need `relative_path` in `PatchEntry` to be the path INSIDE the GRF (win32 style).
    // And `mkpatch` uses `relative_path` for BOTH sourcing the file AND destination.
    // Code:
    // let native_path = patch_data_directory.join(posix_path(entry.relative_path));
    // archive_builder.append_file_update(target_win32_relative_path, file)?;
    
    // `target_win32_relative_path` comes from `entry.in_grf_path` OR `entry.relative_path`.
    // So:
    // 1. `entry.relative_path` -> Must be the path on disk (can be absolute).
    // 2. `entry.in_grf_path` -> Must be the path in GRF.
    
    // So I should populate `relative_path` with the Absolute Path from the picker.
    // And `in_grf_path` with just the filename (or user edited path).
    
    let entries_mapped: Vec<PatchEntry> = input.files.iter().map(|f| {
        let path = Path::new(f);
        let filename = path.file_name().unwrap().to_string_lossy().to_string();
        PatchEntry {
            relative_path: f.clone(), // Absolute path
            is_removed: false,
            in_grf_path: Some(filename), // Flat in GRF for now
        }
    }).collect();
    
    let def_for_gen = PatchDefinition {
        include_checksums: true,
        use_grf_merging: input.merge_grf,
        target_grf_name: if input.target_grf.is_empty() { None } else { Some(input.target_grf) },
        entries: entries_mapped,
    };

    let output_path = PathBuf::from(&input.output_filename);
    // Ensure .thor extension
    let output_path = if output_path.extension().is_none() {
        output_path.with_extension("thor")
    } else {
        output_path
    };

    webview.eval("logMessage('Generating patch...')").ok();
    
    // We pass "." as base dir because our relative_paths are absolute
    match generate_patch_from_definition(def_for_gen, ".", &output_path) {
        Ok(_) => {
            webview.eval(&format!("logMessage('Success! Patch saved to: {}')", output_path.display().to_string().replace("\\", "\\\\"))).ok();
            webview.eval("alert('Patch Generated Successfully!')").ok();
        },
        Err(e) => {
            webview.eval(&format!("logMessage('Error: {}')", e)).ok();
        }
    }
}
