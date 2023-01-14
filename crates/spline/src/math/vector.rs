use std::fmt::Debug;
use std::iter::Sum;
use std::ops::{Add, Div, Mul, Sub};

pub use vec2::*;
pub use vec2h::*;
pub use vec3::*;
pub use vec3h::*;
pub use vec4::*;

pub trait Vector:
    Debug
    + Copy
    + Clone
    + Sized
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + Add<f64, Output = Self>
    + Sub<f64, Output = Self>
    + Mul<f64, Output = Self>
    + Div<f64, Output = Self>
    + Sum<Self>
{
    fn zero() -> Self;
    fn dot(&self, rhs: &Self) -> f64;

    fn magnitude(&self) -> f64 {
        self.dot(self).sqrt()
    }

    fn normalize(&self) -> Self {
        *self / self.magnitude()
    }
}

pub trait Homogeneous:
    Debug
    + Copy
    + Clone
    + Sized
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + Add<f64, Output = Self>
    + Sub<f64, Output = Self>
    + Mul<f64, Output = Self>
    + Div<f64, Output = Self>
    + Sum<Self>
{
    type Projected: Vector;
    type Weighted: Vector;

    fn zero() -> Self;
    fn project(self) -> Self::Projected;
    fn weight(self) -> Self::Weighted;
    fn cast_from_weighted(weighted: Self::Weighted) -> Self;
    fn cartesian_components(self) -> Self::Projected;
    fn homogeneous_component(self) -> f64;
}

mod vec2 {
    use std::iter::Sum;

    use auto_ops::{impl_op_ex, impl_op_ex_commutative};

    use super::Vector;

    #[derive(Debug, Copy, Clone)]
    pub struct Vec2 {
        pub x: f64,
        pub y: f64,
    }
    impl Vec2 {
        pub fn new(x: f64, y: f64) -> Self {
            Self { x, y }
        }

        pub fn f32s(&self) -> [f32; 2] {
            [self.x as f32, self.y as f32]
        }
    }
    impl Vector for Vec2 {
        fn zero() -> Self {
            Self { x: 0.0, y: 0.0 }
        }

        fn dot(&self, rhs: &Self) -> f64 {
            (self.x * rhs.x) + (self.y * rhs.y)
        }
    }
    impl Sum for Vec2 {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(Self::zero(), |a, b| a + b)
        }
    }

    // Vec / Vec operations
    impl_op_ex!(+ |a: &Vec2, b: &Vec2| -> Vec2 {
        Vec2 {
            x: a.x + b.x,
            y: a.y + b.y,
        }
    });

    impl_op_ex!(+= |a: &mut Vec2, b: &Vec2| {
        a.x += b.x;
        a.y += b.y;
    });

    impl_op_ex!(-|a: &Vec2, b: &Vec2| -> Vec2 {
        Vec2 {
            x: a.x - b.x,
            y: a.y - b.y,
        }
    });

    impl_op_ex!(-= |a: &mut Vec2, b: &Vec2| {
        a.x -= b.x;
        a.y -= b.y;
    });

    impl_op_ex!(*|a: &Vec2, b: &Vec2| -> Vec2 {
        Vec2 {
            x: a.x * b.x,
            y: a.y * b.y,
        }
    });

    impl_op_ex!(*= |a: &mut Vec2, b: &Vec2| {
        a.x *= b.x;
        a.y *= b.y;
    });

    impl_op_ex!(/ |a: &Vec2, b: &Vec2| -> Vec2 {
        Vec2 {
            x: a.x /  b.x,
            y: a.y /  b.y,
        }
    });

    impl_op_ex!(/= |a: &mut Vec2, b: &Vec2| {
        a.x /= b.x;
        a.y /= b.y;
    });

    // Vec / Float operations
    impl_op_ex_commutative!(+ |a: &Vec2, b: &f64| -> Vec2 {
        Vec2 {
            x: a.x + b,
            y: a.y + b,
        }
    });

    impl_op_ex!(+= |a: &mut Vec2, b: &f64| {
        a.x += b;
        a.y += b;
    });

    impl_op_ex_commutative!(-|a: &Vec2, b: &f64| -> Vec2 {
        Vec2 {
            x: a.x - b,
            y: a.y - b,
        }
    });

    impl_op_ex!(-= |a: &mut Vec2, b: &f64| {
        a.x -= b;
        a.y -= b;
    });

    impl_op_ex_commutative!(*|a: &Vec2, b: &f64| -> Vec2 {
        Vec2 {
            x: a.x * b,
            y: a.y * b,
        }
    });

    impl_op_ex!(*= |a: &mut Vec2, b: &f64| {
        a.x *= b;
        a.y *= b;
    });

    impl_op_ex!(/|a: &Vec2, b: &f64| -> Vec2 {
        Vec2 {
            x: a.x / b,
            y: a.y / b,
        }
    });

    impl_op_ex!(/= |a: &mut Vec2, b: &f64| {
        a.x /= b;
        a.y /= b;
    });
}

mod vec2h {
    use std::iter::Sum;

    use auto_ops::{impl_op_ex, impl_op_ex_commutative};

    use super::{Homogeneous, Vec2, Vec3};

    #[derive(Copy, Clone, Debug)]
    pub struct Vec2H {
        pub x: f64,
        pub y: f64,
        pub h: f64,
    }
    impl Vec2H {
        pub fn new(x: f64, y: f64, h: f64) -> Self {
            Self { x, y, h }
        }
    }

    impl Homogeneous for Vec2H {
        type Projected = Vec2;
        type Weighted = Vec3;

        fn project(self) -> Self::Projected {
            Vec2 {
                x: self.x / self.h,
                y: self.y / self.h,
            }
        }

        fn weight(self) -> Self::Weighted {
            Vec3 {
                x: self.x * self.h,
                y: self.y * self.h,
                z: self.h,
            }
        }

        fn cast_from_weighted(weighted: Self::Weighted) -> Self {
            Self {
                x: weighted.x,
                y: weighted.y,
                h: weighted.z,
            }
        }

        fn cartesian_components(self) -> Self::Projected {
            Vec2 {
                x: self.x,
                y: self.y,
            }
        }

        fn homogeneous_component(self) -> f64 {
            self.h
        }

        fn zero() -> Self {
            Self {
                x: 0.0,
                y: 0.0,
                h: 0.0,
            }
        }
    }
    impl Sum for Vec2H {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(Self::zero(), |a, b| a + b)
        }
    }

    // Vec / Vec operations
    impl_op_ex!(+ |a: &Vec2H, b: &Vec2H| -> Vec2H {
        Vec2H {
            x: a.x + b.x,
            y: a.y + b.y,
            h: a.h + b.h,
        }
    });

    impl_op_ex!(+= |a: &mut Vec2H, b: &Vec2H| {
        a.x += b.x;
        a.y += b.y;
        a.h += b.h;
    });

    impl_op_ex!(-|a: &Vec2H, b: &Vec2H| -> Vec2H {
        Vec2H {
            x: a.x - b.x,
            y: a.y - b.y,
            h: a.h - b.h,
        }
    });

    impl_op_ex!(-= |a: &mut Vec2H, b: &Vec2H| {
        a.x -= b.x;
        a.y -= b.y;
        a.h -= b.h;
    });

    impl_op_ex!(*|a: &Vec2H, b: &Vec2H| -> Vec2H {
        Vec2H {
            x: a.x * b.x,
            y: a.y * b.y,
            h: a.h * b.h,
        }
    });

    impl_op_ex!(*= |a: &mut Vec2H, b: &Vec2H| {
        a.x *= b.x;
        a.y *= b.y;
        a.h *= b.h;
    });

    impl_op_ex!(/ |a: &Vec2H, b: &Vec2H| -> Vec2H {
        Vec2H {
            x: a.x /  b.x,
            y: a.y /  b.y,
            h: a.h /  b.h,
        }
    });

    impl_op_ex!(/= |a: &mut Vec2H, b: &Vec2H| {
        a.x /= b.x;
        a.y /= b.y;
        a.h /= b.h;
    });

    // Vec / Float operations
    impl_op_ex_commutative!(+ |a: &Vec2H, b: &f64| -> Vec2H {
        Vec2H {
            x: a.x + b,
            y: a.y + b,
            h: a.h + b
        }
    });

    impl_op_ex!(+= |a: &mut Vec2H, b: &f64| {
        a.x += b;
        a.y += b;
        a.h += b;
    });

    impl_op_ex_commutative!(-|a: &Vec2H, b: &f64| -> Vec2H {
        Vec2H {
            x: a.x - b,
            y: a.y - b,
            h: a.h - b,
        }
    });

    impl_op_ex!(-= |a: &mut Vec2H, b: &f64| {
        a.x -= b;
        a.y -= b;
        a.h -= b;
    });

    impl_op_ex_commutative!(*|a: &Vec2H, b: &f64| -> Vec2H {
        Vec2H {
            x: a.x * b,
            y: a.y * b,
            h: a.h * b,
        }
    });

    impl_op_ex!(*= |a: &mut Vec2H, b: &f64| {
        a.x *= b;
        a.y *= b;
        a.h *= b;
    });

    impl_op_ex!(/|a: &Vec2H, b: &f64| -> Vec2H {
        Vec2H {
            x: a.x / b,
            y: a.y / b,
            h: a.h / b,
        }
    });

    impl_op_ex!(/= |a: &mut Vec2H, b: &f64| {
        a.x /= b;
        a.y /= b;
        a.h /= b;
    });
}

mod vec3 {
    use std::iter::Sum;

    use auto_ops::{impl_op_ex, impl_op_ex_commutative};

    use super::Vector;

    #[derive(Debug, Copy, Clone)]
    pub struct Vec3 {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }
    impl Vec3 {
        pub fn new(x: f64, y: f64, z: f64) -> Self {
            Self { x, y, z }
        }

        pub fn f32s(&self) -> [f32; 3] {
            [self.x as f32, self.y as f32, self.z as f32]
        }
    }
    impl Vector for Vec3 {
        fn zero() -> Self {
            Self {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        }

        fn dot(&self, rhs: &Self) -> f64 {
            (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
        }
    }
    impl Sum for Vec3 {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(Self::zero(), |a, b| a + b)
        }
    }

    // Vec / Vec operations
    impl_op_ex!(+ |a: &Vec3, b: &Vec3| -> Vec3 {
        Vec3 {
            x: a.x + b.x,
            y: a.y + b.y,
            z: a.z + b.z,
        }
    });

    impl_op_ex!(+= |a: &mut Vec3, b: &Vec3| {
        a.x += b.x;
        a.y += b.y;
        a.z += b.z;
    });

    impl_op_ex!(-|a: &Vec3, b: &Vec3| -> Vec3 {
        Vec3 {
            x: a.x - b.x,
            y: a.y - b.y,
            z: a.z - b.z,
        }
    });

    impl_op_ex!(-= |a: &mut Vec3, b: &Vec3| {
        a.x -= b.x;
        a.y -= b.y;
        a.z -= b.z;
    });

    impl_op_ex!(*|a: &Vec3, b: &Vec3| -> Vec3 {
        Vec3 {
            x: a.x * b.x,
            y: a.y * b.y,
            z: a.z * b.z,
        }
    });

    impl_op_ex!(*= |a: &mut Vec3, b: &Vec3| {
        a.x *= b.x;
        a.y *= b.y;
        a.z *= b.z;
    });

    impl_op_ex!(/ |a: &Vec3, b: &Vec3| -> Vec3 {
        Vec3 {
            x: a.x /  b.x,
            y: a.y /  b.y,
            z: a.z /  b.z,
        }
    });

    impl_op_ex!(/= |a: &mut Vec3, b: &Vec3| {
        a.x /= b.x;
        a.y /= b.y;
        a.z /= b.z;
    });

    // Vec / Float operations
    impl_op_ex_commutative!(+ |a: &Vec3, b: &f64| -> Vec3 {
        Vec3 {
            x: a.x + b,
            y: a.y + b,
            z: a.z + b,
        }
    });

    impl_op_ex!(+= |a: &mut Vec3, b: &f64| {
        a.x += b;
        a.y += b;
        a.z += b;
    });

    impl_op_ex_commutative!(-|a: &Vec3, b: &f64| -> Vec3 {
        Vec3 {
            x: a.x - b,
            y: a.y - b,
            z: a.z - b,
        }
    });

    impl_op_ex!(-= |a: &mut Vec3, b: &f64| {
        a.x -= b;
        a.y -= b;
        a.z -= b;
    });

    impl_op_ex_commutative!(*|a: &Vec3, b: &f64| -> Vec3 {
        Vec3 {
            x: a.x * b,
            y: a.y * b,
            z: a.z * b,
        }
    });

    impl_op_ex!(*= |a: &mut Vec3, b: &f64| {
        a.x *= b;
        a.y *= b;
        a.z *= b;
    });

    impl_op_ex!(/|a: &Vec3, b: &f64| -> Vec3 {
        Vec3 {
            x: a.x / b,
            y: a.y / b,
            z: a.z / b,
        }
    });

    impl_op_ex!(/= |a: &mut Vec3, b: &f64| {
        a.x /= b;
        a.y /= b;
        a.z /= b;
    });
}

mod vec3h {
    use std::iter::Sum;

    use auto_ops::{impl_op_ex, impl_op_ex_commutative};

    use super::{Homogeneous, Vec3, Vec4};

    #[derive(Copy, Clone, Debug)]
    pub struct Vec3H {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub h: f64,
    }
    impl Vec3H {
        pub fn new(x: f64, y: f64, z: f64, h: f64) -> Self {
            Self { x, y, z, h }
        }
    }

    impl Homogeneous for Vec3H {
        type Projected = Vec3;
        type Weighted = Vec4;

        fn project(self) -> Self::Projected {
            Vec3 {
                x: self.x / self.h,
                y: self.y / self.h,
                z: self.z / self.h,
            }
        }

        fn weight(self) -> Self::Weighted {
            Vec4 {
                x: self.x * self.h,
                y: self.y * self.h,
                z: self.z * self.h,
                w: self.h,
            }
        }

        fn cast_from_weighted(weighted: Self::Weighted) -> Self {
            Self {
                x: weighted.x,
                y: weighted.y,
                z: weighted.z,
                h: weighted.w,
            }
        }

        fn cartesian_components(self) -> Self::Projected {
            Vec3 {
                x: self.x,
                y: self.y,
                z: self.z,
            }
        }

        fn homogeneous_component(self) -> f64 {
            self.h
        }

        fn zero() -> Self {
            Self {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                h: 0.0,
            }
        }
    }
    impl Sum for Vec3H {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(Self::zero(), |a, b| a + b)
        }
    }

    // Vec / Vec operations
    impl_op_ex!(+ |a: &Vec3H, b: &Vec3H| -> Vec3H {
        Vec3H {
            x: a.x + b.x,
            y: a.y + b.y,
            z: a.z + b.z,
            h: a.h + b.h,
        }
    });

    impl_op_ex!(+= |a: &mut Vec3H, b: &Vec3H| {
        a.x += b.x;
        a.y += b.y;
        a.z += b.z;
        a.h += b.h;
    });

    impl_op_ex!(-|a: &Vec3H, b: &Vec3H| -> Vec3H {
        Vec3H {
            x: a.x - b.x,
            y: a.y - b.y,
            z: a.z - b.z,
            h: a.h - b.h,
        }
    });

    impl_op_ex!(-= |a: &mut Vec3H, b: &Vec3H| {
        a.x -= b.x;
        a.y -= b.y;
        a.z -= b.z;
        a.h -= b.h;
    });

    impl_op_ex!(*|a: &Vec3H, b: &Vec3H| -> Vec3H {
        Vec3H {
            x: a.x * b.x,
            y: a.y * b.y,
            z: a.z * b.z,
            h: a.h * b.h,
        }
    });

    impl_op_ex!(*= |a: &mut Vec3H, b: &Vec3H| {
        a.x *= b.x;
        a.y *= b.y;
        a.z *= b.z;
        a.h *= b.h;
    });

    impl_op_ex!(/ |a: &Vec3H, b: &Vec3H| -> Vec3H {
        Vec3H {
            x: a.x /  b.x,
            y: a.y /  b.y,
            z: a.z /  b.z,
            h: a.h /  b.h,
        }
    });

    impl_op_ex!(/= |a: &mut Vec3H, b: &Vec3H| {
        a.x /= b.x;
        a.y /= b.y;
        a.z /= b.z;
        a.h /= b.h;
    });

    // Vec / Float operations
    impl_op_ex_commutative!(+ |a: &Vec3H, b: &f64| -> Vec3H {
        Vec3H {
            x: a.x + b,
            y: a.y + b,
            z: a.z + b,
            h: a.h + b
        }
    });

    impl_op_ex!(+= |a: &mut Vec3H, b: &f64| {
        a.x += b;
        a.y += b;
        a.z += b;
        a.h += b;
    });

    impl_op_ex_commutative!(-|a: &Vec3H, b: &f64| -> Vec3H {
        Vec3H {
            x: a.x - b,
            y: a.y - b,
            z: a.z - b,
            h: a.h - b,
        }
    });

    impl_op_ex!(-= |a: &mut Vec3H, b: &f64| {
        a.x -= b;
        a.y -= b;
        a.z -= b;
        a.h -= b;
    });

    impl_op_ex_commutative!(*|a: &Vec3H, b: &f64| -> Vec3H {
        Vec3H {
            x: a.x * b,
            y: a.y * b,
            z: a.z * b,
            h: a.h * b,
        }
    });

    impl_op_ex!(*= |a: &mut Vec3H, b: &f64| {
        a.x *= b;
        a.y *= b;
        a.z *= b;
        a.h *= b;
    });

    impl_op_ex!(/|a: &Vec3H, b: &f64| -> Vec3H {
        Vec3H {
            x: a.x / b,
            y: a.y / b,
            z: a.z / b,
            h: a.h / b,
        }
    });

    impl_op_ex!(/= |a: &mut Vec3H, b: &f64| {
        a.x /= b;
        a.y /= b;
        a.z /= b;
        a.h /= b;
    });
}

mod vec4 {
    use std::iter::Sum;

    use auto_ops::{impl_op_ex, impl_op_ex_commutative};

    use super::Vector;

    #[derive(Debug, Copy, Clone)]
    pub struct Vec4 {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub w: f64,
    }
    impl Vector for Vec4 {
        fn zero() -> Self {
            Self {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            }
        }

        fn dot(&self, rhs: &Self) -> f64 {
            (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z) + (self.w * rhs.w)
        }
    }
    impl Sum for Vec4 {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(Self::zero(), |a, b| a + b)
        }
    }

    // Vec / Vec operations
    impl_op_ex!(+ |a: &Vec4, b: &Vec4| -> Vec4 {
        Vec4 {
            x: a.x + b.x,
            y: a.y + b.y,
            z: a.z + b.z,
            w: a.w + b.w,
        }
    });

    impl_op_ex!(+= |a: &mut Vec4, b: &Vec4| {
        a.x += b.x;
        a.y += b.y;
        a.z += b.z;
        a.w += b.w;
    });

    impl_op_ex!(-|a: &Vec4, b: &Vec4| -> Vec4 {
        Vec4 {
            x: a.x - b.x,
            y: a.y - b.y,
            z: a.z - b.z,
            w: a.w - b.w,
        }
    });

    impl_op_ex!(-= |a: &mut Vec4, b: &Vec4| {
        a.x -= b.x;
        a.y -= b.y;
        a.z -= b.z;
        a.w -= b.w;
    });

    impl_op_ex!(*|a: &Vec4, b: &Vec4| -> Vec4 {
        Vec4 {
            x: a.x * b.x,
            y: a.y * b.y,
            z: a.z * b.z,
            w: a.w * b.w,
        }
    });

    impl_op_ex!(*= |a: &mut Vec4, b: &Vec4| {
        a.x *= b.x;
        a.y *= b.y;
        a.z *= b.z;
        a.w *= b.w;
    });

    impl_op_ex!(/ |a: &Vec4, b: &Vec4| -> Vec4 {
        Vec4 {
            x: a.x /  b.x,
            y: a.y /  b.y,
            z: a.z /  b.z,
            w: a.w /  b.w,
        }
    });

    impl_op_ex!(/= |a: &mut Vec4, b: &Vec4| {
        a.x /= b.x;
        a.y /= b.y;
        a.z /= b.z;
        a.w /= b.w;
    });

    // Vec / Float operations
    impl_op_ex_commutative!(+ |a: &Vec4, b: &f64| -> Vec4 {
        Vec4 {
            x: a.x + b,
            y: a.y + b,
            z: a.z + b,
            w: a.w + b,
        }
    });

    impl_op_ex!(+= |a: &mut Vec4, b: &f64| {
        a.x += b;
        a.y += b;
        a.z += b;
        a.w += b;
    });

    impl_op_ex_commutative!(-|a: &Vec4, b: &f64| -> Vec4 {
        Vec4 {
            x: a.x - b,
            y: a.y - b,
            z: a.z - b,
            w: a.w - b,
        }
    });

    impl_op_ex!(-= |a: &mut Vec4, b: &f64| {
        a.x -= b;
        a.y -= b;
        a.z -= b;
        a.w -= b;
    });

    impl_op_ex_commutative!(*|a: &Vec4, b: &f64| -> Vec4 {
        Vec4 {
            x: a.x * b,
            y: a.y * b,
            z: a.z * b,
            w: a.w * b,
        }
    });

    impl_op_ex!(*= |a: &mut Vec4, b: &f64| {
        a.x *= b;
        a.y *= b;
        a.z *= b;
        a.w *= b;
    });

    impl_op_ex!(/|a: &Vec4, b: &f64| -> Vec4 {
        Vec4 {
            x: a.x / b,
            y: a.y / b,
            z: a.z / b,
            w: a.w / b,
        }
    });

    impl_op_ex!(/= |a: &mut Vec4, b: &f64| {
        a.x /= b;
        a.y /= b;
        a.z /= b;
        a.w /= b;
    });
}
