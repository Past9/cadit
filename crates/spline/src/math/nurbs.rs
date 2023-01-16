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

pub fn curve_insert_knots<C: Vector>(
    control_points: &[C],
    degree: usize,
    knot_vector: &KnotVector,
    u: f64,
    num_insertions: usize,
) -> (KnotVector, Vec<C>) {
    let knot_span_index = knot_vector.find_span(degree, control_points.len(), u);
    let knot_multiplicity = knot_vector.find_multiplicity(u);

    assert!(
        num_insertions + knot_multiplicity <= degree,
        "Maximum number of knots already exist at u = {}",
        u
    );

    let mut new_knot_vector = KnotVector::zeros(knot_vector.len() + num_insertions);
    let mut new_control_points = vec![C::zero(); control_points.len() + num_insertions];

    let mut temp = vec![C::zero(); degree + 1];

    // Load new knot vector
    for i in 0..=knot_span_index {
        new_knot_vector[i] = knot_vector[i];
    }

    for i in 1..=num_insertions {
        new_knot_vector[knot_span_index + i] = u;
    }

    for i in (knot_span_index + 1)..knot_vector.len() {
        new_knot_vector[i + num_insertions] = knot_vector[i];
    }

    // Save unaltered control points
    for i in 0..=(knot_span_index - degree) {
        new_control_points[i] = control_points[i];
    }

    for i in (knot_span_index - knot_multiplicity)..control_points.len() {
        new_control_points[i + num_insertions] = control_points[i];
    }

    for i in 0..=(degree - knot_multiplicity) {
        temp[i] = control_points[knot_span_index - degree + i];
    }

    // Insert the knot `num_new_points` times
    for j in 1..=num_insertions {
        let leg = knot_span_index - degree + j;
        for i in 0..=(degree - j - knot_multiplicity) {
            let alpha = (u - knot_vector[leg + i])
                / (knot_vector[i + knot_span_index + 1] - knot_vector[leg + i]);
            temp[i] = temp[i + 1] * alpha + temp[i] * (1.0 - alpha);
        }

        new_control_points[leg] = temp[0];
        new_control_points[knot_span_index + num_insertions - j - knot_multiplicity] =
            temp[degree - j - knot_multiplicity];
    }

    // Load remaining control points
    let l = knot_span_index - degree + num_insertions;
    for i in (l + 1)..(knot_span_index - knot_multiplicity) {
        new_control_points[i] = temp[i - l];
    }

    (new_knot_vector, new_control_points)
}

pub fn curve_refine<C: Vector>(
    control_points: &[C],
    degree: usize,
    knot_vector: &KnotVector,
    knots_to_insert: &[f64],
) -> (KnotVector, Vec<C>) {
    let n = control_points.len() - 1;
    let r = knots_to_insert.len() - 1;

    let m = n + degree + 1;
    let a = knot_vector.find_span(degree, n, knots_to_insert[0]);
    let b = knot_vector.find_span(degree, n, knots_to_insert[knots_to_insert.len() - 1]) + 1;

    // Initialize and fill new control points
    let mut new_ctrl_pts = vec![C::zero(); n + r + 2];

    for j in 0..=(a - degree) {
        new_ctrl_pts[j] = control_points[j];
    }

    for j in (b - 1)..=n {
        new_ctrl_pts[j + r + 1] = control_points[j];
    }

    // Initialize and fill new knot vector
    let mut new_knot_vector = KnotVector::zeros(m + r + 2);

    for j in 0..=a {
        new_knot_vector[j] = knot_vector[j];
    }

    for j in (b + degree)..=m {
        new_knot_vector[j + r + 1] = knot_vector[j];
    }

    let mut i = b + degree - 1;
    let mut k = b + degree + r;

    for j in (0..=r).rev() {
        while knots_to_insert[j] <= knot_vector[i] && i > a {
            new_ctrl_pts[k - degree - 1] = control_points[i - degree - 1];
            new_knot_vector[k] = knot_vector[i];
            k -= 1;
            i -= 1;
        }

        new_ctrl_pts[k - degree - 1] = new_ctrl_pts[k - degree];

        for l in 1..=degree {
            let ind = k - degree + l;
            let mut alfa = new_knot_vector[k + l] - knots_to_insert[j];

            if alfa == 0.0 {
                new_ctrl_pts[ind - 1] = new_ctrl_pts[ind];
            } else {
                alfa = alfa / (new_knot_vector[k + l] - knot_vector[i - degree + l]);
                new_ctrl_pts[ind - 1] =
                    new_ctrl_pts[ind - 1] * alfa + new_ctrl_pts[ind] * (1.0 - alfa);
            }
        }

        new_knot_vector[k] = knots_to_insert[j];
        k = k - 1;
    }

    (new_knot_vector, new_ctrl_pts)
}

pub fn curve_decompose<C: Vector>(
    control_points: &[C],     // control_points
    degree: usize,            // degree
    knot_vector: &KnotVector, // knot_vector
) -> Vec<Vec<C>> {
    let n = control_points.len() - 1;
    let m = n + degree + 1;
    let mut a = degree;
    let mut b = degree + 1;
    let mut nb = 0;

    let new_bezier_points = vec![C::zero(); degree + 1];
    let mut bezier_ctrl_pts: Vec<Vec<C>> = Vec::new();

    bezier_ctrl_pts.push(new_bezier_points.clone());

    for i in 0..=degree {
        bezier_ctrl_pts[nb][i] = control_points[i];
    }

    while b < m {
        let i = b;
        while b < m && knot_vector[b + 1] == knot_vector[b] {
            b += 1;
        }

        let mult = b - i + 1;
        if mult < degree {
            let numer = knot_vector[b] - knot_vector[a];
            let mut alphas = vec![0.0; degree - mult];
            for j in ((mult + 1)..=degree).rev() {
                alphas[j - mult - 1] = numer / (knot_vector[a + j] - knot_vector[a]);
            }

            let r = degree - mult;
            for j in 1..=r {
                let save = r - j;
                let s = mult + j;
                for k in (s..=degree).rev() {
                    let alpha = alphas[k - s];
                    bezier_ctrl_pts[nb][k] =
                        bezier_ctrl_pts[nb][k] * alpha + bezier_ctrl_pts[nb][k - 1] * (1.0 - alpha);
                }

                if b < m {
                    if bezier_ctrl_pts.len() - 1 < nb + 1 {
                        bezier_ctrl_pts.push(new_bezier_points.clone());
                    }
                    bezier_ctrl_pts[nb + 1][save] = bezier_ctrl_pts[nb][degree];
                }
            }
        }

        nb += 1;

        if b < m {
            for i in (degree - mult)..=degree {
                if bezier_ctrl_pts.len() - 1 < nb {
                    bezier_ctrl_pts.push(new_bezier_points.clone());
                }
                bezier_ctrl_pts[nb][i] = control_points[b - degree + i];
            }
            a = b;
            b += 1;
        }
    }

    bezier_ctrl_pts
}
