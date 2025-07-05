use glam::{Mat4, Vec3};
use crate::core::{Mesh, Renderer};
use crate::graphics2d;


const SCALE_FACTOR: f32 = 1.0;

pub struct Drawable {
    mesh: Mesh,
    x: f32,
    y: f32,
}

impl Drawable {
    pub fn new(mesh: Mesh, x: f32, y: f32) -> Self {
        Self { mesh, x, y }
    }
    pub fn draw(&mut self, renderer: &Renderer, viewport_width: f32, viewport_height: f32) {
        let transform = graphics2d::ortho_2d(viewport_width, viewport_height)
            * Mat4::from_translation(Vec3::new(self.x, self.y, 0.0))
            * Mat4::from_scale(Vec3::splat(SCALE_FACTOR));
        self.mesh.set_transform(transform);
        renderer.draw_mesh(&self.mesh);
    }
}