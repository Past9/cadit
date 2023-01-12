use std::slice::Iter;

#[derive(Clone, Debug, PartialEq)]
pub struct KnotVector {
    knots: Vec<f64>,
}
impl KnotVector {
    pub fn new<const N: usize>(knots: [f64; N]) -> Self {
        Self {
            knots: knots.to_vec(),
        }
    }

    pub fn first(&self) -> f64 {
        self.knots[0]
    }

    pub fn last(&self) -> f64 {
        self.knots[self.knots.len() - 1]
    }

    pub fn from_slice(knots: &[f64]) -> Self {
        Self {
            knots: knots.to_vec(),
        }
    }

    pub fn from_vec(knots: Vec<f64>) -> Self {
        Self { knots }
    }

    pub fn zeros(size: usize) -> Self {
        Self {
            knots: vec![0.0; size],
        }
    }

    pub fn len(&self) -> usize {
        self.knots.len()
    }

    pub fn iter(&self) -> Iter<f64> {
        self.knots.iter()
    }

    pub fn find_span(&self, degree: usize, num_ctrl_points: usize, pos: f64) -> usize {
        if pos == self.knots[num_ctrl_points] {
            return num_ctrl_points - 1;
        }

        let mut low = degree;
        let mut high = num_ctrl_points + 1;
        let mut mid = (low + high) / 2;

        while pos < self.knots[mid] || pos >= self.knots[mid + 1] {
            if pos < self.knots[mid] {
                high = mid;
            } else {
                low = mid;
            }

            mid = (low + high) / 2;
        }

        return mid;
    }

    pub fn find_multiplicity(&self, pos: f64) -> usize {
        let mut multiplicity = 0;

        for knot in self.knots.iter() {
            if (pos - knot).abs() == 0.0 {
                multiplicity += 1;
            }
        }

        multiplicity
    }

    /// Returns the index of the knot in the knot vector, or None if
    /// if isn't in the vector. If the knot exists multiple times,
    /// it will return the index of the first occurrence.
    pub fn find_index(&self, pos: f64) -> Option<usize> {
        for (i, knot) in self.knots.iter().enumerate() {
            if pos == *knot {
                return Some(i);
            }
        }

        None
    }
}
impl FromIterator<f64> for KnotVector {
    fn from_iter<I: IntoIterator<Item = f64>>(knots: I) -> Self {
        Self {
            knots: Vec::from_iter(knots),
        }
    }
}
impl<Idx> std::ops::Index<Idx> for KnotVector
where
    Idx: std::slice::SliceIndex<[f64]>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.knots[index]
    }
}
impl<Idx> std::ops::IndexMut<Idx> for KnotVector
where
    Idx: std::slice::SliceIndex<[f64]>,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.knots[index]
    }
}
