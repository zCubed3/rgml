#![allow(unused)]
#![allow(dead_code)]

use std::cmp::*;
use std::fmt::*;
use std::ops::*;
use std::iter::*;

use crate::real::RealNumber;

#[cfg(feature="swizzle")]
pub mod swizzle_gen;

#[cfg(feature="swizzle")]
pub use swizzle_gen::*;

#[cfg(feature="serialization")]
use serde::Serialize;

///
/// Configurable vector type for usage with Vector math
///
/// A vector is simply a wrapper for an array of the given real type and count
/// Supports any real that can be implemented as a [Real] trait
///
#[cfg_attr(feature="serialization", derive(Serialize))]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vector<T: RealNumber, const COUNT: usize> {
    /// The underlying array of the vector, the vector dereferences into this array
    #[cfg_attr(feature="serialization", serde(with = "serde_arrays"))]
    pub underlying: [T; COUNT],
}

impl<T: RealNumber, const COUNT: usize> Vector<T, COUNT> {
    /// Creates a new [Vector] by copying the given array into the backing array
    pub fn from_array(array: [T; COUNT]) -> Self {
        Vector { underlying: array }
    }

    /// Creates a new [Vector] by copying the provided value into each element
    pub fn from_scalar(value: T) -> Self {
        Vector { underlying: [value; COUNT] }
    }

    /// Returns the sum of all components ([Real]) within this [Vector]
    pub fn sum(&self) -> T {
        let mut sum = T::default();

        for c in 0..COUNT {
            sum += self[c];
        }

        return sum;
    }

    /// The length of this [Vector], not to be confused with [Vector::sum]!
    pub fn magnitude(&self) -> T {
        return self.dot(*self).real_sqrt();
    }

    /// Returns the normalized version of this [Vector]
    pub fn normalize(&self) -> Self {
        return *self / self.magnitude();
    }

    /// Returns the dot product of this [Vector] and another
    pub fn dot(&self, rhs: Self) -> T {
        let mut d = T::default();

        for c in 0..COUNT {
            d += self[c] * rhs[c];
        }

        return d;
    }

    pub fn lerp(&self, to: Self, alpha: T) -> Self {
        let mut a = *self;

        for c in 0..COUNT {
            a[c] = self[c].real_lerp(to[c], alpha)
        }

        return a;
    }
}

//
// Default
//
impl<T: RealNumber, const COUNT: usize> Default for Vector<T, COUNT> {
    fn default() -> Self {
        Self { underlying: [T::default(); COUNT] }
    }
}

//
// Deref
//
/// Deref to allow the Vector to be treated as its underlying backing array
impl<T: RealNumber, const COUNT: usize> Deref for Vector<T, COUNT> {
    type Target = [T; COUNT];

    fn deref(&self) -> &Self::Target {
        &self.underlying
    }
}

impl<T: RealNumber, const COUNT: usize> DerefMut for Vector<T, COUNT> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.underlying
    }
}

//
// Formatting Traits
//
impl<T: RealNumber, const COUNT: usize> Debug for Vector<T, COUNT> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector<{}, {}> {{\n", std::any::type_name::<T>(), COUNT).expect("Failed to write!");

        for c in 0..COUNT {
            writeln!(f, "\t[{}] = {}", c, self[c]).expect("Failed to write!");
        }

        write!(f, "}}").expect("Failed to write!");

        Ok(())
    }
}

impl<T: RealNumber, const COUNT: usize> Display for Vector<T, COUNT> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<").expect("Failed to write!");

        let mut first = true;
        for c in self.underlying {
            if !first {
                write!(f, ", ").expect("Failed to write!");
            }

            write!(f, "{c}").expect("Failed to write!");
            first = false;
        }
        write!(f, ">").expect("Failed to write!");

        Ok(())
    }
}

//
// Additional impls
//
macro_rules! vector_per_comp_func {
    ($func:ident, $op:tt, $call:tt) => {
        impl<T: RealNumber, const COUNT: usize> Vector<T, COUNT> {
            pub fn $func(&self) -> Self {
                let mut v = Self::default();

                for c in 0 .. COUNT {
                    v[c] $op self[c].$call();
                }

                return v;
            }
        }
    };
}

macro_rules! vector_comp_comp_func {
    ($func:ident, $op:tt, $call:tt) => {
        impl<T: RealNumber, const COUNT: usize> Vector<T, COUNT> {
            pub fn $func(&self, rhs: Self) -> Self {
                let mut v = Self::default();

                for c in 0 .. COUNT {
                    v[c] $op self[c].$call(rhs[c]);
                }

                return v;
            }
        }
    };
}

vector_comp_comp_func!(min, =, real_min);
vector_comp_comp_func!(max, =, real_max);
vector_comp_comp_func!(pow, =, real_pow);
vector_per_comp_func!(abs, =, real_abs);
vector_per_comp_func!(floor, =, real_floor);
vector_per_comp_func!(ceil, =, real_ceil);
vector_per_comp_func!(round, =, real_round);
vector_per_comp_func!(saturate, =, real_saturate);
vector_per_comp_func!(sqrt, =, real_sqrt);
vector_per_comp_func!(sin, =, real_sin);
vector_per_comp_func!(cos, =, real_cos);
vector_per_comp_func!(tan, =, real_tan);


//
// Real Math Traits
//
macro_rules! vector_by_real_op_assign {
    ($op:ident, $func:ident, $call:tt) => {
        impl<T: RealNumber, const COUNT: usize> $op<T> for Vector<T, COUNT> {
            fn $func(&mut self, rhs: T) {
                for c in 0 .. COUNT {
                    self[c] $call rhs;
                }
            }
        }
    };
}

macro_rules! vector_by_real_op {
    ($op:ident, $func:ident, $call:tt) => {
        impl<T: RealNumber, const COUNT: usize> $op<T> for Vector<T, COUNT> {
            type Output = Self;

            fn $func(self, rhs: T) -> Self::Output {
                let mut prod = self;

                for c in 0 .. COUNT {
                    prod[c] $call rhs;
                }

                return prod;
            }
        }
    };
}

vector_by_real_op_assign!(AddAssign, add_assign, +=);
vector_by_real_op_assign!(SubAssign, sub_assign, -=);
vector_by_real_op_assign!(MulAssign, mul_assign, *=);
vector_by_real_op_assign!(DivAssign, div_assign, /=);

vector_by_real_op!(Add, add, +=);
vector_by_real_op!(Sub, sub, -=);
vector_by_real_op!(Mul, mul, *=);
vector_by_real_op!(Div, div, /=);

//
// Vector math traits
//
macro_rules! vector_op_assign {
    ($op:ident, $func:ident, $call:tt) => {
        impl<T: RealNumber, const COUNT: usize> $op<Self> for Vector<T, COUNT> {
            fn $func(&mut self, rhs: Self) {
                for c in 0 .. COUNT {
                    self[c] $call rhs[c];
                }
            }
        }
    };
}

macro_rules! vector_op {
    ($op:ident, $func:ident, $call:tt) => {
        impl<T: RealNumber, const COUNT: usize> $op<Self> for Vector<T, COUNT> {
            type Output = Self;

            fn $func(self, rhs: Self) -> Self::Output {
                let mut prod = self;

                for c in 0 .. COUNT {
                    prod[c] $call rhs[c];
                }

                prod
            }
        }
    };
}

vector_op_assign!(AddAssign, add_assign, +=);
vector_op_assign!(SubAssign, sub_assign, -=);
vector_op_assign!(MulAssign, mul_assign, *=);
vector_op_assign!(DivAssign, div_assign, /=);

vector_op!(Add, add, +=);
vector_op!(Sub, sub, -=);
vector_op!(Mul, mul, *=);
vector_op!(Div, div, /=);

//
// Vector negation
//
impl<T: RealNumber, const COUNT: usize> Neg for Vector<T, COUNT> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut d = self.clone();

        for c in 0..COUNT {
            d[c] = -d[c];
        }

        d
    }
}

//
// Vector comparison
//
impl<T: RealNumber, const COUNT: usize> PartialEq for Vector<T, COUNT> {
    fn eq(&self, other: &Self) -> bool {
        for c in 0..COUNT {
            if self[c] != other[c] {
                return false;
            }
        }

        true
    }
}

//
// Extension macros
//

/// Macro to provide a [From] implementation for casting this [Vector] into another [Vector]
macro_rules! vector_from_vector {
    ($from_count:literal, $into_count:literal) => {
        impl<T: RealNumber> From<Vector<T, $from_count>> for Vector<T, $into_count> {
            fn from(rhs: Vector<T, $from_count>) -> Self {
                let mut o = Vector::<T, $into_count>::default();

                for c in 0 .. min($into_count, $from_count) {
                    o[c] = rhs[c];
                }

                o
            }
        }
    };
}

//
// Common vector types
//

/// Contains commonly used [Vector] aliases with additional implementations for ease of use
pub mod common {
    use crate::real::Real;

    use super::*;

    /// Common Vector2 types
    pub type Vector2 = Vector<Real, 2>;
    pub type Vector2F32 = Vector<f32, 2>;
    pub type Vector2F64 = Vector<f64, 2>;

    vector_from_vector!(2, 3);
    vector_from_vector!(2, 4);

    impl<T: RealNumber> Vector<T, 2> {
        pub fn new(x: T, y: T) -> Self {
            Self::from_array([x, y])
        }
    }

    /// Common Vector2 types
    pub type Vector3 = Vector<Real, 3>;
    pub type Vector3F32 = Vector<f32, 3>;
    pub type Vector3F64 = Vector<f64, 3>;

    vector_from_vector!(3, 2);
    vector_from_vector!(3, 4);

    impl<T: RealNumber> Vector<T, 3> {
        pub fn new(x: T, y: T, z: T) -> Self {
            Self::from_array([x, y, z])
        }

        /// Returns the cross product of the this [Vector] and another
        /// *Only implemented for 3 dimensional vectors due to cross product being 3D specific!*
        pub fn cross(&self, rhs: Self) -> Self {
            Self::from_array([
                self[1] * rhs[2] - self[2] * rhs[1],
                self[2] * rhs[0] - self[0] * rhs[2],
                self[0] * rhs[1] - self[1] * rhs[0]
            ])
        }

        pub fn reflect(&self, normal: Self) -> Self {
            return *self - normal * self.dot(normal) * (T::real_get_one() + T::real_get_one());
        }
    }

    /// Common 4D vector types (same type as [Quaternion])
    pub type Vector4 = Vector<Real, 4>;
    pub type Vector4F32 = Vector<f32, 4>;
    pub type Vector4F64 = Vector<f64, 4>;

    vector_from_vector!(4, 2);
    vector_from_vector!(4, 3);

    impl<T: RealNumber> Vector<T, 4> {
        pub fn new(x: T, y: T, z: T, w: T) -> Self {
            Self::from_array([x, y, z, w])
        }

        pub fn from_w(rhs: Vector<T, 3>, w: T) -> Self {
            let mut o = Vector::<T, 4>::default();

            for c in 0..3 {
                o[c] = rhs[c];
            }

            o[3] = w;

            return o;
        }
    }

    /// Quaternion (same type as [Vector4])
    pub type Quaternion = Vector4;
    pub type QuaternionF32 = Vector4F32;
    pub type QuaternionF64 = Vector4F64;
}