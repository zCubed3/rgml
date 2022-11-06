use std::cmp::*;
use std::fmt::*;
use std::ops::*;
use crate::prelude::Vector2;
use crate::vector::*;

// https://www.worthe-it.co.za/blog/2017-01-15-aliasing-traits-in-rust.html

/// Strict trait for constraining what types can be used for math
///
/// This trait is already implemented for [f32] and [f64]
pub trait RealNumber:
Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> + Neg<Output=Self> + // Arithmetic with self
AddAssign + SubAssign + MulAssign + DivAssign + // Arithmetic with assign with self
PartialEq + PartialOrd + // Equality
Clone + Copy + Default + Display // Usability
    where Self: Sized {
    fn real_sqrt(&self) -> Self;

    fn real_sin(&self) -> Self;
    fn real_cos(&self) -> Self;
    fn real_tan(&self) -> Self;

    fn real_exp(&self) -> Self;

    fn real_abs(&self) -> Self;

    fn real_min(&self, min: Self) -> Self;
    fn real_max(&self, max: Self) -> Self;
    fn real_clamp(&self, min: Self, max: Self) -> Self;
    fn real_saturate(&self) -> Self;

    fn real_floor(&self) -> Self;
    fn real_ceil(&self) -> Self;
    fn real_round(&self) -> Self;

    fn real_pow(&self, exp: Self) -> Self;

    fn real_lerp(&self, to: Self, alpha: Self) -> Self;

    fn real_to_radians(&self) -> Self;
    fn real_to_degrees(&self) -> Self;

    fn real_rad_to_deg() -> Self;
    fn real_deg_to_rad() -> Self;

    fn real_pi() -> Self;

    fn real_get_one() -> Self;

    #[cfg(feature="swizzle")]
    fn x(&self) -> Self;

    #[cfg(feature="swizzle")]
    fn xx(&self) -> Vector<Self, 2>;

    #[cfg(feature="swizzle")]
    fn xxx(&self) -> Vector<Self, 3>;

    #[cfg(feature="swizzle")]
    fn xxxx(&self) -> Vector<Self, 4>;
}

//
// Real impls for float types
//

// F32
impl RealNumber for f32 {
    fn real_sqrt(&self) -> Self {
        self.sqrt()
    }

    fn real_sin(&self) -> Self {
        self.sin()
    }

    fn real_cos(&self) -> Self {
        self.cos()
    }

    fn real_tan(&self) -> Self {
        self.tan()
    }

    fn real_exp(&self) -> Self {
        return self.exp();
    }

    fn real_abs(&self) -> Self {
        self.abs()
    }

    fn real_min(&self, min: Self) -> Self {
        return self.min(min);
    }

    fn real_max(&self, max: Self) -> Self {
        return self.max(max);
    }

    fn real_clamp(&self, min: Self, max: Self) -> Self {
        return self.clamp(min, max);
    }

    fn real_saturate(&self) -> Self {
        return self.min(1.0).max(0.0);
    }

    fn real_floor(&self) -> Self {
        return self.floor();
    }

    fn real_ceil(&self) -> Self {
        return self.ceil();
    }

    fn real_round(&self) -> Self {
        return self.round();
    }

    fn real_pow(&self, exp: Self) -> Self {
        return self.powf(exp);
    }

    fn real_lerp(&self, to: Self, alpha: Self) -> Self {
        return (1.0 - alpha) * self + alpha * to;
    }

    fn real_to_radians(&self) -> Self {
        return self.to_radians();
    }

    fn real_to_degrees(&self) -> Self {
        return self.to_degrees();
    }

    fn real_rad_to_deg() -> Self {
        57.2957795131f32
    }

    fn real_deg_to_rad() -> Self {
        0.01745329251f32
    }

    fn real_pi() -> Self {
        std::f32::consts::PI
    }

    fn real_get_one() -> Self {
        1f32
    }

    #[cfg(feature="swizzle")]
    fn x(&self) -> Self {
        return *self;
    }

    #[cfg(feature="swizzle")]
    fn xx(&self) -> Vector<Self, 2> {
        return Vector::<Self, 2>::from_scalar(*self);
    }

    #[cfg(feature="swizzle")]
    fn xxx(&self) -> Vector<Self, 3> {
        return Vector::<Self, 3>::from_scalar(*self);
    }

    #[cfg(feature="swizzle")]
    fn xxxx(&self) -> Vector<Self, 4> {
        return Vector::<Self, 4>::from_scalar(*self);
    }
}

// F64
impl RealNumber for f64 {
    fn real_sqrt(&self) -> Self {
        self.sqrt()
    }

    fn real_sin(&self) -> Self {
        self.sin()
    }

    fn real_cos(&self) -> Self {
        self.cos()
    }

    fn real_tan(&self) -> Self {
        self.tan()
    }

    fn real_exp(&self) -> Self {
        return self.exp();
    }

    fn real_abs(&self) -> Self {
        self.abs()
    }

    fn real_min(&self, min: Self) -> Self {
        return self.min(min);
    }

    fn real_max(&self, max: Self) -> Self {
        return self.max(max);
    }

    fn real_clamp(&self, min: Self, max: Self) -> Self {
        return self.clamp(min, max);
    }

    fn real_saturate(&self) -> Self {
        return self.min(1.0).max(0.0);
    }

    fn real_floor(&self) -> Self {
        return self.floor();
    }

    fn real_ceil(&self) -> Self {
        return self.ceil();
    }

    fn real_round(&self) -> Self {
        return self.round();
    }

    fn real_pow(&self, exp: Self) -> Self {
        return self.powf(exp);
    }

    fn real_lerp(&self, to: Self, alpha: Self) -> Self {
        return (1.0 - alpha) * self + alpha * to;
    }

    fn real_to_radians(&self) -> Self {
        return self.to_radians();
    }

    fn real_to_degrees(&self) -> Self {
        return self.to_degrees();
    }

    fn real_rad_to_deg() -> Self {
        57.2957795131f64
    }

    fn real_deg_to_rad() -> Self {
        0.01745329251f64
    }

    fn real_pi() -> Self {
        std::f64::consts::PI
    }

    fn real_get_one() -> Self {
        1f64
    }

    #[cfg(feature="swizzle")]
    fn x(&self) -> Self {
        return *self;
    }

    #[cfg(feature="swizzle")]
    fn xx(&self) -> Vector<Self, 2> {
        return Vector::<Self, 2>::from_scalar(*self);
    }

    #[cfg(feature="swizzle")]
    fn xxx(&self) -> Vector<Self, 3> {
        return Vector::<Self, 3>::from_scalar(*self);
    }

    #[cfg(feature="swizzle")]
    fn xxxx(&self) -> Vector<Self, 4> {
        return Vector::<Self, 4>::from_scalar(*self);
    }
}

#[cfg(feature = "real_t_is_double")]
pub type Real = f64;

#[cfg(not(feature = "real_t_is_double"))]
pub type Real = f32;