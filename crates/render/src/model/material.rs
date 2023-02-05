use std::sync::Arc;

use crevice::std140::AsStd140;
use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer},
    memory::allocator::MemoryAllocator,
};

use crate::{Rgb, Rgba};

#[derive(AsStd140, Clone, Debug)]
pub struct OpaqueMaterial {
    diffuse: Rgb,
    roughness: f32,
}
impl OpaqueMaterial {
    pub fn new(diffuse: Rgb, roughness: f32) -> Self {
        Self { diffuse, roughness }
    }

    pub fn buffer(
        allocator: &(impl MemoryAllocator + ?Sized),
        materials: &[OpaqueMaterial],
    ) -> Arc<CpuAccessibleBuffer<[Std140OpaqueMaterial]>> {
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

#[derive(AsStd140, Clone, Debug)]
pub struct TranslucentMaterial {
    diffuse: Rgba,
    roughness: f32,
}
impl TranslucentMaterial {
    pub fn new(diffuse: Rgba, roughness: f32) -> Self {
        Self { diffuse, roughness }
    }

    pub fn buffer(
        allocator: &(impl MemoryAllocator + ?Sized),
        materials: &[TranslucentMaterial],
    ) -> Arc<CpuAccessibleBuffer<[Std140TranslucentMaterial]>> {
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
