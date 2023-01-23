use std::collections::LinkedList;

use render::{
    mesh::{Edge, EdgeVertex},
    model::{ModelEdge, ModelObjectId},
    Rgba,
};
use space::{ELine, ELine3, EVector, HVec3, HVector};
use spline::bezier_curve::{BezierCurve, HausdorffResult};

/*
pub trait TesselateCurve<H: HVector> {
    fn tesselate(&self, tolerance: f64) -> TesselatedEdge<H::Projected>;

    fn to_model_edge(&self, tolerance: f64, object_id: ModelObjectId, color: Rgba) -> ModelEdge {
        /*
        ModelEdge::new(
            object_id,
            Edge {
                vertices: self
                    .tesselate(tolerance)
                    .to_vec()
                    .into_iter()
                    .map(|vert| EdgeVertex {
                        position: vert.pos.f32s(),
                        expand: [0.0, 0.0, 0.0],
                    })
                    .collect(),
            },
            color,
        )
        */
        todo!()
    }
}
impl TesselateCurve<HVec3> for BezierCurve<HVec3> {
    fn tesselate(&self, tolerance: f64) -> TesselatedEdge<<HVec3 as HVector>::Projected> {
        let start = self.point(0.0);
        let end = self.point(1.0);
        let line = ELine3::from_pos_and_dir(start, end - start);
        let hausdorff = self.hausdorff_to_line(&line, Some(0.0), Some(1.0));

        if let Some(hausdorff) = hausdorff {
            TesselatedEdge {
                vertices: vec![Vertex { u: 0.0, pos: start }, Vertex { u: 1.0, pos: end }],
                lines: vec![Line {
                    left: 0,
                    error: hausdorff.distance,
                    right: 1,
                }],
            }
        } else {
            panic!("Failed to find Hausdorff distance");
        }
    }
}

pub struct TesselatedEdge<V: EVector> {
    vertices: Vec<Vertex<V>>,
    lines: Vec<Line>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vertex<V: EVector> {
    u: f64,
    pos: V,
}

pub struct EuclideanLine {
    left: usize,
    error: f64,
    right: usize,
}
*/
