use std::fs;
use std::path::Path;

use cmake::Config;

fn main() {
    let dst = Config::new("cpp").build_target("glrenderer").build();

    println!("Build folder: {}", dst.display());

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();

    if target_os == "linux" {
        let libs_dir = "build";

        let cargo_build_output = dst.join("build/libglrenderer.so");
        println!("Cargp build output: {}", cargo_build_output.display());

        let dest_path = Path::new(&libs_dir).join("libglrenderer.so");
        println!("Copying to destination: {}", dest_path.display());

        if !Path::new(libs_dir).exists() {
            match fs::create_dir(libs_dir) {
                Ok(_) => println!("Destination directory created successfully."),
                Err(e) => eprintln!("Failed to create destination directory: {}", e),
            }
        }

        fs::copy(cargo_build_output, dest_path)
            .expect("Failed to copy build artifact to the destination");

        println!("cargo:rustc-link-search=native={}", libs_dir);
        println!("cargo:rustc-link-lib=dylib=glrenderer");
    }
}
