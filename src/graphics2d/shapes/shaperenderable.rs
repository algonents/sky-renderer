use glam::{Mat4, Vec3};
use std::rc::Rc;
use std::cell::OnceCell;
use std::f32::consts::PI;
use crate::core::engine::opengl::{GLfloat, GL_POINTS, GL_TRIANGLES, GL_TRIANGLE_FAN, GL_TRIANGLE_STRIP};
use crate::core::{Color, Mesh, Renderable, Renderer, Shader, generate_texture_from_image, load_image, Geometry, Attribute};
use crate::graphics2d::shapes::{Shape, ShapeKind};
use crate::graphics2d::svg::ToSvg;

const MIN_STROKE_WIDTH: f32 = 1.5;
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
        // use the line below if you want to keep objects of constant size during zoom    
        //* Mat4::from_scale(Vec3::splat(SCALE_FACTOR/renderer.zoom_level));
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
        S: Shape
    {
        match &shape.kind() {
            ShapeKind::Rectangle { width, height } => {
                ShapeRenderable::rectangle(x, y, *width, *height, color)
            },
            _ => unimplemented!("ShapeRenderable::from_shape not implemented for this shape kind"),
        }
    }

    pub fn create_multiple_instances(&mut self, capacity: usize) {
        self.mesh.geometry.enable_instancing_xy(capacity);
    }

    pub fn set_instance_positions(&mut self, positions: &[(f32, f32)]) {
        self.mesh.geometry.update_instance_xy(positions);
    }

    pub fn clear_instances(&mut self) {
        self.mesh.geometry.clear_instancing();
    }

    pub fn point(x: GLfloat, y: GLfloat, color: Color) -> Self {
        let geometry = ShapeRenderable::point_geometry();
        let mesh = Mesh::with_color(point_shader(), geometry, Some(color));
        ShapeRenderable::new(x, y, mesh, ShapeKind::Point {})
    }

    pub fn points(points: &[(GLfloat, GLfloat)], color: Color) -> Self {
        let (x0, y0) = points[0];

        // Shift points to be relative to anchor
        let rel_points: Vec<(GLfloat, GLfloat)> =
            points.iter().map(|(x, y)| (x - x0, y - y0)).collect();

        let geometry = ShapeRenderable::point_list_geometry(&rel_points);
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
        let geometry = ShapeRenderable::line_geometry(0.0, 0.0, rel_x2, rel_y2, stroke_width);
        let mesh = Mesh::with_color(default_shader(), geometry, Some(stroke));

        // Drawable positioned at the original start point (x1, y1)
        ShapeRenderable::new(x1, y1, mesh, ShapeKind::Line { x2, y2 })
    }

    pub fn polyline(points: &[(GLfloat, GLfloat)], stroke: Color, stroke_width: f32) -> Self {
        assert!(points.len() >= 2, "Polyline requires at least two points");

        assert!(points.len() >= 2);

        let (x0, y0) = points[0];
        let rel_points: Vec<(f32, f32)> = points.iter().map(|(x, y)| (x - x0, y - y0)).collect();

        let geometry = ShapeRenderable::polyline_geometry(&rel_points, stroke_width);
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
        let geometry = ShapeRenderable::triangle_geometry(vertices);
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
        let geometry = ShapeRenderable::rectangle_geometry(width,height);
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
        let geometry = ShapeRenderable::rounded_rectangle_geometry(width, height, radius, 8);
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

        let geometry = ShapeRenderable::polygon_geometry(&rel_points);
        let mesh = Mesh::with_color(default_shader(), geometry, Some(color));

        ShapeRenderable::new(x0, y0, mesh, ShapeKind::Polygon { points: rel_points })
    }
    pub fn circle(x: f32, y: f32, radius: f32, color: Color) -> Self {
        // Geometry is built as a circle centered at (0, 0)
        let geometry = ShapeRenderable::circle_geometry(radius, 100);
        let mesh = Mesh::with_color(default_shader(), geometry, Some(color));
        // Drawable is positioned at (x, y), which will be the circle's center
        ShapeRenderable::new(x, y, mesh, ShapeKind::Circle { radius })
    }

    pub fn ellipse(x: f32, y: f32, radius_x: f32, radius_y: f32, color: Color) -> Self {
        let geometry = ShapeRenderable::ellipse_geometry(radius_x, radius_y, 64); // 64 segments for smoothness
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
        let geometry = ShapeRenderable::image_geometry(width, height);

        // Use image shader and attach texture
        let shader = image_shader(); // assumes you have an Rc<Shader> loader
        let mesh = Mesh::with_texture(shader, geometry, Some(texture_id));

        ShapeRenderable::new(x, y, mesh, ShapeKind::Image { width, height })
    }

    pub fn image(x: f32, y: f32, path: &str) -> Self {
        let image = load_image(path);
        Self::image_with_size(x, y, path, image.width as f32, image.height as f32)
    }

    fn point_geometry() -> Geometry {
        let vertex = vec![0.0, 0.0];
        let mut geometry = Geometry::new(GL_POINTS);
        geometry.add_buffer(&vertex, 2);

        geometry.add_vertex_attribute(Attribute::new(0, 2, 2, 0));

        geometry
    }

    fn point_list_geometry(points: &[(GLfloat, GLfloat)]) -> Geometry {
        let mut vertices = Vec::with_capacity(points.len() * 2);

        for &(x, y) in points {
            vertices.push(x);
            vertices.push(y);
        }

        let values_per_vertex = 2;

        let mut geometry = Geometry::new(GL_POINTS);
        geometry.add_buffer(&vertices, values_per_vertex);

        geometry.add_vertex_attribute(Attribute::new(
            0, // position
            values_per_vertex,
            values_per_vertex as usize,
            0,
        ));

        geometry
    }

    fn line_geometry(x1: GLfloat, y1: GLfloat, x2: GLfloat, y2: GLfloat, stroke_width:f32) -> Geometry {
        let stroke_width = stroke_width.max(MIN_STROKE_WIDTH);
        let dx = x2 - x1;
        let dy = y2 - y1;
        let length = (dx * dx + dy * dy).sqrt();

        if length == 0.0 {
            return Geometry::new(GL_TRIANGLES);
        }

        // Unit perpendicular vector
        let nx = -dy / length;
        let ny = dx / length;
        let half_thickness = stroke_width / 2.0;

        // Offset vector
        let ox = nx * half_thickness;
        let oy = ny * half_thickness;

        // Four corners of the quad
        let v0 = [x1 - ox, y1 - oy];
        let v1 = [x2 - ox, y2 - oy];
        let v2 = [x2 + ox, y2 + oy];
        let v3 = [x1 + ox, y1 + oy];

        let vertices: Vec<GLfloat> = vec![
            v0[0], v0[1],
            v1[0], v1[1],
            v2[0], v2[1],
            v2[0], v2[1],
            v3[0], v3[1],
            v0[0], v0[1],
        ];

        let position_values_per_vertex = 2;

        let mut geometry = Geometry::new(GL_TRIANGLES);
        geometry.add_buffer(&vertices, position_values_per_vertex);
        geometry.add_vertex_attribute(Attribute::new(
            0,
            position_values_per_vertex,
            position_values_per_vertex as usize,
            0,
        ));

        geometry
    }


    /// Polyline triangulation adapted from JVPolyline by Julien Vernay (2025)
    ///
    /// Original C implementation:
    /// https://jvernay.fr/en/blog/polyline-triangulation/
    /// Source: https://git.sr.ht/~jvernay/JV/tree/main/item/src/jv_polyline/jv_polyline.c
    ///
    /// This implementation is based on the original algorithm,
    /// restructured and translated to idiomatic Rust for use in sky_renderer.
    fn polyline_geometry(points: &[(GLfloat, GLfloat)], stroke_width:f32) -> Geometry {
        const MITER_LIMIT: f32 = 4.0; // Equivalent to JV default

        if points.len() < 2 {
            return Geometry::new(GL_TRIANGLES);
        }

        let half_thickness = stroke_width.max(1.0) / 2.0;
        let miter_limit_squared = (stroke_width * MITER_LIMIT).powi(2) / 4.0;
        let mut vertices: Vec<GLfloat> = Vec::new();

        let mut a = points[0];
        let mut b = points[1];

        let mut idx = 1;
        while idx < points.len() && (b.0 - a.0).hypot(b.1 - a.1) == 0.0 {
            idx += 1;
            if idx < points.len() {
                b = points[idx];
            }
        }
        if (b.0 - a.0).hypot(b.1 - a.1) == 0.0 {
            return Geometry::new(GL_TRIANGLES);
        }

        for i in idx + 1..=points.len() {
            let c = if i < points.len() { points[i] } else { a }; // fake point if last

            let ab = (b.0 - a.0, b.1 - a.1);
            let len_ab = (ab.0 * ab.0 + ab.1 * ab.1).sqrt();
            let normal_ab = (-ab.1 / len_ab * half_thickness, ab.0 / len_ab * half_thickness);

            let a1 = (a.0 + normal_ab.0, a.1 + normal_ab.1);
            let a2 = (a.0 - normal_ab.0, a.1 - normal_ab.1);
            let b1 = (b.0 + normal_ab.0, b.1 + normal_ab.1);
            let b2 = (b.0 - normal_ab.0, b.1 - normal_ab.1);

            // segment quad
            vertices.extend_from_slice(&[
                a1.0, a1.1,
                a2.0, a2.1,
                b1.0, b1.1,

                a2.0, a2.1,
                b1.0, b1.1,
                b2.0, b2.1,
            ]);

            let bc = (c.0 - b.0, c.1 - b.1);
            let len_bc = (bc.0 * bc.0 + bc.1 * bc.1).sqrt();
            if len_bc > 0.0 {
                let normal_bc = (-bc.1 / len_bc * half_thickness, bc.0 / len_bc * half_thickness);
                let b3 = (b.0 + normal_bc.0, b.1 + normal_bc.1);
                let b4 = (b.0 - normal_bc.0, b.1 - normal_bc.1);

                // turn direction
                let z = ab.0 * bc.1 - ab.1 * bc.0;

                // bevel join
                if z < 0.0 {
                    vertices.extend_from_slice(&[
                        b.0, b.1,
                        b1.0, b1.1,
                        b3.0, b3.1,
                    ]);
                } else if z > 0.0 {
                    vertices.extend_from_slice(&[
                        b.0, b.1,
                        b2.0, b2.1,
                        b4.0, b4.1,
                    ]);
                }

                // optional miter
                if z != 0.0 {
                    let (a_j, b_j, norm_j) = if z < 0.0 {
                        (a1, b3, ab)
                    } else {
                        (a2, b4, ab)
                    };

                    let denom = z;
                    let alpha = (bc.1 * (b_j.0 - a_j.0) + bc.0 * (a_j.1 - b_j.1)) / denom;
                    let mx = a_j.0 + alpha * norm_j.0;
                    let my = a_j.1 + alpha * norm_j.1;

                    let dist2 = (mx - b.0).powi(2) + (my - b.1).powi(2);
                    if dist2 <= miter_limit_squared {
                        if z < 0.0 {
                            vertices.extend_from_slice(&[
                                mx, my,
                                b1.0, b1.1,
                                b3.0, b3.1,
                            ]);
                        } else {
                            vertices.extend_from_slice(&[
                                mx, my,
                                b2.0, b2.1,
                                b4.0, b4.1,
                            ]);
                        }
                    }
                }
            }

            a = b;
            b = c;
        }

        let mut geometry = Geometry::new(GL_TRIANGLES);
        geometry.add_buffer(&vertices, 2);
        geometry.add_vertex_attribute(Attribute::new(0, 2, 2, 0));
        geometry
    }

    fn triangle_geometry(vertices: &[(f32, f32);3])->Geometry{
        let mut geometry = Geometry::new(GL_TRIANGLES);
        let flattened: Vec<f32> = vertices.iter()
            .flat_map(|(x, y)| [*x, *y])
            .collect();

        geometry.add_buffer(&flattened, 2);
        geometry.add_vertex_attribute(Attribute::new(0, 2, 2, 0));

        geometry
    }

    fn rectangle_geometry(width: GLfloat, height: GLfloat) -> Geometry {
        let vertices: Vec<GLfloat> = vec![
            // bottom-left
            0.0, 0.0, // bottom-right
            width, 0.0, // top-left
            0.0, height, // top-right
            width, height,
        ];

        let position_values_per_vertex = 2;
        let values_per_vertex = position_values_per_vertex;

        let mut geometry = Geometry::new(GL_TRIANGLE_STRIP);
        geometry.add_buffer(&vertices, values_per_vertex);

        geometry.add_vertex_attribute(Attribute::new(
            0,
            position_values_per_vertex,
            values_per_vertex as usize,
            0,
        ));

        geometry
    }

    fn circle_geometry(radius: GLfloat, segments: usize) -> Geometry {
        let mut vertices: Vec<GLfloat> = Vec::with_capacity((segments + 2) * 5); // center + segments + wrap-around

        // Center of the circle
        vertices.extend_from_slice(&[0.0, 0.0]);

        // Outer vertices
        for i in 0..=segments {
            let theta = (i as f32 / segments as f32) * std::f32::consts::TAU; // TAU = 2π
            let x = radius * theta.cos();
            let y = radius * theta.sin();
            vertices.extend_from_slice(&[x, y]);
        }

        let position_values_per_vertex = 2;
        let values_per_vertex = position_values_per_vertex;

        let mut geometry = Geometry::new(GL_TRIANGLE_FAN);
        geometry.add_buffer(&vertices, values_per_vertex);

        geometry.add_vertex_attribute(Attribute::new(
            0,
            position_values_per_vertex,
            values_per_vertex as usize,
            0,
        ));
        geometry
    }

    fn ellipse_geometry(rx: f32, ry: f32, segments: usize) -> Geometry {
        use std::f32::consts::PI;

        let mut vertices: Vec<GLfloat> = Vec::with_capacity((segments + 2) * 2);

        // Center point (at origin)
        vertices.extend_from_slice(&[0.0, 0.0]);

        // Perimeter points
        for i in 0..=segments {
            let angle = 2.0 * PI * (i as f32) / (segments as f32);
            let x = rx * angle.cos();
            let y = ry * angle.sin();
            vertices.extend_from_slice(&[x, y]);
        }

        let values_per_vertex = 2;
        let mut geometry = Geometry::new(GL_TRIANGLE_FAN);
        geometry.add_buffer(&vertices, values_per_vertex);

        geometry.add_vertex_attribute(Attribute::new(
            0, // position
            2,
            values_per_vertex as usize,
            0,
        ));

        geometry
    }

    pub fn rounded_rectangle_geometry(
        width: f32,
        height: f32,
        radius: f32,
        segments: usize,
    ) -> Geometry {
        assert!(radius * 2.0 <= width && radius * 2.0 <= height);

        let mut vertices: Vec<GLfloat> = Vec::new();

        // 1. Add center point for triangle fan
        let center_x = width / 2.0;
        let center_y = height / 2.0;
        vertices.push(center_x);
        vertices.push(center_y);

        // 2. Define arcs for each corner: (cx, cy, start_angle, end_angle)
        let corners = [
            (radius, radius, PI, 1.5 * PI),                      // top-left
            (width - radius, radius, 1.5 * PI, 2.0 * PI),        // top-right
            (width - radius, height - radius, 0.0, 0.5 * PI),    // bottom-right
            (radius, height - radius, 0.5 * PI, PI),             // bottom-left
        ];

        let mut first_arc_x = 0.0;
        let mut first_arc_y = 0.0;
        let mut is_first = true;

        // 3. Generate corner arcs
        for &(cx, cy, start_angle, end_angle) in &corners {
            for i in 0..=segments {
                let theta = start_angle + (end_angle - start_angle) * (i as f32) / (segments as f32);
                let x = cx + radius * theta.cos();
                let y = cy + radius * theta.sin();

                if is_first {
                    first_arc_x = x;
                    first_arc_y = y;
                    is_first = false;
                }

                vertices.push(x);
                vertices.push(y);
            }
        }

        // 4. Close the fan by repeating the first outer point
        vertices.push(first_arc_x);
        vertices.push(first_arc_y);

        // 5. Build Geometry
        let values_per_vertex = 2;
        let mut geometry = Geometry::new(GL_TRIANGLE_FAN);
        geometry.add_buffer(&vertices, values_per_vertex);

        geometry.add_vertex_attribute(Attribute::new(
            0, // location 0 → position
            2, // x and y
            values_per_vertex as usize,
            0,
        ));

        geometry
    }

    fn polygon_geometry(points: &[(GLfloat, GLfloat)]) -> Geometry {
        assert!(points.len() >= 3, "Polygon requires at least 3 points");

        let mut vertices = Vec::with_capacity(points.len() * 2);
        for &(x, y) in points {
            vertices.extend_from_slice(&[x, y]);
        }

        let values_per_vertex = 2;
        let mut geometry = Geometry::new(GL_TRIANGLE_FAN); // Or TRIANGLE_FAN if filled
        geometry.add_buffer(&vertices, values_per_vertex);
        geometry.add_vertex_attribute(Attribute::new(0, 2, values_per_vertex as usize, 0));
        geometry

    }

    pub fn image_geometry(width: f32, height: f32) -> Geometry {
        // Vertex format: [x, y, u, v]
        let vertices: Vec<f32> = vec![
            // Triangle 1
            0.0,      0.0,       0.0, 0.0, // bottom-left
            width,    0.0,       1.0, 0.0, // bottom-right
            width,    height,    1.0, 1.0, // top-right

            // Triangle 2
            0.0,      0.0,       0.0, 0.0, // bottom-left
            width,    height,    1.0, 1.0, // top-right
            0.0,      height,    0.0, 1.0, // top-left
        ];

        let values_per_vertex = 4; // x, y, u, v

        let mut geometry = Geometry::new(GL_TRIANGLES);
        geometry.add_buffer(&vertices, values_per_vertex);

        geometry.add_vertex_attribute(Attribute::new(
            0, // location 0 in shader: position
            2, // x, y
            values_per_vertex as usize,
            0,
        ));

        geometry.add_vertex_attribute(Attribute::new(
            1, // location 1 in shader: texcoord
            2, // u, v
            values_per_vertex as usize,
            2, // offset by 2 floats (x, y)
        ));

        geometry
    }


    fn svg_color(&self) -> String {
        self.mesh
            .color
            .as_ref()
            .map(|c| c.to_hex())
            .unwrap_or_else(|| "#000000".to_string())
    }
}
impl ToSvg for ShapeRenderable{
    fn to_svg(&self) -> String {
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
