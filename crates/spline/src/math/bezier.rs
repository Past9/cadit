use std::ops::{Add, Mul, Sub};

use crate::TOL;

use super::{
    basis::eval_basis_function_derivatives, binomial_coefficient, knot_vector::KnotVector,
    Homogeneous, Vector,
};

pub fn decasteljau<T>(coefficients: &[T], u: f64) -> T
where
    T: Clone + Copy + Sub<f64> + Mul<f64, Output = T> + Add<Output = T>,
{
    let mut q = coefficients.to_vec();
    let degree = coefficients.len() - 1;

    for k in 1..=degree as usize {
        for i in 0..=(degree as usize - k) {
            q[i] = q[i] * (1.0 - u) + (q[i + 1] * u);
        }
    }

    q[0]
}

pub fn implicit_zero_nearest(
    self_coefficients: &[f64],
    der_coefficients: &[f64],
    u_guess: f64,
    max_iter: usize,
) -> Option<f64> {
    let mut u = u_guess;
    for _ in 0..max_iter {
        let self_val = decasteljau(self_coefficients, u);
        let der_val = decasteljau(der_coefficients, u);

        if self_val.abs() <= TOL {
            return Some(u);
        } else {
            //println!("U VAL {} {} {}", u, self_val, der_val);
            u -= self_val / der_val;
            if u < 0.0 {
                u = 0.0;
            } else if u > 1.0 {
                u = 1.0;
            }
        }
    }

    None
}

pub fn differentiate_coefficients(coefficients: &[f64]) -> Vec<f64> {
    let deg = (coefficients.len() - 1) as f64;

    let mut derivative = Vec::new();
    for i in 0..coefficients.len() - 1 {
        derivative.push((coefficients[i + 1] - coefficients[i]) * deg);
    }

    derivative
}

pub fn derivatives<H: Homogeneous>(
    control_points: &[H],
    u: f64,
    num_ders: usize,
) -> Vec<H::Projected> {
    let degree = control_points.len() - 1;

    let kv = KnotVector::from_iter(
        control_points
            .iter()
            .map(|_| 0.0)
            .chain(control_points.iter().map(|_| 1.0))
            .collect::<Vec<_>>(),
    );

    let ders = curve_derivatives_1(
        &control_points
            .iter()
            .map(|p| p.weight())
            .collect::<Vec<_>>(),
        degree,
        &kv,
        num_ders,
        u,
    )
    .into_iter()
    .map(H::cast_from_weighted)
    .collect::<Vec<_>>();

    curve_derivatives(&ders, num_ders)
}

fn curve_derivatives_1<C: Vector>(
    control_points: &[C],
    degree: usize,
    knot_vector: &KnotVector,
    num_derivatives: usize,
    u: f64,
) -> Vec<C> {
    let du = usize::min(num_derivatives, degree);
    let mut derivatives = vec![C::zero(); du + 1];

    let knot_span = knot_vector.find_span(degree, control_points.len(), u);
    let basis_derivatives = eval_basis_function_derivatives(degree, knot_span, knot_vector, du, u);

    for k in 0..=du {
        for j in 0..=degree {
            derivatives[k] =
                derivatives[k] + control_points[knot_span - degree + j] * basis_derivatives[k][j];
        }
    }

    derivatives
}

pub fn curve_derivatives<H: Homogeneous>(
    weighted_derivatives: &[H],
    num_derivatives: usize,
) -> Vec<H::Projected> {
    let mut derivatives = vec![H::Projected::zero(); num_derivatives + 1];

    for k in 0..=num_derivatives {
        let mut v = weighted_derivatives[k].euclidean_components();
        for i in 1..=k {
            v = v - derivatives[k - i]
                * binomial_coefficient(k, i)
                * weighted_derivatives[i].homogeneous_component();
        }
        derivatives[k] = v / weighted_derivatives[0].homogeneous_component();
    }

    derivatives
}
