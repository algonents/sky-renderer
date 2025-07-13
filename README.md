# sky_renderer

**sky_renderer** is a minimalist 2D graphics engine built in Rust with native bindings to OpenGL. Ultimately, the 
goal is to provide a robust platform for drawing 2D shapes and visualizing data in real-time.

This version provides low-level bindings to OpenGL and is not yet production-ready. Future versions will
add support for additional drawing primitives such as lines, texts and other shapes, as well as a 
higher-level rendering API abstracting OpenGL.

## 🚧 Status

⚠️ *The APIs are evolving, please make sure to always use the latest release*.

This release (0.3.0) supports additional drawing primitives and introduces experimental support for SVG output.
It is now also possible to render images. The modules have also been reorganized to provide better cohesion. 
`RenderableShape` in release 0.2.0 has been renamed to`ShapeRenderable`.  

```rust
extern crate sky_renderer;

use sky_renderer::core::{App, Color, Renderable, Renderer, Window};
use sky_renderer::graphics2d::shape::Rectangle;
use sky_renderer::graphics2d::shaperenderable::ShapeRenderable;

fn main() {
    let window = Window::new("Shapes", 800, 800);
    let mut app = App::new(window);

    let mut shapes = vec![
        ShapeRenderable::line(100.0, 200.0, 300.0, 250.0, Color::from_rgb(0.0, 1.0, 0.0)),
        ShapeRenderable::polyline(
            &[
                (100.0, 300.0),
                (150.0, 430.0),
                (200.0, 410.0),
                (250.0, 460.0),
            ],
            Color::from_rgb(0.0, 1.0, 0.0),
        ),
        ShapeRenderable::rectangle(50.0, 50.0, 200.0, 80.0, Color::from_rgb(0.2, 0.5, 0.9)),
        ShapeRenderable::rectangle(400.0, 200.0, 100.0, 50.0, Color::from_rgb(1.0, 0.0, 0.0)),
        ShapeRenderable::circle(400.0, 400.0, 50.0, Color::from_rgb(0.0, 0.0, 1.0)),
        ShapeRenderable::point(600.0, 300.0, Color::from_rgb(1.0, 0.0, 0.0)),
        ShapeRenderable::points(
            &[
                (600.0, 100.0), // anchor point
                (620.0, 120.0),
                (580.0, 120.0),
            ],
            Color::from_rgb(0.0, 0.0, 1.0),
        ),
        ShapeRenderable::ellipse(600.0, 200.0, 80.0, 40.0, Color::from_rgb(0.5, 0.2, 0.8)),
        ShapeRenderable::rounded_rectangle(
            100.0,
            600.0,
            200.0,
            80.0,
            10.0,
            Color::from_rgb(0.3, 0.6, 0.9),
        ),
        ShapeRenderable::polygon(
            &[
                (600.0, 600.0),
                (575.0, 643.3),
                (525.0, 643.3),
                (500.0, 600.0),
                (525.0, 556.6),
                (575.0, 556.6),
            ],
            Color::from_rgb(1.0, 0.0, 0.0),
        ),
        ShapeRenderable::from_shape(
            600.0,
            400.0,
            Rectangle::new(100.0, 50.0),
            Color::from_rgb(0.0, 1.0, 0.0),
        ),
        ShapeRenderable::image_with_size(200.0, 300.0, "images/smiley.png", 40.0, 40.0),
        ShapeRenderable::image(400.0, 500.0, "images/bunny.png"),
    ];

    let renderer = Renderer::new();
    renderer.set_point_size(6.0);

    app.on_render(move || {
        for shape in &mut shapes {
            shape.render(&renderer);
        }
    });
    app.run();
}

```
The output is displayed below:

![Shapes](images/shapes.png)

Note that this is still a (very) early release: this version provides a limited set of bindings to OpenGL and is 
not yet suitable for production use. Additional `graphics2d` primitives need to be implemented and the APIs may 
continue to evolve. 

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

Make sure you are using release 0.1.7 or later (there was no Windows support for earlier versions). Ensure you have Visual C/C++ and CMake installed on your system.
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

