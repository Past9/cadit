use render::{
    model::SurfaceVertex,
    model::{MaterialId, ModelObjectId, ModelSurface},
};
use space::{hspace::HSpace, EVector};
use spline::{bezier_surface::BezierSurface, math::FloatRange};

pub fn tesselate_bezier_surface<H: HSpace>(
    surface: &BezierSurface<H>,
    segments: usize,
    object_id: ModelObjectId,
    material_id: MaterialId,
) -> ModelSurface {
    let mut vertices: Vec<SurfaceVertex> = Vec::with_capacity(segments.pow(2));
    for v in FloatRange::new(0.0, 1.0, segments) {
        for u in FloatRange::new(0.0, 1.0, segments) {
            let ders = surface.derivatives(u, v, 1);
            let point = ders[0][0];
            let normal = ders[0][1].cross(&ders[1][0]).normalize();
            vertices.push(SurfaceVertex {
                position: point.f32s(),
                normal: normal.f32s(),
            });
        }
    }

    let mut indices = Vec::new();
    for u in 1..=segments as u32 {
        for v in 1..=segments as u32 {
            // Quad corners
            let bl = index(u - 1, v - 1, segments as u32); // Bottom left
            let br = index(u, v - 1, segments as u32); // Bottom right
            let tl = index(u - 1, v, segments as u32); // Top left
            let tr = index(u, v, segments as u32); // Top right

            // Triangle 1
            indices.push(bl);
            indices.push(br);
            indices.push(tr);

            // Triangle 2
            indices.push(bl);
            indices.push(tr);
            indices.push(tl);
        }
    }

    ModelSurface::new(object_id, vertices, indices, material_id)
}

fn index(u: u32, v: u32, segments: u32) -> u32 {
    u * (segments + 1) + v
}
