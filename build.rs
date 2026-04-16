//! Скрипт сборки для ESP32-C3
//!
//! esp-hal предоставляет свой linker script (linkall.x).
//! defmt.x генерируется автоматически.

fn main() {
    // Ничего не нужно копировать — esp-hal и defmt сами
    // предоставляют linker scripts через cargo:rustc-link-arg
    println!("cargo:rerun-if-changed=build.rs");
}
