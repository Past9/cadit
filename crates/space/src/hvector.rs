use crate::{EVec1, EVec2, EVec3, EVec4, EVector, HSpace, HSpace1, HSpace2, HSpace3};
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
    type Space: HSpace;
    type Projected: EVector<Space = <<Self as HVector>::Space as HSpace>::Projected>;
    type Weighted: EVector<Space = <<Self as HVector>::Space as HSpace>::Weighted>;

    fn zero() -> Self;
    fn weight(&self) -> Self::Weighted;
    fn project(&self) -> Self::Projected;
    fn cast_from_weighted(weighted: Self::Weighted) -> Self;
    fn euclidean_components(&self) -> Self::Projected;
    fn homogeneous_component(&self) -> f64;
    fn unweight(weighted: Self::Weighted) -> Self;
    fn split_dimensions(&self) -> Vec<HVec1>;
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
    type Space = HSpace1;
    type Projected = EVec1;
    type Weighted = EVec2;

    fn zero() -> Self {
        Self { x: 0.0, h: 0.0 }
    }

    fn weight(&self) -> Self::Weighted {
        Self::Weighted {
            x: self.x * self.h,
            y: self.h,
        }
    }

    fn project(&self) -> Self::Projected {
        Self::Projected { x: self.x / self.h }
    }

    fn cast_from_weighted(weighted: Self::Weighted) -> Self {
        Self {
            x: weighted.x,
            h: weighted.y,
        }
    }

    fn euclidean_components(&self) -> Self::Projected {
        Self::Projected { x: self.x }
    }

    fn homogeneous_component(&self) -> f64 {
        self.h
    }

    fn unweight(weighted: Self::Weighted) -> Self {
        Self {
            x: weighted.x / weighted.y,
            h: weighted.y,
        }
    }

    fn split_dimensions(&self) -> Vec<HVec1> {
        vec![HVec1 {
            x: self.x,
            h: self.h,
        }]
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
    type Space = HSpace2;
    type Projected = EVec2;
    type Weighted = EVec3;

    fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            h: 0.0,
        }
    }

    fn weight(&self) -> Self::Weighted {
        Self::Weighted {
            x: self.x * self.h,
            y: self.y * self.h,
            z: self.h,
        }
    }

    fn project(&self) -> Self::Projected {
        Self::Projected {
            x: self.x / self.h,
            y: self.y / self.h,
        }
    }

    fn cast_from_weighted(weighted: Self::Weighted) -> Self {
        Self {
            x: weighted.x,
            y: weighted.y,
            h: weighted.z,
        }
    }

    fn euclidean_components(&self) -> Self::Projected {
        Self::Projected {
            x: self.x,
            y: self.y,
        }
    }

    fn homogeneous_component(&self) -> f64 {
        self.h
    }

    fn unweight(weighted: Self::Weighted) -> Self {
        Self {
            x: weighted.x / weighted.z,
            y: weighted.y / weighted.z,
            h: weighted.z,
        }
    }

    fn split_dimensions(&self) -> Vec<HVec1> {
        vec![
            HVec1 {
                x: self.x,
                h: self.h,
            },
            HVec1 {
                x: self.y,
                h: self.h,
            },
        ]
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
    type Space = HSpace3;
    type Projected = EVec3;
    type Weighted = EVec4;

    fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            h: 0.0,
        }
    }

    fn weight(&self) -> Self::Weighted {
        Self::Weighted {
            x: self.x * self.h,
            y: self.y * self.h,
            z: self.z * self.h,
            w: self.h,
        }
    }

    fn project(&self) -> Self::Projected {
        Self::Projected {
            x: self.x / self.h,
            y: self.y / self.h,
            z: self.z / self.h,
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

    fn euclidean_components(&self) -> Self::Projected {
        Self::Projected {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    fn homogeneous_component(&self) -> f64 {
        self.h
    }

    fn unweight(weighted: Self::Weighted) -> Self {
        Self {
            x: weighted.x / weighted.w,
            y: weighted.y / weighted.w,
            z: weighted.z / weighted.w,
            h: weighted.w,
        }
    }

    fn split_dimensions(&self) -> Vec<HVec1> {
        vec![
            HVec1 {
                x: self.x,
                h: self.h,
            },
            HVec1 {
                x: self.y,
                h: self.h,
            },
            HVec1 {
                x: self.z,
                h: self.h,
            },
        ]
    }
}
impl_hvector!(HVec3, x, y, z, h);
