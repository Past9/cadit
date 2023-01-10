use auto_ops::{impl_op_ex, impl_op_ex_commutative};
use std::{
    iter::Sum,
    ops::{Add, Mul, Sub},
};

use super::{Float, Homogeneous, Vec2, Vector, Zero, ZeroHomogeneous};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WPoint3 {
    x: Float,
    y: Float,
    z: Float,
    h: Float,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HPoint3 {
    x: Float,
    y: Float,
    z: Float,
    h: Float,
}
impl HPoint3 {
    pub fn new(x: Float, y: Float, z: Float, h: Float) -> Self {
        Self { x, y, z, h }
    }

    pub fn weight(&self) -> WPoint3 {
        WPoint3 {
            x: self.x * self.h,
            y: self.y * self.h,
            z: self.z * self.h,
            h: self.h,
        }
    }
}
impl Zero for HPoint3 {
    fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            h: 1.0,
        }
    }
}
impl Sub for HPoint3 {
    type Output = HPoint3;

    fn sub(self, rhs: Self) -> Self::Output {
        let xa = self.x;
        let ya = self.y;
        let za = self.z;
        let wa = self.h;

        let xb = rhs.x;
        let yb = rhs.y;
        let zb = rhs.z;
        let wb = rhs.h;

        Self {
            x: wa * xb - wb * xa,
            y: wa * yb - wb * ya,
            z: wa * zb - wb * za,
            h: wa * wb,
        }
    }
}
impl Add for HPoint3 {
    type Output = HPoint3;

    fn add(self, rhs: Self) -> Self::Output {
        let xa = self.x;
        let ya = self.y;
        let za = self.z;
        let wa = self.h;

        let xb = rhs.x;
        let yb = rhs.y;
        let zb = rhs.z;
        let wb = rhs.h;

        Self {
            x: wa * xb + wb * xa,
            y: wa * yb + wb * ya,
            z: wa * zb + wb * za,
            h: wa * wb,
        }
    }
}
impl Mul<Float> for HPoint3 {
    type Output = HPoint3;

    fn mul(self, rhs: Float) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            h: self.h * rhs,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point3 {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}
impl Point3 {
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
impl Vector for Point3 {
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

    fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}
impl Homogeneous<Vec2> for Point3 {
    fn homogeneous_component(&self) -> Float {
        self.z
    }

    fn cartesian_components(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    fn from_cartesian(cartesian: Vec2, homogeneous: Float) -> Self {
        Point3::new(cartesian.x, cartesian.y, homogeneous)
    }
}
impl ZeroHomogeneous for Point3 {
    fn zero_h() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }
}
impl Zero for Point3 {
    fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}
impl From<Point3> for [f32; 3] {
    fn from(vec: Point3) -> Self {
        [vec.x as f32, vec.y as f32, vec.z as f32]
    }
}
impl Sum for Point3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Point3::zero(), |a, b| a + b)
    }
}
impl Default for Point3 {
    fn default() -> Self {
        Self::zero()
    }
}
impl std::fmt::Display for Point3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {}, {})", self.x, self.y, self.z))
    }
}

// Vec3/Vec3 operators
// Add
impl_op_ex!(+ |a: &Point3, b: &Point3| -> Point3 {
    Point3 {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z
    }
});
impl_op_ex!(+= |a: &mut Point3, b: &Point3| {
    a.x += b.x;
    a.y += b.y;
    a.z += b.z;
});

// Subtract
impl_op_ex!(-|a: &Point3, b: &Point3| -> Point3 {
    Point3 {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
    }
});
impl_op_ex!(-= |a: &mut Point3, b: &Point3| {
    a.x -= b.x;
    a.y -= b.y;
    a.z -= b.z;
});

// Multiply
impl_op_ex!(*|a: &Point3, b: &Point3| -> Point3 {
    Point3 {
        x: a.x * b.x,
        y: a.y * b.y,
        z: a.z * b.z,
    }
});
impl_op_ex!(*= |a: &mut Point3, b: &Point3| {
    a.x *= b.x;
    a.y *= b.y;
    a.z *= b.z;
});

// Divide
impl_op_ex!(/ |a: &Point3, b: &Point3| -> Point3 {
    Point3 {
        x: a.x / b.x,
        y: a.y / b.y,
        z: a.z / b.z,
    }
});
impl_op_ex!(/= |a: &mut Point3, b: &Point3| {
    a.x /= b.x;
    a.y /= b.y;
    a.z /= b.z;
});

// Vec3/Float operators
// Add
impl_op_ex_commutative!(+ |a: &Point3, b: &Float| -> Point3 {
    Point3 {
        x: a.x + b,
        y: a.y + b,
        z: a.z + b
    }
});
impl_op_ex!(+= |a: &mut Point3, b: &Float| {
    a.x += b;
    a.y += b;
    a.z += b;
});

// Subtract
impl_op_ex!(-|a: &Point3, b: &Float| -> Point3 {
    Point3 {
        x: a.x - b,
        y: a.y - b,
        z: a.z - b,
    }
});
impl_op_ex!(-= |a: &mut Point3, b: &Float| {
    a.x -= b;
    a.y -= b;
    a.z -= b;
});

// Multiply
impl_op_ex_commutative!(*|a: &Point3, b: &Float| -> Point3 {
    Point3 {
        x: a.x * b,
        y: a.y * b,
        z: a.z * b,
    }
});
impl_op_ex!(*= |a: &mut Point3, b: &Float| {
    a.x *= b;
    a.y *= b;
    a.z *= b;
});

// Divide
impl_op_ex!(/ |a: &Point3, b: &Float| -> Point3 {
    Point3 {
        x: a.x / b,
        y: a.y / b,
        z: a.z / b,
    }
});
impl_op_ex!(/= |a: &mut Point3, b: &Float| {
    a.x /= b;
    a.y /= b;
    a.z /= b;
});
