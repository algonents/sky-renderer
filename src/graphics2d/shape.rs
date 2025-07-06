use crate::core::Geometry;
use crate::graphics2d;

pub trait Shape{
    fn to_geometry(&self, color: (f32, f32, f32))->Geometry;
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

impl Shape for Rectangle{
    fn to_geometry(&self, color:(f32, f32, f32)) -> Geometry {
        graphics2d::rectangle_geometry(self.width, self.height, color)
    }
}






