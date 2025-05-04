use std::env;

fn main() {
    println!("cargo:rerun-if-changed=cpp/CMakeLists.txt");
    println!("cargo:rerun-if-changed=cpp/src/glrenderer.cpp");
    println!("cargo:rerun-if-changed=cpp/include/glrenderer.h");

    let target = env::var("TARGET").unwrap();

    let dst = cmake::Config::new("cpp")
        .build_target("glrenderer")
        .static_crt(true)
        .build();

    let lib_dir = dst.join("build");

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=glrenderer");
    println!("cargo:rustc-link-lib=dylib=glfw");

    // handle platform-specific configuration
    if target.contains("linux") {
        println!("cargo:rustc-link-lib=dylib=GL");
        println!("cargo:rustc-link-lib=dylib=stdc++");
    } else if target.contains("apple") {
        let homebrew_lib_location = "/opt/homebrew/lib";
        println!("cargo:rustc-link-search=native={}", homebrew_lib_location);
        println!("cargo:rustc-link-lib=dylib=c++");
    } else if target.contains("windows") {
        // todo
    }
}
