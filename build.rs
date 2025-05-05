use std::env;

fn main() {
    println!("cargo:rerun-if-changed=cpp/CMakeLists.txt");
    println!("cargo:rerun-if-changed=cpp/src/glrenderer.cpp");
    println!("cargo:rerun-if-changed=cpp/include/glrenderer.h");

    let target = env::var("TARGET").unwrap();

    let dst = cmake::Config::new("cpp")
        .define("CMAKE_PREFIX_PATH", "D:/GitHub/vcpkg/installed/x64-windows")
        .build_target("glrenderer")
        .static_crt(true)
        .build();

    let lib_dir = dst.join("build");

    
    // handle platform-specific configuration
    if target.contains("linux") {
        
        println!("cargo:rustc-link-search=native={}", lib_dir.display());
        println!("cargo:rustc-link-lib=static=glrenderer");
    
        println!("cargo:rustc-link-lib=dylib=glfw");
        println!("cargo:rustc-link-lib=dylib=GL");
        println!("cargo:rustc-link-lib=dylib=stdc++");
    
    } else if target.contains("apple") {
        
        println!("cargo:rustc-link-search=native={}", lib_dir.display());
        println!("cargo:rustc-link-lib=static=glrenderer");
    
        let homebrew_lib_location = "/opt/homebrew/lib";
        println!("cargo:rustc-link-search=native={}", homebrew_lib_location);
        println!("cargo:rustc-link-lib=dylib=glfw");

        println!("cargo:rustc-link-lib=dylib=c++");

    } else if target.contains("windows") {
        
        println!("cargo:rustc-link-search=native={}", lib_dir.join("Debug").display());
        println!("cargo:rustc-link-lib=static=glrenderer");
    
        
        let vcpkg_lib_location = "D:/GitHub/vcpkg/installed/x64-windows/lib";
        println!("cargo:rustc-link-search=native={}", vcpkg_lib_location);
        println!("cargo:rustc-link-lib=dylib=glfw3dll");
    }
}
