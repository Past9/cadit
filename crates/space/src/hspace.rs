use crate::{
    ELine, ELine2, ELine3, EUnimplementedLine, EUnimplementedVector, EVec1, EVec2, EVec3, EVec4,
    EVector, HUnimplementedVector, HVec1, HVec2, HVec3, HVector, TOL,
};
use std::fmt::Debug;

pub trait HSpace: Debug + Clone {
    const DIMENSIONS: usize;
    type EuclideanLine: ELine;
    type Lower: HSpace;
    type Vector: HVector;
    type ProjectedVector: EVector;
    type WeightedVector: EVector;

    fn cast_vec_from_weighted(weighted: Self::WeightedVector) -> Self::Vector;
    fn weight_vec(hvec: Self::Vector) -> Self::WeightedVector;
    fn unweight_vec(weighted: Self::WeightedVector) -> Self::Vector;
    fn project_vec(hvec: Self::Vector) -> Self::ProjectedVector;
    fn make_line(pos: Self::ProjectedVector, dir: Self::ProjectedVector) -> Self::EuclideanLine;
    fn line_dist_to_projected_point(
        line: &Self::EuclideanLine,
        point: &Self::ProjectedVector,
    ) -> f64;
    fn line_contains_projected_point(
        line: &Self::EuclideanLine,
        point: &Self::ProjectedVector,
    ) -> bool;
    fn make_point_implicit_by_line(
        line: &Self::EuclideanLine,
        point: &Self::Vector,
    ) -> <Self::Lower as HSpace>::Vector;
    fn split_implicit_vec_dimensions(point: <Self::Lower as HSpace>::Vector) -> Vec<HVec1>;
    fn euclidean_vec_components(hvec: Self::Vector) -> Self::ProjectedVector;
    fn weight_implicit_vec(vec: <Self::Lower as HSpace>::Vector) -> Self::ProjectedVector;
    fn truncate_projected_vec(
        vec: Self::ProjectedVector,
    ) -> <Self::Lower as HSpace>::ProjectedVector;
    fn truncate_weighted_vec(weighted: Self::WeightedVector) -> Self::ProjectedVector;

    fn make_line_through_points(
        p1: Self::ProjectedVector,
        p2: Self::ProjectedVector,
    ) -> Self::EuclideanLine {
        Self::make_line(p1, p2 - p1)
    }
}

#[derive(Debug, Clone)]
pub struct HUnimplementedSpace {}
impl HSpace for HUnimplementedSpace {
    const DIMENSIONS: usize = 0;
    type EuclideanLine = EUnimplementedLine;
    type Lower = HUnimplementedSpace;
    type Vector = HUnimplementedVector;
    type ProjectedVector = EUnimplementedVector;
    type WeightedVector = EUnimplementedVector;

    fn cast_vec_from_weighted(_weighted: Self::WeightedVector) -> Self::Vector {
        unimplemented!()
    }

    fn weight_vec(_hvec: Self::Vector) -> Self::WeightedVector {
        unimplemented!()
    }

    fn unweight_vec(_weighted: Self::WeightedVector) -> Self::Vector {
        unimplemented!()
    }

    fn project_vec(_hvec: Self::Vector) -> Self::ProjectedVector {
        unimplemented!()
    }

    fn make_line(_pos: Self::ProjectedVector, _dir: Self::ProjectedVector) -> Self::EuclideanLine {
        unimplemented!()
    }

    fn line_dist_to_projected_point(
        _line: &Self::EuclideanLine,
        _point: &Self::ProjectedVector,
    ) -> f64 {
        unimplemented!()
    }

    fn line_contains_projected_point(
        _line: &Self::EuclideanLine,
        _point: &Self::ProjectedVector,
    ) -> bool {
        unimplemented!()
    }

    fn make_point_implicit_by_line(
        _line: &Self::EuclideanLine,
        _point: &Self::Vector,
    ) -> <Self::Lower as HSpace>::Vector {
        unimplemented!()
    }

    fn split_implicit_vec_dimensions(_point: <Self::Lower as HSpace>::Vector) -> Vec<HVec1> {
        unimplemented!()
    }

    fn euclidean_vec_components(_hvec: Self::Vector) -> Self::ProjectedVector {
        unimplemented!()
    }

    fn weight_implicit_vec(_vec: <Self::Lower as HSpace>::Vector) -> Self::ProjectedVector {
        unimplemented!()
    }

    fn truncate_projected_vec(
        _vec: Self::ProjectedVector,
    ) -> <Self::Lower as HSpace>::ProjectedVector {
        unimplemented!()
    }

    fn truncate_weighted_vec(_weighted: Self::WeightedVector) -> Self::ProjectedVector {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub struct HSpace1 {}
impl HSpace for HSpace1 {
    const DIMENSIONS: usize = 1;
    type Lower = HUnimplementedSpace;
    type EuclideanLine = EUnimplementedLine;
    type Vector = HVec1;
    type ProjectedVector = EVec1;
    type WeightedVector = EVec2;

    fn weight_vec(hvec: Self::Vector) -> Self::WeightedVector {
        Self::WeightedVector {
            x: hvec.x * hvec.h,
            y: hvec.h,
        }
    }

    fn unweight_vec(weighted: Self::WeightedVector) -> Self::Vector {
        Self::Vector {
            x: weighted.x / weighted.y,
            h: weighted.y,
        }
    }

    fn project_vec(hvec: Self::Vector) -> Self::ProjectedVector {
        Self::ProjectedVector { x: hvec.x / hvec.h }
    }

    fn make_line(_pos: Self::ProjectedVector, _dir: Self::ProjectedVector) -> Self::EuclideanLine {
        unimplemented!()
    }

    fn line_dist_to_projected_point(
        _line: &Self::EuclideanLine,
        _point: &Self::ProjectedVector,
    ) -> f64 {
        todo!()
    }

    fn line_contains_projected_point(
        _line: &Self::EuclideanLine,
        _point: &Self::ProjectedVector,
    ) -> bool {
        todo!()
    }

    fn make_point_implicit_by_line(
        _line: &Self::EuclideanLine,
        _point: &Self::Vector,
    ) -> <Self::Lower as HSpace>::Vector {
        unimplemented!()
    }

    fn split_implicit_vec_dimensions(_point: <Self::Lower as HSpace>::Vector) -> Vec<HVec1> {
        unimplemented!()
    }

    fn cast_vec_from_weighted(weighted: Self::WeightedVector) -> Self::Vector {
        Self::Vector {
            x: weighted.x,
            h: weighted.y,
        }
    }

    fn euclidean_vec_components(hvec: Self::Vector) -> Self::ProjectedVector {
        Self::ProjectedVector { x: hvec.x }
    }

    fn truncate_weighted_vec(weighted: Self::WeightedVector) -> Self::ProjectedVector {
        Self::ProjectedVector { x: weighted.x }
    }

    fn weight_implicit_vec(_vec: <Self::Lower as HSpace>::Vector) -> Self::ProjectedVector {
        unimplemented!()
    }

    fn truncate_projected_vec(
        _vec: Self::ProjectedVector,
    ) -> <Self::Lower as HSpace>::ProjectedVector {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub struct HSpace2 {}
impl HSpace for HSpace2 {
    const DIMENSIONS: usize = 2;
    type Lower = HSpace1;
    type Vector = HVec2;
    type WeightedVector = EVec3;
    type ProjectedVector = EVec2;
    type EuclideanLine = ELine2;

    fn weight_vec(hvec: Self::Vector) -> Self::WeightedVector {
        Self::WeightedVector {
            x: hvec.x * hvec.h,
            y: hvec.y * hvec.h,
            z: hvec.h,
        }
    }

    fn unweight_vec(weighted: Self::WeightedVector) -> Self::Vector {
        Self::Vector {
            x: weighted.x / weighted.z,
            y: weighted.y / weighted.z,
            h: weighted.z,
        }
    }

    fn project_vec(hvec: Self::Vector) -> Self::ProjectedVector {
        Self::ProjectedVector {
            x: hvec.x / hvec.h,
            y: hvec.y / hvec.h,
        }
    }

    fn make_line(pos: Self::ProjectedVector, dir: Self::ProjectedVector) -> Self::EuclideanLine {
        let dir = dir.normalize();

        let a = dir.y;
        let b = -dir.x;
        let c = dir.x * pos.y - dir.y * pos.x;

        Self::EuclideanLine { a, b, c }
    }

    fn line_dist_to_projected_point(
        line: &Self::EuclideanLine,
        point: &Self::ProjectedVector,
    ) -> f64 {
        (line.a * point.x + line.b * point.y + line.c).abs()
            / (line.a.powi(2) + line.b.powi(2)).sqrt()
    }

    fn line_contains_projected_point(
        line: &Self::EuclideanLine,
        point: &Self::ProjectedVector,
    ) -> bool {
        let eval = line.a * point.x + line.b * point.y + line.c;
        eval.abs() <= TOL
    }

    fn make_point_implicit_by_line(
        line: &Self::EuclideanLine,
        point: &Self::Vector,
    ) -> <Self::Lower as HSpace>::Vector {
        HVec1 {
            x: point.x * line.a + point.y * line.b + line.c,
            h: point.h,
        }
    }

    fn split_implicit_vec_dimensions(point: <Self::Lower as HSpace>::Vector) -> Vec<HVec1> {
        vec![HVec1 {
            x: point.x,
            h: point.h,
        }]
    }

    fn cast_vec_from_weighted(weighted: Self::WeightedVector) -> Self::Vector {
        Self::Vector {
            x: weighted.x,
            y: weighted.y,
            h: weighted.z,
        }
    }

    fn euclidean_vec_components(hvec: Self::Vector) -> Self::ProjectedVector {
        Self::ProjectedVector {
            x: hvec.x,
            y: hvec.y,
        }
    }

    fn truncate_weighted_vec(weighted: Self::WeightedVector) -> Self::ProjectedVector {
        Self::ProjectedVector {
            x: weighted.x,
            y: weighted.y,
        }
    }

    fn weight_implicit_vec(vec: <Self::Lower as HSpace>::Vector) -> Self::ProjectedVector {
        Self::ProjectedVector {
            x: vec.x * vec.h,
            y: vec.h,
        }
    }

    fn truncate_projected_vec(
        vec: Self::ProjectedVector,
    ) -> <Self::Lower as HSpace>::ProjectedVector {
        EVec1 { x: vec.x }
    }
}

#[derive(Debug, Clone)]
pub struct HSpace3 {}
impl HSpace for HSpace3 {
    const DIMENSIONS: usize = 3;
    type Lower = HSpace2;
    type Vector = HVec3;
    type ProjectedVector = EVec3;
    type WeightedVector = EVec4;
    type EuclideanLine = ELine3;

    fn weight_vec(hvec: Self::Vector) -> Self::WeightedVector {
        Self::WeightedVector {
            x: hvec.x * hvec.h,
            y: hvec.y * hvec.h,
            z: hvec.z * hvec.h,
            w: hvec.h,
        }
    }

    fn unweight_vec(weighted: Self::WeightedVector) -> Self::Vector {
        Self::Vector {
            x: weighted.x / weighted.w,
            y: weighted.y / weighted.w,
            z: weighted.z / weighted.w,
            h: weighted.w,
        }
    }

    fn project_vec(hvec: Self::Vector) -> Self::ProjectedVector {
        Self::ProjectedVector {
            x: hvec.x / hvec.h,
            y: hvec.y / hvec.h,
            z: hvec.z / hvec.h,
        }
    }

    fn make_line(pos: Self::ProjectedVector, dir: Self::ProjectedVector) -> Self::EuclideanLine {
        let dir = dir.normalize();

        let x0 = pos.x;
        let y0 = pos.y;
        let z0 = pos.z;

        let a = dir.x;
        let b = dir.y;
        let c = dir.z;

        let a1 = -b * c;
        let b1 = a * c;
        let c1 = 0.0;
        let d1 = (b * c * x0) - (a * c * y0);

        let a2 = -b * c;
        let b2 = 0.0;
        let c2 = a * b;
        let d2 = (b * c * x0) - (a * b * z0);

        Self::EuclideanLine {
            a1,
            b1,
            c1,
            d1,
            a2,
            b2,
            c2,
            d2,
        }
    }

    fn line_dist_to_projected_point(
        line: &Self::EuclideanLine,
        point: &Self::ProjectedVector,
    ) -> f64 {
        let dist1 = (line.a1 * point.x + line.b1 * point.y + line.c1 * point.z + line.d1).abs()
            / (line.a1.powi(2) + line.b1.powi(2) + line.c1.powi(2)).sqrt();

        let dist2 = (line.a2 * point.x + line.b2 * point.y + line.c2 * point.z + line.d2).abs()
            / (line.a2.powi(2) + line.b2.powi(2) + line.c2.powi(2)).sqrt();

        EVec2::new(dist1, dist2).magnitude()
    }

    fn line_contains_projected_point(
        line: &Self::EuclideanLine,
        point: &Self::ProjectedVector,
    ) -> bool {
        // Check if point lies in first plane
        let eval = line.a1 * point.x + line.b1 * point.y + line.c1 * point.z + line.d1;
        if eval.abs() > TOL {
            return false;
        }

        // Check if point lies in second plane
        let eval = line.a2 * point.x + line.b2 * point.y + line.c2 * point.z + line.d2;
        if eval.abs() > TOL {
            return false;
        }

        // If point lies in both planes, it's on the line
        true
    }

    fn make_point_implicit_by_line(
        line: &Self::EuclideanLine,
        point: &Self::Vector,
    ) -> <Self::Lower as HSpace>::Vector {
        HVec2 {
            x: point.x * line.a1 + point.y * line.b1 + point.z * line.c1 + line.d1,
            y: point.x * line.a2 + point.y * line.b2 + point.z * line.c2 + line.d2,
            h: point.h,
        }
    }

    fn split_implicit_vec_dimensions(point: <Self::Lower as HSpace>::Vector) -> Vec<HVec1> {
        vec![
            HVec1 {
                x: point.x,
                h: point.h,
            },
            HVec1 {
                x: point.y,
                h: point.h,
            },
        ]
    }

    fn cast_vec_from_weighted(weighted: Self::WeightedVector) -> Self::Vector {
        Self::Vector {
            x: weighted.x,
            y: weighted.y,
            z: weighted.z,
            h: weighted.w,
        }
    }

    fn euclidean_vec_components(hvec: Self::Vector) -> Self::ProjectedVector {
        Self::ProjectedVector {
            x: hvec.x,
            y: hvec.y,
            z: hvec.z,
        }
    }

    fn truncate_weighted_vec(weighted: Self::WeightedVector) -> Self::ProjectedVector {
        Self::ProjectedVector {
            x: weighted.x,
            y: weighted.y,
            z: weighted.z,
        }
    }

    fn weight_implicit_vec(vec: <Self::Lower as HSpace>::Vector) -> Self::ProjectedVector {
        Self::ProjectedVector {
            x: vec.x * vec.h,
            y: vec.y * vec.h,
            z: vec.h,
        }
    }

    fn truncate_projected_vec(
        vec: Self::ProjectedVector,
    ) -> <Self::Lower as HSpace>::ProjectedVector {
        EVec2 { x: vec.x, y: vec.y }
    }
}
