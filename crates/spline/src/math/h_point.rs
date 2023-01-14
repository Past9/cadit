use std::ops::{Add, Div, Mul, Sub};

use super::{Point, WPoint};

#[derive(Debug, Clone, Copy)]
pub struct HPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub h: f64,
}
impl HPoint {
    pub fn new(x: f64, y: f64, z: f64, h: f64) -> Self {
        Self { x, y, z, h }
    }

    pub fn to_wpoint(&self) -> WPoint {
        WPoint {
            x: self.x,
            y: self.y,
            z: self.z,
            h: self.h,
        }
    }

    pub fn weight(&self) -> WPoint {
        WPoint {
            x: self.x * self.h,
            y: self.y * self.h,
            z: self.z * self.h,
            h: self.h,
        }
    }

    pub fn project(&self) -> Point {
        Point {
            x: self.x / self.h,
            y: self.y / self.h,
            z: self.z / self.h,
        }
    }

    pub fn cartesian(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl Mul<f64> for HPoint {
    type Output = HPoint;

    fn mul(self, rhs: f64) -> Self::Output {
        HPoint {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            h: self.h * rhs,
        }
    }
}
impl Add for HPoint {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            h: self.h + rhs.h,
        }
    }
}
impl Sub for HPoint {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            h: self.h - rhs.h,
        }
    }
}
impl Div<f64> for HPoint {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            h: self.h / rhs,
        }
    }
}
