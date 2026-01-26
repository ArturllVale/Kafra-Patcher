fn main() {
    // No special linking needed if we provide the shim or don't use the feature
    // However, tinyfiledialogs still wants it apparently.
    // We let the shim in main.rs handle it, or cargo link shcore if strictly needed (but it failed).
    // So we keep this empty or minimal.
    println!("cargo:rerun-if-changed=build.rs");
}
