use crate::TOL;

use super::{Vec2, Vector};

pub trait Line<V: Vector> {
    fn from_pos_and_dir(pos: V, dir: V) -> Self;
    fn normalize(&self) -> Self;
    fn contains_point(&self, point: &Vec2) -> bool;
}

#[derive(Debug, Clone)]
pub struct Line2 {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}
impl Line<Vec2> for Line2 {
    fn from_pos_and_dir(pos: Vec2, dir: Vec2) -> Self {
        let dir = dir.normalize();

        let a = dir.y;
        let b = -dir.x;
        let c = dir.x * pos.y - dir.y * pos.x;

        Self { a, b, c }
    }

    fn normalize(&self) -> Self {
        let vec_mag = (self.a.powi(2) + self.b.powi(2)).sqrt();
        Self {
            a: self.a / vec_mag,
            b: self.b / vec_mag,
            c: self.c,
        }
    }

    fn contains_point(&self, point: &Vec2) -> bool {
        let eval = self.a * point.x + self.b * point.y + self.c;
        eval.abs() <= TOL
    }
}
/*
impl Line2 {
    pub fn from_pos_and_dir(pos: Vec2, dir: Vec2) -> Self {
        let dir = dir.normalize();

        let a = dir.y;
        let b = -dir.x;
        let c = dir.x * pos.y - dir.y * pos.x;

        Self { a, b, c }
    }

    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Self { a, b, c }
    }

    pub fn normalize(&self) -> Self {
        let vec_mag = (self.a.powi(2) + self.b.powi(2)).sqrt();
        Self {
            a: self.a / vec_mag,
            b: self.b / vec_mag,
            c: self.c,
        }
    }
}
*/
