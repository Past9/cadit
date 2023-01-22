struct HVec1 {
    pub x: f64,
    pub h: f64,
}

struct HVec2 {
    pub x: f64,
    pub y: f64,
    pub h: f64,
}

struct HVec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub h: f64,
}

struct EVec1 {
    pub x: f64,
}

struct EVec2 {
    pub x: f64,
    pub y: f64,
}

struct EVec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

struct EVec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

struct ELine2 {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

struct ELine3 {
    pub a1: f64,
    pub b1: f64,
    pub c1: f64,
    pub d1: f64,
    pub a2: f64,
    pub b2: f64,
    pub c2: f64,
    pub d2: f64,
}

trait HSpace {
    type Vector;
    type ProjectedVector;
    type WeightedVector;
    type Line;

    fn weight_vec(hvec: Self::Vector) -> Self::WeightedVector;
    fn project_vec(hvec: Self::Vector) -> Self::ProjectedVector;
    fn make_line(pos: Self::ProjectedVector, dir: Self::ProjectedVector) -> Self::Line;
}

struct HSpace3 {}
impl HSpace for HSpace3 {
    type Vector = HVec3;
    type ProjectedVector = EVec3;
    type WeightedVector = EVec4;
    type Line = ELine3;

    fn weight_vec(hvec: Self::Vector) -> Self::WeightedVector {
        todo!()
    }

    fn project_vec(hvec: Self::Vector) -> Self::ProjectedVector {
        todo!()
    }

    fn make_line(pos: Self::ProjectedVector, dir: Self::ProjectedVector) -> Self::Line {
        todo!()
    }
}
