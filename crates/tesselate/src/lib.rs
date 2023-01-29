use render::{
    mesh::{Edge, EdgeVertex},
    model::{ModelEdge, ModelObjectId},
    Rgba,
};
use space::{hspace::HSpace, EVector};
use spline::bezier_curve::{BezierCurve, HausdorffResult};

pub fn tesselate_bezier_curve<H: HSpace>(
    curve: &BezierCurve<H>,
    tolerance: f64,
) -> TesselatedCurve<H> {
    let start = curve.point(0.0);
    let end = curve.point(1.0);

    let mut tesselated: TesselatedCurve<H> = curve
        .hausdorff_to_line(
            &H::make_line_through_points(start, end),
            Some(0.0),
            Some(1.0),
            false,
        )
        .map(|err| TesselatedCurve {
            start: CurveVertex {
                u: 0.0,
                point: start,
            },
            segments: vec![CurveSegment {
                err: err,
                end: CurveVertex { u: 1.0, point: end },
            }],
        })
        .expect("Could not find Hausdorff distance");

    while let Some(refined) = iter_tesselate_curve(curve, tolerance, &tesselated) {
        tesselated = refined;
    }

    tesselated
}

fn iter_tesselate_curve<H: HSpace>(
    curve: &BezierCurve<H>,
    tolerance: f64,
    tesselated: &TesselatedCurve<H>,
) -> Option<TesselatedCurve<H>> {
    let mut changed = false;
    let mut new_segments: Vec<CurveSegment<H>> = Vec::new();

    let mut start = tesselated.start.clone();
    for segment in tesselated.segments.iter() {
        if segment.err.distance <= tolerance {
            new_segments.push(segment.clone());
            start = segment.end.clone();
            continue;
        }

        let (seg1, seg2) = split_segment(curve, &start, segment);

        start = seg2.end.clone();

        new_segments.push(seg1);
        new_segments.push(seg2);

        changed = true;
    }

    if changed {
        Some(TesselatedCurve {
            start: tesselated.start.clone(),
            segments: new_segments,
        })
    } else {
        None
    }
}

fn split_segment<H: HSpace>(
    curve: &BezierCurve<H>,
    start: &CurveVertex<H>,
    segment: &CurveSegment<H>,
) -> (CurveSegment<H>, CurveSegment<H>) {
    let seg1 = curve
        .hausdorff_to_line(
            &H::make_line_through_points(start.point, segment.err.point),
            Some(start.u),
            Some(segment.err.u),
            false,
        )
        .map(|err| CurveSegment {
            err,
            end: CurveVertex {
                u: segment.err.u,
                point: segment.err.point,
            },
        })
        .expect("Cannot find Hausdorff");

    let seg2 = curve
        .hausdorff_to_line(
            &H::make_line_through_points(segment.err.point, segment.end.point),
            Some(segment.err.u),
            Some(segment.end.u),
            false,
        )
        .map(|err| CurveSegment {
            err,
            end: CurveVertex {
                u: segment.end.u,
                point: segment.end.point,
            },
        })
        .expect("Cannot find Hausdorff");

    (seg1, seg2)
}

#[derive(Debug, Clone)]
pub struct TesselatedCurve<H: HSpace> {
    start: CurveVertex<H>,
    segments: Vec<CurveSegment<H>>,
}
impl<H: HSpace> TesselatedCurve<H> {
    pub fn to_model_edge(&self, object_id: ModelObjectId, color: Rgba) -> ModelEdge {
        let vertices = [EdgeVertex {
            position: self.start.point.f32s(),
            expand: [0.0, 0.0, 0.0],
        }]
        .into_iter()
        .chain(self.segments.iter().map(|seg| EdgeVertex {
            position: seg.end.point.f32s(),
            expand: [0.0, 0.0, 0.0],
        }))
        .collect::<Vec<EdgeVertex>>();

        ModelEdge::new(object_id, Edge { vertices }, color)
    }
}

#[derive(Debug, Clone)]
pub struct CurveVertex<H: HSpace> {
    u: f64,
    point: H::ProjectedVector,
}

#[derive(Debug, Clone)]
pub struct CurveSegment<H: HSpace> {
    err: HausdorffResult<H>,
    end: CurveVertex<H>,
}

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
