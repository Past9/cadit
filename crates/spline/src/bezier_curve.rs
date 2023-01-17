use std::collections::HashSet;

use crate::{
    math::{
        bezier::{decasteljau, differentiate_coefficients, implicit_zero_nearest},
        line::{Line, Line2},
        FloatRange, Homogeneous, Vec2, Vec2H, Vector,
    },
    TOL,
};

#[derive(Debug)]
pub struct BezierCurve<H: Homogeneous> {
    control_points: Vec<H>,
}
impl<H: Homogeneous> BezierCurve<H> {
    pub fn new(control_points: Vec<H>) -> Self {
        Self { control_points }
    }

    pub fn point(&self, u: f64) -> H::Projected {
        let p = decasteljau(
            &self
                .control_points
                .iter()
                .map(|p| p.weight())
                .collect::<Vec<_>>(),
            u,
        );

        H::cast_from_weighted(p).project()
    }

    pub fn degree(&self) -> usize {
        self.control_points.len() - 1
    }

    pub fn derivative_curve(&self) -> Self {
        let mut der_points = Vec::new();
        let self_deg = self.degree() as f64;

        for i in 0..self.control_points.len() - 1 {
            der_points.push((self.control_points[i + 1] - self.control_points[i]) * self_deg);
        }

        Self::new(der_points)
    }
}
impl BezierCurve<Vec2H> {
    fn line_intersection_coefficients(&self, line: &Line2) -> Vec<f64> {
        self.control_points
            .iter()
            .map(|pt| pt.x * line.a + pt.y * line.b + line.c)
            .collect::<Vec<_>>()
    }

    fn derivative_intersection_coefficients(&self, line: &Line2) -> Vec<f64> {
        let self_deg = self.degree() as f64;
        let self_co = self.line_intersection_coefficients(line);

        let mut der_co = Vec::new();
        for i in 0..self_co.len() - 1 {
            der_co.push(self_co[i + 1] - self_co[i] * self_deg);
        }

        der_co
    }

    pub fn intersection_curve_plot(&self, line: &Line2) -> Vec<Vec2> {
        let coefficients = self.line_intersection_coefficients(line);

        let mut points = Vec::new();
        for x in FloatRange::new(0.0, 1.0, 300) {
            let point = Vec2::new(x, decasteljau(&coefficients, x));
            points.push(point);
        }

        points
    }

    pub fn line_intersections(&self, line: &Line2) -> Vec<Vec2> {
        // Get coefficients for an implicit Bezier curve oriented so the line is
        // along the X-axis. Do the same for this curve's derivative curve, which
        // we'll need for Newton iteration.
        let self_coefficients = self.line_intersection_coefficients(line);
        let der_coefficients = self.derivative_curve().line_intersection_coefficients(line);

        // Find the points where the implicit curve crosses the X-axis using Newton's method.
        let mut params = Vec::new();
        let num_tests = self.degree() + 2;
        for i in 0..num_tests {
            let u = i as f64 / (num_tests - 1) as f64;
            if let Some(zero) = implicit_zero_nearest(&self_coefficients, &der_coefficients, u, 50)
            {
                params.push(zero);
            }
        }

        // Evaluate the curve at the found parameter values to find the intersection points
        let mut points = params
            .into_iter()
            .map(|u| self.point(u))
            .collect::<Vec<_>>();

        // Newton iteration may have converged on the same points, so remove any duplicates.
        points.dedup_by(|a, b| (*a - *b).magnitude() <= TOL);

        points
    }

    pub fn line_hausdorff_candidates(&self, line: &Line2) -> Vec<(f64, Vec2)> {
        // Get coefficients for an implicit bezier curve oriented so the line is
        // along the X-axis. Finding the Hausdorff distance requires finding all the
        // "peaks and valleys" of this curve, which are the same as where its derivative
        // curve crosses the X-axis. Therefore we need to also create a derivative of
        // the implicit curve (not the same as the implicit form of the derivative curve
        // of this Bezier), and then also get the derivative of that for use in Newton
        // iteration.
        let self_coefficients = self.line_intersection_coefficients(line);
        let der_coefficients_1 = differentiate_coefficients(&self_coefficients);
        let der_coefficients_2 = differentiate_coefficients(&der_coefficients_1);

        println!("SELF {:?}", self_coefficients);
        println!("DER 1 {:?}", der_coefficients_1);
        println!("DER 2 {:?}", der_coefficients_2);

        // Find the points where the first derivative curve crosses the X-axis using Newton's method.
        let mut params = Vec::new();
        let num_tests = self.degree() + 2;
        for i in 0..num_tests {
            let u = i as f64 / (num_tests - 1) as f64;
            if let Some(zero) =
                implicit_zero_nearest(&der_coefficients_1, &der_coefficients_2, u, 50)
            {
                params.push(zero);
            }
        }

        // Add the start point because it can also be furthest from the line
        let mut points = Vec::new();
        let start_point = self.point(0.0);
        if !line.contains_point(&start_point) {
            points.push((0.0, start_point));
        }

        // Evaluate the Bezier curve to find the points at each param value
        points.extend(params.into_iter().map(|u| (u, self.point(u))));

        // Add the end point because it can also be furthest from the line
        let end_point = self.point(1.0);
        if !line.contains_point(&end_point) {
            points.push((1.0, end_point));
        }

        // Remove any duplicates if the Newton iteration converged on the same point(s)
        points.dedup_by(|a, b| (a.1 - b.1).magnitude() <= TOL);

        points
    }

    pub fn line_hausdorff(&self, line: &Line2) -> Hausdorff {
        let mut max = 0.0;
        let mut max_u = None;
        let mut max_point = None;

        for (u, point) in self.line_hausdorff_candidates(line) {
            let dist = (line.a * point.x + line.b * point.y + line.c).abs()
                / (line.a.powi(2) + line.b.powi(2)).sqrt();

            println!("PD {:?} {}", point, dist);

            if dist > max {
                max = dist;
                max_u = Some(u);
                max_point = Some(point);
            }
        }

        Hausdorff {
            distance: max,
            u: max_u,
            point: max_point,
        }
    }
}

pub struct Hausdorff {
    pub distance: f64,
    pub u: Option<f64>,
    pub point: Option<Vec2>,
}
