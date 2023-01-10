use std::slice::Iter;

use crate::math::{Float, HPoint3, Homogeneous, Point3, WPoint3, Zero, ZeroHomogeneous};

#[derive(Clone, Debug)]
pub struct ControlPolygon {
    vertices: Vec<HPoint3>,
}
impl ControlPolygon {
    pub fn new<const N: usize>(vertices: [HPoint3; N]) -> Self {
        Self {
            vertices: vertices.to_vec(),
        }
    }

    pub fn truncate(&mut self, len: usize) {
        self.vertices.truncate(len);
    }

    pub fn from_slice(vertices: &[HPoint3]) -> Self {
        Self {
            vertices: vertices.to_vec(),
        }
    }

    pub fn zeros(size: usize) -> Self {
        Self {
            vertices: vec![HPoint3::zero(); size],
        }
    }

    pub fn len(&self) -> usize {
        self.vertices.len()
    }

    pub fn iter(&self) -> Iter<HPoint3> {
        self.vertices.iter()
    }

    pub fn weight(&self) -> Vec<WPoint3> {
        self.vertices.iter().map(|v| v.weight()).collect()
    }

    pub fn to_cartesian(&self) -> Vec<Point3> {
        //ControlPolygon::from_iter(self.vertices.iter().map(|v| v.to_cartesian()))
        todo!()
    }
}
impl FromIterator<HPoint3> for ControlPolygon {
    fn from_iter<I: IntoIterator<Item = HPoint3>>(vertices: I) -> Self {
        Self {
            vertices: Vec::from_iter(vertices),
        }
    }
}
impl<Idx> std::ops::Index<Idx> for ControlPolygon
where
    Idx: std::slice::SliceIndex<[HPoint3]>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.vertices[index]
    }
}
impl<Idx> std::ops::IndexMut<Idx> for ControlPolygon
where
    Idx: std::slice::SliceIndex<[HPoint3]>,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.vertices[index]
    }
}

#[derive(Clone, Debug)]
pub struct ControlMesh {
    rows: Vec<ControlPolygon>,
}
impl ControlMesh {
    pub fn new<const N: usize>(rows: [ControlPolygon; N]) -> Self {
        Self {
            rows: rows.to_vec(),
        }
    }

    pub fn from_slice(rows: &[ControlPolygon]) -> Self {
        Self {
            rows: rows.to_vec(),
        }
    }

    pub fn zeros_h(u_size: usize, v_size: usize) -> Self {
        Self {
            rows: vec![ControlPolygon::zeros(v_size); u_size],
        }
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn iter(&self) -> Iter<ControlPolygon> {
        self.rows.iter()
    }

    pub fn weight(&self) -> Vec<Vec<WPoint3>> {
        self.rows.iter().map(|v| v.weight()).collect()
    }

    pub fn to_unweighted(&self) -> Self {
        ControlMesh::from_iter(self.rows.iter().map(|v| v.to_unweighted()))
    }

    pub fn to_cartesian(&self) -> ControlMesh {
        ControlMesh::from_iter(self.rows.iter().map(|v| v.to_cartesian()))
    }
}
impl FromIterator<ControlPolygon> for ControlMesh {
    fn from_iter<I: IntoIterator<Item = ControlPolygon>>(rows: I) -> Self {
        Self {
            rows: Vec::from_iter(rows),
        }
    }
}
impl<Idx> std::ops::Index<Idx> for ControlMesh
where
    Idx: std::slice::SliceIndex<[ControlPolygon]>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.rows[index]
    }
}
impl<Idx> std::ops::IndexMut<Idx> for ControlMesh
where
    Idx: std::slice::SliceIndex<[ControlPolygon]>,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.rows[index]
    }
}
