use std::collections::HashSet;

use crate::{
    math::{
        b_spline::curve_derivative_control_points,
        bezier::{decasteljau, differentiate_coefficients, implicit_zero_nearest},
        knot_vector::KnotVector,
        line::{Line, Line2},
        FloatRange, Homogeneous, Vec1H, Vec2, Vec2H, Vector,
    },
    nurbs_curve::NurbsCurve,
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

    /*
    pub fn derivative_curve(&self) -> Self {
        let mut der_points = Vec::new();
        let self_deg = self.degree() as f64;

        let weighted = self
            .control_points
            .iter()
            .cloned()
            .map(|pt| pt.weight())
            .collect::<Vec<_>>();

        for i in 0..weighted.len() - 1 {
            let point = H::cast_from_weighted((weighted[i + 1] - weighted[i]) * self_deg);
            der_points.push(point);
        }

        Self::new(der_points)
    }
    */

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

        println!("CP {:?}", cp);

        Self::new(cp.into_iter().map(|pt| H::cast_from_weighted(pt)).collect())

        /*
        let nurbs = NurbsCurve::new(
            self.control_points.clone(),
            self.control_points
                .iter()
                .map(|_| 0.0)
                .chain(self.control_points.iter().map(|_| 1.0))
                .collect(),
        );

        let nurbs_hodograph = nurbs.derivative_curve(1);

        assert!(nurbs_hodograph.control_points().len() == self.control_points.len() - 1);
        assert!(nurbs_hodograph.knot_vector().len() == nurbs_hodograph.control_points().len() * 2);

        for (i, knot) in nurbs_hodograph.knot_vector().iter().enumerate() {
            if i < nurbs_hodograph.control_points().len() {
                assert!(*knot == 0.0);
            } else {
                assert!(*knot == 1.0);
            }
        }

        Self::new(nurbs_hodograph.control_points().to_vec())
        */
    }
}
impl BezierCurve<Vec2H> {
    pub fn example_quarter_circle() -> Self {
        Self::new(Vec::from([
            Vec2H::new(-1.0, 0.0, 1.0),
            Vec2H::new(-1.0, -1.0, 2.0_f64.sqrt() / 2.0),
            Vec2H::new(-0.0, -1.0, 1.0),
        ]))
    }

    fn line_intersection_coefficients(&self, line: &Line2) -> Vec<f64> {
        self.control_points
            .iter()
            .map(|pt| (pt.x * line.a + pt.y * line.b + line.c) * pt.h)
            .collect::<Vec<_>>()
    }

    fn line_derivative_coefficients(&self, line: &Line2) -> Vec<f64> {
        let c1 = BezierCurve::new(
            self.control_points
                .iter()
                .enumerate()
                .map(|(i, pt)| {
                    Vec1H::new(
                        //i as f64 / (self.control_points.len() as f64 - 1.0),
                        (pt.x * line.a + pt.y * line.b + line.c) * pt.h,
                        pt.h,
                    )
                })
                .collect::<Vec<_>>(),
        );

        println!("C1 {:#?}", c1);

        let hodo = c1.hodograph();

        println!("HODO {:#?}", hodo);

        let der = hodo
            .control_points
            .iter()
            .map(|pt| pt.x)
            .collect::<Vec<_>>();

        der

        /*
        self.hodograph()
            .control_points
            .iter()
            .map(|pt| (pt.x * line.a + pt.y * line.b + line.c))
            .collect::<Vec<_>>()
            */

        /*
        let self_bezier = BezierCurve::new(
            self.control_points
                .iter()
                .map(|pt| Vec1H::new((pt.x * line.a + pt.y * line.b + line.c) * pt.h, pt.h))
                .collect::<Vec<_>>(),
        );

        println!("SELF BEZIER {:#?}", self_bezier);

        let hodograph = self_bezier.hodograph();

        println!("HODO {:#?}", hodograph);

        let der_coefficients = hodograph
            .control_points
            .into_iter()
            .map(|pt| pt.weight().x)
            .collect::<Vec<_>>();

        der_coefficients
        */
        /*
        let line_coefficients = self
            .control_points
            .iter()
            .map(|pt| (pt.x * line.a + pt.y * line.b + line.c) * pt.h)
            .collect::<Vec<_>>();

        let der_coefficients = differentiate_coefficients(&line_coefficients);

        der_coefficients
        */
    }

    pub fn line_intersection_plot(&self, line: &Line2) -> Vec<Vec2> {
        let coefficients = self.line_intersection_coefficients(line);

        let mut points = Vec::new();
        for x in FloatRange::new(0.0, 1.0, 300) {
            let point = Vec2::new(x, decasteljau(&coefficients, x));
            points.push(Vec2::new(point.x, point.y / 10.0));
        }

        points
    }

    pub fn line_derivative_plot(&self, line: &Line2) -> Vec<Vec2> {
        let coefficients = self.line_derivative_coefficients(line);

        println!("DER PLOT {:?}", coefficients);

        let mut points = Vec::new();
        for x in FloatRange::new(0.0, 1.0, 300) {
            let point = Vec2::new(x, decasteljau(&coefficients, x));
            points.push(Vec2::new(point.x, point.y / 50.0));
        }

        points
    }

    pub fn line_intersections(&self, line: &Line2) -> Vec<Vec2> {
        // Get coefficients for an implicit Bezier curve oriented so the line is
        // along the X-axis. Do the same for this curve's derivative curve, which
        // we'll need for Newton iteration.
        let self_coefficients = self.line_intersection_coefficients(line);
        let der_coefficients = differentiate_coefficients(&self_coefficients);
        //let der_coefficients = self.derivative_curve().line_intersection_coefficients(line);

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
        // TODO: Sort these somehow, deduplication does nothing unless duplicates are next to each other
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

        //let self_coefficients = self.line_intersection_coefficients(line);
        //let der_coefficients_1 = differentiate_coefficients(&self_coefficients);
        //let der_coefficients_2 = differentiate_coefficients(&der_coefficients_1);

        //let der_coefficients_1 = self.line_derivative_coefficients(line);
        //let der_coefficients_2 = differentiate_coefficients(&der_coefficients_1);

        //println!("SC {:?}", self.line_intersection_coefficients(line));
        //println!("DC1 {:?}", der_coefficients_1);
        //println!("DC2 {:?}", der_coefficients_2);

        let nurbs_ctrl = self
            .control_points
            .iter()
            .enumerate()
            .map(|(i, pt)| {
                Vec1H::new(
                    //i as f64 / (self.control_points.len() as f64 - 1.0),
                    (pt.x * line.a + pt.y * line.b + line.c),
                    pt.h,
                )
            })
            .collect::<Vec<_>>();

        let nurbs_kv = nurbs_ctrl
            .iter()
            .map(|_| 0.0)
            .chain(self.control_points.iter().map(|_| 1.0))
            .collect::<Vec<_>>();

        let nurbs = NurbsCurve::new(nurbs_ctrl, KnotVector::from_iter(nurbs_kv.into_iter()));

        // Find the points where the first derivative crosses the X-axis using Newton's method.
        let mut params = Vec::new();
        let num_tests = self.degree() + 2;
        for i in 0..num_tests {
            let u_initial = i as f64 / (num_tests - 1) as f64;
            let mut result = None;

            let zero = {
                let mut u = u_initial;

                for _ in 0..50 {
                    let ders = nurbs.derivatives(u, 2);
                    let self_val = ders[1];
                    let der_val = ders[2];

                    if self_val.x.abs() <= TOL {
                        result = Some(u);
                        break;
                    } else {
                        u -= self_val.x / der_val.x;
                        if u < 0.0 {
                            u = 0.0;
                        } else if u > 1.0 {
                            u = 1.0;
                        }
                    }
                }

                result
            };

            if let Some(zero) = zero
            //implicit_zero_nearest(&der_coefficients_1, &der_coefficients_2, u_initial, 50)
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
        points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        points.dedup_by(|a, b| (a.1 - b.1).magnitude() <= TOL);

        points
    }

    pub fn line_hausdorff(&self, line: &Line2) -> Hausdorff {
        //println!("LINE HAUS");
        let mut max = 0.0;
        let mut max_u = None;
        let mut max_point = None;

        let candidates = self.line_hausdorff_candidates(line);

        for (u, point) in candidates {
            //println!("CANDIDATE {} {:?}", u, point);
            let dist = (line.a * point.x + line.b * point.y + line.c).abs()
                / (line.a.powi(2) + line.b.powi(2)).sqrt();

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
