# sky_renderer

**sky_renderer** is a lightweight, high-performance 2D rendering engine built in Rust with native bindings to OpenGL.
This initial release provides only low-level, minimalistic OpenGL bindings and is not yet usable for real applications. Future versions will add support for drawing primitives (lines, shapes, text) and a higher-level rendering API on top of OpenGL.

## 🚧 Status

Early release on crates.io.
The current version provides only low-level OpenGL bindings and is not yet suitable for production use.
Future versions will include 2D primitives, text rendering, and higher-level abstractions.

## 🔧 Installation

### Linux
Make sure you have all dependencies installed on your system:

```shell script
sudo apt-get install libgl1-mesa-dev
sudo apt install mesa-utils
sudo apt install libglfw3-dev
```
You will also need a C/C++ compiler as well as cmake installed on your system.

You can simply add **sky_renderer** as a dependency to your project. When building your project, cargo will first trigger a C/C++ build (using cmake) of the static library (libglrenderer) containing the **sky_renderer** ffi bindings to OpenGL. The entire process should be totally transparent to your project.

### MacOS
Coming soon...

### Windows
Coming soon...

### 📦 Examples

Please refer to the [examples](https://github.com/algonents/sky-renderer/tree/master/examples) provided in the **sky_renderer** GitHub repository.  These will be updated as new features are added.




