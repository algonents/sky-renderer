# sky_renderer

**sky_renderer** is a minimalistic 2D rendering engine built in Rust with native bindings to OpenGL.
This version only provides low-level bindings to OpenGL and is not yet production-ready. Future versions will
add support for additional drawing primitives such as lines, shapes, and texts as well as a higher-level rendering API on top of OpenGL.

## 🚧 Status

This is a (very) early release: this version provides a limited set of bindings to OpenGL and is not yet suitable for production use.
The bindings can nonetheless be used in your own code if you wish to experiment with a minimalistic API to OpenGL. If a specific binding for your use-case is missing, please let me know.

## 🔧 Installation

### Linux

Make sure you have all dependencies installed on your system (including a C/C++ compiler and CMake):

```shell script
sudo apt-get install libgl1-mesa-dev
sudo apt install mesa-utils
sudo apt install libglfw3-dev
```

You can simply add **sky_renderer** as a dependency to your project. When building your project, cargo will first trigger a C/C++ build (using CMake) of a static library containing the **sky_renderer** ffi bindings to OpenGL. The entire process should be totally transparent to your project.

### Windows

Make sure you are are using release 0.1.7 or later (there was no Windows support for earlier versions). Also ensure you have Visual C/C++ and CMake installed on your system.

### macOS

Ensure you have CMake and a C/C++ compiler installed on your system.
Use [Homebrew](https://brew.sh/) to install glfw on your system:

```shell script
brew install glfw
brew info glfw
```

Once glfw is installed, the crate's build script will look for the glfw libraries under `/opt/homebrew/lib`

### 📦 Examples

Refer to the [examples](https://github.com/algonents/sky-renderer/tree/master/examples) provided in the **sky_renderer** GitHub repository. These will be updated as new features are added.
