# sky-renderer

A high-performance 2D engine for rendering aviation data in radar displays.

## How to build

On Linux, make sure you have installed all the project dependencies:

```shell script
sudo apt-get install libgl1-mesa-dev
sudo apt install mesa-utils
sudo apt install libglfw3-dev
```
To build the project, you also need CMake installed on your system.

You can then build the project with cargo as usual:

```shell script
cargo build
```

The build command will first trigger a native build of `libglrenderer.so` (using cmake) before building this crate.
During the build, `libglrenderer.so` is copied to the project's libs folder. Make sure to add this folder to your `LD_LIBRARY_PATH` before running any of the applications in this crate.
