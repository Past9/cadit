use crevice::std140::AsStd140;

use self::cgmath_types::{vec3, Vec3};

pub mod camera;
pub mod egui_transfer;
pub mod lights;
pub mod mesh;
pub mod renderer;

#[derive(Clone, Copy, Debug)]
pub struct Color([f32; 4]);
impl Color {
    pub const RED: Self = Self([1.0, 0.0, 0.0, 1.0]);
    pub const GREEN: Self = Self([0.0, 1.0, 0.0, 1.0]);
    pub const BLUE: Self = Self([0.0, 0.0, 1.0, 1.0]);

    pub const YELLOW: Self = Self([1.0, 1.0, 0.0, 1.0]);
    pub const MAGENTA: Self = Self([1.0, 0.0, 1.0, 1.0]);
    pub const CYAN: Self = Self([0.0, 1.0, 1.0, 1.0]);

    pub const BLACK: Self = Self([0.0, 0.0, 0.0, 1.0]);
    pub const WHITE: Self = Self([1.0, 1.0, 1.0, 1.0]);

    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self([r, g, b, a])
    }

    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::rgba(r, g, b, 1.0)
    }

    pub fn r(&self) -> f32 {
        self.0[0]
    }

    pub fn g(&self) -> f32 {
        self.0[1]
    }

    pub fn b(&self) -> f32 {
        self.0[2]
    }

    pub fn a(&self) -> f32 {
        self.0[3]
    }

    pub fn to_vec3(&self) -> Vec3 {
        vec3(self.0[0], self.0[1], self.0[2])
    }

    pub fn from_vec3(vec: Vec3) -> Self {
        rgb(vec.x, vec.y, vec.z)
    }

    pub fn set_r(&mut self, r: f32) {
        self.0[0] = r;
    }

    pub fn set_g(&mut self, g: f32) {
        self.0[1] = g;
    }

    pub fn set_b(&mut self, b: f32) {
        self.0[2] = b;
    }

    pub fn set_a(&mut self, a: f32) {
        self.0[3] = a;
    }
}
impl AsStd140 for Color {
    type Output = crevice::std140::Vec3;

    fn as_std140(&self) -> Self::Output {
        self.to_vec3().as_std140()
    }

    fn from_std140(val: Self::Output) -> Self {
        Self::from_vec3(Vec3::from_std140(val))
    }
}

pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
    Color::rgba(r, g, b, a)
}

pub fn rgb(r: f32, g: f32, b: f32) -> Color {
    Color::rgb(r, g, b)
}

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
