macro_rules! count_args {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + crate::math::vector_macros::count_args!($($xs)*));
}

macro_rules! impl_vector {
    ( $typ:ident, $( $comp:ident ),* ) => {
        crate::math::vector_macros::vector_ops!($typ, $($comp),*);
        crate::math::vector_macros::vector_arithmetic!($typ, $($comp),*);
    };
}

macro_rules! vector_ops {
    ( $typ:ident, $( $comp:ident ),* ) => {
        impl $typ {
            pub fn new(
                $(
                    $comp: f64,
                )*
            ) -> Self {
                Self {
                    $(
                        $comp,
                    )*
                }
            }

            pub fn f32s(&self) -> [f32; crate::math::vector_macros::count_args!($($comp)*)] {
                [
                    $(
                        self.$comp as f32,
                    )*
                ]
            }
        }
        impl crate::math::vector::Vector for $typ {
            fn zero() -> Self {
                Self {
                    $(
                        $comp: 0.0,
                    )*
                }
            }

            fn dot(&self, rhs: &Self) -> f64 {
                0.0 $(
                    + (self.$comp * rhs.$comp)
                )*
            }
        }
        impl std::iter::Sum for $typ {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(super::Vector::zero(), |a, b| a + b)
            }
        }
    };
}

macro_rules! vector_arithmetic {
    ( $typ:ident, $( $comp:ident ),* ) => {
        // Vec / Vec operations
        auto_ops::impl_op_ex!(+ |a: &$typ, b: &$typ| -> $typ {
            $typ {
                $(
                    $comp: a.$comp + b.$comp,
                )*
            }
        });

        auto_ops::impl_op_ex!(+= |a: &mut $typ, b: &$typ| {
            a.x += b.x;
        });

        auto_ops::impl_op_ex!(-|a: &$typ, b: &$typ| -> $typ {
            $typ {
                $(
                    $comp: a.$comp - b.$comp,
                )*
            }
        });

        auto_ops::impl_op_ex!(-= |a: &mut $typ, b: &$typ| {
            $(
                a.$comp -= b.$comp;
            )*
        });

        auto_ops::impl_op_ex!(*|a: &$typ, b: &$typ| -> $typ {
            $typ {
                $(
                    $comp: a.$comp * b.$comp,
                )*
            }
        });

        auto_ops::impl_op_ex!(*= |a: &mut $typ, b: &$typ| {
            $(
                a.$comp *= b.$comp;
            )*
        });

        auto_ops::impl_op_ex!(/ |a: &$typ, b: &$typ| -> $typ {
            $typ {
                $(
                    $comp: a.$comp /  b.$comp,
                )*
            }
        });

        auto_ops::impl_op_ex!(/= |a: &mut $typ, b: &$typ| {
            $(
                a.$comp /= b.$comp;
            )*
        });

        // Vec / Float operations
        auto_ops::impl_op_ex_commutative!(+ |a: &$typ, b: &f64| -> $typ {
            $typ {
                $(
                    $comp: a.$comp + b,
                )*
            }
        });

        auto_ops::impl_op_ex!(+= |a: &mut $typ, b: &f64| {
            $(
                a.$comp += b;
            )*
        });

        auto_ops::impl_op_ex_commutative!(-|a: &$typ, b: &f64| -> $typ {
            $typ {
                $(
                    $comp: a.$comp - b,
                )*
            }
        });

        auto_ops::impl_op_ex!(-= |a: &mut $typ, b: &f64| {
            $(
                a.$comp -= b;
            )*
        });

        auto_ops::impl_op_ex_commutative!(*|a: &$typ, b: &f64| -> $typ {
            $typ {
                $(
                    $comp: a.$comp * b,
                )*
            }
        });

        auto_ops::impl_op_ex!(*= |a: &mut $typ, b: &f64| {
            $(
                a.$comp *= b;
            )*
        });

        auto_ops::impl_op_ex!(/|a: &$typ, b: &f64| -> $typ {
            $typ {
                $(
                    $comp: a.$comp / b,
                )*
            }
        });

        auto_ops::impl_op_ex!(/= |a: &mut $typ, b: &f64| {
            $(
                a.$comp /= b;
            )*
        });

    };
}

pub(crate) use count_args;
pub(super) use impl_vector;
pub(super) use vector_arithmetic;
pub(super) use vector_ops;
