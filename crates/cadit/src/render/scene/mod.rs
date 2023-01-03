use std::sync::Arc;

use cgmath::Zero;
use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer},
    memory::allocator::MemoryAllocator,
};

use super::{
    camera::Camera,
    cgmath_types::{vec3, Mat4, Quat, Vec3},
    lights::{AmbientLight, DirectionalLight, PointLight},
    mesh::Vertex,
    model::Model,
    Color,
};

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
    bg_color: Color,
    orientation: Orientation,
    lights: SceneLights,
    camera: Camera,
    models: Vec<Model>,
}
impl Scene {
    pub fn new(bg_color: Color, lights: SceneLights, camera: Camera, models: Vec<Model>) -> Self {
        Self {
            bg_color,
            orientation: Orientation::zero(),
            lights,
            camera,
            models,
        }
    }

    pub fn bg_color(&self) -> Color {
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

    pub fn geometry_buffers(
        &self,
        allocator: &impl MemoryAllocator,
    ) -> (
        Arc<CpuAccessibleBuffer<[Vertex]>>,
        Arc<CpuAccessibleBuffer<[u32]>>,
    ) {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        let mut index_offset = 0;
        for model in self.models.iter() {
            for surface in model.surfaces().iter() {
                let surface_vertices = surface.surface().vertices();
                vertices.extend(surface_vertices.iter());

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
