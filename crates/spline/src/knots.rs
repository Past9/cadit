use std::slice::Iter;

use crate::math::Float;

#[derive(Clone, Debug, PartialEq)]
pub struct KnotVector {
    knots: Vec<Float>,
}
impl KnotVector {
    pub fn new<const N: usize>(knots: [Float; N]) -> Self {
        Self {
            knots: knots.to_vec(),
        }
    }

    pub fn first(&self) -> Float {
        self.knots[0]
    }

    pub fn last(&self) -> Float {
        self.knots[self.knots.len() - 1]
    }

    pub fn from_slice(knots: &[Float]) -> Self {
        Self {
            knots: knots.to_vec(),
        }
    }

    pub fn from_vec(knots: Vec<Float>) -> Self {
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

    pub fn iter(&self) -> Iter<Float> {
        self.knots.iter()
    }

    pub fn find_span(&self, degree: usize, num_ctrl_points: usize, pos: Float) -> usize {
        if pos == self.knots[num_ctrl_points] {
            return num_ctrl_points - 1;
        }

        let mut low = degree;
        let mut high = num_ctrl_points + 1;
        let mut mid = (low + high) / 2;

        let mut last_low = low;
        let mut last_mid = mid;
        let mut last_high = high;

        while pos < self.knots[mid] || pos >= self.knots[mid + 1] {
            if pos < self.knots[mid] {
                high = mid;
            } else {
                low = mid;
            }

            mid = (low + high) / 2;

            if low == last_low && mid == last_mid && high == last_high {
                panic!("Infinite loop while searching for knot span");
            }

            last_low = low;
            last_mid = mid;
            last_high = high;
        }

        return mid;
    }

    pub fn find_multiplicity(&self, pos: Float) -> usize {
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
    pub fn find_index(&self, pos: Float) -> Option<usize> {
        for (i, knot) in self.knots.iter().enumerate() {
            if pos == *knot {
                return Some(i);
            }
        }

        None
    }
}
impl FromIterator<Float> for KnotVector {
    fn from_iter<I: IntoIterator<Item = Float>>(knots: I) -> Self {
        Self {
            knots: Vec::from_iter(knots),
        }
    }
}
impl<Idx> std::ops::Index<Idx> for KnotVector
where
    Idx: std::slice::SliceIndex<[Float]>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.knots[index]
    }
}
impl<Idx> std::ops::IndexMut<Idx> for KnotVector
where
    Idx: std::slice::SliceIndex<[Float]>,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.knots[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slices() {
        let knots = [0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 5.0, 5.0, 5.0];
        let knot_vector = KnotVector::new(knots.clone());
        assert_eq!(knots[4], knot_vector[4]);
        assert_eq!(knots[0..4], knot_vector[0..4]);
        assert_eq!(knots[..4], knot_vector[..4]);
        assert_eq!(knots[4..], knot_vector[4..]);
        assert_eq!(knots[..], knot_vector[..]);
        assert_eq!(knots[0..=4], knot_vector[0..=4]);
        assert_eq!(knots[..=4], knot_vector[..=4]);
        assert_eq!(knots[4..], knot_vector[4..]);
        assert_eq!(knots[..], knot_vector[..]);
    }

    #[test]
    fn from_slice() {
        let knots = [0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 5.0, 5.0, 5.0];
        let knot_vector = KnotVector::new(knots.clone());
        assert_eq!(
            KnotVector::from_slice(&knots[0..4]),
            KnotVector::from_slice(&knot_vector[0..4])
        );
        assert_eq!(
            KnotVector::from_slice(&knots[..4]),
            KnotVector::from_slice(&knot_vector[..4])
        );
        assert_eq!(
            KnotVector::from_slice(&knots[4..]),
            KnotVector::from_slice(&knot_vector[4..])
        );
        assert_eq!(
            KnotVector::from_slice(&knots[..]),
            KnotVector::from_slice(&knot_vector[..])
        );
        assert_eq!(
            KnotVector::from_slice(&knots[0..=4]),
            KnotVector::from_slice(&knot_vector[0..=4])
        );
        assert_eq!(
            KnotVector::from_slice(&knots[..=4]),
            KnotVector::from_slice(&knot_vector[..=4])
        );
        assert_eq!(
            KnotVector::from_slice(&knots[4..]),
            KnotVector::from_slice(&knot_vector[4..])
        );
        assert_eq!(
            KnotVector::from_slice(&knots[..]),
            KnotVector::from_slice(&knot_vector[..])
        );
    }

    #[test]
    fn finds_knot_span() {
        let degree = 2;
        let knot_vector = KnotVector::new([0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 5.0, 5.0, 5.0]);
        let num_points = knot_vector.len() - degree - 1;
        let u = 5.0 / 2.0;

        let result = knot_vector.find_span(degree, num_points, u);

        assert_eq!(4, result);
    }

    #[test]
    fn finds_knot_index() {
        let knot_vector = KnotVector::new([0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 5.0, 5.0, 5.0]);
        let result = knot_vector.find_index(1.0);
        assert_eq!(3, result.unwrap());
    }

    #[test]
    fn finds_first_knot_index_when_multiple() {
        let knot_vector = KnotVector::new([0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 5.0, 5.0, 5.0]);
        assert_eq!(0, knot_vector.find_index(0.0).unwrap());
        assert_eq!(6, knot_vector.find_index(4.0).unwrap());
        assert_eq!(8, knot_vector.find_index(5.0).unwrap());
    }

    #[test]
    fn finds_no_knot_index() {
        let knot_vector = KnotVector::new([0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 5.0, 5.0, 5.0]);
        assert_eq!(None, knot_vector.find_index(-1.0));
        assert_eq!(None, knot_vector.find_index(-0.1));
        assert_eq!(None, knot_vector.find_index(0.1));
        assert_eq!(None, knot_vector.find_index(4.9));
        assert_eq!(None, knot_vector.find_index(5.1));
    }
}
