use super::knot_vector::KnotVector;

/// Evaluates the basis functions at `u`
pub fn eval_basis_function(
    degree: usize,
    knot_span: usize,
    knot_vector: &KnotVector,
    u: f64,
) -> Vec<f64> {
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
    u: f64,
) -> Vec<Vec<f64>> {
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
    u: f64,
) -> Vec<Vec<f64>> {
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

    let mut derivatives: Vec<Vec<f64>> =
        vec![vec![0.0; degree + 1]; usize::min(degree, num_derivatives) + 1];

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
    let mut r = degree as f64;
    for k in 1..=num_derivatives {
        for j in 0..=degree {
            derivatives[k][j] *= r;
        }
        r *= degree as f64 - k as f64;
    }

    derivatives
}

/// Evaluates a basis function for a single parameter at `u`.
pub fn eval_single_basis_function(
    degree: usize,
    knot_span_index: usize,
    knot_vector: &KnotVector,
    u: f64,
) -> f64 {
    if knot_span_index == 0 && u == knot_vector[0] {
        return 1.0;
    }

    if u < knot_vector[knot_span_index] || u >= knot_vector[knot_span_index + degree + 1] {
        return 0.0;
    }

    // TODO: initialize this
    let mut table = Vec::<f64>::new();

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
    u: f64,
) -> Vec<f64> {
    // TODO: Initialize this
    let mut derivatives = Vec::<f64>::new();

    if u < knot_vector[knot_span_index] || u >= knot_vector[knot_span_index + degree + 1] {
        for k in 0..=num_derivatives {
            derivatives[k] = 0.0;
        }
        return derivatives;
    }

    // TODO: Initialize this
    let mut table = Vec::<Vec<f64>>::new();

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
                saved = (knot_span_index as f64 - knot_left) * temp;
            }
        }
    }

    derivatives[0] = table[0][degree]; // The function value
    for k in 1..=num_derivatives {
        // compute the derivatives
        // TODO: Initialize this
        let mut column = Vec::<f64>::new();
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
                    column[j] = (degree - k + jj) as f64 * saved;
                    saved = 0.0;
                } else {
                    let temp = column[j + 1] / (knot_right - knot_left);
                    column[j] = (degree - k + jj) as f64 * (saved - temp);
                    saved = temp;
                }
            }
        }

        derivatives[k] = column[0]; // kth derivative
    }

    derivatives
}
