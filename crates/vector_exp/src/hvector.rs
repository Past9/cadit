use crate::{HSpace1, HSpace2, HSpace3, HomogeneousSpace};

/// Trait for vectors in homogeneous space
pub trait HVector<S: HomogeneousSpace> {}

/// A vector in 1-dimensional homogeneous space
pub struct HVec1 {
    pub x: f64,
    pub h: f64,
}
impl HVector<HSpace1> for HVec1 {}

/// A vector in 2-dimensional homogeneous space
pub struct HVec2 {
    pub x: f64,
    pub y: f64,
    pub h: f64,
}
impl HVector<HSpace2> for HVec2 {}

/// A vector in 3-dimensional homogeneous space
pub struct HVec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub h: f64,
}
impl HVector<HSpace3> for HVec3 {}
