use std::ops::{Add, Mul, Sub};

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
