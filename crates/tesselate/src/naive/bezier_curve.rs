use render::{
    mesh::{Edge, EdgeVertex},
    model::{ModelEdge, ModelObjectId},
    Rgba,
};
use space::{hspace::HSpace, EVector};
use spline::{bezier_curve::BezierCurve, math::FloatRange};

pub fn tesselate_bezier_curve<H: HSpace>(
    curve: &BezierCurve<H>,
    segments: usize,
    object_id: ModelObjectId,
    color: Rgba,
) -> ModelEdge {
    ModelEdge::new(
        object_id,
        Edge {
            vertices: FloatRange::new(0.0, 1.0, segments)
                .map(|u| EdgeVertex {
                    position: curve.point(u).f32s(),
                    expand: [0.0, 0.0, 0.0],
                })
                .collect::<Vec<_>>(),
        },
        color,
    )
}
