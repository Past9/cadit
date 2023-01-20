use crate::{ESpace, ESpace2, ESpace3, HVec1, HVec2, HVec3, HVector};

pub trait ELine<S: ESpace> {}

pub struct ELine2 {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}
impl ImplicitifyControlPoint<ESpace2, HVec2, HVec1> for ELine2 {
    fn implicitify_control_point(&self, control_point: HVec2) -> HVec1 {
        HVec1 {
            x: control_point.x * self.a + control_point.y * self.b + self.c,
            h: control_point.h,
        }
    }
}

pub struct ELine3 {
    pub a1: f64,
    pub b1: f64,
    pub c1: f64,
    pub d1: f64,
    pub a2: f64,
    pub b2: f64,
    pub c2: f64,
    pub d2: f64,
}
impl ImplicitifyControlPoint<ESpace3, HVec3, HVec2> for ELine3 {
    fn implicitify_control_point(&self, control_point: HVec3) -> HVec2 {
        HVec2 {
            x: control_point.x * self.a1
                + control_point.y * self.b1
                + control_point.z * self.c1
                + self.d1,
            y: control_point.x * self.a2
                + control_point.y * self.b2
                + control_point.z * self.c2
                + self.d2,
            h: control_point.h,
        }
    }
}

pub trait ImplicitifyControlPoint<
    S: ESpace,
    TControlPoint: HVector<S::Homogeneous>,
    TOutput: HVector<<S::Lower as ESpace>::Homogeneous>,
>
{
    fn implicitify_control_point(&self, control_point: TControlPoint) -> TOutput;
}
