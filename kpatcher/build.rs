#[cfg(windows)]
fn main() {
    use std::env;
    use std::process::Command;
    use std::path::Path;

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir).join("libresource.a");

    let status = Command::new("windres")
        .arg("resources/kpatcher.rc")
        .arg(&out_path)
        .status();
    
    match status {
        Ok(s) if s.success() => {
            println!("cargo:rustc-link-search=native={}", out_dir);
            println!("cargo:rustc-link-lib=static=resource");
        }
        _ => {
            // Fallback: build without icon if windres fails, to unblock user
            println!("cargo:warning=Failed to compile resources. Application will have no icon.");
        }
    }
}

#[cfg(not(windows))]
fn main() {}
