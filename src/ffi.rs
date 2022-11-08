//
// FFI for working with the library in C
//

#![allow(clippy::needless_return)]

use crate::prelude::*;

// C style functions for creating vectors
// Things are explicit due to cbindgen failing to expand macros

//
// Vector2F32
//
#[no_mangle]
pub extern "C" fn vec2_new(x: f32, y: f32) -> Vector2F32 {
    return Vector2F32::new(x, y);
}

#[no_mangle]
pub extern "C" fn vec2_zero() -> Vector2F32 {
    return Vector2F32::default();
}

#[no_mangle]
pub extern "C" fn vec2_add(lhs: Vector2F32, rhs: Vector2F32) -> Vector2F32 {
    return lhs + rhs;
}

#[no_mangle]
pub extern "C" fn vec2_sub(lhs: Vector2F32, rhs: Vector2F32) -> Vector2F32 {
    return lhs - rhs;
}

#[no_mangle]
pub extern "C" fn vec2_mul(lhs: Vector2F32, rhs: Vector2F32) -> Vector2F32 {
    return lhs * rhs;
}

#[no_mangle]
pub extern "C" fn vec2_div(lhs: Vector2F32, rhs: Vector2F32) -> Vector2F32 {
    return lhs / rhs;
}

#[no_mangle]
pub extern "C" fn vec2_sum(v: Vector2F32) -> f32 {
    return v.sum();
}

#[no_mangle]
pub extern "C" fn vec2_magnitude(v: Vector2F32) -> f32 {
    return v.magnitude();
}

#[no_mangle]
pub extern "C" fn vec2_magnitude_sqr(v: Vector2F32) -> f32 {
    return v.magnitude_sqr();
}

#[no_mangle]
pub extern "C" fn vec2_dot(lhs: Vector2F32, rhs: Vector2F32) -> f32 {
    return lhs.dot(rhs);
}

#[no_mangle]
pub extern "C" fn vec2_min(lhs: Vector2F32, rhs: Vector2F32) -> Vector2F32 {
    return lhs.min(rhs);
}

#[no_mangle]
pub extern "C" fn vec2_max(lhs: Vector2F32, rhs: Vector2F32) -> Vector2F32 {
    return lhs.max(rhs);
}

#[no_mangle]
pub extern "C" fn vec2_abs(v: Vector2F32) -> Vector2F32 {
    return v.abs();
}

#[no_mangle]
pub extern "C" fn vec2_saturate(v: Vector2F32) -> Vector2F32 {
    return v.saturate();
}

#[no_mangle]
pub extern "C" fn vec2_sign(v: Vector2F32) -> Vector2F32 {
    return v.sign();
}

#[no_mangle]
pub extern "C" fn vec2_normalize(v: Vector2F32) -> Vector2F32 {
    return v.normalize();
}

#[no_mangle]
pub extern "C" fn vec2_lerp(from: Vector2F32, to: Vector2F32, alpha: f32) -> Vector2F32 {
    return from.lerp(to, alpha);
}

#[no_mangle]
pub extern "C" fn vec2_step(lhs: Vector2F32, rhs: Vector2F32) -> Vector2F32 {
    return lhs.step(rhs);
}

#[no_mangle]
pub extern "C" fn vec2_pow(lhs: Vector2F32, rhs: Vector2F32) -> Vector2F32 {
    return lhs.pow(rhs);
}

#[no_mangle]
pub extern "C" fn vec2_floor(v: Vector2F32) -> Vector2F32 {
    return v.floor()
}

#[no_mangle]
pub extern "C" fn vec2_ceil(v: Vector2F32) -> Vector2F32 {
    return v.ceil()
}

#[no_mangle]
pub extern "C" fn vec2_round(v: Vector2F32) -> Vector2F32 {
    return v.ceil()
}

#[no_mangle]
pub extern "C" fn vec2_sqrt(v: Vector2F32) -> Vector2F32 {
    return v.sqrt()
}

#[no_mangle]
pub extern "C" fn vec2_sin(v: Vector2F32) -> Vector2F32 {
    return v.sin()
}

#[no_mangle]
pub extern "C" fn vec2_cos(v: Vector2F32) -> Vector2F32 {
    return v.cos()
}

#[no_mangle]
pub extern "C" fn vec2_tan(v: Vector2F32) -> Vector2F32 {
    return v.tan()
}

//
// Vector3F32
//
#[no_mangle]
pub extern "C" fn vec3_new(x: f32, y: f32, z: f32) -> Vector3F32 {
    return Vector3F32::new(x, y, z);
}

#[no_mangle]
pub extern "C" fn vec3_zero() -> Vector3F32 {
    return Vector3F32::default();
}

#[no_mangle]
pub extern "C" fn vec3_add(lhs: Vector3F32, rhs: Vector3F32) -> Vector3F32 {
    return lhs + rhs;
}

#[no_mangle]
pub extern "C" fn vec3_sub(lhs: Vector3F32, rhs: Vector3F32) -> Vector3F32 {
    return lhs - rhs;
}

#[no_mangle]
pub extern "C" fn vec3_mul(lhs: Vector3F32, rhs: Vector3F32) -> Vector3F32 {
    return lhs * rhs;
}

#[no_mangle]
pub extern "C" fn vec3_div(lhs: Vector3F32, rhs: Vector3F32) -> Vector3F32 {
    return lhs / rhs;
}

#[no_mangle]
pub extern "C" fn vec3_sum(v: Vector3F32) -> f32 {
    return v.sum();
}

#[no_mangle]
pub extern "C" fn vec3_magnitude(v: Vector3F32) -> f32 {
    return v.magnitude();
}

#[no_mangle]
pub extern "C" fn vec3_magnitude_sqr(v: Vector3F32) -> f32 {
    return v.magnitude_sqr();
}

#[no_mangle]
pub extern "C" fn vec3_dot(lhs: Vector3F32, rhs: Vector3F32) -> f32 {
    return lhs.dot(rhs);
}

#[no_mangle]
pub extern "C" fn vec3_min(lhs: Vector3F32, rhs: Vector3F32) -> Vector3F32 {
    return lhs.min(rhs);
}

#[no_mangle]
pub extern "C" fn vec3_max(lhs: Vector3F32, rhs: Vector3F32) -> Vector3F32 {
    return lhs.max(rhs);
}

#[no_mangle]
pub extern "C" fn vec3_abs(v: Vector3F32) -> Vector3F32 {
    return v.abs();
}

#[no_mangle]
pub extern "C" fn vec3_saturate(v: Vector3F32) -> Vector3F32 {
    return v.saturate();
}

#[no_mangle]
pub extern "C" fn vec3_sign(v: Vector3F32) -> Vector3F32 {
    return v.sign();
}

#[no_mangle]
pub extern "C" fn vec3_cross(lhs: Vector3F32, rhs: Vector3F32) -> Vector3F32 {
    return lhs.cross(rhs);
}

#[no_mangle]
pub extern "C" fn vec3_normalize(v: Vector3F32) -> Vector3F32 {
    return v.normalize();
}

#[no_mangle]
pub extern "C" fn vec3_lerp(from: Vector3F32, to: Vector3F32, alpha: f32) -> Vector3F32 {
    return from.lerp(to, alpha);
}

#[no_mangle]
pub extern "C" fn vec3_step(lhs: Vector3F32, rhs: Vector3F32) -> Vector3F32 {
    return lhs.step(rhs);
}

#[no_mangle]
pub extern "C" fn vec3_pow(lhs: Vector3F32, rhs: Vector3F32) -> Vector3F32 {
    return lhs.pow(rhs);
}

#[no_mangle]
pub extern "C" fn vec3_floor(v: Vector3F32) -> Vector3F32 {
    return v.floor()
}

#[no_mangle]
pub extern "C" fn vec3_ceil(v: Vector3F32) -> Vector3F32 {
    return v.ceil()
}

#[no_mangle]
pub extern "C" fn vec3_round(v: Vector3F32) -> Vector3F32 {
    return v.ceil()
}

#[no_mangle]
pub extern "C" fn vec3_sqrt(v: Vector3F32) -> Vector3F32 {
    return v.sqrt()
}

#[no_mangle]
pub extern "C" fn vec3_sin(v: Vector3F32) -> Vector3F32 {
    return v.sin()
}

#[no_mangle]
pub extern "C" fn vec3_cos(v: Vector3F32) -> Vector3F32 {
    return v.cos()
}

#[no_mangle]
pub extern "C" fn vec3_tan(v: Vector3F32) -> Vector3F32 {
    return v.tan()
}

//
// Vector3F64
//
#[no_mangle]
pub extern "C" fn dvec3_new(x: f64, y: f64, z: f64) -> Vector3F64 {
    return Vector3F64::new(x, y, z);
}

#[no_mangle]
pub extern "C" fn dvec3_zero() -> Vector3F64 {
    return Vector3F64::default();
}

#[no_mangle]
pub extern "C" fn dvec3_add(lhs: Vector3F64, rhs: Vector3F64) -> Vector3F64 {
    return lhs + rhs;
}

#[no_mangle]
pub extern "C" fn dvec3_sub(lhs: Vector3F64, rhs: Vector3F64) -> Vector3F64 {
    return lhs - rhs;
}

#[no_mangle]
pub extern "C" fn dvec3_mul(lhs: Vector3F64, rhs: Vector3F64) -> Vector3F64 {
    return lhs * rhs;
}

#[no_mangle]
pub extern "C" fn dvec3_div(lhs: Vector3F64, rhs: Vector3F64) -> Vector3F64 {
    return lhs / rhs;
}

#[no_mangle]
pub extern "C" fn dvec3_sum(v: Vector3F64) -> f64 {
    return v.sum();
}

#[no_mangle]
pub extern "C" fn dvec3_magnitude(v: Vector3F64) -> f64 {
    return v.magnitude();
}

#[no_mangle]
pub extern "C" fn dvec3_magnitude_sqr(v: Vector3F64) -> f64 {
    return v.magnitude_sqr();
}

#[no_mangle]
pub extern "C" fn dvec3_dot(lhs: Vector3F64, rhs: Vector3F64) -> f64 {
    return lhs.dot(rhs);
}

#[no_mangle]
pub extern "C" fn dvec3_min(lhs: Vector3F64, rhs: Vector3F64) -> Vector3F64 {
    return lhs.min(rhs);
}

#[no_mangle]
pub extern "C" fn dvec3_max(lhs: Vector3F64, rhs: Vector3F64) -> Vector3F64 {
    return lhs.max(rhs);
}

#[no_mangle]
pub extern "C" fn dvec3_abs(v: Vector3F64) -> Vector3F64 {
    return v.abs();
}

#[no_mangle]
pub extern "C" fn dvec3_saturate(v: Vector3F64) -> Vector3F64 {
    return v.saturate();
}

#[no_mangle]
pub extern "C" fn dvec3_sign(v: Vector3F64) -> Vector3F64 {
    return v.sign();
}

#[no_mangle]
pub extern "C" fn dvec3_cross(lhs: Vector3F64, rhs: Vector3F64) -> Vector3F64 {
    return lhs.cross(rhs);
}

#[no_mangle]
pub extern "C" fn dvec3_normalize(v: Vector3F64) -> Vector3F64 {
    return v.normalize();
}

#[no_mangle]
pub extern "C" fn dvec3_lerp(from: Vector3F64, to: Vector3F64, alpha: f64) -> Vector3F64 {
    return from.lerp(to, alpha);
}

#[no_mangle]
pub extern "C" fn dvec3_step(lhs: Vector3F64, rhs: Vector3F64) -> Vector3F64 {
    return lhs.step(rhs);
}

#[no_mangle]
pub extern "C" fn dvec3_pow(lhs: Vector3F64, rhs: Vector3F64) -> Vector3F64 {
    return lhs.pow(rhs);
}

#[no_mangle]
pub extern "C" fn dvec3_floor(v: Vector3F64) -> Vector3F64 {
    return v.floor()
}

#[no_mangle]
pub extern "C" fn dvec3_ceil(v: Vector3F64) -> Vector3F64 {
    return v.ceil()
}

#[no_mangle]
pub extern "C" fn dvec3_round(v: Vector3F64) -> Vector3F64 {
    return v.ceil()
}

#[no_mangle]
pub extern "C" fn dvec3_sqrt(v: Vector3F64) -> Vector3F64 {
    return v.sqrt()
}

#[no_mangle]
pub extern "C" fn dvec3_sin(v: Vector3F64) -> Vector3F64 {
    return v.sin()
}

#[no_mangle]
pub extern "C" fn dvec3_cos(v: Vector3F64) -> Vector3F64 {
    return v.cos()
}

#[no_mangle]
pub extern "C" fn dvec3_tan(v: Vector3F64) -> Vector3F64 {
    return v.tan()
}