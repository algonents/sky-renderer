use glam::{Mat4, Vec3};
use std::rc::Rc;
use std::cell::OnceCell;



use crate::core::engine::opengl::GLfloat;
use crate::core::{
    Color, GeometryProvider, Mesh, Renderable, Renderer, Shader, generate_texture_from_image,
    load_image,
};
use crate::graphics2d;
use crate::graphics2d::shapes::{Rectangle, Shape, ShapeKind};
use crate::graphics2d::{
    circle_geometry, image_geometry, point_geometry, triangle_geometry,
};

const SCALE_FACTOR: f32 = 1.0;

thread_local! {
    static DEFAULT_SHADER: OnceCell<Rc<Shader>> = OnceCell::new();
}


fn default_shader() -> Rc<Shader> {
    DEFAULT_SHADER.with(|cell| {
        cell.get_or_init(|| {
            let vert_src = include_str!("../shaders/shape.vert");
            let frag_src = include_str!("../shaders/shape.frag");
            Rc::new(
                Shader::compile(vert_src, frag_src, None)
                    .expect("Failed to compile default shader"),
            )
        }).clone()
    })
}

thread_local! {
    static POINT_SHADER: OnceCell<Rc<Shader>> = OnceCell::new();
}

fn point_shader() -> Rc<Shader> {
    POINT_SHADER.with(|cell| {
        cell.get_or_init(|| {
            let vert_src = include_str!("../shaders/shape.vert");
            let frag_src = include_str!("../shaders/point.frag");
            Rc::new(
                Shader::compile(vert_src, frag_src, None)
                    .expect("Failed to compile point shader"),
            )
        })
            .clone()
    })
}


thread_local! {
    static IMAGE_SHADER: OnceCell<Rc<Shader>> = OnceCell::new();
}
fn image_shader() -> Rc<Shader> {
    IMAGE_SHADER.with(|cell| {
        cell.get_or_init(|| {
            let vert_src = include_str!("../shaders/image.vert");
            let frag_src = include_str!("../shaders/image.frag");
            Rc::new(
                Shader::compile(vert_src, frag_src, None)
                    .expect("Failed to compile image shader"),
            )
        })
            .clone()
    })
}


fn ortho_2d_with_zoom(width: f32, height: f32, zoom: f32) -> Mat4 {
    let half_w = width * 0.5 / zoom;
    let half_h = height * 0.5 / zoom;

    let left = width * 0.5 - half_w;
    let right = width * 0.5 + half_w;
    let top = height * 0.5 - half_h;
    let bottom = height * 0.5 + half_h;

    Mat4::orthographic_rh_gl(left, right, bottom, top, 0.0, 1.0)
}
pub struct ShapeRenderable {
    x: f32,
    y: f32,
    mesh: Mesh,
    kind: ShapeKind,
}
impl Renderable for ShapeRenderable {
    fn render(&mut self, renderer: &Renderer) {
        let (vw, vh) = renderer.viewport_size();
        let transform = ortho_2d_with_zoom(vw as f32, vh as f32, renderer.zoom_level)
            * Mat4::from_scale(Vec3::splat(SCALE_FACTOR));
        self.mesh.set_transform(transform);

        if self.mesh.geometry.instance_count() > 0 {
            // instanced: u_offset = (0,0), positions come from attrib 1
            renderer.draw_mesh_instanced(&self.mesh);
        } else {
            // single: use u_offset
            self.mesh.set_screen_offset(self.x, self.y);
            renderer.draw_mesh(&self.mesh);
        }
    }
}

impl ShapeRenderable {
    fn new(x: f32, y: f32, mesh: Mesh, kind: ShapeKind) -> Self {
        Self { x, y, mesh, kind }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
    pub fn from_shape<S: Shape>(x: f32, y: f32, shape: S, color: Color) -> Self
    where
        S: Shape + GeometryProvider,
    {
        let geometry = shape.to_geometry();
        let mesh = Mesh::with_color(default_shader(), geometry, Some(color));
        ShapeRenderable::new(x, y, mesh, shape.kind())
    }

    pub fn enable_instancing(&mut self, capacity: usize) {
        self.mesh.geometry.enable_instancing_xy(capacity);
    }

    pub fn set_instances(&mut self, positions: &[(f32, f32)]) {
        self.mesh.geometry.update_instance_xy(positions);
    }

    pub fn clear_instances(&mut self) {
        self.mesh.geometry.clear_instancing();
    }

    pub fn point(x: GLfloat, y: GLfloat, color: Color) -> Self {
        let geometry = point_geometry();
        let mesh = Mesh::with_color(point_shader(), geometry, Some(color));
        ShapeRenderable::new(x, y, mesh, ShapeKind::Point {})
    }

    pub fn points(points: &[(GLfloat, GLfloat)], color: Color) -> Self {
        let (x0, y0) = points[0];

        // Shift points to be relative to anchor
        let rel_points: Vec<(GLfloat, GLfloat)> =
            points.iter().map(|(x, y)| (x - x0, y - y0)).collect();

        let geometry = graphics2d::point_list_geometry(&rel_points);
        let mesh = Mesh::with_color(point_shader(), geometry, Some(color));

        ShapeRenderable::new(x0, y0, mesh, ShapeKind::MultiPoint { points: rel_points })
    }

    pub fn simple_line(x1: GLfloat, y1: GLfloat, x2: GLfloat, y2: GLfloat, stroke: Color) -> Self {
        ShapeRenderable::line(x1, y1, x2, y2, stroke, 1.0)
    }

    pub fn line(
        x1: GLfloat,
        y1: GLfloat,
        x2: GLfloat,
        y2: GLfloat,
        stroke: Color,
        stroke_width: f32,
    ) -> Self {
        // Shift line coordinates so that the line starts at (0,0)
        let rel_x2 = x2 - x1;
        let rel_y2 = y2 - y1;

        // Build geometry with points relative to (0,0)
        let geometry = graphics2d::line_geometry(0.0, 0.0, rel_x2, rel_y2, stroke_width);
        let mesh = Mesh::with_color(default_shader(), geometry, Some(stroke));

        // Drawable positioned at the original start point (x1, y1)
        ShapeRenderable::new(x1, y1, mesh, ShapeKind::Line { x2, y2 })
    }

    pub fn polyline(points: &[(GLfloat, GLfloat)], stroke: Color, stroke_width: f32) -> Self {
        assert!(points.len() >= 2, "Polyline requires at least two points");

        assert!(points.len() >= 2);

        let (x0, y0) = points[0];
        let rel_points: Vec<(f32, f32)> = points.iter().map(|(x, y)| (x - x0, y - y0)).collect();

        let geometry = graphics2d::polyline_geometry(&rel_points, stroke_width);
        let mesh = Mesh::with_color(default_shader(), geometry, Some(stroke));

        ShapeRenderable::new(x0, y0, mesh, ShapeKind::Polyline { points: rel_points })
    }

    pub fn arc(
        center: (f32, f32),
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        stroke: Color,
        stroke_width: f32,
        segments: usize,
    ) -> Self {
        use std::f32::consts::TAU;

        let (cx, cy) = center;

        // Normalize sweep to [0, TAU)
        let mut sweep = end_angle - start_angle;
        if sweep < 0.0 {
            sweep += TAU;
        }

        // Generate points counter-clockwise from start to end
        let mut points = Vec::with_capacity(segments + 1);
        for i in 0..=segments {
            let t = i as f32 / segments as f32;
            let theta = start_angle + t * sweep;
            let x = cx + radius * theta.cos();
            let y = cy - radius * theta.sin(); // flip y screen coordinate to match math conventions
            points.push((x, y));
        }

        Self::polyline(&points, stroke, stroke_width)
    }

    pub fn triangle(x: f32, y: f32, vertices: &[(f32, f32); 3], color: Color) -> Self {
        let geometry = triangle_geometry(vertices);
        let mesh = Mesh::with_color(default_shader(), geometry, Some(color));

        ShapeRenderable::new(
            x,
            y,
            mesh,
            ShapeKind::Triangle {
                vertices: vertices.clone(),
            },
        )
    }

    pub fn rectangle(x: f32, y: f32, width: f32, height: f32, color: Color) -> Self {
        // Geometry is created at (0, 0) with given width and height
        let rectangle = Rectangle::new(width, height);
        let geometry = rectangle.to_geometry();
        let mesh = Mesh::with_color(default_shader(), geometry, Some(color));
        // Drawable will be positioned at (x, y) — the top-left corner
        ShapeRenderable::new(x, y, mesh, ShapeKind::Rectangle { width, height })
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

        let (x0, y0) = points[0]; // Anchor
        let rel_points: Vec<(f32, f32)> = points.iter().map(|(x, y)| (x - x0, y - y0)).collect();

        let geometry = graphics2d::polygon_geometry(&rel_points);
        let mesh = Mesh::with_color(default_shader(), geometry, Some(color));

        ShapeRenderable::new(x0, y0, mesh, ShapeKind::Polygon { points: rel_points })
    }
    pub fn circle(x: f32, y: f32, radius: f32, color: Color) -> Self {
        // Geometry is built as a circle centered at (0, 0)
        let geometry = circle_geometry(radius, 100);
        let mesh = Mesh::with_color(default_shader(), geometry, Some(color));
        // Drawable is positioned at (x, y), which will be the circle's center
        ShapeRenderable::new(x, y, mesh, ShapeKind::Circle { radius })
    }

    pub fn ellipse(x: f32, y: f32, radius_x: f32, radius_y: f32, color: Color) -> Self {
        let geometry = graphics2d::ellipse_geometry(radius_x, radius_y, 64); // 64 segments for smoothness
        let mesh = Mesh::with_color(default_shader(), geometry, Some(color));
        ShapeRenderable::new(x, y, mesh, ShapeKind::Ellipse { radius_x, radius_y })
    }

    pub fn image_with_size(x: f32, y: f32, path: &str, width: f32, height: f32) -> ShapeRenderable {
        // Load image data and upload to GPU
        let image = load_image(path);

        let texture_id = generate_texture_from_image(&image);

        // You likely want to query dimensions in `generate_texture_from_image`
        // But if not, load dimensions again:
        //let (width, height, _) = load_image(path); // image module only used for size

        // Create image geometry (2-triangle quad)
        let geometry = image_geometry(width, height);

        // Use image shader and attach texture
        let shader = image_shader(); // assumes you have an Rc<Shader> loader
        let mesh = Mesh::with_texture(shader, geometry, Some(texture_id));

        ShapeRenderable::new(x, y, mesh, ShapeKind::Image { width, height })
    }

    pub fn image(x: f32, y: f32, path: &str) -> Self {
        let image = load_image(path);
        Self::image_with_size(x, y, path, image.width as f32, image.height as f32)
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
            ShapeKind::RoundedRectangle {
                width,
                height,
                radius,
            } => {
                format!(
                    r#"<rect x="{x}" y="{y}" width="{w}" height="{h}" rx="{r}" ry="{r}" fill="{color}"/>"#,
                    x = self.x,
                    y = self.y,
                    w = width,
                    h = height,
                    r = radius,
                    color = self.svg_color(),
                )
            }
            ShapeKind::Polygon { points } => {
                let path = points
                    .iter()
                    .map(|(px, py)| format!("{},{}", px + self.x, py + self.y))
                    .collect::<Vec<_>>()
                    .join(" ");

                format!(
                    r#"<polygon points="{path}" fill="{color}" stroke="{color}" stroke-width="1"/>"#,
                    path = path,
                    color = self.svg_color(),
                )
            }
            ShapeKind::Circle { radius } => {
                format!(
                    r#"<circle cx="{cx}" cy="{cy}" r="{r}" fill="{color}"/>"#,
                    cx = self.x + radius,
                    cy = self.y + radius,
                    r = radius,
                    color = self.svg_color(),
                )
            }
            ShapeKind::Ellipse { radius_x, radius_y } => {
                format!(
                    r#"<ellipse cx="{cx}" cy="{cy}" rx="{rx}" ry="{ry}" fill="{color}"/>"#,
                    cx = self.x + radius_x,
                    cy = self.y + radius_y,
                    rx = radius_x,
                    ry = radius_y,
                    color = self.svg_color(),
                )
            }
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
            ShapeKind::Image {
                width: _width,
                height: _height,
            } => String::new(),
            ShapeKind::Triangle { vertices } => {
                let points: String = vertices
                    .iter()
                    .map(|(vx, vy)| format!("{:.2},{:.2}", vx + self.x, vy + self.y))
                    .collect::<Vec<_>>()
                    .join(" ");
                format!(
                    r#"<polygon points="{points}" fill="{color}"/>"#,
                    points = points,
                    color = self.svg_color(),
                )
            }
        }
    }
}
