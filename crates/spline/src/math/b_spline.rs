use crate::{
    control_points::{ControlMesh, ControlPolygon},
    knots::KnotVector,
};

use super::{Float, Zero};

/// Evaluates the basis functions at `u`
pub fn eval_basis_function(
    degree: usize,
    knot_span: usize,
    knot_vector: &KnotVector,
    u: Float,
) -> Vec<Float> {
    // The additional element here (the first one) won't be used.
    // It's just needed to make the indexing in the loop work.
    let mut left = vec![0.0; degree + 1];
    let mut right = vec![0.0; degree + 1];

    let mut result = vec![1.0; degree + 1];

    for j in 1..=degree {
        left[j] = u - knot_vector[knot_span + 1 - j];
        right[j] = knot_vector[knot_span + j] - u;
        let mut saved = 0.0;

        for r in 0..j {
            let temp = result[r] / (right[r + 1] + left[j - r]);
            result[r] = saved + right[r + 1] * temp;
            saved = left[j - r] * temp;
        }

        result[j] = saved;
    }

    result
}

pub fn eval_all_basis_functions(
    degree: usize,
    knot_span: usize,
    knot_vector: &KnotVector,
    u: Float,
) -> Vec<Vec<Float>> {
    let mut result = vec![vec![0.0; degree + 1]; degree + 1];

    for i in 0..=degree {
        let basis_functions = eval_basis_function(i, knot_span, knot_vector, u);
        for j in 0..=i {
            result[j][i] = basis_functions[j];
        }
    }

    result
}

/// Evaluates the basis functions and their derivatives up to `num_derivatives` at `u`.
pub fn eval_basis_function_derivatives(
    degree: usize, // degree
    knot_span_index: usize,
    knot_vector: &KnotVector,
    num_derivatives: usize,
    u: Float,
) -> Vec<Vec<Float>> {
    let mut left = vec![1.0; degree + 1];
    let mut right = vec![1.0; degree + 1];
    let mut ndu = vec![vec![1.0; degree + 1]; degree + 1];

    for j in 1..=degree {
        left[j] = u - knot_vector[knot_span_index + 1 - j];
        right[j] = knot_vector[knot_span_index + j] - u;
        let mut saved = 0.0;

        for r in 0..j {
            // Lower triangle
            ndu[j][r] = right[r + 1] + left[j - r];
            let temp = ndu[r][j - 1] / ndu[j][r];

            // Upper triangle
            ndu[r][j] = saved + right[r + 1] * temp;
            saved = left[j - r] * temp;
        }
        ndu[j][j] = saved;
    }

    let mut derivatives: Vec<Vec<Float>> =
        vec![vec![0.0; degree + 1]; usize::min(degree, num_derivatives) + 1];

    // Load the basis functions
    for j in 0..=degree {
        derivatives[0][j] = ndu[j][degree];
    }

    // Begin calculating derivatives
    let mut a: Vec<Vec<Float>> = vec![vec![1.0; degree + 1]; 2];

    // This section computes the derivatives.
    // Loop over the function index
    for r in 0..=degree {
        // Alternate rows in array a
        let mut s1 = 0;
        let mut s2 = 1;

        a[0][0] = 1.0;

        // Loop to compute kth derivative
        for k in 1..=num_derivatives {
            let mut d = 0.0;

            let rk = r as i32 - k as i32;
            let pk = degree as i32 - k as i32;

            if r >= k {
                a[s2][0] = a[s1][0] / ndu[(pk + 1) as usize][rk as usize];
                d = a[s2][0] * ndu[rk as usize][pk as usize];
            }

            let j1 = if rk >= -1 { 1 } else { -rk } as usize;

            let j2 = if r as i32 - 1 <= pk {
                k - 1
            } else {
                degree - r
            };

            for j in j1..=j2 {
                a[s2][j] =
                    (a[s1][j] - a[s1][j - 1]) / ndu[(pk + 1) as usize][(rk + j as i32) as usize];
                d += a[s2][j] * ndu[(rk + j as i32) as usize][pk as usize];
            }

            if r <= pk as usize {
                a[s2][k] = -a[s1][k - 1] / ndu[((pk + 1) as usize)][r];
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
    let mut r = degree as Float;
    for k in 1..=num_derivatives {
        for j in 0..=degree {
            derivatives[k][j] *= r;
        }
        r *= degree as Float - k as Float;
    }

    derivatives
}

/// Evaluates a basis function for a single parameter at `u`.
pub fn eval_single_basis_function(
    degree: usize,
    knot_span_index: usize,
    knot_vector: &KnotVector,
    u: Float,
) -> Float {
    if knot_span_index == 0 && u == knot_vector[0] {
        return 1.0;
    }

    if u < knot_vector[knot_span_index] || u >= knot_vector[knot_span_index + degree + 1] {
        return 0.0;
    }

    // TODO: initialize this
    let mut table = Vec::<Float>::new();

    // Initialize 0th-degree funcs
    for j in 0..=degree {
        if u >= knot_vector[knot_span_index + j] && u < knot_vector[knot_span_index + j + 1] {
            table[j] = 1.0;
        } else {
            table[j] = 0.0;
        }
    }

    // compute triangular table
    for k in 1..=degree {
        let mut saved = if table[0] == 0.0 {
            0.0
        } else {
            ((u - knot_vector[knot_span_index]) * table[0])
                / (knot_vector[knot_span_index + k] - knot_vector[knot_span_index])
        };

        for j in 0..(degree - k + 1) {
            let knot_left = knot_vector[knot_span_index + j + 1];
            let knot_right = knot_vector[knot_span_index + j + k + 1];
            if table[j + 1] == 0.0 {
                table[j] = saved;
                saved = 0.0;
            } else {
                let temp = table[j + 1] / (knot_right - knot_left);
                table[j] = saved + (knot_left - u) * temp;
                saved = (u - knot_left) * temp;
            }
        }
    }

    table[0]
}

/// Evaluates a basis function and its derivatives up to `num_derivatives` for a single parameter at `u`.
pub fn eval_single_basis_function_derivatives(
    degree: usize,
    knot_span_index: usize,
    knot_vector: &KnotVector,
    num_derivatives: usize,
    u: Float,
) -> Vec<Float> {
    // TODO: Initialize this
    let mut derivatives = Vec::<Float>::new();

    if u < knot_vector[knot_span_index] || u >= knot_vector[knot_span_index + degree + 1] {
        for k in 0..=num_derivatives {
            derivatives[k] = 0.0;
        }
        return derivatives;
    }

    // TODO: Initialize this
    let mut table = Vec::<Vec<Float>>::new();

    // Initialize 0th-degree functions
    for j in 0..=degree {
        if u >= knot_vector[knot_span_index + j] && u < knot_vector[knot_span_index + j + 1] {
            table[j][0] = 1.0;
        } else {
            table[j][0] = 0.0;
        }
    }

    // Compute full triangular table
    for k in 1..=degree {
        let mut saved = if table[0][k - 1] == 0.0 {
            0.0
        } else {
            ((u - knot_vector[knot_span_index]) * table[0][k - 1])
                / (knot_vector[knot_span_index + k] - knot_vector[knot_span_index])
        };

        for j in 0..(degree - k + 1) {
            let knot_left = knot_vector[knot_span_index + j + 1];
            let knot_right = knot_vector[knot_span_index + j + k + 1];

            if table[j + 1][k - 1] == 0.0 {
                table[j][k] = saved;
                saved = 0.0;
            } else {
                let temp = table[j + 1][k - 1] / (knot_right - knot_left);
                table[j][k] = saved + (knot_right - u) * temp;
                saved = (knot_span_index as Float - knot_left) * temp;
            }
        }
    }

    derivatives[0] = table[0][degree]; // The function value
    for k in 1..=num_derivatives {
        // compute the derivatives
        // TODO: Initialize this
        let mut column = Vec::<Float>::new();
        for j in 0..=k {
            // Load appropriate column
            column[j] = table[j][degree - k];
        }

        for jj in 1..=k {
            // Compute table of width k
            let mut saved = if column[0] == 0.0 {
                0.0
            } else {
                column[0]
                    / (knot_vector[knot_span_index + degree - k + jj]
                        - knot_vector[knot_span_index])
            };

            for j in 0..(k - jj + 1) {
                let knot_left = knot_vector[knot_span_index + j + 1];
                let knot_right = knot_vector[knot_span_index + j + degree + jj + 1];
                if column[j + 1] == 0.0 {
                    column[j] = (degree - k + jj) as Float * saved;
                    saved = 0.0;
                } else {
                    let temp = column[j + 1] / (knot_right - knot_left);
                    column[j] = (degree - k + jj) as Float * (saved - temp);
                    saved = temp;
                }
            }
        }

        derivatives[k] = column[0]; // kth derivative
    }

    derivatives
}

/// Evaluates a B-Spline curve at `u`.
pub fn curve_point<T>(control_points: &[T], degree: usize, knot_vector: &KnotVector, u: Float) -> T
where
    T: Copy
        + Clone
        + Zero
        + std::ops::Mul<Float, Output = T>
        + std::ops::Add<Float, Output = T>
        + std::ops::Add<T, Output = T>,
{
    let knot_span = knot_vector.find_span(degree, control_points.len(), u);
    let basis_values = eval_basis_function(degree, knot_span, knot_vector, u);

    let mut point = T::zero();
    for i in 0..=degree {
        point = point + control_points[knot_span - degree + i] * basis_values[i];
    }

    point
}

/// Evaluates a B-Spline curve and its derivatives up to `num_derivatives` at `u`.
/// Directly evaluates the derivatives instead of creating separate curves for each
/// derivative and evaluating them.
///
/// Appears to be slower than `curve_derivatives_2`.
pub fn curve_derivatives_1<T>(
    control_points: &[T],
    degree: usize,
    knot_vector: &KnotVector,
    num_derivatives: usize,
    u: Float,
) -> Vec<T>
where
    T: Copy
        + Clone
        + Zero
        + std::ops::Mul<Float, Output = T>
        + std::ops::Add<Float, Output = T>
        + std::ops::Add<T, Output = T>,
{
    let du = usize::min(num_derivatives, degree);
    let mut derivatives = vec![T::zero(); du + 1];

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

/// Evaluates a B-Spline curve and its derivatives up to `num_derivatives` at `u`.
/// Creates a new derivative curve for each derivative and evaluates it at `u`.
///
/// Appears to be faster than `curve_derivatives_1`.
pub fn curve_derivatives_2<T>(
    control_points: &ControlPolygon<T>,
    degree: usize,
    knot_vector: &KnotVector,
    num_derivatives: usize,
    u: Float,
) -> ControlPolygon<T>
where
    T: Copy
        + Clone
        + Zero
        + std::ops::Mul<Float, Output = T>
        + std::ops::Add<Float, Output = T>
        + std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>
        + std::ops::Div<Float, Output = T>,
{
    let du = usize::min(num_derivatives, degree);
    let mut derivatives = ControlPolygon::zeros(du + 1);

    for k in (degree + 1)..=num_derivatives {
        derivatives[k] = T::zero();
    }

    let knot_span = knot_vector.find_span(degree, control_points.len(), u);
    let basis_functions = eval_all_basis_functions(degree, knot_span, knot_vector, u);
    let derivative_control_points = curve_derivative_control_points(
        control_points,
        degree,
        knot_vector,
        knot_span - degree,
        knot_span,
        num_derivatives,
    );

    for k in 0..=du {
        derivatives[k] = T::zero();
        for j in 0..=(degree - k) {
            derivatives[k] =
                derivatives[k] + (derivative_control_points[k][j] * basis_functions[j][degree - k]);
        }
    }

    derivatives
}

/// Creates derivative curves up to `num_derivatives` for a B-Spline curve.
pub fn curve_derivative_control_points<T>(
    control_points: &ControlPolygon<T>,
    degree: usize,
    knot_vector: &KnotVector,
    min_control_point: usize,
    max_control_point: usize,
    num_derivatives: usize,
) -> Vec<ControlPolygon<T>>
where
    T: Copy
        + Clone
        + Zero
        + std::ops::Mul<Float, Output = T>
        + std::ops::Add<Float, Output = T>
        + std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>
        + std::ops::Div<Float, Output = T>,
{
    let r = max_control_point - min_control_point;

    let mut points = vec![ControlPolygon::zeros(r + 1); num_derivatives + 1];

    for i in 0..=r {
        points[0][i] = control_points[min_control_point + i];
    }

    for k in 1..=num_derivatives {
        let tmp = (degree - k + 1) as Float;
        for i in 0..=(r - k) {
            points[k][i] = ((points[k - 1][i + 1] - points[k - 1][i])
                / (knot_vector[min_control_point + i + degree + 1]
                    - knot_vector[min_control_point + i + k]))
                * tmp;
        }
    }

    points
}

pub fn surface_point<T>(
    control_points: &[Vec<T>],
    degree_u: usize,
    degree_v: usize,
    knot_vector_u: &KnotVector,
    knot_vector_v: &KnotVector,
    u: Float,
    v: Float,
) -> T
where
    T: Copy
        + Clone
        + Zero
        + std::ops::Mul<Float, Output = T>
        + std::ops::Add<Float, Output = T>
        + std::ops::Add<T, Output = T>,
{
    let num_points_u = control_points.len();
    let num_points_v = control_points[0].len();

    let knot_span_u = knot_vector_u.find_span(degree_u, num_points_u, u);
    let knot_span_v = knot_vector_v.find_span(degree_v, num_points_v, v);

    let basis_values_u = eval_basis_function(degree_u, knot_span_u, knot_vector_u, u);
    let basis_values_v = eval_basis_function(degree_v, knot_span_v, knot_vector_v, v);

    let index_u = knot_span_u - degree_u;

    let mut point = T::zero();

    for l in 0..=degree_v {
        let mut temp = T::zero();
        let index_v = knot_span_v - degree_v + l;
        for k in 0..=degree_u {
            temp = temp + control_points[index_u + k][index_v] * basis_values_u[k];
        }
        point = point + temp * basis_values_v[l];
    }

    point
}

/// Evaluates a B-Spline surface and its derivatives up to `num_derivatives` at (`u`, `v`).
/// Directly evaluates the derivatives instead of creating separate surfaces for each
/// derivative and evaluating them.
///
/// Appears to be faster than `surface_derivatives_2`.
pub fn surface_derivatives_1<T>(
    control_points: &[Vec<T>],
    degree_u: usize,
    degree_v: usize,
    knot_vector_u: &KnotVector,
    knot_vector_v: &KnotVector,
    num_derivatives: usize,
    u: Float,
    v: Float,
) -> Vec<Vec<T>>
where
    T: Copy
        + Clone
        + Zero
        + std::ops::Mul<Float, Output = T>
        + std::ops::Add<Float, Output = T>
        + std::ops::Add<T, Output = T>,
{
    let num_points_u = control_points.len();
    let num_points_v = control_points[0].len();

    let du = usize::min(num_derivatives, degree_u);
    let dv = usize::min(num_derivatives, degree_v);
    let mut derivatives = vec![vec![T::zero(); dv + 1]; du + 1];

    for k in (degree_u + 1)..=num_derivatives {
        for l in 0..=(num_derivatives - k) {
            derivatives[k][l] = T::zero();
        }
    }

    for l in (degree_v + 1)..=num_derivatives {
        for k in 0..=(num_derivatives - l) {
            derivatives[k][l] = T::zero();
        }
    }

    let knot_span_u = knot_vector_u.find_span(degree_u, num_points_u, u);
    let basis_derivative_values_u =
        eval_basis_function_derivatives(degree_u, knot_span_u, knot_vector_u, num_derivatives, u);

    let knot_span_v = knot_vector_v.find_span(degree_v, num_points_v, v);
    let basis_derivative_values_v =
        eval_basis_function_derivatives(degree_v, knot_span_v, knot_vector_v, num_derivatives, v);

    let mut temp = vec![T::zero(); degree_v + 1];

    for k in 0..=du {
        for s in 0..=degree_v {
            temp[s] = T::zero();
            for r in 0..=degree_u {
                temp[s] = temp[s]
                    + control_points[knot_span_u - degree_u + r][knot_span_v - degree_v + s]
                        * basis_derivative_values_u[k][r];
            }
        }
        let dd = usize::min(num_derivatives - k, dv);
        for l in 0..=dd {
            derivatives[k][l] = T::zero();
            for s in 0..=degree_v {
                derivatives[k][l] = derivatives[k][l] + temp[s] * basis_derivative_values_v[l][s];
            }
        }
    }

    derivatives
}

/// Evaluates a B-Spline surface and its derivatives up to `num_derivatives` at (`u`, `v`).
/// Creates a new derivative curve for each derivative and evaluates it at (`u`, `v`).
///
/// Appears to be slower than `surface_derivatives_1`.
pub fn surface_derivatives_2<T>(
    control_points: &ControlMesh<T>,
    degree_u: usize,
    degree_v: usize,
    knot_vector_u: &KnotVector,
    knot_vector_v: &KnotVector,
    num_derivatives: usize,
    u: Float,
    v: Float,
) -> ControlMesh<T>
where
    T: Copy
        + Clone
        + Zero
        + std::ops::Mul<Float, Output = T>
        + std::ops::Add<Float, Output = T>
        + std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>
        + std::ops::Div<Float, Output = T>,
{
    let du = usize::min(num_derivatives, degree_u);
    let dv = usize::min(num_derivatives, degree_v);

    let mut derivatives = ControlMesh::zeros(dv + 1, du + 1);

    let knot_span_u = knot_vector_u.find_span(degree_u, control_points.len(), u);
    let knot_span_v = knot_vector_v.find_span(degree_v, control_points[0].len(), v);

    let basis_functions_u = eval_all_basis_functions(degree_u, knot_span_u, knot_vector_u, u);
    let basis_functions_v = eval_all_basis_functions(degree_v, knot_span_v, knot_vector_v, v);

    let derivative_control_points = surface_derivative_control_points(
        control_points,
        degree_u,
        degree_v,
        knot_vector_u,
        knot_vector_v,
        knot_span_u - degree_u,
        knot_span_u,
        knot_span_v - degree_v,
        knot_span_v,
        num_derivatives,
    );

    for k in 0..=du {
        let dd = usize::min(num_derivatives - k, dv);

        for l in 0..=dd {
            derivatives[k][l] = T::zero();
            for i in 0..=(degree_v - l) {
                let mut tmp = T::zero();
                for j in 0..=(degree_u - k) {
                    tmp = tmp
                        + derivative_control_points[k][l][j][i]
                            * basis_functions_u[j][degree_u - k];
                }

                derivatives[k][l] = derivatives[k][l] + tmp * basis_functions_v[i][degree_v - l];
            }
        }
    }

    derivatives
}

pub fn surface_derivative_control_points<T>(
    control_points: &ControlMesh<T>,
    degree_u: usize,
    degree_v: usize,
    knot_vector_u: &KnotVector,
    knot_vector_v: &KnotVector,
    min_control_point_u: usize,
    max_control_point_u: usize,
    min_control_point_v: usize,
    max_control_point_v: usize,
    num_derivatives: usize,
) -> Vec<Vec<ControlMesh<T>>>
where
    T: Copy
        + Clone
        + Zero
        + std::ops::Mul<Float, Output = T>
        + std::ops::Add<Float, Output = T>
        + std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>
        + std::ops::Div<Float, Output = T>,
{
    let du = usize::min(num_derivatives, degree_u);
    let dv = usize::min(num_derivatives, degree_v);
    let r = max_control_point_u - min_control_point_u;
    let s = max_control_point_v - min_control_point_v;

    let mut points = vec![vec![ControlMesh::zeros(r + 1, s + 1); dv + 1]; du + 1];

    for j in min_control_point_v..=max_control_point_v {
        let temp = curve_derivative_control_points(
            &ControlPolygon::from_iter(control_points.iter().map(|row| row[j]).collect::<Vec<T>>()),
            degree_u,
            knot_vector_u,
            min_control_point_u,
            max_control_point_u,
            num_derivatives,
        );

        for k in 0..=du {
            for i in 0..=(r - k) {
                points[k][0][i][j - min_control_point_v] = temp[k][i];
            }
        }
    }

    for k in 0..du {
        for i in 0..(r - k) {
            let dd = usize::min(num_derivatives - k, dv);
            let temp = curve_derivative_control_points(
                &control_points[k],
                degree_v,
                knot_vector_v,
                min_control_point_v,
                max_control_point_v,
                num_derivatives,
            );

            for l in 1..=dd {
                for j in 0..=(s - l) {
                    points[k][l][i][j] = temp[l][j];
                }
            }
        }
    }

    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_basis_polynomials() {
        let degree = 2;
        let knot_vector = KnotVector::new([0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 5.0, 5.0, 5.0]);
        let span = 4;
        let u = 5.0 / 2.0;

        let result = eval_basis_function(degree, span, &knot_vector, u);

        assert_eq!(vec![0.125, 0.75, 0.125], result);
    }
}
