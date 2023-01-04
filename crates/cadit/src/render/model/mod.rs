use std::sync::Arc;

use super::mesh::{Surface, Vertex};

mod material;

use bytemuck::{Pod, Zeroable};
pub use material::*;
use vulkano::buffer::CpuAccessibleBuffer;

pub struct ModelObjectId(u32);
impl From<u32> for ModelObjectId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Zeroable, Pod)]
pub struct BufferedVertex {
    position: [f32; 3],
    normal: [f32; 3],
    material_idx: u32,
}
impl BufferedVertex {
    pub fn new(vertex: &Vertex, material_idx: u32) -> Self {
        Self {
            position: vertex.position.clone(),
            normal: vertex.normal.clone(),
            material_idx,
        }
    }
}
vulkano::impl_vertex!(BufferedVertex, position, normal, material_idx);

#[derive(Clone)]
pub struct GeometryBuffers {
    pub vertex_buffer: Arc<CpuAccessibleBuffer<[BufferedVertex]>>,
    pub index_buffer: Arc<CpuAccessibleBuffer<[u32]>>,
}

pub struct ModelSurface {
    id: ModelObjectId,
    surface: Surface,
    material_idx: u32,
}
impl ModelSurface {
    pub fn new(id: ModelObjectId, surface: Surface, material_idx: u32) -> Self {
        Self {
            id,
            surface,
            material_idx,
        }
    }

    pub fn surface(&self) -> &Surface {
        &self.surface
    }

    pub fn material_idx(&self) -> u32 {
        self.material_idx
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
