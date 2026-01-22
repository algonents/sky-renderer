mod shaperenderable;

pub use shaperenderable::ShapeRenderable;

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
pub trait Shape {
    fn kind(&self) -> ShapeKind;
}

pub struct Point;

impl Shape for Point {
    fn kind(&self) -> ShapeKind {
        ShapeKind::Point
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
        ShapeKind::MultiPoint {
            points: self.points.clone(),
        }
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
        ShapeKind::Line {
            x2: self.x2,
            y2: self.y2,
        }
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
        ShapeKind::Polyline {
            points: self.points.clone(),
        }
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
        ShapeKind::Triangle {
            vertices: self.vertices,
        }
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
        ShapeKind::Rectangle {
            width: self.width,
            height: self.height,
        }
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
        ShapeKind::RoundedRectangle {
            width: self.width,
            height: self.height,
            radius: self.radius,
        }
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
        ShapeKind::Polygon {
            points: self.points.clone(),
        }
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
        ShapeKind::Circle {
            radius: self.radius,
        }
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
        ShapeKind::Ellipse {
            radius_x: self.radius_x,
            radius_y: self.radius_y,
        }
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
        ShapeKind::Image {
            width: self.width,
            height: self.height,
        }
    }
}
