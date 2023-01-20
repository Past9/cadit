use crate::{HSpace1, HSpace2, HSpace3, HUnimplementedSpace, HomogeneousSpace};

pub trait ESpace {
    type Lower: ESpace;
    type Homogeneous: HomogeneousSpace;
}

pub struct ESpace1;
impl ESpace for ESpace1 {
    type Lower = EUnimplementedSpace;
    type Homogeneous = HSpace1;
}

pub struct ESpace2;
impl ESpace for ESpace2 {
    type Lower = ESpace1;
    type Homogeneous = HSpace2;
}

pub struct ESpace3;
impl ESpace for ESpace3 {
    type Lower = ESpace2;
    type Homogeneous = HSpace3;
}

pub struct ESpace4;
impl ESpace for ESpace4 {
    type Lower = ESpace3;
    type Homogeneous = HUnimplementedSpace;
}

pub struct EUnimplementedSpace;
impl ESpace for EUnimplementedSpace {
    type Lower = EUnimplementedSpace;
    type Homogeneous = HUnimplementedSpace;
}
