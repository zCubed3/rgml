#![allow(unused)]
#![allow(dead_code)]

use std::cmp::*;
use std::fmt::*;
use std::ops::*;
use std::iter::*;

use crate::real::RealNumber;

#[cfg(feature="swizzle")]
#[cfg_attr(feature="swizzle", doc(hidden))]
pub mod swizzle;

#[cfg(feature="swizzle")]
#[cfg_attr(feature="swizzle", doc(hidden))]
pub use swizzle::*;

#[cfg(feature="serialization")]
use serde::Serialize;

///
/// A vector type. Commonly used for 2D and 3D math
///
/// At an underlying level a [Vector] is an equivalent to using `[T; C]`
///
#[cfg_attr(feature="serialization", derive(Serialize))]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vector<T: RealNumber, const COUNT: usize> {
    /// The underlying array of the vector, dereferencing [Vector] returns this
    #[cfg_attr(feature="serialization", serde(with = "serde_arrays"))]
    pub backing: [T; COUNT],
}

impl<T: RealNumber, const COUNT: usize> Vector<T, COUNT> {
    /// Creates a new [Vector] by copying the given array into the backing array
    pub fn from_array(array: [T; COUNT]) -> Self {
        Vector { backing: array }
    }

    /// Creates a new [Vector] by copying the provided value into each element
    pub fn from_scalar(value: T) -> Self {
        Vector { backing: [value; COUNT] }
    }

    /// Returns the sum of all components within this [Vector]
    pub fn sum(&self) -> T {
        let mut sum = T::default();

        for c in 0..COUNT {
            sum += self[c];
        }

        return sum;
    }

    /// Returns the square length of this [Vector] (not sqrt(length)!)
    pub fn magnitude_sqr(&self) -> T {
        return self.dot(*self);
    }

    /// Returns the length of this [Vector]
    pub fn magnitude(&self) -> T {
        return self.dot(*self).rl_sqrt();
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

    /// Returns the interpolated value of this to another [Vector] by T
    pub fn lerp(&self, to: Self, alpha: T) -> Self {
        let mut a = *self;

        for c in 0..COUNT {
            a[c] = self[c].rl_lerp(to[c], alpha)
        }

        return a;
    }

    /// Returns the interpolated value of this to another [Vector] by another [Vector]
    pub fn vlerp(&self, to: Self, alpha: Self) -> Self {
        let mut a = *self;

        for c in 0..COUNT {
            a[c] = self[c].rl_lerp(to[c], alpha[c])
        }

        return a;
    }

    /// Returns the step function of this vector
    /// Refer to [GLSL specs](https://registry.khronos.org/OpenGL-Refpages/gl4/html/step.xhtml)
    pub fn step(&self, rhs: Self) -> Self {
        let mut a = Self::default();

        for c in 0..COUNT {
            a[c] = if self[c] < rhs[c] { T::ZERO } else { T::ONE };
        }

        return a;
    }

    /// Returns the sign of this vector
    pub fn sign(&self) -> Self {
        let mut a = Self::default();

        for c in 0..COUNT {
            a[c] = if self[c] < T::ZERO { -T::ONE } else { T::ONE };
        }

        return a;
    }

    /// Returns the fractional component of this vector
    pub fn fract(&self) -> Self {
        let mut a = Self::default();

        for c in 0..COUNT {
            a[c] = self[c] - self[c].rl_floor();
        }

        return a;
    }
}

//
// Default
//
impl<T: RealNumber, const COUNT: usize> Default for Vector<T, COUNT> {
    fn default() -> Self {
        Self { backing: [T::default(); COUNT] }
    }
}

//
// Deref
//
/// Deref to allow the Vector to be treated as its underlying backing array
impl<T: RealNumber, const COUNT: usize> Deref for Vector<T, COUNT> {
    type Target = [T; COUNT];

    fn deref(&self) -> &Self::Target {
        &self.backing
    }
}

impl<T: RealNumber, const COUNT: usize> DerefMut for Vector<T, COUNT> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.backing
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
        for c in self.backing {
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

vector_comp_comp_func!(min, =, rl_min);
vector_comp_comp_func!(max, =, rl_max);
vector_comp_comp_func!(pow, =, rl_pow);
vector_per_comp_func!(abs, =, rl_abs);
vector_per_comp_func!(floor, =, rl_floor);
vector_per_comp_func!(ceil, =, rl_ceil);
vector_per_comp_func!(round, =, rl_round);
vector_per_comp_func!(saturate, =, rl_saturate);
vector_per_comp_func!(sqrt, =, rl_sqrt);
vector_per_comp_func!(sin, =, rl_sin);
vector_per_comp_func!(cos, =, rl_cos);
vector_per_comp_func!(tan, =, rl_tan);


//
// Real Math Traits
//
macro_rules! vector_by_rl_op_assign {
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

macro_rules! vector_by_rl_op {
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
    }
}

macro_rules! rl_by_vector_op {
    ($op:ident, $func:ident, $call:tt, $tipe:ty) => {
        impl<const COUNT: usize> $op<Vector<$tipe, COUNT>> for $tipe {
            type Output = Vector<$tipe, COUNT>;

            fn $func(self, rhs: Vector<$tipe, COUNT>) -> Self::Output {
                let mut prod = rhs;

                for c in 0 .. COUNT {
                    prod[c] $call self;
                }

                return prod;
            }
        }
    };
}

vector_by_rl_op_assign!(AddAssign, add_assign, +=);
vector_by_rl_op_assign!(SubAssign, sub_assign, -=);
vector_by_rl_op_assign!(MulAssign, mul_assign, *=);
vector_by_rl_op_assign!(DivAssign, div_assign, /=);

vector_by_rl_op!(Add, add, +=);
vector_by_rl_op!(Sub, sub, -=);
vector_by_rl_op!(Mul, mul, *=);
vector_by_rl_op!(Div, div, /=);

rl_by_vector_op!(Add, add, +=, f32);
rl_by_vector_op!(Sub, sub, -=, f32);
rl_by_vector_op!(Mul, mul, *=, f32);
rl_by_vector_op!(Div, div, /=, f32);

rl_by_vector_op!(Add, add, +=, f64);
rl_by_vector_op!(Sub, sub, -=, f64);
rl_by_vector_op!(Mul, mul, *=, f64);
rl_by_vector_op!(Div, div, /=, f64);



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
// Boolean ops
//
impl<T: RealNumber, const COUNT: usize> PartialOrd for Vector<T, COUNT> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        None
    }

    fn gt(&self, other: &Self) -> bool {
        for c in 0..COUNT {
            if self[c] > other[c] {
                return true;
            }
        }

        return false;
    }

    fn ge(&self, other: &Self) -> bool {
        for c in 0..COUNT {
            if self[c] >= other[c] {
                return true;
            }
        }

        return false;
    }

    fn lt(&self, other: &Self) -> bool {
        for c in 0..COUNT {
            if self[c] < other[c] {
                return true;
            }
        }

        return false;
    }

    fn le(&self, other: &Self) -> bool {
        for c in 0..COUNT {
            if self[c] <= other[c] {
                return true;
            }
        }

        return false;
    }
}
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
#[macro_export]
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
vector_from_vector!(2, 3);
vector_from_vector!(2, 4);

impl<T: RealNumber> Vector<T, 2> {
    pub fn new(x: T, y: T) -> Self {
        Self::from_array([x, y])
    }
}

vector_from_vector!(3, 2);
vector_from_vector!(3, 4);

impl<T: RealNumber> Vector<T, 3> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self::from_array([x, y, z])
    }

    /// Returns the cross product of the this [Vector] and another
    ///
    /// Note:
    ///   Only implemented for 3 dimensional vectors due to cross product being 3D specific!
    pub fn cross(&self, rhs: Self) -> Self {
        Self::from_array([
            self[1] * rhs[2] - self[2] * rhs[1],
            self[2] * rhs[0] - self[0] * rhs[2],
            self[0] * rhs[1] - self[1] * rhs[0]
        ])
    }

    /// Reflects this [Vector] by the given normal
    pub fn reflect(&self, normal: Self) -> Self {
        return *self - normal * self.dot(normal) * (T::ONE + T::ONE);
    }
}

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

    pub fn identity() -> Self {
        return Self::new(T::ZERO, T::ZERO, T::ZERO, T::ONE);
    }
}


/// Contains commonly used [Vector] aliases with additional implementations for ease of use
pub mod common {
    use crate::real::Real;
    use super::*;

    /// [Vector<T, 2>] backed by [Real]
    pub type Vector2 = Vector<Real, 2>;
    /// [Vector<T, 2>] backed by [f32]
    pub type Vector2F32 = Vector<f32, 2>;
    /// [Vector<T, 2>] backed by [f64]
    pub type Vector2F64 = Vector<f64, 2>;

    /// [Vector<T, 3>] backed by [Real]
    pub type Vector3 = Vector<Real, 3>;
    /// [Vector<T, 3>] backed by [f32]
    pub type Vector3F32 = Vector<f32, 3>;
    /// [Vector<T, 3>] backed by [f64]
    pub type Vector3F64 = Vector<f64, 3>;

    /// [Vector<T, 4>] backed by [Real]
    pub type Vector4 = Vector<Real, 4>;
    /// [Vector<T, 4>] backed by [f32]
    pub type Vector4F32 = Vector<f32, 4>;
    /// [Vector<T, 4>] backed by [f64]
    pub type Vector4F64 = Vector<f64, 4>;

    /// [Vector<T, 4>] backed by [Real]
    pub type Quaternion = Vector4;
    /// [Vector<T, 4>] backed by [f32]
    pub type QuaternionF32 = Vector4F32;
    /// [Vector<T, 4>] backed by [f64]
    pub type QuaternionF64 = Vector4F64;
}