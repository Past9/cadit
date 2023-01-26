use std::ops::{Add, Mul, Sub};

use space::{hspace::HSpace, EVector, HVector, TOL};

use super::binomial_coefficient;

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

pub fn decas<T>(coefficients: &[T], u: f64) -> T
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

pub fn newton_f64<F>(u_guess: f64, max_iter: usize, min_u: f64, max_u: f64, eval: F) -> Option<f64>
where
    F: Fn(f64) -> (f64, f64),
{
    let mut u = u_guess;
    for _ in 0..max_iter {
        let (self_val, der_val) = eval(u);

        if self_val.abs() <= TOL {
            return Some(u);
        } else {
            let correction = self_val / der_val;

            if correction.abs() < 0.03 * TOL {
                //return None;
            }

            u -= correction;
            if u < min_u || u > max_u {
                return None;
            }
        }
    }

    None
}

pub fn newton_vec<F, E: EVector>(
    u_guess: f64,
    max_iter: usize,
    min_u: f64,
    max_u: f64,
    eval: F,
) -> Option<f64>
where
    F: Fn(f64) -> (E, E),
{
    let mut u = u_guess;
    for _ in 0..max_iter {
        let (self_val, der_val) = eval(u);

        if self_val.magnitude() <= TOL {
            return Some(u);
        } else {
            let correction = (self_val / der_val).max_component();

            if correction.abs() < 0.03 * TOL {
                return None;
            }

            u -= correction;
            if u < min_u || u > max_u {
                return None;
            }
        }
    }

    None
}

pub fn differentiate_coefficients<C: EVector>(coefficients: &[C]) -> Vec<C> {
    let deg = (coefficients.len() - 1) as f64;

    let mut derivative = Vec::new();
    for i in 0..coefficients.len() - 1 {
        derivative.push((coefficients[i + 1] - coefficients[i]) * deg);
    }

    derivative
}

pub fn rational_bezier_derivatives<H: HSpace>(
    control_points: &[H::Vector],
    u: f64,
    num_ders: usize,
) -> Vec<H::ProjectedVector> {
    let ders = curve_derivatives_1(
        &control_points
            .iter()
            .map(|p| H::weight_vec(*p))
            .collect::<Vec<_>>(),
        num_ders,
        u,
    )
    .into_iter()
    .map(H::cast_vec_from_weighted)
    .collect::<Vec<_>>();

    curve_derivatives::<H>(&ders, num_ders)
}

fn curve_derivatives_1<E: EVector>(control_points: &[E], num_derivatives: usize, u: f64) -> Vec<E> {
    let degree = control_points.len() - 1;
    let num_ders = usize::min(num_derivatives, degree);
    let mut derivatives = vec![E::zero(); num_ders + 1];

    let basis_derivatives = eval_basis_function_derivatives(degree, num_ders, u);

    for k in 0..=num_ders {
        for j in 0..=degree {
            derivatives[k] = derivatives[k] + control_points[j] * basis_derivatives[k][j];
        }
    }

    derivatives
}

pub fn curve_derivatives<H: HSpace>(
    weighted_derivatives: &[H::Vector],
    num_derivatives: usize,
) -> Vec<H::ProjectedVector> {
    let mut derivatives = vec![H::ProjectedVector::zero(); num_derivatives + 1];

    for k in 0..=num_derivatives {
        let mut v = H::euclidean_vec_components(weighted_derivatives[k]);
        for i in 1..=k {
            v = v - derivatives[k - i]
                * binomial_coefficient(k, i)
                * weighted_derivatives[i].homogeneous_component();
        }
        derivatives[k] = v / weighted_derivatives[0].homogeneous_component();
    }

    derivatives
}

pub fn eval_basis_function_derivatives(degree: usize, num_ders: usize, u: f64) -> Vec<Vec<f64>> {
    let mut ndu = vec![vec![1.0; degree + 1]; degree + 1];

    for j in 1..=degree {
        let mut saved = 0.0;

        for r in 0..j {
            // Lower triangle
            let temp = ndu[r][j - 1];

            // Upper triangle
            ndu[r][j] = saved + (1.0 - u) * temp;
            saved = u * temp;
        }
        ndu[j][j] = saved;
    }

    let mut derivatives: Vec<Vec<f64>> =
        vec![vec![0.0; degree + 1]; usize::min(degree, num_ders) + 1];

    // Load the basis functions
    for j in 0..=degree {
        derivatives[0][j] = ndu[j][degree];
    }

    // Begin calculating derivatives
    let mut a: Vec<Vec<f64>> = vec![vec![1.0; degree + 1]; 2];

    // This section computes the derivatives.
    // Loop over the function index
    for r in 0..=degree {
        // Alternate rows in array a
        let mut s1 = 0;
        let mut s2 = 1;

        a[0][0] = 1.0;

        // Loop to compute kth derivative
        for k in 1..=num_ders {
            let mut d = 0.0;

            let rk = r as i32 - k as i32;
            let pk = degree as i32 - k as i32;

            if r >= k {
                a[s2][0] = a[s1][0];
                d = a[s2][0] * ndu[rk as usize][pk as usize];
            }

            let j1 = if rk >= -1 { 1 } else { -rk } as usize;

            let j2 = if r as i32 - 1 <= pk {
                k - 1
            } else {
                degree - r
            };

            for j in j1..=j2 {
                a[s2][j] = a[s1][j] - a[s1][j - 1];
                d += a[s2][j] * ndu[(rk + j as i32) as usize][pk as usize];
            }

            if r <= pk as usize {
                a[s2][k] = -a[s1][k - 1];
                d += a[s2][k] * ndu[r][pk as usize];
            }

            derivatives[k][r] = d;

            // Switch rows
            let temp = s1;
            s1 = s2;
            s2 = temp;
        }
    }

    // Multiply through by the correct factors
    let mut r = degree as f64;
    for k in 1..=num_ders {
        for j in 0..=degree {
            derivatives[k][j] *= r;
        }
        r *= (degree - k) as f64;
    }

    derivatives
}
