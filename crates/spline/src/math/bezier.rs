use super::Vector;

pub fn decasteljau<C: Vector>(coefficients: &[C], u: f64) -> C {
    let mut q = coefficients.to_vec();
    let degree = coefficients.len() - 1;

    for k in 1..=degree as usize {
        for i in 0..=(degree as usize - k) {
            q[i] = q[i] * (1.0 - u) + (q[i + 1] * u);
        }
    }

    q[0]
}
