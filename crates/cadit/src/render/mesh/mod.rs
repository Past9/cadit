use std::sync::Arc;

use bytemuck::{Pod, Zeroable};
use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer},
    memory::allocator::MemoryAllocator,
};

pub struct ObjectId(u32);
impl From<u32> for ObjectId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

pub struct Vertex {
    position: [f32; 3],
}
impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: [x, y, z],
        }
    }
}

pub struct PbrSurfaceBuffers {
    pub vertex_buffer: Arc<CpuAccessibleBuffer<[PbrVertex]>>,
    pub index_buffer: Arc<CpuAccessibleBuffer<[u32]>>,
}

pub struct Surface {
    pub id: ObjectId,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}
impl Surface {
    pub fn new<const V: usize, const I: usize>(
        id: u32,
        vertices: [Vertex; V],
        indices: [u32; I],
    ) -> Self {
        Self {
            id: id.into(),
            vertices: Vec::from_iter(vertices.into_iter()),
            indices: Vec::from_iter(indices.into_iter()),
        }
    }

    pub fn as_pbr(
        &self,
        material: &PbrMaterial,
        allocator: &impl MemoryAllocator,
    ) -> PbrSurfaceBuffers {
        let vertex_buffer = CpuAccessibleBuffer::from_iter(
            allocator,
            BufferUsage {
                vertex_buffer: true,
                ..BufferUsage::empty()
            },
            false,
            self.vertices.iter().map(|v| PbrVertex {
                position: v.position,
                albedo: material.albedo,
            }),
        )
        .unwrap();

        let index_buffer = CpuAccessibleBuffer::from_iter(
            allocator,
            BufferUsage {
                index_buffer: true,
                ..BufferUsage::empty()
            },
            false,
            self.indices.iter().cloned(),
        )
        .unwrap();

        PbrSurfaceBuffers {
            vertex_buffer,
            index_buffer,
        }
    }
}

pub struct PbrMaterial {
    pub albedo: [f32; 4],
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Zeroable, Pod)]
pub struct PbrVertex {
    pub position: [f32; 3],
    pub albedo: [f32; 4],
}
vulkano::impl_vertex!(PbrVertex, position, albedo);

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Zeroable, Pod)]
pub struct IdVertex {
    pub position: [f32; 3],
    pub id: u32,
}
vulkano::impl_vertex!(IdVertex, position, id);
