use crevice::std140::AsStd140;

use self::cgmath_types::{vec3, vec4, Vec3, Vec4};

pub mod camera;
pub mod egui_transfer;
pub mod lights;
pub mod mesh;
pub mod model;
pub mod renderer;
pub mod scene;

#[derive(Clone, Copy, Debug)]
pub struct Rgba([f32; 4]);
impl Rgba {
    pub const RED: Self = Self([1.0, 0.0, 0.0, 1.0]);
    pub const GREEN: Self = Self([0.0, 1.0, 0.0, 1.0]);
    pub const BLUE: Self = Self([0.0, 0.0, 1.0, 1.0]);

    pub const YELLOW: Self = Self([1.0, 1.0, 0.0, 1.0]);
    pub const MAGENTA: Self = Self([1.0, 0.0, 1.0, 1.0]);
    pub const CYAN: Self = Self([0.0, 1.0, 1.0, 1.0]);

    pub const BLACK: Self = Self([0.0, 0.0, 0.0, 1.0]);
    pub const WHITE: Self = Self([1.0, 1.0, 1.0, 1.0]);

    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self([r, g, b, a])
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

    pub fn from_vec(vec: Vec4) -> Self {
        Self::new(vec.x, vec.y, vec.z, vec.w)
    }

    pub fn to_vec(&self) -> Vec4 {
        vec4(self.0[0], self.0[1], self.0[2], self.0[3])
    }

    pub fn to_floats(&self) -> [f32; 4] {
        self.0
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
impl AsStd140 for Rgba {
    type Output = crevice::std140::Vec4;

    fn as_std140(&self) -> Self::Output {
        self.to_vec().as_std140()
    }

    fn from_std140(val: Self::Output) -> Self {
        Self::from_vec(Vec4::from_std140(val))
    }
}

pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Rgba {
    Rgba::new(r, g, b, a)
}

#[derive(Clone, Copy, Debug)]
pub struct Rgb([f32; 3]);
impl Rgb {
    pub const RED: Self = Self([1.0, 0.0, 0.0]);
    pub const GREEN: Self = Self([0.0, 1.0, 0.0]);
    pub const BLUE: Self = Self([0.0, 0.0, 1.0]);

    pub const YELLOW: Self = Self([1.0, 1.0, 0.0]);
    pub const MAGENTA: Self = Self([1.0, 0.0, 1.0]);
    pub const CYAN: Self = Self([0.0, 1.0, 1.0]);

    pub const BLACK: Self = Self([0.0, 0.0, 0.0]);
    pub const WHITE: Self = Self([1.0, 1.0, 1.0]);

    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self([r, g, b])
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

    pub fn from_vec(vec: Vec3) -> Self {
        Self::new(vec.x, vec.y, vec.z)
    }

    pub fn to_vec(&self) -> Vec3 {
        vec3(self.0[0], self.0[1], self.0[2])
    }

    pub fn to_floats(&self) -> [f32; 3] {
        self.0
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
}
impl AsStd140 for Rgb {
    type Output = crevice::std140::Vec3;

    fn as_std140(&self) -> Self::Output {
        self.to_vec().as_std140()
    }

    fn from_std140(val: Self::Output) -> Self {
        Self::from_vec(Vec3::from_std140(val))
    }
}

pub fn rgb(r: f32, g: f32, b: f32) -> Rgb {
    Rgb::new(r, g, b)
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
