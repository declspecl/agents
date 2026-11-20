//! Rebuilds the crate when the embedded content or templates change, which `include_dir!` does not track on its own.

fn main() {
    println!("cargo:rerun-if-changed=content");
    println!("cargo:rerun-if-changed=templates");
}
