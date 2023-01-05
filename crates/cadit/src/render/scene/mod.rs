use std::sync::Arc;

use cgmath::Zero;
use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer},
    descriptor_set::{allocator::DescriptorSetAllocator, PersistentDescriptorSet},
    memory::allocator::MemoryAllocator,
};

use super::{
    camera::Camera,
    cgmath_types::{vec3, Mat4, Quat, Vec3},
    lights::{AmbientLight, DirectionalLight, PointLight},
    mesh::Vertex,
    model::{BufferedVertex, Material, Model},
    Rgba,
};
use crate::render::lights::{Std140AmbientLight, Std140DirectionalLight, Std140PointLight};
use crate::render::model::Std140Material;

pub struct SceneLights {
    ambient: Vec<AmbientLight>,
    directional: Vec<DirectionalLight>,
    point: Vec<PointLight>,
}
impl SceneLights {
    pub fn new(
        ambient: Vec<AmbientLight>,
        directional: Vec<DirectionalLight>,
        point: Vec<PointLight>,
    ) -> Self {
        Self {
            ambient,
            directional,
            point,
        }
    }

    pub fn light_buffers(
        &self,
        allocator: &(impl MemoryAllocator + ?Sized),
    ) -> (
        Arc<CpuAccessibleBuffer<[Std140AmbientLight]>>,
        Arc<CpuAccessibleBuffer<[Std140DirectionalLight]>>,
        Arc<CpuAccessibleBuffer<[Std140PointLight]>>,
    ) {
        (
            AmbientLight::buffer(allocator, self.ambient.clone()),
            DirectionalLight::buffer(allocator, self.directional.clone()),
            PointLight::buffer(allocator, self.point.clone()),
        )
    }
}

pub struct Orientation {
    offset: Vec3,
    rotation: Quat,
}
impl Orientation {
    pub fn zero() -> Self {
        Self {
            offset: vec3(0.0, 0.0, 0.0),
            rotation: Quat::zero(),
        }
    }

    pub fn set_offset(&mut self, offset: Vec3) {
        self.offset = offset;
    }

    pub fn set_rotation(&mut self, rotation: Quat) {
        self.rotation = rotation;
    }

    pub fn matrix(&self) -> Mat4 {
        Mat4::from_translation(self.offset) * Mat4::from(self.rotation)
    }
}

pub struct Scene {
    bg_color: Rgba,
    orientation: Orientation,
    lights: SceneLights,
    camera: Camera,
    models: Vec<Model>,
    materials: Vec<Material>,
}
impl Scene {
    pub fn new(
        bg_color: Rgba,
        lights: SceneLights,
        camera: Camera,
        models: Vec<Model>,
        materials: Vec<Material>,
    ) -> Self {
        Self {
            bg_color,
            orientation: Orientation::zero(),
            lights,
            camera,
            models,
            materials,
        }
    }

    pub fn bg_color(&self) -> Rgba {
        self.bg_color
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    pub fn orientation(&self) -> &Orientation {
        &self.orientation
    }

    pub fn orientation_mut(&mut self) -> &mut Orientation {
        &mut self.orientation
    }

    pub fn lights(&self) -> &SceneLights {
        &self.lights
    }

    pub fn material_buffer(
        &self,
        allocator: &impl MemoryAllocator,
    ) -> Arc<CpuAccessibleBuffer<[Std140Material]>> {
        Material::buffer(allocator, &self.materials)
    }

    pub fn geometry_buffers(
        &self,
        allocator: &impl MemoryAllocator,
    ) -> (
        Arc<CpuAccessibleBuffer<[BufferedVertex]>>,
        Arc<CpuAccessibleBuffer<[u32]>>,
    ) {
        let mut vertices: Vec<BufferedVertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        let mut index_offset = 0;
        for model in self.models.iter() {
            for surface in model.surfaces().iter() {
                let surface_vertices = surface.surface().vertices();
                vertices.extend(
                    surface_vertices
                        .iter()
                        .map(|vert| BufferedVertex::new(vert, surface.material_idx())),
                );

                let surface_indices = surface.surface().indices();
                indices.extend(surface_indices.iter().map(|i| i + index_offset));

                index_offset += surface_vertices.len() as u32;
            }
        }

        let vertex_buffer = CpuAccessibleBuffer::from_iter(
            allocator,
            BufferUsage {
                vertex_buffer: true,
                ..BufferUsage::empty()
            },
            false,
            vertices,
        )
        .unwrap();

        let index_buffer = CpuAccessibleBuffer::from_iter(
            allocator,
            BufferUsage {
                index_buffer: true,
                ..BufferUsage::empty()
            },
            false,
            indices,
        )
        .unwrap();

        (vertex_buffer, index_buffer)
    }
}
