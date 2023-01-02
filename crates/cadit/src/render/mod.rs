use std::sync::Arc;

use eframe::epaint::PaintCallbackInfo;
use egui_winit_vulkano::RenderResources;
use vulkano::image::ImageViewAbstract;

use self::cgmath_types::{Quat, Vec3};

pub mod camera;
pub mod egui_transfer;
pub mod mesh;
pub mod pbr_scene;

pub mod cgmath_types {
    pub type Quat = cgmath::Quaternion<f32>;

    pub type Rad = cgmath::Rad<f32>;

    pub type Point3 = cgmath::Point3<f32>;

    pub type Vec2 = cgmath::Vector2<f32>;
    pub type Vec3 = cgmath::Vector3<f32>;
    pub type Vec4 = cgmath::Vector4<f32>;

    pub type Mat3 = cgmath::Matrix3<f32>;
    pub type Mat4 = cgmath::Matrix4<f32>;

    pub fn point3(x: f32, y: f32, z: f32) -> Point3 {
        Point3::new(x, y, z)
    }

    pub fn vec2(x: f32, y: f32) -> Vec2 {
        Vec2::new(x, y)
    }

    pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3::new(x, y, z)
    }

    pub fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        Vec4::new(x, y, z, w)
    }
}

pub trait Scene {
    fn render<'a>(&mut self, info: &PaintCallbackInfo, resources: &RenderResources<'a>);
    fn view(&self) -> Arc<dyn ImageViewAbstract>;
    fn set_rotation(&mut self, rotation: Quat);
    fn set_position(&mut self, position: Vec3);
}
