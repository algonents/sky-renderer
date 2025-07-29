use std::env;

fn main() {
    if std::env::var("DOCS_RS").is_ok() {
        // don't build the native dependencies for doc generation
        return;
    }
    println!("cargo:rerun-if-changed=cpp/CMakeLists.txt");
    println!("cargo:rerun-if-changed=cpp/src/glrenderer.cpp");
    println!("cargo:rerun-if-changed=cpp/include/glrenderer.h");

    let target = env::var("TARGET").unwrap();

    let dst = cmake::Config::new("cpp")
        .build_target("glrenderer")
        .static_crt(true)
        .build();

    let cmake_build_output = dst.join("build");

    // handle platform-specific configuration
    if target.contains("linux") {
        println!(
            "cargo:rustc-link-search=native={}",
            cmake_build_output.display()
        );
        println!("cargo:rustc-link-lib=static=glrenderer");
        println!("cargo:rustc-link-lib=static=glfw3");

        println!("cargo:rustc-link-lib=dylib=GL");
        println!("cargo:rustc-link-lib=dylib=stdc++");
    } else if target.contains("apple") {
        println!(
            "cargo:rustc-link-search=native={}",
            cmake_build_output.display()
        );
        println!("cargo:rustc-link-lib=static=glrenderer");
        println!("cargo:rustc-link-lib=static=glfw3");

        // Add these:
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=IOKit");
        println!("cargo:rustc-link-lib=framework=Cocoa");
        println!("cargo:rustc-link-lib=framework=CoreVideo");

        println!("cargo:rustc-link-lib=dylib=c++");
    } else if target.contains("windows") {
        let profile = std::env::var("PROFILE").unwrap();
        let build_dir = if profile == "debug" {
            cmake_build_output.join("Debug")
        } else {
            cmake_build_output.join("Release")
        };

        println!("cargo:rustc-link-search=native={}", build_dir.display());
        println!("cargo:rustc-link-lib=static=glrenderer");
        println!("cargo:rustc-link-lib=static=glfw3"); // ← link statically, not glfw3dll

        // Link Windows system libraries
        println!("cargo:rustc-link-lib=dylib=opengl32");
        println!("cargo:rustc-link-lib=dylib=gdi32");
        println!("cargo:rustc-link-lib=dylib=user32");
        println!("cargo:rustc-link-lib=dylib=shell32");
        println!("cargo:rustc-link-lib=dylib=kernel32");

        // Link the C++ runtime (adjust depending on compiler)
        // println!("cargo:rustc-link-lib=dylib=stdc++"); // for MinGW/gcc
        println!("cargo:rustc-link-lib=dylib=msvcrt"); // for MSVC (uncomment if needed)
    }
}
