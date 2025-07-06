
use crate::core::{Attribute, Geometry};
use crate::engine::opengl::{GLfloat, GL_LINES, GL_TRIANGLE_FAN, GL_TRIANGLE_STRIP};

pub mod shape;
pub mod renderable;


fn line_geometry(
    x1: GLfloat,
    y1: GLfloat,
    x2: GLfloat,
    y2: GLfloat,
    color:(f32, f32, f32)
) -> Geometry {
    let vertices: Vec<GLfloat> = vec![
        x1, y1, color.0, color.1, color.2,  // start point
        x2, y2, color.0, color.1, color.2  // end point
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
    color:(f32, f32, f32)
) -> Geometry {
    let vertices: Vec<GLfloat> = vec![
        // bottom-left
        0.0,     0.0,      color.0, color.1, color.2,
        // bottom-right
        width,   0.0,      color.0, color.1, color.2,
        // top-left
        0.0,     height,   color.0, color.1, color.2,
        // top-right
        width,   height,   color.0, color.1, color.2,
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
    color:(f32, f32, f32)
) -> Geometry {
    let mut vertices: Vec<GLfloat> = Vec::with_capacity((segments + 2) * 5); // center + segments + wrap-around

    // Center of the circle
    vertices.extend_from_slice(&[0.0, 0.0, color.0, color.1, color.2]);

    // Outer vertices
    for i in 0..=segments {
        let theta = (i as f32 / segments as f32) * std::f32::consts::TAU; // TAU = 2π
        let x = radius * theta.cos();
        let y = radius * theta.sin();
        vertices.extend_from_slice(&[x, y, color.0, color.1, color.2]);
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



