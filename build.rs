use std::env;
use std::fs;
use std::path::PathBuf;

use cmake::Config;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Build the C++ project
    let dst = Config::new("cpp")
        .build_target("glrenderer")
        .out_dir(&out_dir) // make sure it builds into OUT_DIR
        .build();

    // Platform-specific library file
    let lib_filename = if cfg!(target_os = "linux") {
        "libglrenderer.so"
    } else if cfg!(target_os = "macos") {
        "libglrenderer.dylib"
    } else if cfg!(target_os = "windows") {
        "glrenderer.dll"
    } else {
        panic!("Unsupported platform");
    };

    // Path to the generated lib inside the cmake build directory
    let built_lib_path = dst.join("build").join(lib_filename);

    // Final location: OUT_DIR
    let final_lib_path = out_dir.join(lib_filename);

    // Copy to OUT_DIR
    fs::copy(&built_lib_path, &final_lib_path)
        .unwrap_or_else(|e| panic!("Failed to copy {} to OUT_DIR: {}", lib_filename, e));

    // Emit link instructions
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=dylib=glrenderer");

    // Optionally export the path to the lib for the client crate to use at runtime
    println!(
        "cargo:rustc-env=DEP_GLRENDERER_LIB_PATH={}",
        final_lib_path.display()
    );
}
