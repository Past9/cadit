use once_cell::unsync::OnceCell;
use space::{
    hspace::{HSpace, HSpace2, HSpace3},
    EVector, HVec2, HVec3, HVector, TOL,
};

use crate::math::{
    bezier::{
        decasteljau, differentiate_coefficients, newton_f64, newton_vec, rational_curve_derivatives,
    },
    FloatRange,
};

#[derive(Debug)]
pub struct BezierCurve<H: HSpace> {
    control_points: Vec<H::Vector>,
    weighted_control_points: OnceCell<Vec<H::WeightedVector>>,
}
impl<H: HSpace> BezierCurve<H> {
    pub fn new(control_points: Vec<H::Vector>) -> Self {
        Self {
            control_points,
            weighted_control_points: OnceCell::new(),
        }
    }

    fn weighted_control_points(&self) -> &[H::WeightedVector] {
        self.weighted_control_points.get_or_init(|| {
            self.control_points
                .iter()
                .map(|p| H::weight_vec(p.clone()))
                .collect()
        })
    }

    pub fn point(&self, u: f64) -> H::ProjectedVector {
        let p = decasteljau(self.weighted_control_points(), u);
        H::project_vec(H::cast_vec_from_weighted(p))
    }

    pub fn degree(&self) -> usize {
        self.control_points.len() - 1
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

            let zero = newton_vec(u_initial, 50, 0.0, 1.0, |u| {
                (
                    decasteljau(&self_coefficients, u),
                    decasteljau(&der_coefficients, u),
                )
            });

            if let Some(zero) = zero {
                params.push(zero);
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
            .map(|pt| <H::Lower as HSpace>::weight_vec(pt))
            .collect::<Vec<_>>();

        let mut self_points = Vec::new();
        let mut der1_points = Vec::new();
        let mut der2_points = Vec::new();

        for u in FloatRange::new(0.0, 1.0, segments) {
            let ders = rational_curve_derivatives::<H::Lower>(&ctrl_pts, u, 2);
            self_points.push((u, ders[0]));
            der1_points.push((u, ders[1]));
            der2_points.push((u, ders[2]));
        }

        (self_points, der1_points, der2_points)
    }

    pub fn hausdorff_candidates(
        &self,
        line: &H::EuclideanLine,
        min_u: Option<f64>,
        max_u: Option<f64>,
        include_endpoints: bool,
    ) -> Vec<(f64, H::ProjectedVector)> {
        let (min_u, max_u) = {
            let min_u = min_u.unwrap_or(0.0);
            let max_u = max_u.unwrap_or(1.0);

            if min_u < max_u {
                (min_u, max_u)
            } else {
                (max_u, min_u)
            }
        };

        let try_point = |u_initial: f64| {
            newton_f64(u_initial, 10000, min_u, max_u, |u| {
                let ders: Vec<H::ProjectedVector> =
                    rational_curve_derivatives::<H>(&self.weighted_control_points(), u, 2);

                let closest = H::closest_to_point(line, &ders[0]);
                let between = closest - ders[0];

                let d1_norm = ders[1].normalize();

                let num = ders[1].dot(&between);
                let denom = ders[2].dot(&between) + d1_norm.dot(&d1_norm);

                (num, denom)
            })
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
            let u = accum_weight / total_weight;

            if u >= min_u && u <= max_u {
                initial_us.push(accum_weight / total_weight);
            }
        }

        if initial_us.len() == 0 {
            initial_us.push((min_u + max_u) / 2.0)
        }

        let mut skipped_start = false;
        for u_initial in initial_us.into_iter() {
            if u_initial < min_u {
                skipped_start = true;
            }

            if skipped_start {
                if let Some(point) = try_point(min_u) {
                    params.push(point);
                }
                skipped_start = false;
            }

            if u_initial > max_u {
                if let Some(point) = try_point(max_u) {
                    params.push(point);
                }
                break;
            }

            if let Some(point) = try_point(u_initial) {
                params.push(point);
            }
        }

        // All candidate points
        let mut points = Vec::new();

        if include_endpoints {
            // Add the start point because it can also be furthest from the line
            let start_point = self.point(min_u);
            if !H::line_contains_projected_point(line, &start_point) {
                points.push((min_u, start_point));
            }
        }

        // Evaluate the Bezier curve to find the points at each param value
        points.extend(params.into_iter().map(|u| (u, self.point(u))));

        if include_endpoints {
            // Add the end point because it can also be furthest from the line
            let end_point = self.point(max_u);
            if !H::line_contains_projected_point(line, &end_point) {
                points.push((max_u, end_point));
            }
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
        include_endpoints: bool,
    ) -> Option<HausdorffResult<H>> {
        let mut max = 0.0;
        let mut max_u_and_point: Option<(f64, H::ProjectedVector)> = None;

        let candidates = self.hausdorff_candidates(line, min_u, max_u, include_endpoints);

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
impl BezierCurve<HSpace3> {
    pub fn example_quarter_circle_xy() -> Self {
        Self::new(Vec::from([
            HVec3::new(-1.0, 0.0, 0.0, 1.0),
            HVec3::new(-1.0, -1.0, 0.0, 2.0_f64.sqrt() / 2.0),
            HVec3::new(-0.0, -1.0, 0.0, 1.0),
        ]))
    }
}

#[derive(Debug, Clone)]
pub struct HausdorffResult<H: HSpace> {
    pub distance: f64,
    pub u: f64,
    pub point: H::ProjectedVector,
}
