# sky_renderer

**sky_renderer** is a high-performance minimalistic 2D rendering engine built in Rust with native bindings to OpenGL.
This initial release provides only basic low-level bindings to OpenGL and is not yet usable for real applications. Future versions will add support for additional drawing primitives such as lines, shapes, and texts as well as a higher-level rendering API on top of OpenGL.

## 🚧 Status

Early release on crates.io.
The current version provides only low-level OpenGL bindings and is not yet suitable for production use.
The bindings can nonetheless be used in your own code if you are looking for a minimalistic API to OpenGL. If a specific binding for your use-case is missing, please let me know.

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




