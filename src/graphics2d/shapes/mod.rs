mod shaperenderable;

use std::any::Any;
pub use shaperenderable::ShapeRenderable;
pub use shaperenderable::ShapeStyle;

#[derive(Debug, Clone)]
pub enum ShapeKind {
    Point,
    MultiPoint,
    Line,
    Polyline,
    Triangle,
    Rectangle,
    RoundedRectangle,
    Polygon,
    Circle,
    Ellipse,
    Arc,
    Image,
}
/// A trait representing a 2D shape.
pub trait Shape {
    fn kind(&self) -> ShapeKind;
    fn as_any(&self) -> &dyn Any;
}

pub fn cast_shape<T: Shape + 'static>(shape: &dyn Shape) -> &T {
    shape
        .as_any()
        .downcast_ref::<T>()
        .expect("Invalid shape type")
}

pub struct Point;
impl Point{
    pub fn new() -> Self{
        Self{}
    }
}

impl Shape for Point {
    fn kind(&self) -> ShapeKind {
        ShapeKind::Point
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct MultiPoint {
    pub points: Vec<(f32, f32)>,
}

impl MultiPoint {
    pub fn new(points: Vec<(f32, f32)>) -> Self {
        Self { points }
    }
}

impl Shape for MultiPoint {
    fn kind(&self) -> ShapeKind {
        ShapeKind::MultiPoint
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Line {
    pub x2: f32,
    pub y2: f32,
}

impl Line {
    pub fn new(x2: f32, y2: f32) -> Self {
        Self { x2, y2 }
    }
}

impl Shape for Line {
    fn kind(&self) -> ShapeKind {
        ShapeKind::Line
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Polyline {
    pub points: Vec<(f32, f32)>,
}

impl Polyline {
    pub fn new(points: Vec<(f32, f32)>) -> Self {
        Self { points }
    }
}

impl Shape for Polyline {
    fn kind(&self) -> ShapeKind {
        ShapeKind::Polyline
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Triangle {
    pub vertices: [(f32, f32); 3],
}

impl Triangle {
    pub fn new(vertices: [(f32, f32); 3]) -> Self {
        Self { vertices }
    }
}

impl Shape for Triangle {
    fn kind(&self) -> ShapeKind {
        ShapeKind::Triangle
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Rectangle {
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

impl Shape for Rectangle {
    fn kind(&self) -> ShapeKind {
        ShapeKind::Rectangle
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct RoundedRectangle {
    pub width: f32,
    pub height: f32,
    pub radius: f32,
}

impl RoundedRectangle {
    pub fn new(width: f32, height: f32, radius: f32) -> Self {
        Self { width, height, radius }
    }
}

impl Shape for RoundedRectangle {
    fn kind(&self) -> ShapeKind {
        ShapeKind::RoundedRectangle
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Polygon {
    pub points: Vec<(f32, f32)>,
}

impl Polygon {
    pub fn new(points: Vec<(f32, f32)>) -> Self {
        Self { points }
    }
}

impl Shape for Polygon {
    fn kind(&self) -> ShapeKind {
        ShapeKind::Polygon
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Circle {
    pub radius: f32,
}

impl Circle {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

impl Shape for Circle {
    fn kind(&self) -> ShapeKind {
        ShapeKind::Circle
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Ellipse {
    pub radius_x: f32,
    pub radius_y: f32,
}

impl Ellipse {
    pub fn new(radius_x: f32, radius_y: f32) -> Self {
        Self { radius_x, radius_y }
    }
}

impl Shape for Ellipse {
    fn kind(&self) -> ShapeKind {
        ShapeKind::Ellipse
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Image {
    pub width: f32,
    pub height: f32,
}

impl Image {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

impl Shape for Image {
    fn kind(&self) -> ShapeKind {
        ShapeKind::Image
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Arc{
    pub radius: f32,
    pub start_angle: f32,
    pub end_angle: f32,
}

impl Arc {
    pub fn new(radius: f32, start_angle: f32, end_angle: f32) -> Self {
        Self { radius, start_angle, end_angle }
    }
}

impl Shape for Arc {
    fn kind(&self) -> ShapeKind {
        ShapeKind::Arc
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
