use std::fmt::Debug;

use crate::{EVec3, EVector, TOL};

pub trait EPlane: Debug + Clone {}

#[derive(Debug, Clone)]
pub struct EUnimplementedPlane;
impl EPlane for EUnimplementedPlane {}

#[derive(Debug, Clone)]
pub struct EPlane3 {
    pub norm: EVec3,
    pub d: f64,
}
impl EPlane3 {
    pub fn new_from_normal_vec(normal: EVec3, d: f64) -> Self {
        Self { norm: normal, d }
    }

    pub fn new_general_form(a: f64, b: f64, c: f64, d: f64) -> Self {
        Self::new_from_normal_vec(EVec3::new(a, b, c), d)
    }

    pub fn normalize(&self) -> Self {
        let mag = self.norm.magnitude();
        Self {
            norm: self.norm / mag,
            d: self.d / mag,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.norm
            != EVec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
    }

    pub fn is_perpendicular_to(&self, other: &Self) -> bool {
        self.norm.dot(&other.norm).abs() <= TOL
    }

    pub fn dist_to_point(&self, point: &EVec3) -> f64 {
        (self.norm.x * point.x + self.norm.y * point.y + self.norm.z * point.z + self.d).abs()
            / self.norm.magnitude()
    }

    pub fn contains_point(&self, point: &EVec3) -> bool {
        (self.norm.x * point.x + self.norm.y * point.y + self.norm.z * point.z + self.d).abs()
            <= TOL
    }

    pub fn closest_to_point(&self, point: &EVec3) -> EVec3 {
        let a = self.norm.x;
        let b = self.norm.y;
        let c = self.norm.z;
        let d = self.d;

        let EVec3 { x, y, z } = point.clone();

        let numer = -a * x - b * y - c * z - d;
        let denom = a.powi(2) + b.powi(2) + c.powi(2);

        let n = numer / denom;

        let pip = point + n * self.norm;

        pip
    }
}
impl EPlane for EPlane3 {}
