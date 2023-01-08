pub fn decasteljau<T>(coefficients: &[T], u: f64) -> T
where
    T: Copy
        + Clone
        + std::ops::Mul<f64, Output = T>
        + std::ops::Add<f64, Output = T>
        + std::ops::Add<T, Output = T>,
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

pub fn decasteljau2<T>(coefficients: &[Vec<T>], u: f64, v: f64) -> T
where
    T: Copy
        + Clone
        + std::ops::Mul<f64, Output = T>
        + std::ops::Add<f64, Output = T>
        + std::ops::Add<T, Output = T>,
{
    let mut q = Vec::new();

    let degree_u = coefficients[0].len() - 1;
    let degree_v = coefficients.len() - 1;

    if degree_u <= degree_v {
        for j in 0..=degree_v {
            q.push(decasteljau(&coefficients[j], u));
        }
        decasteljau(&q, v)
    } else {
        for i in 0..=degree_u {
            q.push(decasteljau(
                &coefficients.iter().map(|row| row[i]).collect::<Vec<T>>(),
                v,
            ));
        }
        decasteljau(&q, u)
    }
}
