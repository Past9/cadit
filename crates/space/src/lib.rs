mod eline;
mod espace;
mod evector;
mod hspace;
mod hvector;

pub use eline::*;
pub use espace::*;
pub use evector::*;
pub use hspace::*;
pub use hvector::*;

pub const TOL: f64 = 0.0000001;

macro_rules! count_args {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + crate::count_args!($($xs)*));
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
            $(
                a.$comp += b.$comp;
            )*
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
pub(crate) use vector_arithmetic;
