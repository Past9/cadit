use std::{
    fmt::Debug,
    iter::Sum,
    ops::{Add, Div, Mul, Sub},
};

use crate::{ESpace, ESpace1, ESpace2, ESpace3, ESpace4};

/// Trait for vectors in Euclidean space
pub trait EVector<S: ESpace>:
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
        self.magnitude2().sqrt()
    }

    fn magnitude2(&self) -> f64 {
        self.dot(self)
    }

    fn normalize(&self) -> Self {
        *self / self.magnitude()
    }
}

macro_rules! evector_ops {
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
        impl EVector<$space> for $typ {
            fn zero() -> Self {
                Self {
                    $(
                        $comp: 0.0,
                    )*
                }
            }

            fn dot(&self, rhs: &Self) -> f64 {
                0.0 $(
                    + (self.$comp * rhs.$comp)
                )*
            }
        }
        impl std::iter::Sum for $typ {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(EVector::zero(), |a, b| a + b)
            }
        }
    };
}

macro_rules! impl_evector {
    ( $typ:ident, $space:ident, $( $comp:ident ),* ) => {
        evector_ops!($typ, $space, $($comp),*);
        crate::vector_arithmetic!($typ, $($comp),*);
    };
}

/// A vector in 1-dimensional Euclidean space
#[derive(Debug, Copy, Clone)]
pub struct EVec1 {
    pub x: f64,
}
impl_evector!(EVec1, ESpace1, x);

/// A vector in 2-dimensional Euclidean space
#[derive(Debug, Copy, Clone)]
pub struct EVec2 {
    pub x: f64,
    pub y: f64,
}
impl_evector!(EVec2, ESpace2, x, y);

/// A vector in 3-dimensional Euclidean space
#[derive(Debug, Copy, Clone)]
pub struct EVec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl_evector!(EVec3, ESpace3, x, y, z);

/// A vector in 4-dimensional Euclidean space
#[derive(Debug, Copy, Clone)]

pub struct EVec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
impl_evector!(EVec4, ESpace4, x, y, z, w);
