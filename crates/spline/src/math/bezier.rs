use std::ops::{Add, Mul, Sub};

use crate::TOL;

use super::Vector;

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

    println!("NEWTON FAIL");

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
