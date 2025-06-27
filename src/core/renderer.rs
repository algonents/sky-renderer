use crate::core::geometry::Geometry;

pub struct Renderer {}

impl Renderer {
    pub fn draw_geometry(&self, geometry: &Geometry) {
        geometry.bind();
    }
}
