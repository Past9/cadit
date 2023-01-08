use auto_ops::{impl_op_ex, impl_op_ex_commutative};
use std::iter::Sum;

use super::{Float, Homogeneous, Vec2, Vector, Zero};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}
impl Vec3 {
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Self { x, y, z }
    }

    pub fn magnitude(&self) -> Float {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn to_f32_array(&self) -> [f32; 3] {
        [self.x as f32, self.y as f32, self.z as f32]
    }

    pub fn to_f64_array(&self) -> [f64; 3] {
        [self.x as f64, self.y as f64, self.z as f64]
    }
}
impl Vector for Vec3 {
    fn magnitude(&self) -> Float {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    fn normalize(&self) -> Self {
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }

    fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.x * other.z - self.z * other.x,
            z: self.x * other.y - self.y * other.x,
        }
    }
}
impl Homogeneous<Vec2> for Vec3 {
    fn homogeneous_component(&self) -> Float {
        self.z
    }

    fn cartesian_components(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    fn from_cartesian(cartesian: Vec2, homogeneous: Float) -> Self {
        Vec3::new(cartesian.x, cartesian.y, homogeneous)
    }
}
impl Zero for Vec3 {
    fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}
impl From<Vec3> for [f32; 3] {
    fn from(vec: Vec3) -> Self {
        [vec.x as f32, vec.y as f32, vec.z as f32]
    }
}
impl Sum for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vec3::zero(), |a, b| a + b)
    }
}
impl Default for Vec3 {
    fn default() -> Self {
        Self::zero()
    }
}
impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {}, {})", self.x, self.y, self.z))
    }
}

// Vec3/Vec3 operators
// Add
impl_op_ex!(+ |a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3 {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z
    }
});
impl_op_ex!(+= |a: &mut Vec3, b: &Vec3| {
    a.x += b.x;
    a.y += b.y;
    a.z += b.z;
});

// Subtract
impl_op_ex!(-|a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3 {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
    }
});
impl_op_ex!(-= |a: &mut Vec3, b: &Vec3| {
    a.x -= b.x;
    a.y -= b.y;
    a.z -= b.z;
});

// Multiply
impl_op_ex!(*|a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3 {
        x: a.x * b.x,
        y: a.y * b.y,
        z: a.z * b.z,
    }
});
impl_op_ex!(*= |a: &mut Vec3, b: &Vec3| {
    a.x *= b.x;
    a.y *= b.y;
    a.z *= b.z;
});

// Divide
impl_op_ex!(/ |a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3 {
        x: a.x / b.x,
        y: a.y / b.y,
        z: a.z / b.z,
    }
});
impl_op_ex!(/= |a: &mut Vec3, b: &Vec3| {
    a.x /= b.x;
    a.y /= b.y;
    a.z /= b.z;
});

// Vec3/Float operators
// Add
impl_op_ex_commutative!(+ |a: &Vec3, b: &Float| -> Vec3 {
    Vec3 {
        x: a.x + b,
        y: a.y + b,
        z: a.z + b
    }
});
impl_op_ex!(+= |a: &mut Vec3, b: &Float| {
    a.x += b;
    a.y += b;
    a.z += b;
});

// Subtract
impl_op_ex!(-|a: &Vec3, b: &Float| -> Vec3 {
    Vec3 {
        x: a.x - b,
        y: a.y - b,
        z: a.z - b,
    }
});
impl_op_ex!(-= |a: &mut Vec3, b: &Float| {
    a.x -= b;
    a.y -= b;
    a.z -= b;
});

// Multiply
impl_op_ex_commutative!(*|a: &Vec3, b: &Float| -> Vec3 {
    Vec3 {
        x: a.x * b,
        y: a.y * b,
        z: a.z * b,
    }
});
impl_op_ex!(*= |a: &mut Vec3, b: &Float| {
    a.x *= b;
    a.y *= b;
    a.z *= b;
});

// Divide
impl_op_ex!(/ |a: &Vec3, b: &Float| -> Vec3 {
    Vec3 {
        x: a.x / b,
        y: a.y / b,
        z: a.z / b,
    }
});
impl_op_ex!(/= |a: &mut Vec3, b: &Float| {
    a.x /= b;
    a.y /= b;
    a.z /= b;
});
