use std::slice::Iter;

use crate::math::{Float, Homogeneous, Zero};

#[derive(Clone, Debug)]
pub struct ControlPolygon<H>
where
    H: Clone + Zero,
{
    vertices: Vec<H>,
}
impl<H> ControlPolygon<H>
where
    H: Clone + Zero,
{
    pub fn new<const N: usize>(vertices: [H; N]) -> Self {
        Self {
            vertices: vertices.to_vec(),
        }
    }

    pub fn from_slice(vertices: &[H]) -> Self {
        Self {
            vertices: vertices.to_vec(),
        }
    }

    pub fn zeros(size: usize) -> Self {
        Self {
            vertices: vec![H::zero(); size],
        }
    }

    pub fn len(&self) -> usize {
        self.vertices.len()
    }

    pub fn iter(&self) -> Iter<H> {
        self.vertices.iter()
    }

    pub fn to_weighted<C>(&self) -> Self
    where
        H: Copy
            + Clone
            + Zero
            + std::ops::Mul<Float, Output = H>
            + std::ops::Add<Float, Output = H>
            + std::ops::Add<H, Output = H>
            + Homogeneous<C>,
        C: Copy + Clone + std::ops::Mul<Float, Output = C> + std::ops::Div<Float, Output = C>,
    {
        ControlPolygon::from_iter(self.vertices.iter().map(|v| v.to_weighted()))
    }

    pub fn to_unweighted<C>(&self) -> Self
    where
        H: Copy
            + Clone
            + Zero
            + std::ops::Mul<Float, Output = H>
            + std::ops::Add<Float, Output = H>
            + std::ops::Add<H, Output = H>
            + Homogeneous<C>,
        C: Copy + Clone + std::ops::Mul<Float, Output = C> + std::ops::Div<Float, Output = C>,
    {
        ControlPolygon::from_iter(self.vertices.iter().map(|v| v.to_unweighted()))
    }

    pub fn to_cartesian<C>(&self) -> ControlPolygon<C>
    where
        H: Copy
            + Clone
            + Zero
            + std::ops::Mul<Float, Output = H>
            + std::ops::Add<Float, Output = H>
            + std::ops::Add<H, Output = H>
            + Homogeneous<C>,
        C: Copy
            + Clone
            + std::ops::Mul<Float, Output = C>
            + std::ops::Div<Float, Output = C>
            + std::ops::Add<Float, Output = C>
            + std::ops::Add<C, Output = C>
            + Zero,
    {
        ControlPolygon::from_iter(self.vertices.iter().map(|v| v.to_cartesian()))
    }
}
impl<H> FromIterator<H> for ControlPolygon<H>
where
    H: Copy
        + Clone
        + Zero
        + std::ops::Mul<Float, Output = H>
        + std::ops::Add<Float, Output = H>
        + std::ops::Add<H, Output = H>,
{
    fn from_iter<I: IntoIterator<Item = H>>(vertices: I) -> Self {
        Self {
            vertices: Vec::from_iter(vertices),
        }
    }
}
impl<H, Idx> std::ops::Index<Idx> for ControlPolygon<H>
where
    Idx: std::slice::SliceIndex<[H]>,
    H: Copy
        + Clone
        + Zero
        + std::ops::Mul<Float, Output = H>
        + std::ops::Add<Float, Output = H>
        + std::ops::Add<H, Output = H>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.vertices[index]
    }
}
impl<H, Idx> std::ops::IndexMut<Idx> for ControlPolygon<H>
where
    Idx: std::slice::SliceIndex<[H]>,
    H: Copy
        + Clone
        + Zero
        + std::ops::Mul<Float, Output = H>
        + std::ops::Add<Float, Output = H>
        + std::ops::Add<H, Output = H>,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.vertices[index]
    }
}

#[derive(Clone, Debug)]
pub struct ControlMesh<H>
where
    H: Copy
        + Clone
        + Zero
        + std::ops::Mul<Float, Output = H>
        + std::ops::Add<Float, Output = H>
        + std::ops::Add<H, Output = H>,
{
    rows: Vec<ControlPolygon<H>>,
}
impl<H> ControlMesh<H>
where
    H: Copy
        + Clone
        + Zero
        + std::ops::Mul<Float, Output = H>
        + std::ops::Add<Float, Output = H>
        + std::ops::Add<H, Output = H>,
{
    pub fn new<const N: usize>(rows: [ControlPolygon<H>; N]) -> Self {
        Self {
            rows: rows.to_vec(),
        }
    }

    pub fn from_slice(rows: &[ControlPolygon<H>]) -> Self {
        Self {
            rows: rows.to_vec(),
        }
    }

    pub fn zeros(u_size: usize, v_size: usize) -> Self {
        Self {
            rows: vec![ControlPolygon::zeros(v_size); u_size],
        }
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn iter(&self) -> Iter<ControlPolygon<H>> {
        self.rows.iter()
    }

    pub fn to_weighted<C>(&self) -> Self
    where
        H: Copy
            + Clone
            + Zero
            + std::ops::Mul<Float, Output = H>
            + std::ops::Add<Float, Output = H>
            + std::ops::Add<H, Output = H>
            + Homogeneous<C>,
        C: Copy + Clone + std::ops::Mul<Float, Output = C> + std::ops::Div<Float, Output = C>,
    {
        ControlMesh::from_iter(self.rows.iter().map(|v| v.to_weighted()))
    }

    pub fn to_unweighted<C>(&self) -> Self
    where
        H: Copy
            + Clone
            + Zero
            + std::ops::Mul<Float, Output = H>
            + std::ops::Add<Float, Output = H>
            + std::ops::Add<H, Output = H>
            + Homogeneous<C>,
        C: Copy + Clone + std::ops::Mul<Float, Output = C> + std::ops::Div<Float, Output = C>,
    {
        ControlMesh::from_iter(self.rows.iter().map(|v| v.to_unweighted()))
    }

    pub fn to_cartesian<C>(&self) -> ControlMesh<C>
    where
        H: Copy
            + Clone
            + Zero
            + std::ops::Mul<Float, Output = H>
            + std::ops::Add<Float, Output = H>
            + std::ops::Add<H, Output = H>
            + Homogeneous<C>,
        C: Copy
            + Clone
            + std::ops::Mul<Float, Output = C>
            + std::ops::Div<Float, Output = C>
            + std::ops::Add<Float, Output = C>
            + std::ops::Add<C, Output = C>
            + Zero,
    {
        ControlMesh::from_iter(self.rows.iter().map(|v| v.to_cartesian()))
    }
}
impl<H> FromIterator<ControlPolygon<H>> for ControlMesh<H>
where
    H: Copy
        + Clone
        + Zero
        + std::ops::Mul<Float, Output = H>
        + std::ops::Add<Float, Output = H>
        + std::ops::Add<H, Output = H>,
{
    fn from_iter<I: IntoIterator<Item = ControlPolygon<H>>>(rows: I) -> Self {
        Self {
            rows: Vec::from_iter(rows),
        }
    }
}
impl<H, Idx> std::ops::Index<Idx> for ControlMesh<H>
where
    Idx: std::slice::SliceIndex<[ControlPolygon<H>]>,
    H: Copy
        + Clone
        + Zero
        + std::ops::Mul<Float, Output = H>
        + std::ops::Add<Float, Output = H>
        + std::ops::Add<H, Output = H>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.rows[index]
    }
}
impl<H, Idx> std::ops::IndexMut<Idx> for ControlMesh<H>
where
    Idx: std::slice::SliceIndex<[ControlPolygon<H>]>,
    H: Copy
        + Clone
        + Zero
        + std::ops::Mul<Float, Output = H>
        + std::ops::Add<Float, Output = H>
        + std::ops::Add<H, Output = H>,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.rows[index]
    }
}
