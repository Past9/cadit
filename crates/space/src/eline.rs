use std::fmt::Debug;

pub trait ELine: Debug + Clone {}

#[derive(Debug, Clone)]
pub struct EUnimplementedLine {}
impl ELine for EUnimplementedLine {}

/// An infinite line in 2D Euclidean space
#[derive(Debug, Clone)]
pub struct ELine2 {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}
impl ELine for ELine2 {}

/// An infinite line in 3D Euclidean space
#[derive(Debug, Clone)]
pub struct ELine3 {
    // Plane 1
    pub a1: f64,
    pub b1: f64,
    pub c1: f64,
    pub d1: f64,

    // Plane 2
    pub a2: f64,
    pub b2: f64,
    pub c2: f64,
    pub d2: f64,
}
impl ELine for ELine3 {}
