use std::rc::Rc;
use glam::{Mat4, Vec3};
use crate::core::{Attribute, Geometry, Mesh, Renderer, Shader};
use crate::engine::opengl::{GLfloat, GL_LINES, GL_TRIANGLE_FAN, GL_TRIANGLE_STRIP};
use crate::graphics2d;

pub fn default_shader() -> Rc<Shader> {
    let vert_src = include_str!("shaders/shape.vert");
    let frag_src = include_str!("shaders/shape.frag");
    Rc::new(Shader::compile(vert_src, frag_src, None).expect("Failed to compile shader"))
}

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

const SCALE_FACTOR: f32 = 1.0;

pub struct Drawable {
    mesh: Mesh,
    x: f32,
    y: f32,
}

impl Drawable {
    pub fn new(x: f32, y: f32, mesh: Mesh, ) -> Self {
        Self { mesh, x, y }
    }

    pub fn line(
        x1: GLfloat,
        y1: GLfloat,
        x2: GLfloat,
        y2: GLfloat,
        r: GLfloat,
        g: GLfloat,
        b: GLfloat,
    ) -> Self {
        // Shift line coordinates so that the line starts at (0,0)
        let rel_x2 = x2 - x1;
        let rel_y2 = y2 - y1;

        // Build geometry with points relative to (0,0)
        let geometry = graphics2d::line_geometry(0.0, 0.0, rel_x2, rel_y2, r, g, b);
        let mesh = Mesh::new(geometry, graphics2d::default_shader());

        // Drawable positioned at the original start point (x1, y1)
        Drawable::new(x1, y1, mesh)
    }
    pub fn rectangle(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        r: f32,
        g: f32,
        b: f32,
    ) -> Self {
        // Geometry is created at (0, 0) with given width and height
        let geometry = rectangle_geometry(width, height, r, g, b);
        let mesh = Mesh::new(geometry, default_shader());
        // Drawable will be positioned at (x, y) — the top-left corner
        Self::new(x, y, mesh)
    }
    pub fn circle(
        x: f32,
        y: f32,
        radius: f32,
        r: f32,
        g: f32,
        b: f32,
    ) -> Self {
        // Geometry is built as a circle centered at (0, 0)
        let geometry = circle_geometry(radius, 100, r, g, b);
        let mesh = Mesh::new(geometry, default_shader());
        // Drawable is positioned at (x, y), which will be the circle's center
        Drawable::new(x, y, mesh)
    }
    
    pub fn draw(&mut self, renderer: &Renderer) {
        let (viewport_width, viewport_height) = renderer.viewport_size();
        let transform = graphics2d::ortho_2d(viewport_width as f32, viewport_height as f32)
            * Mat4::from_translation(Vec3::new(self.x, self.y, 0.0))
            * Mat4::from_scale(Vec3::splat(SCALE_FACTOR));
        self.mesh.set_transform(transform);
        renderer.draw_mesh(&self.mesh);
    }
}
fn line_geometry(
    x1: GLfloat,
    y1: GLfloat,
    x2: GLfloat,
    y2: GLfloat,
    r: GLfloat,
    g: GLfloat,
    b: GLfloat,
) -> Geometry {
    let vertices: Vec<GLfloat> = vec![
        x1, y1, r, g, b,  // start point
        x2, y2, r, g, b,  // end point
    ];

    let position_values_per_vertex = 2;
    let color_values_per_vertex = 3;
    let values_per_vertex = position_values_per_vertex + color_values_per_vertex;

    let mut geometry = Geometry::new(GL_LINES);
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

fn rectangle_geometry(
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

fn circle_geometry(
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








