use crate::{EVec1, EVec2, EVec3, EVec4, EVector};
use std::{
    fmt::Debug,
    iter::Sum,
    ops::{Add, Div, Mul, Neg, Sub},
};

/// Trait for vectors in homogeneous space
pub trait HVector:
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
    + Neg
    + Sum<Self>
{
    fn zero() -> Self;
    fn homogeneous_component(&self) -> f64;
}

macro_rules! hvector_ops {
    ( $typ:ident, $( $comp:ident ),* ) => {
        impl $typ {
            pub fn new(
                $(
                    $comp: f64,
                )*
            ) -> Self {
                Self {
                    $(
                        $comp,
                    )*
                }
            }

            pub fn f32s(&self) -> [f32; crate::count_args!($($comp)*)] {
                [
                    $(
                        self.$comp as f32,
                    )*
                ]
            }
        }
        impl std::iter::Sum for $typ {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(HVector::zero(), |a, b| a + b)
            }
        }
    };
}

macro_rules! impl_hvector {
    ( $typ:ident, $( $comp:ident ),* ) => {
        hvector_ops!($typ, $($comp),*);
        crate::vector_arithmetic!($typ, $($comp),*);
    };
}

/// A vector in 1-dimensional homogeneous space
#[derive(Debug, Copy, Clone)]
pub struct HVec1 {
    pub x: f64,
    pub h: f64,
}
impl HVector for HVec1 {
    fn zero() -> Self {
        Self { x: 0.0, h: 0.0 }
    }

    fn homogeneous_component(&self) -> f64 {
        self.h
    }
}
impl_hvector!(HVec1, x, h);

/// A vector in 2-dimensional homogeneous space
#[derive(Debug, Copy, Clone)]
pub struct HVec2 {
    pub x: f64,
    pub y: f64,
    pub h: f64,
}
impl HVector for HVec2 {
    fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            h: 0.0,
        }
    }

    fn homogeneous_component(&self) -> f64 {
        self.h
    }
}
impl_hvector!(HVec2, x, y, h);

/// A vector in 3-dimensional homogeneous space
#[derive(Debug, Copy, Clone)]
pub struct HVec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub h: f64,
}
impl HVector for HVec3 {
    fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            h: 0.0,
        }
    }

    fn homogeneous_component(&self) -> f64 {
        self.h
    }
}
impl_hvector!(HVec3, x, y, z, h);

#[derive(Debug, Copy, Clone)]
pub struct HUnimplementedVector {}
impl HVector for HUnimplementedVector {
    fn zero() -> Self {
        unimplemented!()
    }

    fn homogeneous_component(&self) -> f64 {
        unimplemented!()
    }
}
impl Add for HUnimplementedVector {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self::Output {
        unimplemented!()
    }
}
impl Sub for HUnimplementedVector {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self::Output {
        unimplemented!()
    }
}
impl Mul for HUnimplementedVector {
    type Output = Self;

    fn mul(self, _rhs: Self) -> Self::Output {
        unimplemented!()
    }
}
impl Div for HUnimplementedVector {
    type Output = Self;

    fn div(self, _rhs: Self) -> Self::Output {
        unimplemented!()
    }
}
impl Add<f64> for HUnimplementedVector {
    type Output = Self;

    fn add(self, _rhs: f64) -> Self::Output {
        unimplemented!()
    }
}
impl Sub<f64> for HUnimplementedVector {
    type Output = Self;

    fn sub(self, _rhs: f64) -> Self::Output {
        unimplemented!()
    }
}
impl Mul<f64> for HUnimplementedVector {
    type Output = Self;

    fn mul(self, _rhs: f64) -> Self::Output {
        unimplemented!()
    }
}
impl Div<f64> for HUnimplementedVector {
    type Output = Self;

    fn div(self, _rhs: f64) -> Self::Output {
        unimplemented!()
    }
}
impl Neg for HUnimplementedVector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        unimplemented!()
    }
}
impl Sum for HUnimplementedVector {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        unimplemented!()
    }
}
