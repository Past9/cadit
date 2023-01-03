use crate::render::{cgmath_types::Vec3, Color};

pub struct DirectionalLight {
    direction: Vec3,
    color: Color,
    intensity: f32,
}
