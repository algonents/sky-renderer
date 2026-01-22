extern crate sky_renderer;

use sky_renderer::core::engine::glfw::{
    glfw_get_platform, GLFW_PLATFORM_WAYLAND, GLFW_PLATFORM_X11,
    GLFW_PLATFORM_COCOA, GLFW_PLATFORM_WIN32,
};
use sky_renderer::core::{App, Window};

fn main() {
    let mut  window = Window::new("Hello Window", 800, 600);

    let platform = glfw_get_platform();
    let platform_name = match platform {
        GLFW_PLATFORM_X11 => "X11",
        GLFW_PLATFORM_WAYLAND => "Wayland",
        GLFW_PLATFORM_COCOA => "macOS (Cocoa)",
        GLFW_PLATFORM_WIN32 => "Windows (Win32)",
        _ => "Unknown",
    };
    println!("Running on platform: {}", platform_name);

    window.on_resize(move |w, h| {
        println!("window resized, width:{}, height: {}", w, h);
    });

    (&mut *window).on_cursor_position(move |x_pos:f64, y_pos: f64|{
        println!("Mouse moved, x_pos:{}, y_pos: {}", x_pos, y_pos);
    });

    window.on_scroll(move |x_offset, y_offset| {
        println!("Mouse scrolled, x_offset:{}, y_offset: {}", x_offset, y_offset);
    });

    let app = App::new(window);
    app.run();
}
