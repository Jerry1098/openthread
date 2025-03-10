use std::{env, path::PathBuf};

use anyhow::Result;

#[path = "gen/builder.rs"]
mod builder;

fn main() -> Result<()> {
    let crate_root_path = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());

    builder::OpenThreadBuilder::track(&crate_root_path.join("gen"));
    builder::OpenThreadBuilder::track(&crate_root_path.join("openthread"));

    // If `custom` is enabled, we need to re-build the bindings on-the-fly even if there are
    // pre-generated bindings for the target triple

    let host = env::var("HOST").unwrap();
    let target = env::var("TARGET").unwrap();

    let force_esp_riscv_toolchain = env::var("CARGO_FEATURE_FORCE_ESP_RISCV_TOOLCHAIN").is_ok();

    let pregen_bindings = env::var("CARGO_FEATURE_FORCE_GENERATE_BINDINGS").is_err();
    let pregen_bindings_rs_file = crate_root_path
        .join("src")
        .join("include")
        .join(format!("{target}.rs"));
    let pregen_libs_dir = crate_root_path.join("libs").join(&target);

    let dirs = if pregen_bindings && pregen_bindings_rs_file.exists() {
        // Use the pre-generated bindings
        Some((pregen_bindings_rs_file, pregen_libs_dir))
    } else if target.ends_with("-espidf") {
        // Nothing to do for ESP-IDF, `esp-idf-sys` will do everything for us
        None
    } else {
        // Need to do on-the-fly build and bindings' generation
        let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());

        let builder = builder::OpenThreadBuilder::new(
            crate_root_path.clone(),
            Some(target),
            Some(host),
            None,
            None,
            None,
            force_esp_riscv_toolchain,
        );

        let libs_dir = builder.compile(&out, None)?;
        let bindings = builder.generate_bindings(&out, None)?;

        Some((bindings, libs_dir))
    };

    if let Some((bindings, libs_dir)) = dirs {
        println!(
            "cargo::rustc-env=OPENTHREAD_SYS_BINDINGS_FILE={}",
            bindings.display()
        );

        println!("cargo:rustc-link-lib=everest");
        println!("cargo:rustc-link-lib=mbedcrypto");
        println!("cargo:rustc-link-lib=mbedtls");
        println!("cargo:rustc-link-lib=mbedx509");
        println!("cargo:rustc-link-lib=openthread-mtd");
        println!("cargo:rustc-link-lib=openthread-platform-utils-static");
        println!("cargo:rustc-link-lib=openthread-platform");
        println!("cargo:rustc-link-lib=p256m");
        println!("cargo:rustc-link-lib=platform");
        println!("cargo:rustc-link-lib=tcplp-mtd");
        println!("cargo:rustc-link-search={}", libs_dir.display());
    }

    Ok(())
}
