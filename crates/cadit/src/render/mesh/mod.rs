use std::sync::Arc;

use bytemuck::{Pod, Zeroable};
use vulkano::buffer::CpuAccessibleBuffer;

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Zeroable, Pod)]
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
vulkano::impl_vertex!(Vertex, position);

#[derive(Clone)]
pub struct GeometryBuffers {
    pub vertex_buffer: Arc<CpuAccessibleBuffer<[Vertex]>>,
    pub index_buffer: Arc<CpuAccessibleBuffer<[u32]>>,
}

pub struct Surface {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub buffers: Option<GeometryBuffers>,
}
impl Surface {
    pub fn new<const V: usize, const I: usize>(vertices: [Vertex; V], indices: [u32; I]) -> Self {
        Self {
            vertices: Vec::from_iter(vertices.into_iter()),
            indices: Vec::from_iter(indices.into_iter()),
            buffers: None,
        }
    }

    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    pub fn indices(&self) -> &[u32] {
        &self.indices
    }

    /*
    pub fn buffer(&mut self, allocator: &impl MemoryAllocator) -> &GeometryBuffers {
        self.buffers.get_or_insert_with(|| {
            let vertex_buffer = CpuAccessibleBuffer::from_iter(
                allocator,
                BufferUsage {
                    vertex_buffer: true,
                    ..BufferUsage::empty()
                },
                false,
                self.vertices.iter().map(|v| Vertex {
                    position: v.position,
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

            GeometryBuffers {
                vertex_buffer,
                index_buffer,
            }
        })
    }
    */
}

pub struct PbrMaterial {
    pub albedo: [f32; 4],
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Zeroable, Pod)]
pub struct IdVertex {
    pub position: [f32; 3],
    pub id: u32,
}
vulkano::impl_vertex!(IdVertex, position, id);
