use bytemuck::{Pod, Zeroable};
use cgmath::{Point3, Vector3};

#[derive(Default, Debug, Copy, Clone)]
pub struct SurfaceVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}
impl SurfaceVertex {
    pub fn new(position: Point3<f32>, normal: Vector3<f32>) -> Self {
        Self {
            position: [position.x, position.y, position.z],
            normal: [normal.x, normal.y, normal.z],
        }
    }
}

#[derive(Clone, Debug)]
pub struct Surface {
    pub vertices: Vec<SurfaceVertex>,
    pub indices: Vec<u32>,
}
impl Surface {
    pub fn new<const V: usize, const I: usize>(
        vertices: [SurfaceVertex; V],
        indices: [u32; I],
    ) -> Self {
        Self {
            vertices: Vec::from_iter(vertices.into_iter()),
            indices: Vec::from_iter(indices.into_iter()),
        }
    }

    pub fn vertices(&self) -> &[SurfaceVertex] {
        &self.vertices
    }

    pub fn indices(&self) -> &[u32] {
        &self.indices
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct EdgeVertex {
    pub position: [f32; 3],
    pub expand: [f32; 3],
}
impl EdgeVertex {
    pub fn new(position: Point3<f32>, expand: Vector3<f32>) -> Self {
        Self {
            position: [position.x, position.y, position.z],
            expand: [expand.x, expand.y, expand.z],
        }
    }
}

#[derive(Clone, Debug)]
pub struct Edge {
    pub vertices: Vec<EdgeVertex>,
}
impl Edge {
    pub fn new<const V: usize>(vertices: [EdgeVertex; V]) -> Self {
        Self {
            vertices: Vec::from_iter(vertices.into_iter()),
        }
    }

    pub fn vertices(&self) -> &[EdgeVertex] {
        &self.vertices
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Point {
    pub position: [f32; 3],
    pub expand: [f32; 3],
}
impl Point {
    pub fn new(position: Point3<f32>, expand: Vector3<f32>) -> Self {
        Self {
            position: [position.x, position.y, position.z],
            expand: [expand.x, expand.y, expand.z],
        }
    }
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
