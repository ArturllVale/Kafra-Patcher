#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("resources/mkpatch.ico");

    if let Err(e) = res.compile() {
        println!("cargo:warning=Failed to compile resources: {}", e);
    }
}

#[cfg(not(windows))]
fn main() {}
