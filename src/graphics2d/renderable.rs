use std::rc::Rc;
use glam::{Mat4, Vec3};
use crate::core::{Mesh, Renderer, Shader};
use crate::engine::opengl::GLfloat;
use crate::graphics2d;
use crate::graphics2d::{circle_geometry, rectangle_geometry};
use crate::graphics2d::shape::Shape;

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
fn ortho_2d(width: f32, height: f32) -> Mat4 {
    Mat4::orthographic_rh_gl(0.0, width, height, 0.0, 0.0, 1.0)
}

const SCALE_FACTOR: f32 = 1.0;

pub trait Renderable{
    fn render(&mut self, renderer: &Renderer);
}
pub struct RenderableShape {
    mesh: Mesh,
    x: f32,
    y: f32,
}
impl Renderable for RenderableShape {
    fn render(&mut self, renderer: &Renderer) {
        let (viewport_width, viewport_height) = renderer.viewport_size();
        let transform = ortho_2d(viewport_width as f32, viewport_height as f32)
            * Mat4::from_translation(Vec3::new(self.x, self.y, 0.0))
            * Mat4::from_scale(Vec3::splat(SCALE_FACTOR));
        self.mesh.set_transform(transform);
        renderer.draw_mesh(&self.mesh);
    }
}

impl RenderableShape {
    pub fn new(x: f32, y: f32, mesh: Mesh, ) -> Self {
        Self { mesh, x, y }
    }

    pub fn from_shape<S: Shape>(x: f32, y: f32, shape: S, color:(f32, f32, f32)) -> Self {
        let geometry = shape.to_geometry(color);
        let mesh = Mesh::new(geometry, default_shader() );
        Self::new(x, y, mesh)
    }

    pub fn line(
        x1: GLfloat,
        y1: GLfloat,
        x2: GLfloat,
        y2: GLfloat,
        color:(f32, f32, f32)
    ) -> Self {
        // Shift line coordinates so that the line starts at (0,0)
        let rel_x2 = x2 - x1;
        let rel_y2 = y2 - y1;

        // Build geometry with points relative to (0,0)
        let geometry = graphics2d::line_geometry(0.0, 0.0, rel_x2, rel_y2, color);
        let mesh = Mesh::new(geometry, default_shader());

        // Drawable positioned at the original start point (x1, y1)
        RenderableShape::new(x1, y1, mesh)
    }
    pub fn rectangle(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color:(f32, f32, f32)
    ) -> Self {
        // Geometry is created at (0, 0) with given width and height
        let geometry = rectangle_geometry(width, height, color);
        let mesh = Mesh::new(geometry, default_shader());
        // Drawable will be positioned at (x, y) — the top-left corner
        Self::new(x, y, mesh)
    }
    pub fn circle(
        x: f32,
        y: f32,
        radius: f32,
        color:(f32, f32, f32)
    ) -> Self {
        // Geometry is built as a circle centered at (0, 0)
        let geometry = circle_geometry(radius, 100, color);
        let mesh = Mesh::new(geometry, default_shader());
        // Drawable is positioned at (x, y), which will be the circle's center
        RenderableShape::new(x, y, mesh)
    }
}