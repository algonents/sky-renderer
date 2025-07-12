
use crate::core::{Attribute, Geometry};
use crate::engine::opengl::{GLfloat, GL_LINES, GL_LINE_STRIP, GL_POINTS, GL_TRIANGLE_FAN, GL_TRIANGLE_STRIP};

pub mod shape;
pub mod shaperenderable;


fn point_geometry()-> Geometry{
    let vertex = vec![0.0, 0.0];
    let mut geometry = Geometry::new(GL_POINTS);
    geometry.add_buffer(&vertex, 2 );

    geometry.add_vertex_attribute(Attribute::new(
        0,
        2,
        2,
        0,
    ));

    geometry
}

pub fn multi_point_geometry(points: &[(GLfloat, GLfloat)]) -> Geometry {
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

fn line_geometry(
    x1: GLfloat,
    y1: GLfloat,
    x2: GLfloat,
    y2: GLfloat,
) -> Geometry {
    let vertices: Vec<GLfloat> = vec![
        x1, y1,   // start point
        x2, y2,   // end point
    ];

    let position_values_per_vertex = 2;

    let mut geometry = Geometry::new(GL_LINES);
    geometry.add_buffer(&vertices, position_values_per_vertex);

    geometry.add_vertex_attribute(Attribute::new(
        0,
        position_values_per_vertex,
        position_values_per_vertex as usize,
        0,
    ));

    geometry
}

pub fn polyline_geometry(points: &[(GLfloat, GLfloat)]) -> Geometry {
    assert!(points.len() >= 2, "Polyline requires at least two points");

    // Use the first point as anchor/origin
    let (x0, y0) = points[0];

    let mut vertices: Vec<GLfloat> = Vec::with_capacity(points.len() * 2);

    for &(x, y) in points.iter() {
        let dx = x - x0;
        let dy = y - y0;
        vertices.extend_from_slice(&[dx, dy]);
    }

    let position_values_per_vertex = 2;

    let mut geometry = Geometry::new(GL_LINE_STRIP); // use LINE_STRIP to connect segments
    geometry.add_buffer(&vertices, position_values_per_vertex);

    geometry.add_vertex_attribute(Attribute::new(
        0,
        position_values_per_vertex,
        position_values_per_vertex as usize,
        0,
    ));

    geometry
}


fn rectangle_geometry(
    width: GLfloat,
    height: GLfloat,
) -> Geometry {
    let vertices: Vec<GLfloat> = vec![
        // bottom-left
        0.0,     0.0,
        // bottom-right
        width,   0.0,
        // top-left
        0.0,     height,
        // top-right
        width,   height,
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

fn circle_geometry(
    radius: GLfloat,
    segments: usize,

) -> Geometry {
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



