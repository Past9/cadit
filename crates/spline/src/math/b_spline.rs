use super::{
    basis::{eval_all_basis_functions, eval_basis_function, eval_basis_function_derivatives},
    knot_vector::KnotVector,
    Vector,
};

pub fn curve_point<C: Vector>(
    control_points: &[C],
    degree: usize,
    knot_vector: &KnotVector,
    u: f64,
) -> C {
    let knot_span = knot_vector.find_span(degree, control_points.len(), u);
    let basis_values = eval_basis_function(degree, knot_span, knot_vector, u);

    let mut point = C::zero();
    for i in 0..=degree {
        point = point + control_points[knot_span - degree + i] * basis_values[i];
    }

    point
}

pub fn curve_derivatives_1<C: Vector>(
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

pub fn curve_derivatives_2<C: Vector>(
    control_points: &[C],
    degree: usize,
    knot_vector: &KnotVector,
    num_derivatives: usize,
    u: f64,
) -> Vec<C> {
    let du = usize::min(num_derivatives, degree);
    let mut derivatives = vec![C::zero(); du + 1];

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
        for j in 0..=(degree - k) {
            derivatives[k] =
                derivatives[k] + (derivative_control_points[k][j] * basis_functions[j][degree - k]);
        }
    }

    derivatives
}

pub fn curve_derivative_control_points<C: Vector>(
    control_points: &[C],
    degree: usize,
    knot_vector: &KnotVector,
    min_control_point: usize,
    max_control_point: usize,
    num_derivatives: usize,
) -> Vec<Vec<C>> {
    let r = max_control_point - min_control_point;

    let mut points = vec![vec![C::zero(); r + 1]; num_derivatives + 1];

    for i in 0..=r {
        points[0][i] = control_points[min_control_point + i];
    }

    for k in 1..=num_derivatives {
        let tmp = (degree - k + 1) as f64;
        for i in 0..=(r - k) {
            points[k][i] = ((points[k - 1][i + 1] - points[k - 1][i])
                / (knot_vector[min_control_point + i + degree + 1]
                    - knot_vector[min_control_point + i + k]))
                * tmp;
        }
    }

    points
}
