// Copyright 2021, Benjamin Ludewig
// SPDX-License-Identifier: MIT OR Apache-2.0
fn main() {
    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    //
    // let mut builder = bindgen::Builder::default();
    // // if BINDGEN_TARGET is set it instructs the target bindgen is built for
    // if let Ok(bindgen_target) = env::var("BINDGEN_TARGET") {
    //     builder = builder.clang_arg(format!("--target={}", bindgen_target));
    // }
    // if UCI_DIR is present, use it to look for the header file and precompiled libs
    println!("cargo:rustc-link-search=native=resources/lib");
    // Link to libuci and libubox
    println!("cargo:rustc-link-lib=dylib=uci");
    println!("cargo:rustc-link-lib=dylib=ubox");
    //
    // // Tell cargo to invalidate the built crate whenever the wrapper changes
    // println!("cargo:rerun-if-changed=wrapper.h");
    //
    // let bindings = builder
    //     .header("wrapper.h")
    //     // Tell cargo to invalidate the built crate whenever any of the
    //     // included header files changed.
    //     .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    //     .allowlist_function("uci_.*")
    //     .allowlist_type("uci_.*")
    //     .allowlist_var("uci_.*")
    //     .allowlist_var("UCI_.*")
    //     .no_debug("uci_ptr")
    //     .generate()
    //     .expect("Unable to generate bindings");
    //
    // // Write the bindings to the $OUT_DIR/bindings.rs file.
    // bindings
    //     .write_to_file(out_path)
    //     .expect("Couldn't write bindings!");
}
