// App Task — task-list manager (Rust + Slint).
// Copyright (C) 2026 Alfredo Riveros.
// SPDX-License-Identifier: GPL-3.0-only
// See LICENSE for details.
fn main() {
    println!("cargo::rerun-if-changed=ui/");
    println!("cargo::rerun-if-changed=lang/");
    let config = slint_build::CompilerConfiguration::new()
        .with_bundled_translations(concat!(env!("CARGO_MANIFEST_DIR"), "/lang"));
    slint_build::compile_with_config("ui/app.slint", config).expect("Slint compile failed");
}
