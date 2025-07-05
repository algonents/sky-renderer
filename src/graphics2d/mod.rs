use glam::Mat4;
use crate::core::{Attribute, Geometry};
use crate::engine::opengl::{GLfloat, GL_TRIANGLE_FAN, GL_TRIANGLE_STRIP};


/// Creates a right-handed orthographic projection matrix for 2D rendering.
///
/// The coordinate system origin is at the **top-left corner** of the viewport,
/// with the x-axis pointing right and the y-axis pointing down.
///
/// # Parameters
/// - `width`: The width of the viewport in pixels.
/// - `height`: The height of the viewport in pixels.
///
/// # Returns
/// A [`Mat4`] representing the orthographic projection matrix suitable for OpenGL.
pub fn ortho_2d(width: f32, height: f32) -> Mat4 {
    Mat4::orthographic_rh_gl(0.0, width, height, 0.0, 0.0, 1.0)
}

/// Creates a rectangle starting at (0, 0) with the given width and height, and RGB color.
pub fn rectangle(
    width: GLfloat,
    height: GLfloat,
    r: GLfloat,
    g: GLfloat,
    b: GLfloat,
) -> Geometry {
    let vertices: Vec<GLfloat> = vec![
        // bottom-left
        0.0,     0.0,      r, g, b,
        // bottom-right
        width,   0.0,      r, g, b,
        // top-left
        0.0,     height,   r, g, b,
        // top-right
        width,   height,   r, g, b,
    ];

    let position_values_per_vertex = 2;
    let color_values_per_vertex = 3;
    let values_per_vertex = position_values_per_vertex + color_values_per_vertex;

    let mut geometry = Geometry::new(GL_TRIANGLE_STRIP);
    geometry.add_buffer(&vertices, values_per_vertex);

    geometry.add_vertex_attribute(Attribute::new(
        0,
        position_values_per_vertex,
        values_per_vertex as usize,
        0,
    ));

    geometry.add_vertex_attribute(Attribute::new(
        1,
        color_values_per_vertex,
        values_per_vertex as usize,
        position_values_per_vertex as usize,
    ));

    geometry
}

pub fn circle(
    radius: GLfloat,
    segments: usize,
    r: GLfloat,
    g: GLfloat,
    b: GLfloat,
) -> Geometry {
    let mut vertices: Vec<GLfloat> = Vec::with_capacity((segments + 2) * 5); // center + segments + wrap-around

    // Center of the circle
    vertices.extend_from_slice(&[0.0, 0.0, r, g, b]);

    // Outer vertices
    for i in 0..=segments {
        let theta = (i as f32 / segments as f32) * std::f32::consts::TAU; // TAU = 2π
        let x = radius * theta.cos();
        let y = radius * theta.sin();
        vertices.extend_from_slice(&[x, y, r, g, b]);
    }

    let position_values_per_vertex = 2;
    let color_values_per_vertex = 3;
    let values_per_vertex = position_values_per_vertex + color_values_per_vertex;

    let mut geometry = Geometry::new(GL_TRIANGLE_FAN);
    geometry.add_buffer(&vertices, values_per_vertex);

    geometry.add_vertex_attribute(Attribute::new(
        0,
        position_values_per_vertex,
        values_per_vertex as usize,
        0,
    ));

    geometry.add_vertex_attribute(Attribute::new(
        1,
        color_values_per_vertex,
        values_per_vertex as usize,
        position_values_per_vertex as usize,
    ));

    geometry
}








