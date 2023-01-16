use crate::math::{bezier::decasteljau, Homogeneous};

#[derive(Debug)]
pub struct BezierCurve<H: Homogeneous> {
    control_points: Vec<H>,
}
impl<H: Homogeneous> BezierCurve<H> {
    pub fn new(control_points: Vec<H>) -> Self {
        Self { control_points }
    }

    pub fn point(&self, u: f64) -> H::Projected {
        let p = decasteljau(
            &self
                .control_points
                .iter()
                .map(|p| p.weight())
                .collect::<Vec<_>>(),
            u,
        );

        H::cast_from_weighted(p).project()
    }
}
