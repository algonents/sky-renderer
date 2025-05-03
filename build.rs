fn main() {
    println!("cargo:rerun-if-changed=cpp/CMakeLists.txt");
    println!("cargo:rerun-if-changed=cpp/src/glrenderer.cpp");
    println!("cargo:rerun-if-changed=cpp/include/glrenderer.h");

    let dst = cmake::Config::new("cpp")
        .build_target("glrenderer")
        .static_crt(true)
        .build();

    let lib_dir = dst.join("build");

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=glrenderer");

    // Link against required system libraries
    println!("cargo:rustc-link-lib=dylib=glfw");
    println!("cargo:rustc-link-lib=dylib=GL");
    println!("cargo:rustc-link-lib=dylib=stdc++"); // use c++ on macOS
}
