use super::{
    basis::eval_basis_function, binomial_coefficient, knot_vector::KnotVector, Homogeneous, Vector,
};

pub fn curve_point<H: Homogeneous>(
    control_points: &[H],
    degree: usize,
    knot_vector: &KnotVector,
    u: f64,
) -> H::Projected {
    let knot_span = knot_vector.find_span(degree, control_points.len(), u);
    let basis_values = eval_basis_function(degree, knot_span, knot_vector, u);

    let weighted = (0..=degree)
        .map(|i| control_points[knot_span + i - degree].weight() * basis_values[i])
        .sum::<H::Weighted>();

    H::cast_from_weighted(weighted).project()
}

pub fn curve_derivatives<H: Homogeneous>(
    weighted_derivatives: &[H],
    num_derivatives: usize,
) -> Vec<H::Projected> {
    let mut derivatives = vec![H::Projected::zero(); num_derivatives + 1];

    for k in 0..=num_derivatives {
        let mut v = weighted_derivatives[k].cartesian_components();
        for i in 1..=k {
            v = v - derivatives[k - i]
                * binomial_coefficient(k, i)
                * weighted_derivatives[i].homogeneous_component();
        }
        derivatives[k] = v / weighted_derivatives[0].homogeneous_component();
    }

    derivatives
}
