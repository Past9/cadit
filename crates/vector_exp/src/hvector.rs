use crate::{HSpace1, HSpace2, HSpace3, HomogeneousSpace};

pub trait HVector<S: HomogeneousSpace> {}

pub struct HVec1 {
    pub x: f64,
    pub h: f64,
}
impl HVector<HSpace1> for HVec1 {}

pub struct HVec2 {
    pub x: f64,
    pub y: f64,
    pub h: f64,
}
impl HVector<HSpace2> for HVec2 {}

pub struct HVec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub h: f64,
}
impl HVector<HSpace3> for HVec3 {}
