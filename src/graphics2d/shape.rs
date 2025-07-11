use crate::core::Geometry;
use crate::graphics2d;

/// A trait representing a 2D shape.
pub trait Shape{
    /// Converts the shape into a [Geometry] with the specified color.
    ///
    /// # Parameters
    /// - `color`: An RGB color represented as a tuple `(f32, f32, f32)`, 
    ///   where each component ranges from 0.0 to 1.0.
    ///
    /// # Returns
    /// A [Geometry] object that contains vertex and index data for rendering.
    fn to_geometry(&self)->Geometry;
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
    fn to_geometry(&self) -> Geometry {
        graphics2d::rectangle_geometry(self.width, self.height)
    }
}






