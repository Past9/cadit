use auto_ops::{impl_op_ex, impl_op_ex_commutative};
use std::iter::Sum;

use super::{Float, Homogeneous, Vec3, Zero};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec4 {
    pub x: Float,
    pub y: Float,
    pub z: Float,
    pub w: Float,
}
impl Vec4 {
    pub fn new(x: Float, y: Float, z: Float, w: Float) -> Self {
        Self { x, y, z, w }
    }

    pub fn xyz(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}
impl Homogeneous<Vec3> for Vec4 {
    fn homogeneous_component(&self) -> Float {
        self.w
    }

    fn cartesian_components(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }

    fn from_cartesian(cartesian: Vec3, homogeneous: Float) -> Self {
        Vec4::new(cartesian.x, cartesian.y, cartesian.z, homogeneous)
    }
}
impl Zero for Vec4 {
    fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
}
impl From<Vec4> for [f32; 4] {
    fn from(vec: Vec4) -> Self {
        [vec.x as f32, vec.y as f32, vec.z as f32, vec.w as f32]
    }
}
impl Sum for Vec4 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vec4::zero(), |a, b| a + b)
    }
}

// Vec4/Vec4 operators
// Add
impl_op_ex!(+ |a: &Vec4, b: &Vec4| -> Vec4 {
    Vec4 {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
        w: a.w + b.w
    }
});
impl_op_ex!(+= |a: &mut Vec4, b: &Vec4| {
    a.x += b.x;
    a.y += b.y;
    a.z += b.z;
    a.w += b.w;
});

// Subtract
impl_op_ex!(-|a: &Vec4, b: &Vec4| -> Vec4 {
    Vec4 {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
        w: a.w - b.w,
    }
});
impl_op_ex!(-= |a: &mut Vec4, b: &Vec4| {
    a.x -= b.x;
    a.y -= b.y;
    a.z -= b.z;
    a.w -= b.w;
});

// Multiply
impl_op_ex!(*|a: &Vec4, b: &Vec4| -> Vec4 {
    Vec4 {
        x: a.x * b.x,
        y: a.y * b.y,
        z: a.z * b.z,
        w: a.w * b.w,
    }
});
impl_op_ex!(*= |a: &mut Vec4, b: &Vec4| {
    a.x *= b.x;
    a.y *= b.y;
    a.z *= b.z;
    a.w *= b.w;
});

// Divide
impl_op_ex!(/ |a: &Vec4, b: &Vec4| -> Vec4 {
    Vec4 {
        x: a.x / b.x,
        y: a.y / b.y,
        z: a.z / b.z,
        w: a.w / b.w,
    }
});
impl_op_ex!(/= |a: &mut Vec4, b: &Vec4| {
    a.x /= b.x;
    a.y /= b.y;
    a.z /= b.z;
    a.w /= b.w;
});

// Vec4/Float operators
// Add
impl_op_ex_commutative!(+ |a: &Vec4, b: &Float| -> Vec4 {
    Vec4 {
        x: a.x + b,
        y: a.y + b,
        z: a.z + b,
        w: a.w + b
    }
});
impl_op_ex!(+= |a: &mut Vec4, b: &Float| {
    a.x += b;
    a.y += b;
    a.z += b;
    a.w += b;
});

// Subtract
impl_op_ex!(-|a: &Vec4, b: &Float| -> Vec4 {
    Vec4 {
        x: a.x - b,
        y: a.y - b,
        z: a.z - b,
        w: a.w - b,
    }
});
impl_op_ex!(-= |a: &mut Vec4, b: &Float| {
    a.x -= b;
    a.y -= b;
    a.z -= b;
    a.w -= b;
});

// Multiply
impl_op_ex_commutative!(*|a: &Vec4, b: &Float| -> Vec4 {
    Vec4 {
        x: a.x * b,
        y: a.y * b,
        z: a.z * b,
        w: a.w * b,
    }
});
impl_op_ex!(*= |a: &mut Vec4, b: &Float| {
    a.x *= b;
    a.y *= b;
    a.z *= b;
    a.w *= b;
});

// Divide
impl_op_ex!(/ |a: &Vec4, b: &Float| -> Vec4 {
    Vec4 {
        x: a.x / b,
        y: a.y / b,
        z: a.z / b,
        w: a.w / b,
    }
});
impl_op_ex!(/= |a: &mut Vec4, b: &Float| {
    a.x /= b;
    a.y /= b;
    a.z /= b;
    a.w /= b;
});
