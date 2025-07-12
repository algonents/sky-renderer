use crate::core::{Color, GeometryProvider, Mesh, Renderable, Renderer, Shader};
use crate::engine::opengl::GLfloat;
use crate::graphics2d;
use crate::graphics2d::shape::Shape;
use crate::graphics2d::{circle_geometry, point_geometry, rectangle_geometry};
use glam::{Mat4, Vec3};
use std::rc::Rc;

pub fn default_shader() -> Rc<Shader> {
    let vert_src = include_str!("shaders/shape.vert");
    let frag_src = include_str!("shaders/shape.frag");
    Rc::new(Shader::compile(vert_src, frag_src, None).expect("Failed to compile shader"))
}

pub fn point_shader() ->Rc<Shader>{
    let vert_src = include_str!("shaders/shape.vert");
    let frag_src = include_str!("shaders/point.frag");
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




pub struct ShapeRenderable {
    x: f32,
    y: f32,
    mesh: Mesh,
    
}
impl Renderable for ShapeRenderable {
    fn render(&mut self, renderer: &Renderer) {
        let (viewport_width, viewport_height) = renderer.viewport_size();
        let transform = ortho_2d(viewport_width as f32, viewport_height as f32)
            * Mat4::from_translation(Vec3::new(self.x, self.y, 0.0))
            * Mat4::from_scale(Vec3::splat(SCALE_FACTOR));
        self.mesh.set_transform(transform);
        renderer.draw_mesh(&self.mesh);
    }
}

impl ShapeRenderable {
    pub fn new(x: f32, y: f32, mesh: Mesh) -> Self {
        Self {x, y, mesh }
    }

    pub fn from_shape<S: Shape>(x: f32, y: f32, shape: S, color: Color) -> Self
    where
        S: Shape + GeometryProvider,
    {
        let geometry = shape.to_geometry();
        let mesh = Mesh::with_color(default_shader(), geometry, Some(color));
        ShapeRenderable::new(x, y, mesh)
    }

    pub fn point(x:GLfloat, y:GLfloat, color: Color)->Self{
        let geometry = point_geometry();
        let mesh = Mesh::with_color(point_shader(), geometry, Some(color));
        ShapeRenderable::new(x, y, mesh)
    }

    pub fn points(points: &[(GLfloat, GLfloat)], color: Color) -> Self {
        let geometry = graphics2d::multi_point_geometry(points);
        let mesh = Mesh::with_color(point_shader(), geometry, Some(color));
        ShapeRenderable::new(0.0, 0.0, mesh)
    }


    pub fn line(x1: GLfloat, y1: GLfloat, x2: GLfloat, y2: GLfloat, color: Color) -> Self {
        // Shift line coordinates so that the line starts at (0,0)
        let rel_x2 = x2 - x1;
        let rel_y2 = y2 - y1;

        // Build geometry with points relative to (0,0)
        let geometry = graphics2d::line_geometry(0.0, 0.0, rel_x2, rel_y2);
        let mesh = Mesh::with_color(default_shader(), geometry, Some(color));

        // Drawable positioned at the original start point (x1, y1)
        ShapeRenderable::new(x1, y1, mesh)
    }

    pub fn polyline(points: &[(GLfloat, GLfloat)], color: Color) -> Self {
        assert!(points.len() >= 2, "Polyline requires at least two points");

        let (x0, y0) = points[0];
        let geometry = graphics2d::polyline_geometry(points);
        let mesh = Mesh::with_color(default_shader(), geometry, Some(color));

        ShapeRenderable::new(x0, y0, mesh)
    }


    pub fn rectangle(x: f32, y: f32, width: f32, height: f32, color: Color) -> Self {
        // Geometry is created at (0, 0) with given width and height
        let geometry = rectangle_geometry(width, height);
        let mesh = Mesh::with_color(default_shader(), geometry, Some(color));
        // Drawable will be positioned at (x, y) — the top-left corner
        ShapeRenderable::new(x, y, mesh)
    }
    pub fn circle(x: f32, y: f32, radius: f32, color: Color) -> Self {
        // Geometry is built as a circle centered at (0, 0)
        let geometry = circle_geometry(radius, 100);
        let mesh = Mesh::with_color(default_shader(), geometry, Some(color));
        // Drawable is positioned at (x, y), which will be the circle's center
        ShapeRenderable::new(x, y, mesh)
    }
}
