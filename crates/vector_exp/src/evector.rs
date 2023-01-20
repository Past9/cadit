use crate::{ESpace, ESpace1, ESpace2, ESpace3, ESpace4};

pub trait EVector<S: ESpace> {}

pub struct EVec1 {
    pub x: f64,
}
impl EVector<ESpace1> for EVec1 {}

pub struct EVec2 {
    pub x: f64,
    pub y: f64,
}
impl EVector<ESpace2> for EVec2 {}

pub struct EVec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl EVector<ESpace3> for EVec3 {}

pub struct EVec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
impl EVector<ESpace4> for EVec4 {}
