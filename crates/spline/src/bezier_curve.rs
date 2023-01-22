use space::{ELine, ESpace, EVector, HVec2, HVector, MakeImplicit, TOL};

use crate::math::{
    b_spline::curve_derivative_control_points,
    bezier::{decasteljau, differentiate_coefficients, newton, rational_bezier_derivatives},
    knot_vector::KnotVector,
    FloatRange,
};

#[derive(Debug)]
pub struct BezierCurve<H: HVector> {
    control_points: Vec<H>,
}
impl<H: HVector> BezierCurve<H> {
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
                .map(|pt| pt.weight())
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

        Self::new(cp.into_iter().map(|pt| H::cast_from_weighted(pt)).collect())
    }

    fn make_implicit<'a, L, O>(
        &'a self,
        line: &'a L,
    ) -> impl Iterator<Item = <L as MakeImplicit>::Output> + 'a
    where
        L: ELine + MakeImplicit<Input = H, Output = O>,
        O: HVector<
            Space = <<<H::Projected as EVector>::Space as ESpace>::Lower as ESpace>::Homogeneous,
        >,
    {
        self.control_points.iter().map(|cp| line.make_implicit(cp))
    }

    pub fn line_intersection_plot<L, O>(
        &self,
        line: &L,
        segments: usize,
    ) -> (
        Vec<(f64, <<O as HVector>::Weighted as EVector>::Truncated)>,
        Vec<(f64, <<O as HVector>::Weighted as EVector>::Truncated)>,
    )
    where
        L: ELine + MakeImplicit<Input = H, Output = O>,
        O: HVector<
            Space = <<<H::Projected as EVector>::Space as ESpace>::Lower as ESpace>::Homogeneous,
        >,
    {
        let self_coefficients = self
            .make_implicit(line)
            .map(|pt| pt.weight().truncate())
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

    pub fn line_intersections<L, O>(&self, line: &L) -> Vec<H::Projected>
    where
        L: ELine + MakeImplicit<Input = H, Output = O>,
        O: HVector<
            Space = <<<H::Projected as EVector>::Space as ESpace>::Lower as ESpace>::Homogeneous,
        >,
    {
        // Get coefficients for an implicit Bezier curve oriented so the line is
        // along the X-axis. Do the same for this curve's derivative curve, which
        // we'll need for Newton iteration.
        let self_coefficients = self
            .make_implicit(line)
            .map(|pt| pt.weight().truncate())
            .collect::<Vec<_>>();

        let der_coefficients = differentiate_coefficients(&self_coefficients);

        // Find the points where the implicit curve crosses the X-axis using Newton's method.
        let mut params = Vec::new();
        let num_tests = self.degree() + 2;
        for i in 0..num_tests {
            let u_initial = i as f64 / (num_tests - 1) as f64;

            let zero = newton(u_initial, 50, |u| {
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

    pub fn line_hausdorff_plot<L, O>(
        &self,
        line: &L,
        segments: usize,
    ) -> (
        Vec<(f64, O::Projected)>,
        Vec<(f64, O::Projected)>,
        Vec<(f64, O::Projected)>,
    )
    where
        L: ELine + MakeImplicit<Input = H, Output = O>,
        O: HVector<
            Space = <<<H::Projected as EVector>::Space as ESpace>::Lower as ESpace>::Homogeneous,
        >,
    {
        let ctrl_pts = self.make_implicit(line).collect::<Vec<_>>();

        let mut self_points = Vec::new();
        let mut der1_points = Vec::new();
        let mut der2_points = Vec::new();

        for u in FloatRange::new(0.0, 1.0, segments) {
            let ders = rational_bezier_derivatives(&ctrl_pts, u, 2);
            self_points.push((u, ders[0]));
            der1_points.push((u, ders[1]));
            der2_points.push((u, ders[2]));
        }

        (self_points, der1_points, der2_points)
    }

    pub fn hausdorff_to_line_candidates<L, O>(&self, line: &L) -> Vec<(f64, H::Projected)>
    where
        L: ELine<Point = H::Projected> + MakeImplicit<Input = H, Output = O>,
        O: HVector<
            Space = <<<H::Projected as EVector>::Space as ESpace>::Lower as ESpace>::Homogeneous,
        >,
    {
        // Get coefficients for an implicit bezier curve oriented so the line is
        // along the X-axis. Finding the Hausdorff distance requires finding all the
        // "peaks and valleys" of this curve, which are the same as where its derivative
        // curve crosses the X-axis. Therefore we need to also create a derivative of
        // the implicit curve (not the same as the implicit form of the derivative curve
        // of this Bezier), and then also get the derivative of that for use in Newton
        // iteration.
        let ctrl_pts = self.make_implicit(line).collect::<Vec<_>>();

        // Find the points where the first derivative crosses the X-axis using Newton's method.
        let mut params = Vec::new();
        let num_tests = self.degree() + 2;
        for i in 0..num_tests {
            let u_initial = i as f64 / (num_tests - 1) as f64;

            let zero = newton(u_initial, 50, |u| {
                let ders = rational_bezier_derivatives(&ctrl_pts, u, 2);
                (ders[1], ders[2])
            });

            if let Some(zero) = zero {
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
        points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        points.dedup_by(|a, b| (a.1 - b.1).magnitude() <= TOL);

        points
    }

    pub fn hausdorff_to_line<L, O>(&self, line: &L) -> Option<HausdorffResult<H::Projected>>
    where
        L: ELine<Point = H::Projected> + MakeImplicit<Input = H, Output = O>,
        O: HVector<
            Space = <<<H::Projected as EVector>::Space as ESpace>::Lower as ESpace>::Homogeneous,
        >,
    {
        let mut max = 0.0;
        let mut max_u_and_point: Option<(f64, H::Projected)> = None;

        let candidates = self.hausdorff_to_line_candidates(line);

        for (u, point) in candidates {
            let dist = line.dist_to_point(&point);

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
impl BezierCurve<HVec2> {
    pub fn example_quarter_circle() -> Self {
        Self::new(Vec::from([
            HVec2::new(-1.0, 0.0, 1.0),
            HVec2::new(-1.0, -1.0, 2.0_f64.sqrt() / 2.0),
            HVec2::new(-0.0, -1.0, 1.0),
        ]))
    }
}

pub struct HausdorffResult<E: EVector> {
    pub distance: f64,
    pub u: f64,
    pub point: E,
}
