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
        println!("POINTS {:?} {:?}", p1, p2);
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
        println!("DIR {:?}", dir);
        let dir = dir.normalize();

        /*
        // Find the smallest dimension for the direction (may be 0),
        // and use the other two planes to define a axis-aligned plane
        // from which to "view" the vector
        let min_dim = dir.x.abs().min(dir.y.abs()).min(dir.z.abs());

        let (plane1_d, plane1_norm) = if dir.x.abs() == min_dim {
            //YZ
            let x = dir.y;
            let y = dir.z;

            let (d, vec) = get_3d_plane_params(EVec2::new(pos.y, pos.z), EVec2::new(dir.y, dir.z));

            (d, EVec3::new(0.0, vec.x, vec.y))
        } else if dir.y.abs() == min_dim {
            // XZ
            let x = dir.x;
            let y = dir.z;

            let (d, vec) = get_3d_plane_params(EVec2::new(pos.x, pos.z), EVec2::new(dir.x, dir.z));

            (d, EVec3::new(vec.x, 0.0, vec.y))
        } else {
            // XY
            let x = dir.x;
            let y = dir.y;

            let (d, vec) = get_3d_plane_params(EVec2::new(pos.x, pos.y), EVec2::new(dir.x, dir.y));

            (d, EVec3::new(vec.x, vec.y, 0.0))
        };

        // Does the line pass through the origin?
        let passes_through_origin: bool = {
            todo!();
        };
        */

        let x0 = pos.x;
        let y0 = pos.y;
        let z0 = pos.z;

        let a = dir.x;
        let b = dir.y;
        let c = dir.z;

        println!("IN PARAMS {} {} {}, {} {} {}", x0, y0, z0, a, b, c);

        let (a1, b1, c1, d1, a2, b2, c2, d2) = {
            // Get the coefficients for the equation describing the first plane
            // that contains the line. Select from one of several equations
            // depending on whether the direction vector is zero in each axis,
            // as this can yield an invalid equation.
            let (a1, b1, c1, d1) = if a != 0.0 {
                (0.0, -a * c, a * b, a * c * y0 - a * b * z0)
            } else if b != 0.0 {
                (-b * c, 0.0, a * b, b * c * x0 - a * b * z0)
            } else if c != 0.0 {
                (-b * c, a * c, 0.0, b * c * x0 - a * c * y0)
            } else {
                panic!("Invalid vector, all components are zero");
            };

            // Normalize the coefficients
            let mag1 = EVec3::new(a1, b1, c1).magnitude();
            let (a1, b1, c1, d1) = (a1 / mag1, b1 / mag1, c1 / mag1, d1 * mag1);

            let (a2, b2, c2, d2) = {
                let EVec3 {
                    x: a2,
                    y: b2,
                    z: c2,
                } = EVec3::new(a1, b1, c1).cross(&dir).normalize();

                let d2 = -(a2 * pos.x) - (b2 * pos.y) - (c2 * pos.z);

                let mag2 = EVec3::new(a2, b2, c2).magnitude();
                let (a2, b2, c2, d2) = (a2 / mag2, b2 / mag2, c2 / mag2, d2 * mag2);

                (a2, b2, c2, d2)
            };

            (a1, b1, c1, d1, a2, b2, c2, d2)
        };

        println!(
            "OUT PARAMS {} {} {} {}, {} {} {} {}",
            a1, b1, c1, d1, a2, b2, c2, d2
        );

        let angle_cos = (a1 * a2 + b1 * b2 + c1 * c2)
            / ((a1.powi(2) + b1.powi(2) + c1.powi(2)).sqrt()
                * (a2.powi(2) + b2.powi(2) + c2.powi(2)).sqrt());
        let angle = angle_cos.acos();
        println!("PLANE ANGLE: {}, {}", angle_cos, angle);

        let orth_measure = a1 * a2 + b1 * b2 + c1 * c2;
        println!("ORTH MEASURE {}", orth_measure);

        if a1 == 0.0 && b1 == 0.0 && c1 == 0.0 {
            panic!("Invalid first plane (zero normal)");
        }

        if a2 == 0.0 && b2 == 0.0 && c2 == 0.0 {
            panic!("Invalid second plane (zero normal)");
        }

        if (a1 * a2 + b1 * b2 + c1 * c2).abs() > TOL {
            panic!("Planes are not perpendicular");
        }

        let line = Self::EuclideanLine {
            a1,
            b1,
            c1,
            d1,
            a2,
            b2,
            c2,
            d2,
        };

        println!("LINE {:?}", line);

        line
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

fn get_3d_plane_params(pos: EVec2, dir: EVec2) -> (f64, EVec2) {
    let m = dir.y / dir.x;
    let b = pos.y - m * pos.x;

    let nearest = EVec2::new((-m * b) / (m.powi(2) + 1.0), b / (m.powi(2) + 1.0));
    println!("nearest {:?}", nearest);
    let d = nearest.magnitude();
    let orth_vec = EVec2::new(-dir.y, dir.x);

    (d, orth_vec)
}

#[cfg(test)]
mod tests {
    use crate::{EVec2, EVector, TOL};

    use super::get_3d_plane_params;

    #[test]
    fn test_get_plane_params() {
        let sqrt_2_2 = 2.0f64.sqrt() / 2.0;

        let (d, vec) =
            get_3d_plane_params(EVec2::new(-2.0, 3.0), EVec2::new(1.0, -1.0).normalize());

        assert!((d - sqrt_2_2).abs() < TOL);
        assert!((vec.x - sqrt_2_2).abs() < TOL);
        assert!((vec.y - sqrt_2_2).abs() < TOL);
    }
}
