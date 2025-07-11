use crate::core::{Geometry, GeometryProvider};
use crate::graphics2d;

/// A trait representing a 2D shape.
pub trait Shape{
   
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
    
}

impl GeometryProvider for Rectangle{
    fn to_geometry(&self) -> Geometry {
        graphics2d::rectangle_geometry(self.width, self.height)
    }
}






