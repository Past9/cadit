use std::{
    iter::Sum,
    ops::{Add, Div, Mul, Sub},
};

use super::{HPoint, Point};

#[derive(Debug, Clone, Copy)]
pub struct WPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub h: f64,
}
impl WPoint {
    pub fn new(x: f64, y: f64, z: f64, h: f64) -> Self {
        Self { x, y, z, h }
    }

    pub fn to_hpoint(&self) -> HPoint {
        HPoint {
            x: self.x,
            y: self.y,
            z: self.z,
            h: self.h,
        }
    }

    pub fn to_unweighted(&self) -> HPoint {
        HPoint {
            x: self.x / self.h,
            y: self.y / self.h,
            z: self.z / self.h,
            h: self.h,
        }
    }
}
impl Mul<f64> for WPoint {
    type Output = WPoint;

    fn mul(self, rhs: f64) -> Self::Output {
        WPoint {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            h: self.h * rhs,
        }
    }
}
impl Add for WPoint {
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
impl Sub for WPoint {
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
impl Div<f64> for WPoint {
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
impl Sum for WPoint {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(WPoint::new(0.0, 0.0, 0.0, 0.0), |a, b| a + b)
    }
}
