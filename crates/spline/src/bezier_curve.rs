use space::{
    exp::{HSpace, HSpace1, HSpace2},
    ELine, EVector, HVec1, HVec2, HVector, TOL,
};

use crate::math::{
    b_spline::curve_derivative_control_points,
    bezier::{decasteljau, differentiate_coefficients, newton, rational_bezier_derivatives},
    knot_vector::KnotVector,
    FloatRange,
};

#[derive(Debug)]
pub struct BezierCurve<H: HSpace> {
    control_points: Vec<H::Vector>,
}
impl<H: HSpace> BezierCurve<H> {
    pub fn new(control_points: Vec<H::Vector>) -> Self {
        Self { control_points }
    }

    pub fn point(&self, u: f64) -> H::ProjectedVector {
        let p = decasteljau(
            &self
                .control_points
                .iter()
                .map(|p| H::weight_vec(p.clone()))
                .collect::<Vec<_>>(),
            u,
        );

        H::project_vec(H::cast_vec_from_weighted(p))
    }

    pub fn degree(&self) -> usize {
        self.control_points.len() - 1
    }

    pub fn hodograph(&self) -> Self {
        let kv = self
            .control_points
            .iter()
            .map(|_| 0.0)
            .chain(self.control_points.iter().map(|_| 1.0));

        let cp = curve_derivative_control_points(
            &self
                .control_points
                .iter()
                .map(|pt| H::weight_vec(pt.clone()))
                .collect::<Vec<_>>(),
            self.degree(),
            &KnotVector::from_iter(kv),
            0,
            self.control_points.len() - 1,
            1,
        )
        .swap_remove(1)
        .into_iter()
        .take(self.degree())
        .collect::<Vec<_>>();

        Self::new(
            cp.into_iter()
                .map(|pt| H::cast_vec_from_weighted(pt))
                .collect(),
        )
    }

    pub fn line_intersection_plot(
        &self,
        line: &H::EuclideanLine,
        segments: usize,
    ) -> (
        Vec<(f64, <H::Lower as HSpace>::ProjectedVector)>,
        Vec<(f64, <H::Lower as HSpace>::ProjectedVector)>,
    ) {
        let self_coefficients = self
            .control_points
            .iter()
            .map(|pt| {
                let implicit = H::make_point_implicit_by_line(line, pt);
                let weighted = H::weight_implicit_vec(implicit);
                let truncated = H::truncate_projected_vec(weighted);
                truncated
            })
            .collect::<Vec<_>>();

        let der_coefficients = differentiate_coefficients(&self_coefficients);

        let mut self_points = Vec::new();
        let mut der_points = Vec::new();

        for u in FloatRange::new(0.0, 1.0, segments) {
            self_points.push((u, decasteljau(&self_coefficients, u)));
            der_points.push((u, decasteljau(&der_coefficients, u)));
        }

        (self_points, der_points)
    }

    pub fn line_intersections(&self, line: &H::EuclideanLine) -> Vec<H::ProjectedVector> {
        // Get coefficients for an implicit Bezier curve oriented so the line is
        // along the X-axis. Do the same for this curve's derivative curve, which
        // we'll need for Newton iteration.
        let self_coefficients = self
            .control_points
            .iter()
            .map(|pt| {
                let implicit = H::make_point_implicit_by_line(line, pt);
                let weighted = H::weight_implicit_vec(implicit);
                let truncated = H::truncate_projected_vec(weighted);
                truncated
            })
            .collect::<Vec<_>>();

        let der_coefficients = differentiate_coefficients(&self_coefficients);

        // Find the points where the implicit curve crosses the X-axis using Newton's method.
        let mut params = Vec::new();
        let total_weight: f64 = self
            .control_points
            .iter()
            .map(|pt| pt.homogeneous_component())
            .sum();
        let mut accum_weight = 0.0;
        for i in 0..self.degree() {
            accum_weight += self.control_points[i].homogeneous_component();
            let u_initial = accum_weight / total_weight;

            let zero = newton(u_initial, 50, 0.0, 1.0, |u| {
                (
                    decasteljau(&self_coefficients, u),
                    decasteljau(&der_coefficients, u),
                )
            });

            if let Some(zero) = zero {
                println!("U ZERO {}", zero);
                params.push(zero);
            } else {
                println!("U NOT FOUND");
            }
        }

        // Evaluate the curve at the found parameter values to find the intersection points
        let mut points = params
            .into_iter()
            .map(|u| self.point(u))
            .collect::<Vec<_>>();

        // Newton iteration may have converged on the same points, so remove any duplicates.
        // TODO: Sort these somehow, deduplication does nothing unless duplicates are next to each other
        points.dedup_by(|a, b| (*a - *b).magnitude() <= TOL);

        points
    }

    pub fn line_hausdorff_plot(
        &self,
        line: &H::EuclideanLine,
        segments: usize,
    ) -> (
        Vec<(f64, <H::Lower as HSpace>::ProjectedVector)>,
        Vec<(f64, <H::Lower as HSpace>::ProjectedVector)>,
        Vec<(f64, <H::Lower as HSpace>::ProjectedVector)>,
    ) {
        let ctrl_pts = self
            .control_points
            .iter()
            .map(|pt| H::make_point_implicit_by_line(line, pt))
            .collect::<Vec<_>>();

        let mut self_points = Vec::new();
        let mut der1_points = Vec::new();
        let mut der2_points = Vec::new();

        for u in FloatRange::new(0.0, 1.0, segments) {
            let ders = rational_bezier_derivatives::<H::Lower>(&ctrl_pts, u, 2);
            self_points.push((u, ders[0]));
            der1_points.push((u, ders[1]));
            der2_points.push((u, ders[2]));
        }

        (self_points, der1_points, der2_points)
    }

    pub fn hausdorff_to_line_candidates(
        &self,
        line: &H::EuclideanLine,
        min_u: Option<f64>,
        max_u: Option<f64>,
    ) -> Vec<(f64, H::ProjectedVector)> {
        let min_u = min_u.unwrap_or(0.0);
        let max_u = max_u.unwrap_or(1.0);

        // Get coefficients for an implicit bezier curve oriented so the line is
        // along the X-axis. Finding the Hausdorff distance requires finding all the
        // "peaks and valleys" of this curve, which are the same as where its derivative
        // curve crosses the X-axis. Therefore we need to also create a derivative of
        // the implicit curve (not the same as the implicit form of the derivative curve
        // of this Bezier), and then also get the derivative of that for use in Newton
        // iteration.
        let implicit = self
            .control_points
            .iter()
            .map(|pt| H::make_point_implicit_by_line(line, pt))
            .collect::<Vec<_>>();

        let mut ctrl_pts: Vec<Vec<HVec1>> = vec![Vec::new(); H::DIMENSIONS - 1];

        for pt in implicit.into_iter() {
            let dims = H::split_implicit_vec_dimensions(pt);
            for d in 0..H::DIMENSIONS - 1 {
                ctrl_pts[d].push(dims[d]);
            }
        }

        // Find the points where the first derivative in each dimension crosses an origin plane
        // using Newton's method.
        let try_point = |u_initial: f64, params: &mut Vec<f64>| {
            for d in 0..H::DIMENSIONS - 1 {
                let zero = newton(u_initial, 20, min_u, max_u, |u| {
                    let ders = rational_bezier_derivatives::<HSpace1>(&ctrl_pts[d], u, 2);
                    (ders[1], ders[2])
                });

                if let Some(zero) = zero {
                    params.push(zero);
                }
            }
        };

        let mut params = Vec::new();
        let total_weight: f64 = self
            .control_points
            .iter()
            .map(|pt| pt.homogeneous_component())
            .sum();

        let mut initial_us = Vec::new();
        let mut accum_weight = 0.0;
        for i in 0..self.degree() {
            accum_weight += self.control_points[i].homogeneous_component();
            initial_us.push(accum_weight / total_weight);
        }

        let mut skipped_start = false;
        for u_initial in initial_us.into_iter() {
            if u_initial < min_u {
                skipped_start = true;
            }

            if skipped_start {
                try_point(min_u, &mut params);
                skipped_start = false;
            }

            if u_initial > max_u {
                try_point(max_u, &mut params);
                break;
            }

            try_point(u_initial, &mut params);
        }

        // Add the start point because it can also be furthest from the line
        let mut points = Vec::new();
        let start_point = self.point(min_u);
        if !H::line_contains_projected_point(line, &start_point) {
            points.push((min_u, start_point));
        }

        // Evaluate the Bezier curve to find the points at each param value
        points.extend(params.into_iter().map(|u| (u, self.point(u))));

        // Add the end point because it can also be furthest from the line
        let end_point = self.point(max_u);
        if !H::line_contains_projected_point(line, &end_point) {
            points.push((max_u, end_point));
        }

        // Remove any duplicates if the Newton iteration converged on the same point(s)
        points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        points.dedup_by(|a, b| (a.1 - b.1).magnitude() <= TOL);

        points
    }

    pub fn hausdorff_to_line(
        &self,
        line: &H::EuclideanLine,
        min_u: Option<f64>,
        max_u: Option<f64>,
    ) -> Option<HausdorffResult<H>> {
        let mut max = 0.0;
        let mut max_u_and_point: Option<(f64, H::ProjectedVector)> = None;

        let candidates = self.hausdorff_to_line_candidates(line, min_u, max_u);

        for (u, point) in candidates {
            let dist = H::line_dist_to_projected_point(line, &point);

            if dist > max {
                max = dist;
                max_u_and_point = Some((u, point));
            }
        }

        max_u_and_point.map(|(max_u, max_point)| HausdorffResult {
            distance: max,
            u: max_u,
            point: max_point,
        })
    }
}
impl BezierCurve<HSpace2> {
    pub fn example_quarter_circle() -> Self {
        Self::new(Vec::from([
            HVec2::new(-1.0, 0.0, 1.0),
            HVec2::new(-1.0, -1.0, 2.0_f64.sqrt() / 2.0),
            HVec2::new(-0.0, -1.0, 1.0),
        ]))
    }
}

pub struct HausdorffResult<H: HSpace> {
    pub distance: f64,
    pub u: f64,
    pub point: H::ProjectedVector,
}
