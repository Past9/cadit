use std::sync::Arc;

use crevice::std140::AsStd140;
use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer},
    memory::allocator::MemoryAllocator,
};

use crate::Rgba;

#[derive(AsStd140, Clone, Debug)]
pub struct Material {
    diffuse: Rgba,
    roughness: f32,
}
impl Material {
    pub fn new(diffuse: Rgba, roughness: f32) -> Self {
        Self { diffuse, roughness }
    }

    pub fn buffer(
        allocator: &(impl MemoryAllocator + ?Sized),
        materials: &[Material],
    ) -> Arc<CpuAccessibleBuffer<[Std140Material]>> {
        CpuAccessibleBuffer::from_iter(
            allocator,
            BufferUsage {
                storage_buffer: true,
                ..BufferUsage::empty()
            },
            false,
            materials.iter().map(|material| material.as_std140()),
        )
        .unwrap()
    }
}
