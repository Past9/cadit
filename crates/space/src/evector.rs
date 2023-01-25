use std::{
    fmt::Debug,
    iter::Sum,
    ops::{Add, Div, Mul, Neg, Sub},
};

/// Trait for vectors in Euclidean space
pub trait EVector:
    Debug
    + PartialEq
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
    fn dot(&self, rhs: &Self) -> f64;
    fn cross(&self, rhs: &Self) -> Self;

    fn magnitude(&self) -> f64 {
        self.magnitude2().sqrt()
    }

    fn magnitude2(&self) -> f64 {
        self.dot(self)
    }

    fn normalize(&self) -> Self {
        *self / self.magnitude()
    }

    fn signum_product(&self) -> f64;

    fn max_component(&self) -> f64;

    fn f32s(&self) -> [f32; 3];
}

macro_rules! evector_ops {
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
        }
        impl std::iter::Sum for $typ {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(EVector::zero(), |a, b| a + b)
            }
        }
    };
}

macro_rules! impl_evector_ops {
    ( $typ:ident, $( $comp:ident ),* ) => {
        evector_ops!($typ, $($comp),*);
        crate::vector_arithmetic!($typ, $($comp),*);
    };
}

/// A vector in 1-dimensional Euclidean space
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct EVec1 {
    pub x: f64,
}
impl EVector for EVec1 {
    fn zero() -> Self {
        Self { x: 0.0 }
    }

    fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x
    }

    fn cross(&self, rhs: &Self) -> Self {
        todo!()
    }

    fn signum_product(&self) -> f64 {
        self.x.signum()
    }

    fn max_component(&self) -> f64 {
        self.x
    }

    fn f32s(&self) -> [f32; 3] {
        [self.x as f32, 0.0, 0.0]
    }
}
impl_evector_ops!(EVec1, x);

/// A vector in 2-dimensional Euclidean space
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct EVec2 {
    pub x: f64,
    pub y: f64,
}
impl EVector for EVec2 {
    fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y
    }

    fn cross(&self, rhs: &Self) -> Self {
        todo!()
    }

    fn signum_product(&self) -> f64 {
        self.x.signum() * self.y.signum()
    }

    fn max_component(&self) -> f64 {
        let mut max: f64 = 0.0;
        if self.x.abs() > max.abs() {
            max = self.x;
        }
        if self.y.abs() > max.abs() {
            max = self.y;
        }
        max
    }

    fn f32s(&self) -> [f32; 3] {
        [self.x as f32, self.y as f32, 0.0]
    }
}
impl_evector_ops!(EVec2, x, y);

/// A vector in 3-dimensional Euclidean space
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct EVec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl EVector for EVec3 {
    fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    fn cross(&self, rhs: &Self) -> Self {
        Self::new(
            (self.y * rhs.z) - (self.z * rhs.y),
            (self.z * rhs.x) - (self.x * rhs.z),
            (self.x * rhs.y) - (self.y * rhs.x),
        )
    }

    fn signum_product(&self) -> f64 {
        self.x.signum() * self.y.signum() * self.z.signum()
    }

    fn max_component(&self) -> f64 {
        let mut max: f64 = 0.0;
        if self.x.abs() > max.abs() {
            max = self.x;
        }
        if self.y.abs() > max.abs() {
            max = self.y;
        }
        if self.z.abs() > max.abs() {
            max = self.z;
        }
        max
    }

    fn f32s(&self) -> [f32; 3] {
        [self.x as f32, self.y as f32, self.z as f32]
    }
}
impl_evector_ops!(EVec3, x, y, z);

/// A vector in 4-dimensional Euclidean space
#[derive(Debug, Copy, Clone, PartialEq)]

pub struct EVec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
impl EVector for EVec4 {
    fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    fn cross(&self, rhs: &Self) -> Self {
        todo!()
    }

    fn signum_product(&self) -> f64 {
        self.x.signum() * self.y.signum() * self.z.signum() * self.w.signum()
    }

    fn max_component(&self) -> f64 {
        let mut max: f64 = 0.0;
        if self.x.abs() > max.abs() {
            max = self.x;
        }
        if self.y.abs() > max.abs() {
            max = self.y;
        }
        if self.z.abs() > max.abs() {
            max = self.z;
        }
        if self.w.abs() > max.abs() {
            max = self.w;
        }
        max
    }

    fn f32s(&self) -> [f32; 3] {
        [self.x as f32, self.y as f32, self.z as f32]
    }
}
impl_evector_ops!(EVec4, x, y, z, w);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct EUnimplementedVector {}
impl EVector for EUnimplementedVector {
    fn zero() -> Self {
        unimplemented!()
    }

    fn dot(&self, _rhs: &Self) -> f64 {
        unimplemented!()
    }

    fn cross(&self, rhs: &Self) -> Self {
        unimplemented!()
    }

    fn magnitude(&self) -> f64 {
        unimplemented!()
    }

    fn magnitude2(&self) -> f64 {
        unimplemented!()
    }

    fn normalize(&self) -> Self {
        unimplemented!()
    }

    fn signum_product(&self) -> f64 {
        unimplemented!()
    }

    fn max_component(&self) -> f64 {
        unimplemented!()
    }

    fn f32s(&self) -> [f32; 3] {
        unimplemented!()
    }
}
impl Add for EUnimplementedVector {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self::Output {
        unimplemented!()
    }
}
impl Sub for EUnimplementedVector {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self::Output {
        unimplemented!()
    }
}
impl Mul for EUnimplementedVector {
    type Output = Self;

    fn mul(self, _rhs: Self) -> Self::Output {
        unimplemented!()
    }
}
impl Div for EUnimplementedVector {
    type Output = Self;

    fn div(self, _rhs: Self) -> Self::Output {
        unimplemented!()
    }
}
impl Add<f64> for EUnimplementedVector {
    type Output = Self;

    fn add(self, _rhs: f64) -> Self::Output {
        unimplemented!()
    }
}
impl Sub<f64> for EUnimplementedVector {
    type Output = Self;

    fn sub(self, _rhs: f64) -> Self::Output {
        unimplemented!()
    }
}
impl Mul<f64> for EUnimplementedVector {
    type Output = Self;

    fn mul(self, _rhs: f64) -> Self::Output {
        unimplemented!()
    }
}
impl Div<f64> for EUnimplementedVector {
    type Output = Self;

    fn div(self, _rhs: f64) -> Self::Output {
        unimplemented!()
    }
}
impl Neg for EUnimplementedVector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        unimplemented!()
    }
}
impl Sum for EUnimplementedVector {
    fn sum<I: Iterator<Item = Self>>(_iter: I) -> Self {
        unimplemented!()
    }
}
