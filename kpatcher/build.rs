#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    // The icon path is relative to the Cargo.toml directory
    res.set_icon("resources/kpatcher.ico");

    // Note: winres will automatically use the [package.metadata.winres] info from Cargo.toml
    // for version info, etc.

    if let Err(e) = res.compile() {
        println!("cargo:warning=Failed to compile resources: {}", e);
    }
}

#[cfg(not(windows))]
fn main() {}
