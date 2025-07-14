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
    assert!(points.len() >= 2, "Polyline requires at least two points");

    let stroke_width = stroke_width.max(MIN_STROKE_WIDTH);
    let half_width = stroke_width / 2.0;
    let mut vertices = Vec::with_capacity(points.len() * 6 * 2); // 6 vertices per segment, 2 floats each

    for segment in points.windows(2) {
        let (x1, y1) = segment[0];
        let (x2, y2) = segment[1];

        let dx = x2 - x1;
        let dy = y2 - y1;
        let length = (dx * dx + dy * dy).sqrt();
        if length == 0.0 {
            continue;
        }

        // Perpendicular vector
        let nx = -dy / length;
        let ny = dx / length;

        let ox = nx * half_width;
        let oy = ny * half_width;

        // Build quad
        let v0 = [x1 - ox, y1 - oy];
        let v1 = [x2 - ox, y2 - oy];
        let v2 = [x2 + ox, y2 + oy];
        let v3 = [x1 + ox, y1 + oy];

        // Two triangles: v0 v1 v2 and v2 v3 v0
        vertices.extend_from_slice(&[
            v0[0], v0[1],
            v1[0], v1[1],
            v2[0], v2[1],

            v2[0], v2[1],
            v3[0], v3[1],
            v0[0], v0[1],
        ]);
    }

    let values_per_vertex = 2;
    let mut geometry = Geometry::new(GL_TRIANGLES);
    geometry.add_buffer(&vertices, values_per_vertex);
    geometry.add_vertex_attribute(Attribute::new(0, 2, values_per_vertex as usize, 0));
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
