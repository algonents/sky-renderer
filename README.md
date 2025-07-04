# sky_renderer

**sky_renderer** is a minimalistic 2D rendering engine built in Rust with native bindings to OpenGL. Ultimately, the 
goal for sky_renderer is to provide a robust platform for visualizing 2D data in real-time.

This version provides low-level bindings to OpenGL and is not yet production-ready. Future versions will
add support for additional drawing primitives such as lines, texts and other shapes, as well as a higher-level rendering API abstracting OpenGL.

## 🚧 Status

This is a (very) early release: this version provides a limited set of bindings to OpenGL and is not yet suitable for production use. The bindings can nonetheless be used in your own code if you wish to experiment with a minimalistic API for OpenGL. Note that the bindings are focusing on providing a 2D rendering API. If you feel that a binding is missing, please let me know.

This release (0.1.8) introduces a set of higher-level APIs for reducing boiler-plate code. You still need to understand a fair amount of OpenGL to be able to work with the APIs. 

For examples on how to use the higher-level APIs, refer to `geometry.rs` and `transform.rs` in the [examples](https://github.com/algonents/sky-renderer/tree/master/examples) folder.

Because the API is evolving, please make sure to always use the latest release.

## 📖 Docs

### Wiki

Refer to the **sky_renderer** GitHub [wiki](https://github.com/algonents/sky-renderer/wiki), which will be updated soon.

### 📦 Examples

Refer to the [examples](https://github.com/algonents/sky-renderer/tree/master/examples) provided in the **sky_renderer** GitHub repository. 
The examples will be updated as new features are added.


## 🐞 Issues

You can raise issues directly on [Github](https://github.com/algonents/sky-renderer/issues).

## 🔧 Installation

### Linux

Make sure you have all dependencies installed on your system (including a C/C++ compiler and CMake):

```shell script
sudo apt-get install libgl1-mesa-dev
sudo apt install mesa-utils
sudo apt install libglfw3-dev
```

You can add **sky_renderer** as a dependency to your project. When building your project, cargo will first build (using CMake with your system's C/C++ compiler) a static library containing the **sky_renderer** ffi bindings to OpenGL (the ffi bindings can be found [here](https://github.com/algonents/sky-renderer/tree/master/cpp))


### Windows

Make sure you are are using release 0.1.7 or later (there was no Windows support for earlier versions). Ensure you have Visual C/C++ and CMake installed on your system.
Also use [vcpkg](https://learn.microsoft.com/en-us/vcpkg/get_started/overview) to install glfw on your system:

```shell script
git clone https://github.com/microsoft/vcpkg.git
cd vcpkg
.\bootstrap-vcpkg.bat
.\vcpkg install glfw3
```

You must also update your system's environment variables:

- Define the `VCPKG_LIB_PATH` environment variable to point to vcpkg's lib folder (this is where glfw3.lib is installed, for example `D:\GitHub\vcpkg\installed\x64-windows\lib`)

- Update your system's `PATH` environment variable to point to vcpkg's bin folder (this is where glfw3.dll is installed, for example `D:\GitHub\vcpkg\installed\x64-windows\bin`)

### macOS

Ensure you have CMake and a C/C++ compiler installed on your system.
Use [Homebrew](https://brew.sh/) to install glfw on your system:

```shell script
brew install glfw
brew info glfw
```

Once glfw is installed, the crate's build script will look for the glfw libraries under `/opt/homebrew/lib`

