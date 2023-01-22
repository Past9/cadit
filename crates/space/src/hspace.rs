use crate::{ESpace, ESpace1, ESpace2, ESpace3, ESpace4, EUnimplementedSpace};

/// Trait for homogeneous spaces
pub trait HSpace {
    const DIMENSIONS: usize;

    /// The Euclidean space that vectors in this space become a part
    /// of when weighted. It has the number of Euclidean dimensions as
    /// this space plus one.
    type Weighted: ESpace;

    /// The Euclidean space that vectors in this space are projected into.
    /// It has the same number of Euclidean dimensions as this space (the
    /// non-Euclidean "projective" dimension is removed).
    type Projected: ESpace;
}

/// 1-dimensional homogeneous space
pub struct HSpace1;
impl HSpace for HSpace1 {
    const DIMENSIONS: usize = 1;
    type Weighted = ESpace2;
    type Projected = ESpace1;
}

/// 2-dimensional homogeneous space
pub struct HSpace2;
impl HSpace for HSpace2 {
    const DIMENSIONS: usize = 2;
    type Weighted = ESpace3;
    type Projected = ESpace2;
}

/// 3-dimensional homogeneous space
pub struct HSpace3;
impl HSpace for HSpace3 {
    const DIMENSIONS: usize = 3;
    type Weighted = ESpace4;
    type Projected = ESpace3;
}

/// An unimplemented Homogeneous space. Used where a Homogeneous space is required by the
/// the type system but a space of the appropriate dimensionality is not actually needed.
pub struct HUnimplementedSpace;
impl HSpace for HUnimplementedSpace {
    const DIMENSIONS: usize = 0;
    type Weighted = EUnimplementedSpace;
    type Projected = EUnimplementedSpace;
}
