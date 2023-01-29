use std::fmt::Debug;

use crate::{EPlane3, EVec2, EVec3, EVector, HVec2, HVec3};

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
    pos: EVec3,
    dir: EVec3,
    pub p1: EPlane3,
    pub p2: EPlane3,
}
impl ELine3 {
    pub fn new_from_pos_and_dir(pos: EVec3, dir: EVec3) -> Self {
        let dir = dir.normalize();

        let x0 = pos.x;
        let y0 = pos.y;
        let z0 = pos.z;

        let a = dir.x;
        let b = dir.y;
        let c = dir.z;

        let (p1, p2) = {
            // Get the coefficients for the equation describing the first plane
            // that contains the line. Select from one of several equations
            // depending on whether the direction vector is zero in each axis,
            // as this can yield an invalid equation.
            let p1 = if a == 0.0 && b == 0.0 {
                EPlane3::new_general_form(a.signum(), 0.0, 0.0, -x0)
            } else if a == 0.0 && c == 0.0 {
                EPlane3::new_general_form(a.signum(), 0.0, 0.0, -x0)
            } else if b == 0.0 && c == 0.0 {
                EPlane3::new_general_form(0.0, b.signum(), 0.0, -y0)
            } else if a != 0.0 {
                EPlane3::new_general_form(0.0, -a * c, a * b, a * c * y0 - a * b * z0)
            } else if b != 0.0 {
                EPlane3::new_general_form(-b * c, 0.0, a * b, b * c * x0 - a * b * z0)
            } else if c != 0.0 {
                EPlane3::new_general_form(-b * c, a * c, 0.0, b * c * x0 - a * c * y0)
            } else {
                panic!("Invalid vector, all components are zero");
            }
            .normalize();

            let p2 = {
                let p2_norm = p1.norm.cross(&dir).normalize();
                let d2 = -(p2_norm.x * pos.x) - (p2_norm.y * pos.y) - (p2_norm.z * pos.z);
                EPlane3::new_from_normal_vec(p2_norm, d2).normalize()
            };

            (p1, p2)
        };

        if !p1.is_valid() {
            panic!("Invalid first plane");
        }

        if !p2.is_valid() {
            panic!("Invalid second plane");
        }

        if !p1.is_perpendicular_to(&p2) {
            println!("pos {:?}", pos);
            println!("dir {:?}", dir);
            println!("P1 {:?}", p1);
            println!("P2 {:?}", p2);
            panic!("Planes are not perpendicular");
        }

        let line = Self { pos, dir, p1, p2 };

        line
    }

    pub fn closest_to_point(&self, point: &EVec3) -> EVec3 {
        self.pos + self.dir * ((point - self.pos).dot(&self.dir))
    }

    pub fn dist_to_point(&self, point: &EVec3) -> f64 {
        EVec2::new(self.p1.dist_to_point(point), self.p2.dist_to_point(point)).magnitude()
    }

    pub fn contains_point(&self, point: &EVec3) -> bool {
        self.p1.contains_point(point) && self.p2.contains_point(point)
    }

    pub fn make_implicit_point(&self, point: &HVec3) -> HVec2 {
        HVec2 {
            x: point.x * self.p1.norm.x
                + point.y * self.p1.norm.y
                + point.z * self.p1.norm.z
                + self.p1.d,
            y: point.x * self.p2.norm.x
                + point.y * self.p2.norm.y
                + point.z * self.p2.norm.z
                + self.p2.d,
            h: point.h,
        }
    }
}
impl ELine for ELine3 {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn closest_to_point() {
        let line =
            ELine3::new_from_pos_and_dir(EVec3::new(-2.0, 0.0, -2.0), EVec3::new(0.0, -1.0, 0.0));
        let point = EVec3::new(-5.5, -3.0, -4.0);

        let closest = line.closest_to_point(&point);

        println!("{:?}", closest);
    }
}
