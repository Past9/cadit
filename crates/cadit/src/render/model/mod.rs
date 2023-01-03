use super::{mesh::Surface, Color};

pub struct ModelObjectId(u32);
impl From<u32> for ModelObjectId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

pub struct Material {
    pub diffuse: Color,
    pub roughness: f32,
}

pub struct ModelSurface {
    id: ModelObjectId,
    surface: Surface,
    material: Material,
}
impl ModelSurface {
    pub fn new(id: ModelObjectId, surface: Surface, material: Material) -> Self {
        Self {
            id,
            surface,
            material,
        }
    }

    pub fn surface(&self) -> &Surface {
        &self.surface
    }
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
impl Model {
    pub fn new(
        surfaces: Vec<ModelSurface>,
        edges: Vec<ModelEdge>,
        points: Vec<ModelPoint>,
    ) -> Self {
        Self {
            surfaces,
            edges,
            points,
        }
    }

    pub fn surfaces(&self) -> &[ModelSurface] {
        &self.surfaces
    }
}
