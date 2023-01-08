use std::sync::Arc;

use cgmath::{vec3, Matrix4, Quaternion, Vector3, Zero};
use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer},
    memory::allocator::MemoryAllocator,
};

use super::{
    camera::Camera,
    lights::{AmbientLight, DirectionalLight, PointLight},
    model::{BufferedEdgeVertex, BufferedPointVertex, BufferedSurfaceVertex, Material, Model},
    Rgba,
};
use crate::lights::{Std140AmbientLight, Std140DirectionalLight, Std140PointLight};
use crate::model::Std140Material;

#[derive(Clone)]
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

#[derive(Clone, Debug)]
pub struct Orientation {
    offset: Vector3<f32>,
    rotation: Quaternion<f32>,
}
impl Orientation {
    pub fn zero() -> Self {
        Self {
            offset: vec3(0.0, 0.0, 0.0),
            rotation: Quaternion::zero(),
        }
    }

    pub fn set_offset(&mut self, offset: Vector3<f32>) {
        self.offset = offset;
    }

    pub fn set_rotation(&mut self, rotation: Quaternion<f32>) {
        self.rotation = rotation;
    }

    pub fn matrix(&self) -> Matrix4<f32> {
        Matrix4::from_translation(self.offset) * Matrix4::from(self.rotation)
    }
}

#[derive(Clone)]
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

    pub fn point_geometry_buffer(
        &self,
        allocator: &impl MemoryAllocator,
    ) -> Option<Arc<CpuAccessibleBuffer<[BufferedPointVertex]>>> {
        let mut vertices: Vec<BufferedPointVertex> = Vec::new();

        for model in self.models.iter() {
            for point in model.points().iter() {
                // TODO
                let point_vertex = point.point();
                vertices.push(BufferedPointVertex::new(point_vertex));
            }
        }

        if vertices.len() > 0 {
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

            Some(vertex_buffer)
        } else {
            None
        }
    }

    pub fn edge_geometry_buffer(
        &self,
        allocator: &impl MemoryAllocator,
    ) -> Option<Arc<CpuAccessibleBuffer<[BufferedEdgeVertex]>>> {
        let mut vertices: Vec<BufferedEdgeVertex> = Vec::new();

        for model in self.models.iter() {
            for edge in model.edges().iter() {
                // TODO
                let edge_vertices = edge.edge().vertices();
                vertices.extend(
                    edge_vertices
                        .iter()
                        .map(|vert| BufferedEdgeVertex::new(vert, edge.color())),
                );
            }
        }

        if vertices.len() > 0 {
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

            Some(vertex_buffer)
        } else {
            None
        }
    }

    pub fn surface_geometry_buffers(
        &self,
        allocator: &impl MemoryAllocator,
    ) -> (
        Option<Arc<CpuAccessibleBuffer<[BufferedSurfaceVertex]>>>,
        Option<Arc<CpuAccessibleBuffer<[u32]>>>,
    ) {
        let mut vertices: Vec<BufferedSurfaceVertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        let mut index_offset = 0;
        for model in self.models.iter() {
            for surface in model.surfaces().iter() {
                let surface_vertices = surface.surface().vertices();
                vertices.extend(
                    surface_vertices
                        .iter()
                        .map(|vert| BufferedSurfaceVertex::new(vert, surface.material_idx())),
                );

                let surface_indices = surface.surface().indices();
                indices.extend(surface_indices.iter().map(|i| i + index_offset));

                index_offset += surface_vertices.len() as u32;
            }
        }

        if vertices.len() > 0 && indices.len() > 0 {
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

            (Some(vertex_buffer), Some(index_buffer))
        } else {
            (None, None)
        }
    }
}
