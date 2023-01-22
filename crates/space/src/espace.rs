use crate::{ELine, EVector, HSpace, HSpace1, HSpace2, HSpace3, HUnimplementedSpace};

/// Trait for Euclidean spaces
pub trait ESpace {
    /// The Euclidean space with the next lowest dimension.
    type Lower: ESpace;

    /// The Homogeneous space that is projected into this Euclidean space.
    /// Has the same number of Euclidean dimensions as this space, plus a
    /// projective dimension.
    type Homogeneous: HSpace;
}

/// 1-dimensional Euclidean space
pub struct ESpace1;
impl ESpace for ESpace1 {
    type Lower = EUnimplementedSpace;
    type Homogeneous = HSpace1;
}

/// 2-dimensional Euclidean space
pub struct ESpace2;
impl ESpace for ESpace2 {
    type Lower = ESpace1;
    type Homogeneous = HSpace2;
}

/// 3-dimensional Euclidean space
pub struct ESpace3;
impl ESpace for ESpace3 {
    type Lower = ESpace2;
    type Homogeneous = HSpace3;
}

/// 4-dimensional Euclidean space
pub struct ESpace4;
impl ESpace for ESpace4 {
    type Lower = ESpace3;
    type Homogeneous = HUnimplementedSpace;
}

/// An unimplemented Euclidean space. Used where a Euclidean space is required by the
/// the type system but a space of the appropriate dimensionality is not actually needed.
pub struct EUnimplementedSpace;
impl ESpace for EUnimplementedSpace {
    type Lower = EUnimplementedSpace;
    type Homogeneous = HUnimplementedSpace;
}
