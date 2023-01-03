use super::{mesh::Surface, Color};

pub struct ModelObjectId(u32);
impl From<u32> for ModelObjectId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

pub struct Material {
    diffuse: Color,
    roughness: f32,
}

pub struct ModelSurface {
    id: ModelObjectId,
    surface: Surface,
    material: Material,
}

pub struct ModelEdge {
    id: ModelObjectId,
}
pub struct ModelPoint {
    id: ModelObjectId,
}

pub struct Model {
    surfaces: Vec<ModelSurface>,
    edges: Vec<ModelEdge>,
    points: Vec<ModelPoint>,
}
