mod shaperenderable;

use crate::core::{Geometry, GeometryProvider};
use crate::graphics2d;

pub use shaperenderable::*;

#[derive(Debug, Clone)]
pub enum ShapeKind {
    Point,
    MultiPoint { points: Vec<(f32, f32)> },
    Line { x2: f32, y2: f32 },
    Polyline { points: Vec<(f32, f32)> },
    Triangle {vertices:[(f32, f32); 3]},
    Rectangle { width: f32, height: f32 },
    RoundedRectangle { width: f32, height: f32, radius: f32 },
    Polygon { points: Vec<(f32, f32)> },
    Circle { radius: f32 },
    Ellipse { radius_x: f32, radius_y: f32 },
    Image {width:f32, height: f32}
}
/// A trait representing a 2D shape.
pub trait Shape{
    fn kind(&self)-> ShapeKind;
}

pub struct Rectangle{
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height}
    }
}

impl Shape for Rectangle {
    fn kind(&self)-> ShapeKind {
        ShapeKind::Rectangle {
            width: self.width,
            height: self.height,
        }
    }
}

impl GeometryProvider for Rectangle {
    fn to_geometry(&self) -> Geometry {
        graphics2d::rectangle_geometry(self.width, self.height)
    }
}