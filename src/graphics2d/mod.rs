use std::f32::consts::PI;
use crate::core::{Attribute, Geometry};
use crate::core::engine::opengl::{GL_POINTS, GL_TRIANGLE_FAN, GL_TRIANGLE_STRIP, GLfloat, GL_TRIANGLES};

pub mod shape;
pub mod shaperenderable;
pub mod svg;

const MIN_STROKE_WIDTH: f32 = 1.5;

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
