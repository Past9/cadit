use crate::math::vector_macros::impl_vector;
use std::fmt::Debug;
use std::iter::Sum;
use std::ops::{Add, Div, Mul, Sub};

use super::vector_macros::vector_arithmetic;

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

/// 2-dimensional vector
#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}
impl_vector!(Vec2, x, y);

/// 3-dimensional vector
#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl_vector!(Vec3, x, y, z);

/// 4-dimensional vector
#[derive(Debug, Copy, Clone)]
pub struct Vec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
impl_vector!(Vec4, x, y, z, w);

/// Homogeneous 2-dimensional vector
#[derive(Copy, Clone, Debug)]
pub struct Vec2H {
    pub x: f64,
    pub y: f64,
    pub h: f64,
}
vector_arithmetic!(Vec2H, x, y, h);

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

/// Homogeneous 3-dimensional vector
#[derive(Copy, Clone, Debug)]
pub struct Vec3H {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub h: f64,
}
vector_arithmetic!(Vec3H, x, y, z, h);

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
