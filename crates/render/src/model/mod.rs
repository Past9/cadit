use super::{
    mesh::{Edge, EdgeVertex, Point, Surface, SurfaceVertex},
    Rgba,
};
use bytemuck::{Pod, Zeroable};

mod material;

pub use material::*;

#[derive(Clone, Debug)]
pub struct ModelObjectId(u32);
impl From<u32> for ModelObjectId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Zeroable, Pod)]
pub struct BufferedSurfaceVertex {
    position: [f32; 3],
    normal: [f32; 3],
    material_idx: u32,
}
impl BufferedSurfaceVertex {
    pub fn new(vertex: &SurfaceVertex, material_idx: u32) -> Self {
        Self {
            position: vertex.position.clone(),
            normal: vertex.normal.clone(),
            material_idx,
        }
    }
}
vulkano::impl_vertex!(BufferedSurfaceVertex, position, normal, material_idx);

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Zeroable, Pod)]
pub struct BufferedEdgeVertex {
    position: [f32; 3],
    expand: [f32; 3],
    color: [f32; 4],
}
impl BufferedEdgeVertex {
    pub fn new(vertex: &EdgeVertex, color: Rgba) -> Self {
        Self {
            position: vertex.position.clone(),
            expand: vertex.expand.clone(),
            color: color.to_floats(),
        }
    }
}
vulkano::impl_vertex!(BufferedEdgeVertex, position, expand, color);

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Zeroable, Pod)]
pub struct BufferedPointVertex {
    position: [f32; 3],
    expand: [f32; 3],
    color: [f32; 4],
}
impl BufferedPointVertex {
    pub fn new(vertex: &Point, color: Rgba) -> Self {
        Self {
            position: vertex.position.clone(),
            expand: vertex.expand.clone(),
            color: color.to_floats(),
        }
    }
}
vulkano::impl_vertex!(BufferedPointVertex, position, expand, color);

#[derive(Clone)]
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

#[derive(Clone)]
pub struct ModelEdge {
    id: ModelObjectId,
    edge: Edge,
    color: Rgba,
}
impl ModelEdge {
    pub fn new(id: ModelObjectId, edge: Edge, color: Rgba) -> Self {
        Self { id, edge, color }
    }

    pub fn edge(&self) -> &Edge {
        &self.edge
    }

    pub fn color(&self) -> Rgba {
        self.color
    }
}

#[derive(Clone)]
pub struct ModelPoint {
    id: ModelObjectId,
    point: Point,
    color: Rgba,
}
impl ModelPoint {
    pub fn new(id: ModelObjectId, point: Point, color: Rgba) -> Self {
        Self { id, point, color }
    }

    pub fn point(&self) -> &Point {
        &self.point
    }

    pub fn color(&self) -> &Rgba {
        &self.color
    }
}

#[derive(Clone)]
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

    pub fn edges(&self) -> &[ModelEdge] {
        &self.edges
    }

    pub fn points(&self) -> &[ModelPoint] {
        &self.points
    }
}
