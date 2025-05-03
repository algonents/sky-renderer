extern crate sky_renderer;

use sky_renderer::windowing::glfw::{
    GLFWwindow, glfw_create_window, glfw_poll_events, glfw_swap_buffers, glfw_terminate,
    glfw_window_should_close,
};

use sky_renderer::renderer::opengl::{gl_clear_color, gl_viewport};

extern "C" fn on_viewport_resized(_window: *const GLFWwindow, width: i32, height: i32) {
    println!("Viewport resized, width: {}, height: {}", width, height);
    gl_viewport(0, 0, width, height);
}

fn main() {
    let window = glfw_create_window("Hello Window", 800, 600, Some(on_viewport_resized));

    while !glfw_window_should_close(window) {
        gl_clear_color(0.07, 0.13, 0.17, 1.0);
        glfw_swap_buffers(window);
        glfw_poll_events();
    }
    glfw_terminate();
}
