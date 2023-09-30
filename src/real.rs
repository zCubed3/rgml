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
    fn rl_sqrt(&self) -> Self;

    /// Sinusoidal result of this [RealNumber]
    fn rl_sin(&self) -> Self;
    /// Co-sinusoidal result of this [RealNumber]
    fn rl_cos(&self) -> Self;
    /// Tangent result of this [RealNumber]
    fn rl_tan(&self) -> Self;
    /// Inverse sinusoidal result of this result of this [RealNumber]
    fn rl_asin(&self) -> Self;
    /// Inverse co-sinusoidal result of this [RealNumber]
    fn rl_acos(&self) -> Self;
    /// Inverse tangent result of this [RealNumber]
    fn rl_atan(&self) -> Self;
    /// Atan2 result of this [RealNumber] and X
    fn rl_atan2(&self, x: Self) -> Self;

    /// Exponential result of this [RealNumber]
    fn rl_exp(&self) -> Self;

    // Log10 result of this [RealNumber]
    fn rl_log10(&self) -> Self;

    /// Absolute value of this [RealNumber]
    fn rl_abs(&self) -> Self;

    /// Returns the smallest [RealNumber] between self and min
    fn rl_min(&self, other: Self) -> Self;
    /// Returns the largest [RealNumber] between self and max
    fn rl_max(&self, other: Self) -> Self;
    /// Returns this [RealNumber] clamped between a minimum and maximum
    fn rl_clamp(&self, min: Self, max: Self) -> Self;
    /// Returns this [RealNumber] clamped between 0 and 1
    fn rl_saturate(&self) -> Self {
        return self.rl_clamp(Self::ZERO, Self::ONE);
    }

    /// Alias to [RealNumber::rl_saturate()]
    fn rl_clamp01(&self) -> Self {
        return self.rl_saturate();
    }

    /// Returns the floored value of this [RealNumber]
    fn rl_floor(&self) -> Self;
    /// Returns the raised value of this [RealNumber]
    fn rl_ceil(&self) -> Self;
    /// Returns the rounded value of this [RealNumber]
    fn rl_round(&self) -> Self;

    /// Returns the fractional component of this [RealNumber]
    fn rl_fract(&self) -> Self {
        return *self - self.rl_floor();
    }

    /// Returns this [RealNumber] wrapped between 0 and 1 (alias to [RealNumber::rl_fract])
    fn rl_wrap01(&self) -> Self {
        return self.rl_fract();
    }

    /// Returns this [RealNumber] wrapped between min and max
    fn rl_wrap(&self, min: Self, max: Self) -> Self {
        return *self - (max - min) * (*self / (max - min)).rl_floor();
    }

    /// Returns the sign of this [RealNumber] (if X < 0 then -1 else 1)
    fn rl_sign(&self) -> Self {
        return if *self < Self::ZERO { -Self::ONE } else { Self::ONE }
    }

    /// Returns this [RealNumber] raised to the given power
    fn rl_pow(&self, exp: Self) -> Self;

    /// Linearly interpolates between this and another [RealNumber] by alpha
    fn rl_lerp(&self, to: Self, alpha: Self) -> Self {
        return (Self::ONE - alpha) * *self + alpha * to;
    }

    /// Attempts interpolate this [RealNumber] as an angle
    fn rl_slerp(&self, to: Self, alpha: Self) -> Self {
        let da = Vector::<Self, 2>::new(self.rl_cos(), self.rl_sin());
        let db = Vector::<Self, 2>::new(to.rl_cos(), to.rl_sin());

        let dc = da.lerp(db, alpha);
        return dc[1].rl_atan2(dc[0]);
    }

    /// Returns this [RealNumber] converted to radians
    fn rl_to_radians(&self) -> Self;
    /// Returns this [RealNumber] converted to degrees
    fn rl_to_degrees(&self) -> Self;

    /// Returns radians to degrees constant for this [RealNumber]
    const RAD2DEG: Self;
    /// Provides the degrees to radians constant for this [RealNumber]
    const DEG2RAD: Self;
    /// Provides the TAU constant for this [RealNumber]
    const TAU: Self;
    /// Provides the PI constant for this [RealNumber]
    const PI: Self;
    /// Provides 1 for this [RealNumber] (for opaqueness)
    const ONE: Self;
    /// Provides 0 for this [RealNumber] (for opaqueness)
    const ZERO: Self;

    /// Returns this [RealNumber] as a 2 component vector
    #[cfg(feature="swizzle")]
    fn xx(&self) -> Vector<Self, 2> {
        return Vector::<Self, 2>::from_scalar(*self);
    }

    /// Returns this [RealNumber] as a 3 component vector
    #[cfg(feature="swizzle")]
    fn xxx(&self) -> Vector<Self, 3> {
        return Vector::<Self, 3>::from_scalar(*self);
    }

    /// Returns this [RealNumber] as a 4 component vector
    #[cfg(feature="swizzle")]
    fn xxxx(&self) -> Vector<Self, 4> {
        return Vector::<Self, 4>::from_scalar(*self);
    }
}

//
// Real impls for float types
//

// F32
impl RealDataBounds for f32 {}
impl RealNumber for f32 {
    fn rl_sqrt(&self) -> Self {
        return self.sqrt();
    }

    fn rl_sin(&self) -> Self {
        return self.sin();
    }

    fn rl_cos(&self) -> Self {
        return self.cos();
    }

    fn rl_tan(&self) -> Self {
        return self.tan();
    }

    fn rl_asin(&self) -> Self {
        return self.asin();
    }

    fn rl_acos(&self) -> Self {
        return self.acos();
    }

    fn rl_atan(&self) -> Self {
        return self.atan();
    }

    fn rl_atan2(&self, x: Self) -> Self {
        return self.atan2(x);
    }

    fn rl_exp(&self) -> Self {
        return self.exp();
    }

    fn rl_log10(&self) -> Self {
        return self.log10();
    }

    fn rl_abs(&self) -> Self {
        self.abs()
    }

    fn rl_min(&self, other: Self) -> Self {
        return self.min(other);
    }

    fn rl_max(&self, other: Self) -> Self {
        return self.max(other);
    }

    fn rl_clamp(&self, min: Self, max: Self) -> Self {
        return self.clamp(min, max);
    }

    fn rl_floor(&self) -> Self {
        return self.floor();
    }

    fn rl_ceil(&self) -> Self {
        return self.ceil();
    }

    fn rl_round(&self) -> Self {
        return self.round();
    }

    fn rl_pow(&self, exp: Self) -> Self {
        return self.powf(exp);
    }

    fn rl_to_radians(&self) -> Self {
        return self.to_radians();
    }

    fn rl_to_degrees(&self) -> Self {
        return self.to_degrees();
    }

    const RAD2DEG: Self = 180.0 / std::f32::consts::PI;
    const DEG2RAD: Self = std::f32::consts::PI / 180.0;
    const TAU: Self = std::f32::consts::TAU;
    const PI: Self = std::f32::consts::PI;
    const ONE: Self = 1.0;
    const ZERO: Self = 0.0;
}

// F64
impl RealDataBounds for f64 {}
impl RealNumber for f64 {
    fn rl_sqrt(&self) -> Self {
        return self.sqrt();
    }

    fn rl_sin(&self) -> Self {
        return self.sin();
    }

    fn rl_cos(&self) -> Self {
        return self.cos();
    }

    fn rl_tan(&self) -> Self {
        return self.tan();
    }

    fn rl_asin(&self) -> Self {
        return self.asin();
    }

    fn rl_acos(&self) -> Self {
        return self.acos();
    }

    fn rl_atan(&self) -> Self {
        return self.atan();
    }

    fn rl_atan2(&self, x: Self) -> Self {
        return self.atan2(x);
    }

    fn rl_exp(&self) -> Self {
        return self.exp();
    }

    fn rl_log10(&self) -> Self {
        return self.log10();
    }

    fn rl_abs(&self) -> Self {
        self.abs()
    }

    fn rl_min(&self, other: Self) -> Self {
        return self.min(other);
    }

    fn rl_max(&self, other: Self) -> Self {
        return self.max(other);
    }

    fn rl_clamp(&self, min: Self, max: Self) -> Self {
        return self.clamp(min, max);
    }

    fn rl_floor(&self) -> Self {
        return self.floor();
    }

    fn rl_ceil(&self) -> Self {
        return self.ceil();
    }

    fn rl_round(&self) -> Self {
        return self.round();
    }

    fn rl_pow(&self, exp: Self) -> Self {
        return self.powf(exp);
    }

    fn rl_to_radians(&self) -> Self {
        return self.to_radians();
    }

    fn rl_to_degrees(&self) -> Self {
        return self.to_degrees();
    }

    const RAD2DEG: Self = 180.0 / std::f64::consts::PI;
    const DEG2RAD: Self = std::f64::consts::PI / 180.0;
    const TAU: Self = std::f64::consts::TAU;
    const PI: Self = std::f64::consts::PI;
    const ONE: Self = 1.0;
    const ZERO: Self = 0.0;
}

/// Wrapper around floating point types for opaque math
///
/// # Note:
///   Is [f64] if feature "real_t_is_double" is enabled
#[cfg(feature = "real_t_is_double")]
pub type Real = f64;

/// Wrapper around floating point types for opaque math
///
/// # Note:
///   Is [f64] if feature `real_t_is_double` is enabled
#[cfg(not(feature = "real_t_is_double"))]
pub type Real = f32;