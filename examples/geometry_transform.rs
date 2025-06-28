extern crate sky_renderer;

use std::f32::consts::PI;
use std::ffi::c_void;

use glam::{Mat4, Vec3};
use sky_renderer::core::geometry::{Attribute, Geometry};
use sky_renderer::core::mesh::Mesh;
use sky_renderer::core::renderer::Renderer;
use sky_renderer::core::shader::Shader;

use sky_renderer::engine::opengl::{
    GL_TRIANGLES, GL_VIEWPORT, GLfloat, gl_clear_color, gl_get_integerv, gl_viewport,
};

use sky_renderer::windowing::glfw::{
    GLFWwindow, glfw_create_window, glfw_poll_events, glfw_swap_buffers, glfw_terminate,
    glfw_window_should_close,
};

extern "C" fn on_viewport_resized(_window: *const GLFWwindow, width: i32, height: i32) {
    gl_viewport(0, 0, width, height);
}

fn ortho_2_d(width: f32, height: f32) -> Mat4 {
    Mat4::orthographic_rh_gl(0.0, width, height, 0.0, 0.0, 1.0)
}

fn main() {
    let vertex_shader_source = "
    #version 330 core
    layout (location = 0) in vec2 aPos;
    uniform mat4 transform;
    void main()
    {
       gl_Position = transform * vec4(aPos.x, aPos.y, 0.0, 1.0);
    }
    ";

    let fragment_shader_source = "
    #version 330 core
    out vec4 FragColor;
    void main()
    {
        FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
    ";

    let vertices: Vec<GLfloat> = vec![
        -10.0, 0.0, // bottom-left
        10.0, 0.0, // bottom-right
        0.0, 10.0, // top-center
    ];
    let values_per_vertex = 2;

    let window = glfw_create_window("Geometry", 800, 600, Some(on_viewport_resized));

    let mut geometry = Geometry::new(GL_TRIANGLES);
    geometry.add_buffer(&vertices, values_per_vertex);

    geometry.add_vertex_attribute(Attribute::new(0, 2, 2, 0));

    let shader = Shader::compile(vertex_shader_source, fragment_shader_source)
        .expect("Failed to compile shader");

    let mesh = Mesh::new(geometry, shader);

    let mut renderer = Renderer::new();

    let mut viewport = [0, 0, 0, 0];

    while !glfw_window_should_close(window) {
        gl_clear_color(0.2, 0.3, 0.3, 1.0);

        // invert the triangle and scale it by 1.3
        let local_to_world = Mat4::from_scale(Vec3::new(1.3, -1.3, 1.3));

        // define orthographic projection using the viewport size
        gl_get_integerv(GL_VIEWPORT, viewport.as_mut_ptr() as *mut c_void);
        let ortho_2d_projection = ortho_2_d(viewport[2] as f32, viewport[3] as f32);

        // translate triangle to center of the screen
        let translation = Mat4::from_translation(glam::vec3(
            viewport[2] as f32 / 2.0,
            viewport[3] as f32 / 2.0,
            0.0,
        ));

        // Finally combine into a single transformation
        let transform = ortho_2d_projection * translation * local_to_world;

        renderer.set_transform(transform);

        renderer.draw_mesh(&mesh);

        glfw_swap_buffers(window);
        glfw_poll_events();
    }

    glfw_terminate();
}
