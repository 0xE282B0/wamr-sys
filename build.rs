extern crate bindgen;
extern crate cmake;

use cmake::Config;
use std::{env, path::PathBuf};

fn main() {
    let mut config = Config::new("libiwasm");
    let generator = config
        .generator("Unix Makefiles")
        .define("CMAKE_BUILD_TYPE", "Release")
        // WAMR_BUILD_TARGET seems to want what we have in the first part of the target triple, in uppercase.
        .define("WAMR_BUILD_TARGET", env::var("TARGET").unwrap().split("-").next().unwrap().to_uppercase());

    match env::var("CARGO_FEATURE_STD") {
        // cmake won't know about cargo's toolchain. This assumes that the CMAKE_TOOLCHAIN_FILE env var will be set
        // when invoking cargo to let cmake know how to build. For example:
        // CMAKE_TOOLCHAIN_FILE=~/gba-toolchain/arm-gba-toolchain.cmake
        Err(env::VarError::NotPresent) => {
            generator.define("WAMR_BUILD_PLATFORM", "rust-no-std");
        }
        _ => (),
    };

    // Run cmake to build nng
    let dst = generator.no_build_target(true).build();
    // Check output of `cargo build --verbose`, should see something like:
    // -L native=/path/runng/target/debug/build/runng-sys-abc1234/out
    // That contains output from cmake
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("build").display()
    );
    println!("cargo:rustc-link-lib=iwasm");
    println!("cargo:rustc-link-lib=vmlib");
    println!("cargo:rerun-if-changed=libiwasm/CMakeLists.txt");

    let bindings = bindgen::Builder::default()
        .ctypes_prefix("::core::ffi")
        .use_core()
        .header("wasm-micro-runtime/core/iwasm/include/wasm_export.h")
        // This is needed if use `#include <nng.h>` instead of `#include "path/nng.h"`
        //.clang_arg("-Inng/src/")
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
