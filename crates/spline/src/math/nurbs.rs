use super::{
    basis::eval_basis_function, binomial_coefficient, knot_vector::KnotVector, HPoint, Point,
    WPoint,
};

pub fn curve_point(
    control_points: &[HPoint],
    degree: usize,
    knot_vector: &KnotVector,
    u: f64,
) -> Point {
    let knot_span = knot_vector.find_span(degree, control_points.len(), u);
    let basis_values = eval_basis_function(degree, knot_span, knot_vector, u);

    (0..=degree)
        .map(|i| control_points[knot_span + i - degree].to_weighted() * basis_values[i])
        .sum::<WPoint>()
        .to_hpoint()
        .project()
}

pub fn curve_derivatives(weighted_derivatives: &[HPoint], num_derivatives: usize) -> Vec<Point> {
    let mut derivatives = vec![
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        num_derivatives + 1
    ];

    for k in 0..=num_derivatives {
        let mut v = weighted_derivatives[k].cartesian();
        for i in 1..=k {
            v = v - derivatives[k - i] * binomial_coefficient(k, i) * weighted_derivatives[i].h;
        }
        derivatives[k] = v / weighted_derivatives[0].h;
    }

    derivatives
}
