use crate::{HSpace1, HSpace2, HSpace3, HomogeneousSpace};
use std::{
    fmt::Debug,
    iter::Sum,
    ops::{Add, Div, Mul, Sub},
};

/// Trait for vectors in homogeneous space
pub trait HVector<S: HomogeneousSpace>:
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
}

macro_rules! hvector_ops {
    ( $typ:ident, $space:ident, $( $comp:ident ),* ) => {
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
        impl HVector<$space> for $typ {
            fn zero() -> Self {
                Self {
                    $(
                        $comp: 0.0,
                    )*
                }
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
    ( $typ:ident, $space:ident, $( $comp:ident ),* ) => {
        hvector_ops!($typ, $space, $($comp),*);
        crate::vector_arithmetic!($typ, $($comp),*);
    };
}

/// A vector in 1-dimensional homogeneous space
#[derive(Debug, Copy, Clone)]
pub struct HVec1 {
    pub x: f64,
    pub h: f64,
}
impl_hvector!(HVec1, HSpace1, x, h);

/// A vector in 2-dimensional homogeneous space
#[derive(Debug, Copy, Clone)]
pub struct HVec2 {
    pub x: f64,
    pub y: f64,
    pub h: f64,
}
impl_hvector!(HVec2, HSpace2, x, y, h);

/// A vector in 3-dimensional homogeneous space
#[derive(Debug, Copy, Clone)]
pub struct HVec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub h: f64,
}
impl_hvector!(HVec3, HSpace3, x, y, z, h);
