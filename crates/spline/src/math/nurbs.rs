use crate::{
    control_points::{ControlMesh, ControlPolygon},
    knots::KnotVector,
    surfaces::nurbs::SurfaceDirection,
};

use super::{b_spline::eval_basis_function, binomial_coefficient, Float, Homogeneous, Zero};

pub fn curve_point<H, C>(
    control_points: &ControlPolygon<H>,
    degree: usize,
    knot_vector: &KnotVector,
    u: Float,
) -> C
where
    H: Copy
        + Clone
        + Zero
        + std::ops::Mul<Float, Output = H>
        + std::ops::Add<Float, Output = H>
        + std::ops::Add<H, Output = H>
        + Homogeneous<C>,
    C: Copy + Clone + std::ops::Mul<Float, Output = C> + std::ops::Div<Float, Output = C>,
{
    let knot_span = knot_vector.find_span(degree, control_points.len(), u);
    let basis_values = eval_basis_function(degree, knot_span, knot_vector, u);

    let mut point = H::zero();
    for i in 0..=degree {
        let cp = control_points[knot_span - degree + i].to_weighted();
        point = point + cp * basis_values[i];
    }

    point.to_cartesian()
}

pub fn surface_point<H, C>(
    control_points: &ControlMesh<H>,
    degree_u: usize,
    degree_v: usize,
    knot_vector_u: &KnotVector,
    knot_vector_v: &KnotVector,
    u: Float,
    v: Float,
) -> C
where
    H: Copy
        + Clone
        + Zero
        + std::ops::Mul<Float, Output = H>
        + std::ops::Add<Float, Output = H>
        + std::ops::Add<H, Output = H>
        + Homogeneous<C>,
    C: Copy + Clone + std::ops::Mul<Float, Output = C> + std::ops::Div<Float, Output = C>,
{
    let knot_span_u = knot_vector_u.find_span(degree_u, control_points.len(), u);
    let knot_span_v = knot_vector_v.find_span(degree_v, control_points[0].len(), v);

    let basis_values_u = eval_basis_function(degree_u, knot_span_u, &knot_vector_u, u);
    let basis_values_v = eval_basis_function(degree_v, knot_span_v, &knot_vector_v, v);

    let mut temp = vec![H::zero(); degree_v + 1];

    for l in 0..=degree_v {
        temp[l] = H::zero();
        for k in 0..=degree_u {
            temp[l] = temp[l]
                + control_points[knot_span_u - degree_u + k][knot_span_v - degree_v + l]
                    .to_weighted()
                    * basis_values_u[k];
        }
    }

    let mut sw = H::zero();

    for l in 0..=degree_v {
        sw = sw + temp[l] * basis_values_v[l];
    }

    sw.to_cartesian()
}

pub fn curve_derivatives<H, C>(
    weighted_derivatives: &ControlPolygon<H>,
    num_derivatives: usize,
) -> Vec<C>
where
    H: Copy
        + Clone
        + Zero
        + std::ops::Mul<Float, Output = H>
        + std::ops::Add<Float, Output = H>
        + std::ops::Add<H, Output = H>
        + Homogeneous<C>,
    C: Copy
        + Clone
        + Zero
        + std::ops::Sub<Float, Output = C>
        + std::ops::Sub<C, Output = C>
        + std::ops::Mul<Float, Output = C>
        + std::ops::Div<Float, Output = C>,
{
    let mut derivatives = vec![C::zero(); num_derivatives + 1];

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

pub fn surface_derivatives<H, C>(
    weighted_derivatives: &ControlMesh<H>,
    num_derivatives: usize,
) -> Vec<Vec<C>>
where
    H: Copy
        + Clone
        + Zero
        + std::ops::Mul<Float, Output = H>
        + std::ops::Add<Float, Output = H>
        + std::ops::Add<H, Output = H>
        + Homogeneous<C>,
    C: Copy
        + Clone
        + Zero
        + std::ops::Sub<Float, Output = C>
        + std::ops::Sub<C, Output = C>
        + std::ops::Add<C, Output = C>
        + std::ops::Mul<Float, Output = C>
        + std::ops::Div<Float, Output = C>,
{
    let mut derivatives = vec![vec![C::zero(); num_derivatives + 1]; num_derivatives + 1];

    for k in 0..=num_derivatives {
        for l in 0..=(num_derivatives - k) {
            let mut v = weighted_derivatives[k][l].cartesian_components();
            for j in 1..=l {
                v = v - derivatives[k][l - j]
                    * binomial_coefficient(l, j)
                    * weighted_derivatives[0][j].homogeneous_component();
            }

            for i in 1..=k {
                v = v - derivatives[k - i][l]
                    * binomial_coefficient(k, i)
                    * weighted_derivatives[i][0].homogeneous_component();

                let mut v2 = C::zero();
                for j in 1..=l {
                    v2 = v2
                        + derivatives[k - i][l - j]
                            * binomial_coefficient(l, j)
                            * weighted_derivatives[i][j].homogeneous_component();
                }

                v = v - v2 * binomial_coefficient(k, i);
            }

            derivatives[k][l] = v / weighted_derivatives[0][0].homogeneous_component();
        }
    }

    derivatives
}

/// Inserts the specified number of knots into the curve at `u`, returning
/// a new curve in the form of `(<new knot vector>, <new control points>)`.
pub fn insert_curve_knots<T>(
    degree: usize,
    knot_vector: &KnotVector,
    num_insertions: usize,
    u: Float,
    weighted_control_points: &ControlPolygon<T>,
) -> (KnotVector, ControlPolygon<T>)
where
    T: Copy
        + Clone
        + Zero
        + std::ops::Mul<Float, Output = T>
        + std::ops::Add<Float, Output = T>
        + std::ops::Add<T, Output = T>,
{
    let knot_span_index = knot_vector.find_span(degree, weighted_control_points.len(), u);
    let knot_multiplicity = knot_vector.find_multiplicity(u);

    assert!(
        num_insertions + knot_multiplicity <= degree,
        "Maximum number of knots already exist at u = {}",
        u
    );

    let mut new_knot_vector = KnotVector::zeros(knot_vector.len() + num_insertions);
    let mut new_control_points =
        ControlPolygon::<T>::zeros(weighted_control_points.len() + num_insertions);

    let mut temp = vec![T::zero(); degree + 1];

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
        new_control_points[i] = weighted_control_points[i];
    }

    for i in (knot_span_index - knot_multiplicity)..weighted_control_points.len() {
        new_control_points[i + num_insertions] = weighted_control_points[i];
    }

    for i in 0..=(degree - knot_multiplicity) {
        temp[i] = weighted_control_points[knot_span_index - degree + i];
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

pub fn refine_curve<T>(
    degree: usize,
    knot_vector: &KnotVector,
    knots_to_insert: KnotVector,
    weighted_control_points: &ControlPolygon<T>,
) -> (KnotVector, ControlPolygon<T>)
where
    T: Copy
        + Clone
        + Zero
        + std::ops::Mul<Float, Output = T>
        + std::ops::Add<Float, Output = T>
        + std::ops::Add<T, Output = T>
        + std::fmt::Debug,
{
    let n = weighted_control_points.len() - 1;
    let r = knots_to_insert.len() - 1;

    let m = n + degree + 1;
    let a = knot_vector.find_span(degree, n, knots_to_insert.first());
    let b = knot_vector.find_span(degree, n, knots_to_insert.last()) + 1;

    // Initialize and fill new control points
    let mut new_ctrl_pts = ControlPolygon::zeros(n + r + 2);

    for j in 0..=(a - degree) {
        new_ctrl_pts[j] = weighted_control_points[j];
    }

    for j in (b - 1)..=n {
        new_ctrl_pts[j + r + 1] = weighted_control_points[j];
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
            new_ctrl_pts[k - degree - 1] = weighted_control_points[i - degree - 1];
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

/// Inserts the specified number of knots into the surface at `position`
/// (either a U or V coordinate, depending on `direction`)
/// in the `direction` UV dimension, returning a new surface in
/// the form of
/// `(<new U knot vector>, <new V knot vector>, <new control points>)`.
pub fn insert_surface_knots<T>(
    degree_u: usize,
    degree_v: usize,
    direction: SurfaceDirection,
    knot_vector_u: &KnotVector,
    knot_vector_v: &KnotVector,
    num_insertions: usize,
    position: Float,
    weighted_control_points: &ControlMesh<T>,
) -> (KnotVector, KnotVector, ControlMesh<T>)
where
    T: Copy
        + Clone
        + Zero
        + std::ops::Mul<Float, Output = T>
        + std::ops::Add<Float, Output = T>
        + std::ops::Add<T, Output = T>,
{
    match direction {
        SurfaceDirection::U => {
            let knot_span_index_u =
                knot_vector_u.find_span(degree_u, weighted_control_points.len(), position);
            let knot_multiplicity_u = knot_vector_u.find_multiplicity(position);

            assert!(
                num_insertions + knot_multiplicity_u <= degree_u,
                "Maximum number of knots already exist at u = {}",
                position
            );

            let mut new_knot_vector_u = KnotVector::zeros(knot_vector_u.len() + num_insertions);
            let new_knot_vector_v = knot_vector_v.clone();

            let mut new_control_points = ControlMesh::zeros(
                weighted_control_points.len() + num_insertions,
                weighted_control_points[0].len(),
            );

            let mut temp = vec![T::zero(); degree_u + 1];

            // Load new U knot vector
            for i in 0..=knot_span_index_u {
                new_knot_vector_u[i] = knot_vector_u[i];
            }

            for i in 1..=num_insertions {
                new_knot_vector_u[knot_span_index_u + i] = position;
            }

            for i in (knot_span_index_u + 1)..knot_vector_u.len() {
                new_knot_vector_u[i + num_insertions] = knot_vector_u[i];
            }

            // Save the alphas
            let mut alpha = vec![vec![0.0; num_insertions + 1]; degree_u - knot_multiplicity_u + 1];

            for j in 1..=num_insertions {
                let leg = knot_span_index_u - degree_u + j;
                for i in 0..=(degree_u - j - knot_multiplicity_u) {
                    alpha[i][j] = (position - knot_vector_u[leg + i])
                        / (knot_vector_u[i + knot_span_index_u + 1] - knot_vector_u[leg + i]);
                }
            }

            let u_len = weighted_control_points.len();
            let v_len = weighted_control_points[0].len();

            // For each row...
            for row in 0..v_len {
                // Save unaltered control points
                for i in 0..=(knot_span_index_u - degree_u) {
                    new_control_points[i][row] = weighted_control_points[i][row]
                }

                for i in (knot_span_index_u - knot_multiplicity_u - 1)..u_len {
                    new_control_points[i + num_insertions][row] = weighted_control_points[i][row];
                }

                // Load auxiliary control points
                for i in 0..=(degree_u - knot_multiplicity_u) {
                    temp[i] = weighted_control_points[knot_span_index_u - degree_u + i][row];
                }

                for j in 1..=num_insertions {
                    let leg = knot_span_index_u - degree_u + j;
                    for i in 0..=(degree_u - j - knot_multiplicity_u) {
                        temp[i] = temp[i + 1] * alpha[i][j] + temp[i] * (1.0 - alpha[i][j]);
                    }

                    new_control_points[leg][row] = temp[0];
                    new_control_points
                        [knot_span_index_u + num_insertions - j - knot_multiplicity_u][row] =
                        temp[degree_u - j - knot_multiplicity_u];
                }

                let l = knot_span_index_u - degree_u + num_insertions;
                for i in (l + 1)..(knot_span_index_u - knot_multiplicity_u) {
                    new_control_points[i][row] = temp[i - l];
                }
            }

            (new_knot_vector_u, new_knot_vector_v, new_control_points)
        }
        SurfaceDirection::V => {
            let knot_span_index_v =
                knot_vector_v.find_span(degree_v, weighted_control_points[0].len(), position);
            let knot_multiplicity_v = knot_vector_v.find_multiplicity(position);

            assert!(
                num_insertions + knot_multiplicity_v <= degree_v,
                "Maximum number of knots already exist at v = {}",
                position
            );

            let new_knot_vector_u = knot_vector_u.clone();
            let mut new_knot_vector_v = KnotVector::zeros(knot_vector_v.len() + num_insertions);

            let mut new_control_points = ControlMesh::zeros(
                weighted_control_points.len(),
                weighted_control_points[0].len() + num_insertions,
            );

            let mut temp = vec![T::zero(); degree_v + 1];

            // Load new V knot vector
            for i in 0..=knot_span_index_v {
                new_knot_vector_v[i] = knot_vector_v[i];
            }

            for i in 1..=num_insertions {
                new_knot_vector_v[knot_span_index_v + i] = position;
            }

            for i in (knot_span_index_v + 1)..knot_vector_v.len() {
                new_knot_vector_v[i + num_insertions] = knot_vector_v[i];
            }

            // Save the alphas
            let mut alpha = vec![vec![0.0; num_insertions + 1]; degree_v - knot_multiplicity_v + 1];

            for j in 1..=num_insertions {
                let leg = knot_span_index_v - degree_v + j;
                for i in 0..=(degree_v - j - knot_multiplicity_v) {
                    alpha[i][j] = (position - knot_vector_v[leg + i])
                        / (knot_vector_v[i + knot_span_index_v + 1] - knot_vector_v[leg + i]);
                }
            }

            let u_len = weighted_control_points.len();
            let v_len = weighted_control_points[0].len();

            // For each row...
            for col in 0..u_len {
                // Save unaltered control points
                for i in 0..=(knot_span_index_v - degree_v) {
                    new_control_points[col][i] = weighted_control_points[col][i]
                }

                for i in (knot_span_index_v - knot_multiplicity_v - 1)..v_len {
                    new_control_points[col][i + num_insertions] = weighted_control_points[col][i];
                }

                // Load auxiliary control points
                for i in 0..=(degree_v - knot_multiplicity_v) {
                    temp[i] = weighted_control_points[col][knot_span_index_v - degree_v + i];
                }

                for j in 1..=num_insertions {
                    let leg = knot_span_index_v - degree_v + j;
                    for i in 0..=(degree_v - j - knot_multiplicity_v) {
                        temp[i] = temp[i + 1] * alpha[i][j] + temp[i] * (1.0 - alpha[i][j]);
                    }

                    new_control_points[col][leg] = temp[0];
                    new_control_points[col]
                        [knot_span_index_v + num_insertions - j - knot_multiplicity_v] =
                        temp[degree_v - j - knot_multiplicity_v];
                }

                let l = knot_span_index_v - degree_v + num_insertions;
                for i in (l + 1)..(knot_span_index_v - knot_multiplicity_v) {
                    new_control_points[col][i] = temp[i - l];
                }
            }

            (new_knot_vector_u, new_knot_vector_v, new_control_points)
        }
    }
}

/*
pub fn insert_surface_knots<T>(
    degree_u: usize,
    degree_v: usize,
    direction: SurfaceDirection,
    knot_vector_u: &KnotVector,
    knot_vector_v: &KnotVector,
    num_insertions: usize,
    position: Float,
    weighted_control_points: &ControlMesh<T>,
) -> (KnotVector, KnotVector, ControlMesh<T>)
*/

/*
pub fn refine_surface<T>(
    degree_u: usize,
    degree_v: usize,
    knot_vector_u: &KnotVector,
    knot_vector_v: &KnotVector,
    direction: SurfaceDirection,
    knots_to_insert: KnotVector,
    weighted_control_points: &ControlPolygon<T>,
) -> (KnotVector, KnotVector, ControlPolygon<T>)
where
    T: Copy
        + Clone
        + Zero
        + std::ops::Mul<Float, Output = T>
        + std::ops::Add<Float, Output = T>
        + std::ops::Add<T, Output = T>
        + std::fmt::Debug,
{
    let n = weighted_control_points.len() - 1;
    let r = knots_to_insert.len() - 1;

    match direction {
        SurfaceDirection::U => {
            let a = knot_vector_u.find_span(degree_u, n, knots_to_insert.first());
            let b = knot_vector_u.find_span(degree_u, n, knots_to_insert.last()) + 1;

            // Initialize and fill new control points
            let mut new_ctrl_pts = ControlPolygon::zeros(n + r + 2);

            for j in 0..=(a - degree) {
                new_ctrl_pts[j] = weighted_control_points[j];
            }

            for j in (b - 1)..=n {
                new_ctrl_pts[j + r + 1] = weighted_control_points[j];
            }

            // Initialize and fill new knot vector
            let mut new_knot_vector_u = KnotVector::zeros(m + r + 2);

            for j in 0..=a {
                new_knot_vector_u[j] = knot_vector_u[j];
            }

            for j in (b + degree_u)..=m {
                new_knot_vector_u[j + r + 1] = knot_vector_u[j];
            }

            todo!()
        }
        SurfaceDirection::V => todo!(),
    }
}
*/
