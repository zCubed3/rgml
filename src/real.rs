use std::cmp::*;
use std::fmt::*;
use std::ops::*;
use crate::vector::*;

#[cfg(feature="serde")]
use serde::Serialize;

// https://www.worthe-it.co.za/blog/2017-01-15-aliasing-traits-in-rust.html

#[cfg(feature="serialization")]
/// If serde support is enabled, this requires [Serialize] having been implemented
pub trait RealDataBounds: Serialize {}

#[cfg(not(feature="serialization"))]
/// If serde support is enabled, this requires [Serialize] having been implemented
pub trait RealDataBounds {}

/// Strict trait for constraining what types can be used for math (eg. decimal numbers)
///
/// This trait is already implemented for [f32] and [f64]
pub trait RealNumber:
Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> + Neg<Output=Self> + // Arithmetic with self
AddAssign + SubAssign + MulAssign + DivAssign + // Arithmetic with assign with self
PartialEq + PartialOrd + // Equality
Sized + Clone + Copy + Default + Display + RealDataBounds // Usability
{
    /// Square root of this [RealNumber]
    fn real_sqrt(&self) -> Self;

    /// Sinusoidal result of this [RealNumber]
    fn real_sin(&self) -> Self;
    /// Co-sinusoidal result of this [RealNumber]
    fn real_cos(&self) -> Self;
    /// Tangent result of this [RealNumber]
    fn real_tan(&self) -> Self;

    /// Exponential result of this [RealNumber]
    fn real_exp(&self) -> Self;

    /// Absolute value of this [RealNumber]
    fn real_abs(&self) -> Self;

    /// Returns the smallest [RealNumber] between self and min
    fn real_min(&self, min: Self) -> Self;
    /// Returns the largest [RealNumber] between self and max
    fn real_max(&self, max: Self) -> Self;
    /// Returns this [RealNumber] clamped between a minimum and maximum
    fn real_clamp(&self, min: Self, max: Self) -> Self;
    /// Returns this [RealNumber] clamped between 0 and 1
    fn real_saturate(&self) -> Self;

    /// Returns the floored value of this [RealNumber]
    fn real_floor(&self) -> Self;
    /// Returns the raised value of this [RealNumber]
    fn real_ceil(&self) -> Self;
    /// Returns the rounded value of this [RealNumber]
    fn real_round(&self) -> Self;

    /// Returns the fractional component of this [RealNumber]
    fn real_fract(&self) -> Self {
        return *self - self.real_floor();
    }

    /// Returns the sign of this [RealNumber] (if X < 0 then -1 else 1)
    fn real_sign(&self) -> Self {
        return if *self < Self::real_zero() { -Self::real_one() } else { Self::real_one() }
    }

    /// Returns this [RealNumber] raised to the given power
    fn real_pow(&self, exp: Self) -> Self;

    /// Linearly interpolates between this and another [RealNumber] by alpha
    fn real_lerp(&self, to: Self, alpha: Self) -> Self;

    /// Returns this [RealNumber] converted to radians
    fn real_to_radians(&self) -> Self;
    /// Returns this [RealNumber] converted to degrees
    fn real_to_degrees(&self) -> Self;

    /// Returns radians to degrees constant for this [RealNumber]
    fn real_rad_to_deg() -> Self;
    /// Returns degrees to radians constant for this [RealNumber]
    fn real_deg_to_rad() -> Self;

    /// Returns the PI constant for this [RealNumber]
    fn real_pi() -> Self;

    /// Returns 0 for this [RealNumber] (for opaqueness)
    fn real_zero() -> Self;
    /// Returns 1 for this [RealNumber] (for opaqueness)
    fn real_one() -> Self;

    #[cfg(feature="swizzle")]
    /// Returns this [RealNumber]
    fn x(&self) -> Self;

    #[cfg(feature="swizzle")]
    /// Returns this [RealNumber] as a 2 component vector
    fn xx(&self) -> Vector<Self, 2>;

    #[cfg(feature="swizzle")]
    /// Returns this [RealNumber] as a 3 component vector
    fn xxx(&self) -> Vector<Self, 3>;

    #[cfg(feature="swizzle")]
    /// Returns this [RealNumber] as a 4 component vector
    fn xxxx(&self) -> Vector<Self, 4>;
}

//
// Real impls for float types
//

// F32
impl RealDataBounds for f32 {}
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

    fn real_zero() -> Self {
        0.0
    }

    fn real_one() -> Self {
        1.0
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
impl RealDataBounds for f64 {}
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

    fn real_zero() -> Self {
        0.0
    }

    fn real_one() -> Self {
        1.0
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
/// Wrapper around [f32] or [f64] for [Vector] math (type depends on if "real_t_is_double" is enabled)
pub type Real = f64;

#[cfg(not(feature = "real_t_is_double"))]
/// Wrapper around [f32] or [f64] for [Vector] math (type depends on if "real_t_is_double" is enabled)
pub type Real = f32;