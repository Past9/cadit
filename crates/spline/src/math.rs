use cgmath::{vec2, vec3, vec4, Vector2, Vector3, Vector4};

pub mod b_spline;
pub mod bezier;
pub mod nurbs;

const BINOMIAL_COEFFICIENTS: [[f64; 10]; 10] = [
    [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [1.0, 2.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [1.0, 3.0, 3.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [1.0, 4.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [1.0, 5.0, 10.0, 10.0, 5.0, 1.0, 0.0, 0.0, 0.0, 0.0],
    [1.0, 6.0, 15.0, 20.0, 15.0, 6.0, 1.0, 0.0, 0.0, 0.0],
    [1.0, 7.0, 21.0, 35.0, 35.0, 21.0, 7.0, 1.0, 0.0, 0.0],
    [1.0, 8.0, 28.0, 56.0, 70.0, 56.0, 28.0, 8.0, 1.0, 0.0],
    [1.0, 9.0, 36.0, 84.0, 126.0, 126.0, 84.0, 36.0, 9.0, 1.0],
];

pub fn binomial_coefficient(k: usize, i: usize) -> f64 {
    //factorial(k) / (factorial(i) * factorial(k - i))
    BINOMIAL_COEFFICIENTS[k][i]
}

pub trait Zero {
    fn zero() -> Self;
}
impl Zero for f64 {
    fn zero() -> Self {
        0.0
    }
}
impl Zero for Vector2<f64> {
    fn zero() -> Self {
        vec2(0.0, 0.0)
    }
}
impl Zero for Vector3<f64> {
    fn zero() -> Self {
        vec3(0.0, 0.0, 0.0)
    }
}
impl Zero for Vector4<f64> {
    fn zero() -> Self {
        vec4(0.0, 0.0, 0.0, 0.0)
    }
}

pub fn factorial(n: f64) -> f64 {
    if n == 0.0 || n == 1.0 {
        1.0
    } else {
        (2..=n as i32).product::<i32>() as f64
    }
}

pub trait Vector {
    fn magnitude(&self) -> f64;
    fn normalize(&self) -> Self;
    fn cross(&self, other: &Self) -> Self;
}

pub trait Homogeneous<C>
where
    C: std::ops::Mul<f64, Output = C> + std::ops::Div<f64, Output = C>,
{
    /// Gets the homogeneous component (the last one) of the coordinates.
    fn homogeneous_component(&self) -> f64;

    /// Gets the cartesian components of the coordinate (all but the last one),
    /// as a cartesian coordinate.
    fn cartesian_components(&self) -> C;

    /// Creates homogeneous coordinates from cartesian coordinates and
    /// a homogeneous coordinate.
    fn from_cartesian(cartesian: C, homogeneous: f64) -> Self;

    /// Creates a cartesian coordinate by projecting into cartesian space.
    fn to_cartesian(&self) -> C {
        self.cartesian_components() / self.homogeneous_component()
    }

    /// "Weights" the cartesian components by multipling them by the
    /// homogeneous component.
    fn to_weighted(&self) -> Self
    where
        Self: Sized,
    {
        Self::from_cartesian(
            self.cartesian_components() * self.homogeneous_component(),
            self.homogeneous_component(),
        )
    }

    /// Removes the "weight" from the cartesian components by dividing
    /// them by the homogeneous component (the weight).
    fn to_unweighted(&self) -> Self
    where
        Self: Sized,
    {
        Self::from_cartesian(
            self.cartesian_components() / self.homogeneous_component(),
            self.homogeneous_component(),
        )
    }
}

pub struct FloatRange {
    num_increments: usize,
    start: f64,
    end: f64,
    increment: f64,
    count: usize,
}
impl FloatRange {
    pub fn new(lower_bound: f64, upper_bound: f64, num_increments: usize) -> Self {
        let increment = if num_increments != 0 {
            (upper_bound - lower_bound) / num_increments as f64
        } else {
            0.0
        };
        Self {
            num_increments,
            start: lower_bound,
            end: upper_bound,
            increment,
            count: 0,
        }
    }
}
impl Iterator for FloatRange {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.num_increments + 1 {
            let mut next = self.start + self.increment * self.count as f64;
            if next >= self.end {
                next = self.end;
            }
            self.count += 1;
            Some(next)
        } else {
            None
        }
    }
}
