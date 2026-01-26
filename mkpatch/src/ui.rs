use std::path::{Path, PathBuf};
use tinyfiledialogs as tfd;
use wry::webview::{WebView, WebViewBuilder};
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    dpi::LogicalSize,
};
use serde::Deserialize;

use crate::generator::generate_patch_from_definition;
use crate::patch_definition::{PatchDefinition, PatchEntry};

enum UiEvent {
    SelectFiles,
    Generate(String),
}

pub fn run_ui() {
    let event_loop = EventLoop::<UiEvent>::with_user_event();
    let proxy = event_loop.create_proxy();
    
    let window = WindowBuilder::new()
        .with_title("MKPatch UI")
        .with_inner_size(LogicalSize::new(600.0, 650.0))
        .with_resizable(true)
        .build(&event_loop)
        .unwrap();

    let html_content = include_str!("assets/index.html");

    let handler_proxy = proxy.clone();
    let handler = move |_: &tao::window::Window, req: String| {
        if req == "select_files" {
            let _ = handler_proxy.send_event(UiEvent::SelectFiles);
        } else if req.starts_with("generate:") {
            let json_str = req[9..].to_string();
            let _ = handler_proxy.send_event(UiEvent::Generate(json_str));
        }
    };

    let webview = WebViewBuilder::new(window)
        .unwrap() // WebViewBuilder::new returns Result
        .with_html(html_content)
        .unwrap() // assuming simple html content
        .with_initialization_script("window.external = { invoke: function(s) { window.ipc.postMessage(s); } };")
        .with_ipc_handler(handler)
        // .with_devtools(true) // debug was enabled in old code
        .build()
        .unwrap();
    
    // Open devtools if debug was true? default is usually disabled in release, enabled in debug.
    // wry 0.24 enables devtools by default for debug builds I think.

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::UserEvent(ui_event) => match ui_event {
                UiEvent::SelectFiles => {
                    handle_select_files(&webview);
                },
                UiEvent::Generate(json_str) => {
                    handle_generate(&webview, &json_str);
                }
            },
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            },
            _ => ()
        }
    });
}

fn handle_select_files(webview: &WebView) {
    let files = tfd::open_file_dialog_multi(
        "Select Files to Patch",
        "",
        None,
    );

    if let Some(files) = files {
        // format as json array string
        let files_json = serde_json::to_string(&files).unwrap_or("[]".to_string());
        let js = format!("filesSelected({})", files_json);
        let _ = webview.evaluate_script(&js);
    }
}

#[derive(Deserialize)]
struct UIInput {
    target_grf: String,
    output_filename: String,
    merge_grf: bool,
    files: Vec<String>,
}

fn handle_generate(webview: &WebView, json_str: &str) {
    let input: UIInput = match serde_json::from_str(json_str) {
        Ok(i) => i,
        Err(e) => {
            let _ = webview.evaluate_script(&format!("logMessage('Error parsing input: {}')", e));
            return;
        }
    };

    if input.files.is_empty() {
        let _ = webview.evaluate_script("logMessage('No files selected!')");
        return;
    }

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

    let _ = webview.evaluate_script("logMessage('Generating patch...')");
    
    // We pass "." as base dir because our relative_paths are absolute
    match generate_patch_from_definition(def_for_gen, ".", &output_path) {
        Ok(_) => {
            let output_display = output_path.display().to_string().replace("\\", "\\\\");
            let _ = webview.evaluate_script(&format!("logMessage('Success! Patch saved to: {}')", output_display));
            let _ = webview.evaluate_script("alert('Patch Generated Successfully!')");
        },
        Err(e) => {
            let _ = webview.evaluate_script(&format!("logMessage('Error: {}')", e));
        }
    }
}
