use crate::{ESpace, ESpace1, ESpace2, ESpace3, ESpace4, EUnimplementedSpace};

pub trait HomogeneousSpace {
    type Weighted: ESpace;
    type Projected: ESpace;
}

pub struct HSpace1;
impl HomogeneousSpace for HSpace1 {
    type Weighted = ESpace2;
    type Projected = ESpace1;
}

pub struct HSpace2;
impl HomogeneousSpace for HSpace2 {
    type Weighted = ESpace3;
    type Projected = ESpace2;
}

pub struct HSpace3;
impl HomogeneousSpace for HSpace3 {
    type Weighted = ESpace4;
    type Projected = ESpace3;
}

pub struct HUnimplementedSpace;
impl HomogeneousSpace for HUnimplementedSpace {
    type Weighted = EUnimplementedSpace;
    type Projected = EUnimplementedSpace;
}
