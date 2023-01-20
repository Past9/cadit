//mod vector;
//mod vector_macros;

pub mod b_spline;
pub mod basis;
pub mod bezier;
pub mod knot_vector;
//pub mod line;
pub mod nurbs;

//pub use vector::*;

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
    BINOMIAL_COEFFICIENTS[k][i]
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
