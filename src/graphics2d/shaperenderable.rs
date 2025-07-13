use crate::core::{Color, GeometryProvider, Mesh, Renderable, Renderer, Shader};
use crate::engine::opengl::GLfloat;
use crate::graphics2d;
use crate::graphics2d::shape::{Shape, ShapeKind};
use crate::graphics2d::{circle_geometry, point_geometry, rectangle_geometry};
use glam::{Mat4, Vec3};
use std::rc::Rc;

pub fn default_shader() -> Rc<Shader> {
    let vert_src = include_str!("shaders/shape.vert");
    let frag_src = include_str!("shaders/shape.frag");
    Rc::new(Shader::compile(vert_src, frag_src, None).expect("Failed to compile shader"))
}

pub fn point_shader() -> Rc<Shader> {
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
    kind: ShapeKind,
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
    pub fn new(x: f32, y: f32, mesh: Mesh, kind: ShapeKind) -> Self {
        Self { x, y, mesh, kind }
    }

    pub fn from_shape<S: Shape>(x: f32, y: f32, shape: S, color: Color) -> Self
    where
        S: Shape + GeometryProvider,
    {
        let geometry = shape.to_geometry();
        let mesh = Mesh::with_color(default_shader(), geometry, Some(color));
        ShapeRenderable::new(x, y, mesh, shape.kind())
    }

    pub fn point(x: GLfloat, y: GLfloat, color: Color) -> Self {
        let geometry = point_geometry();
        let mesh = Mesh::with_color(point_shader(), geometry, Some(color));
        ShapeRenderable::new(x, y, mesh, ShapeKind::Point {})
    }

    pub fn points(points: &[(GLfloat, GLfloat)], color: Color) -> Self {
        let geometry = graphics2d::multi_point_geometry(points);
        let mesh = Mesh::with_color(point_shader(), geometry, Some(color));
        ShapeRenderable::new(
            0.0,
            0.0,
            mesh,
            ShapeKind::MultiPoint {
                points: points.to_vec(),
            },
        )
    }

    pub fn line(x1: GLfloat, y1: GLfloat, x2: GLfloat, y2: GLfloat, color: Color) -> Self {
        // Shift line coordinates so that the line starts at (0,0)
        let rel_x2 = x2 - x1;
        let rel_y2 = y2 - y1;

        // Build geometry with points relative to (0,0)
        let geometry = graphics2d::line_geometry(0.0, 0.0, rel_x2, rel_y2);
        let mesh = Mesh::with_color(default_shader(), geometry, Some(color));

        // Drawable positioned at the original start point (x1, y1)
        ShapeRenderable::new(x1, y1, mesh, ShapeKind::Line { x2, y2 })
    }

    pub fn polyline(points: &[(GLfloat, GLfloat)], color: Color) -> Self {
        assert!(points.len() >= 2, "Polyline requires at least two points");

        let (x0, y0) = points[0];
        let geometry = graphics2d::polyline_geometry(points);
        let mesh = Mesh::with_color(default_shader(), geometry, Some(color));

        ShapeRenderable::new(x0, y0, mesh, ShapeKind::Polyline {points:points.to_vec()})
    }

    pub fn rectangle(x: f32, y: f32, width: f32, height: f32, color: Color) -> Self {
        // Geometry is created at (0, 0) with given width and height
        let geometry = rectangle_geometry(width, height);
        let mesh = Mesh::with_color(default_shader(), geometry, Some(color));
        // Drawable will be positioned at (x, y) — the top-left corner
        ShapeRenderable::new(x, y, mesh, ShapeKind::Rectangle {width, height})
    }

    pub fn rounded_rectangle(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        radius: f32,
        color: Color,
    ) -> Self {
        let geometry = graphics2d::rounded_rectangle_geometry(width, height, radius, 8);
        let mesh = Mesh::with_color(default_shader(), geometry, Some(color));
        ShapeRenderable::new(
            x,
            y,
            mesh,
            ShapeKind::RoundedRectangle {
                width,
                height,
                radius,
            },
        )
    }

    pub fn polygon(points: &[(GLfloat, GLfloat)], color: Color) -> Self {
        assert!(points.len() >= 3, "Polygon requires at least 3 points");

        let (x0, y0) = points[0];
        let geometry = graphics2d::polygon_geometry(points);
        let mesh = Mesh::with_color(default_shader(), geometry, Some(color));

        ShapeRenderable::new(
            x0,
            y0,
            mesh,
            ShapeKind::Polygon {
                points: points.to_vec(),
            },
        )
    }
    pub fn circle(x: f32, y: f32, radius: f32, color: Color) -> Self {
        // Geometry is built as a circle centered at (0, 0)
        let geometry = circle_geometry(radius, 100);
        let mesh = Mesh::with_color(default_shader(), geometry, Some(color));
        // Drawable is positioned at (x, y), which will be the circle's center
        ShapeRenderable::new(x, y, mesh, ShapeKind::Circle {radius})
    }

    pub fn ellipse(x: f32, y: f32, radius_x: f32, radius_y: f32, color: Color) -> Self {
        let geometry = graphics2d::ellipse_geometry(radius_x, radius_y, 64); // 64 segments for smoothness
        let mesh = Mesh::with_color(default_shader(), geometry, Some(color));
        ShapeRenderable::new(
            x,
            y,
            mesh,
            ShapeKind::Ellipse {
                radius_x,
                radius_y,
            },
        )
    }
    
    fn svg_color(&self) -> String {
        self.mesh
            .color
            .as_ref()
            .map(|c| c.to_hex())
            .unwrap_or_else(|| "#000000".to_string())
    }

    pub fn to_svg(&self) -> String {
        match &self.kind {
            ShapeKind::Line { x2, y2 } => {
                format!(
                    r#"<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" stroke="{color}" stroke-width="1"/>"#,
                    x1 = self.x,
                    y1 = self.y,
                    x2 = x2,
                    y2 = y2,
                    color = self.svg_color(),
                )
            }
            ShapeKind::Rectangle { width, height } => {
                format!(
                    r#"<rect x="{x}" y="{y}" width="{w}" height="{h}" fill="{color}"/>"#,
                    x = self.x,
                    y = self.y,
                    w = width,
                    h = height,
                    color = self.svg_color(),
                )
            }
            ShapeKind::RoundedRectangle {width, height, radius}=>String::new(),
            ShapeKind::Polygon {points}=>String::new(),
            ShapeKind::Circle { radius } => {
                format!(
                    r#"<circle cx="{cx}" cy="{cy}" r="{r}" fill="{color}"/>"#,
                    cx = self.x + radius,
                    cy = self.y + radius,
                    r = radius,
                    color = self.svg_color(),
                )
            }
            ShapeKind::Ellipse {radius_x, radius_y}=>String::new(),
            ShapeKind::Polyline { points } => {
                let path = points
                    .iter()
                    .map(|(px, py)| format!("{},{}", px + self.x, py + self.y))
                    .collect::<Vec<_>>()
                    .join(" ");
                format!(
                    r#"<polyline points="{path}" fill="none" stroke="{color}" stroke-width="1"/>"#,
                    path = path,
                    color = self.svg_color(),
                )
            }
            ShapeKind::MultiPoint { points } => {
                let mut out = String::new();
                for (px, py) in points {
                    let cx = px + self.x;
                    let cy = py + self.y;
                    out.push_str(&format!(
                        r#"<circle cx="{cx}" cy="{cy}" r="2" fill="{color}"/>"#,
                        cx = cx,
                        cy = cy,
                        color = self.svg_color(),
                    ));
                }
                out
            }
            ShapeKind::Point => {
                format!(
                    r#"<circle cx="{cx}" cy="{cy}" r="2" fill="{color}"/>"#,
                    cx = self.x,
                    cy = self.y,
                    color = self.svg_color(),
                )
            }
        }
    }
}
