use crate::{ESpace, ESpace1, ESpace2, ESpace3, ESpace4};

/// Trait for vectors in Euclidean space
pub trait EVector<S: ESpace> {}

/// A vector in 1-dimensional Euclidean space
pub struct EVec1 {
    pub x: f64,
}
impl EVector<ESpace1> for EVec1 {}

/// A vector in 2-dimensional Euclidean space
pub struct EVec2 {
    pub x: f64,
    pub y: f64,
}
impl EVector<ESpace2> for EVec2 {}

/// A vector in 3-dimensional Euclidean space
pub struct EVec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl EVector<ESpace3> for EVec3 {}

/// A vector in 4-dimensional Euclidean space
pub struct EVec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
impl EVector<ESpace4> for EVec4 {}
