use auto_ops::{impl_op_ex, impl_op_ex_commutative};
use std::iter::Sum;

use super::{Float, Zero};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2 {
    pub x: Float,
    pub y: Float,
}
impl Vec2 {
    pub fn new(x: Float, y: Float) -> Self {
        Self { x, y }
    }

    pub fn magnitude(&self) -> Float {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
impl Zero for Vec2 {
    fn zero() -> Self {
        Self::new(0.0, 0.0)
    }
}
impl From<Vec2> for [f32; 2] {
    fn from(vec: Vec2) -> Self {
        [vec.x as f32, vec.y as f32]
    }
}
impl Sum for Vec2 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vec2::zero(), |a, b| a + b)
    }
}
impl Default for Vec2 {
    fn default() -> Self {
        Self::zero()
    }
}
impl std::fmt::Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.x, self.y))
    }
}

// Vec2/Vec2 operators
// Add
impl_op_ex!(+ |a: &Vec2, b: &Vec2| -> Vec2 {
    Vec2 {
        x: a.x + b.x,
        y: a.y + b.y,
    }
});
impl_op_ex!(+= |a: &mut Vec2, b: &Vec2| {
    a.x += b.x;
    a.y += b.y;
});

// Subtract
impl_op_ex!(-|a: &Vec2, b: &Vec2| -> Vec2 {
    Vec2 {
        x: a.x - b.x,
        y: a.y - b.y,
    }
});
impl_op_ex!(-= |a: &mut Vec2, b: &Vec2| {
    a.x -= b.x;
    a.y -= b.y;
});

// Multiply
impl_op_ex!(*|a: &Vec2, b: &Vec2| -> Vec2 {
    Vec2 {
        x: a.x * b.x,
        y: a.y * b.y,
    }
});
impl_op_ex!(*= |a: &mut Vec2, b: &Vec2| {
    a.x *= b.x;
    a.y *= b.y;
});

// Divide
impl_op_ex!(/ |a: &Vec2, b: &Vec2| -> Vec2 {
    Vec2 {
        x: a.x / b.x,
        y: a.y / b.y,
    }
});
impl_op_ex!(/= |a: &mut Vec2, b: &Vec2| {
    a.x /= b.x;
    a.y /= b.y;
});

// Vec2/Float operators
// Add
impl_op_ex_commutative!(+ |a: &Vec2, b: &Float| -> Vec2 {
    Vec2 {
        x: a.x + b,
        y: a.y + b,
    }
});
impl_op_ex!(+= |a: &mut Vec2, b: &Float| {
    a.x += b;
    a.y += b;
});

// Subtract
impl_op_ex!(-|a: &Vec2, b: &Float| -> Vec2 {
    Vec2 {
        x: a.x - b,
        y: a.y - b,
    }
});
impl_op_ex!(-= |a: &mut Vec2, b: &Float| {
    a.x -= b;
    a.y -= b;
});

// Multiply
impl_op_ex_commutative!(*|a: &Vec2, b: &Float| -> Vec2 {
    Vec2 {
        x: a.x * b,
        y: a.y * b,
    }
});
impl_op_ex!(*= |a: &mut Vec2, b: &Float| {
    a.x *= b;
    a.y *= b;
});

// Divide
impl_op_ex!(/ |a: &Vec2, b: &Float| -> Vec2 {
    Vec2 {
        x: a.x / b,
        y: a.y / b,
    }
});
impl_op_ex!(/= |a: &mut Vec2, b: &Float| {
    a.x /= b;
    a.y /= b;
});
