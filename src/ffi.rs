//
// Generated at 2022-11-10 19:10:19.359204 by generate_ffi.py
//

#![allow(clippy::needless_return)]

use crate::prelude::*;

//
// Vector2F32
//
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
pub extern "C" fn vec2_normalize(i: Vector2F32) -> Vector2F32 {
    return i.normalize();
}

#[no_mangle]
pub extern "C" fn vec2_abs(i: Vector2F32) -> Vector2F32 {
    return i.abs();
}

#[no_mangle]
pub extern "C" fn vec2_floor(i: Vector2F32) -> Vector2F32 {
    return i.floor();
}

#[no_mangle]
pub extern "C" fn vec2_ceil(i: Vector2F32) -> Vector2F32 {
    return i.ceil();
}

#[no_mangle]
pub extern "C" fn vec2_round(i: Vector2F32) -> Vector2F32 {
    return i.round();
}

#[no_mangle]
pub extern "C" fn vec2_saturate(i: Vector2F32) -> Vector2F32 {
    return i.saturate();
}

#[no_mangle]
pub extern "C" fn vec2_sqrt(i: Vector2F32) -> Vector2F32 {
    return i.sqrt();
}

#[no_mangle]
pub extern "C" fn vec2_sin(i: Vector2F32) -> Vector2F32 {
    return i.sin();
}

#[no_mangle]
pub extern "C" fn vec2_cos(i: Vector2F32) -> Vector2F32 {
    return i.cos();
}

#[no_mangle]
pub extern "C" fn vec2_tan(i: Vector2F32) -> Vector2F32 {
    return i.tan();
}

#[no_mangle]
pub extern "C" fn vec2_sign(i: Vector2F32) -> Vector2F32 {
    return i.sign();
}

#[no_mangle]
pub extern "C" fn vec2_fract(i: Vector2F32) -> Vector2F32 {
    return i.fract();
}

#[no_mangle]
pub extern "C" fn vec2_sum(i: Vector2F32) -> f32 {
    return i.sum();
}

#[no_mangle]
pub extern "C" fn vec2_magnitude(i: Vector2F32) -> f32 {
    return i.magnitude();
}

#[no_mangle]
pub extern "C" fn vec2_magnitude_sqr(i: Vector2F32) -> f32 {
    return i.magnitude_sqr();
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
pub extern "C" fn vec2_pow(lhs: Vector2F32, rhs: Vector2F32) -> Vector2F32 {
    return lhs.pow(rhs);
}

#[no_mangle]
pub extern "C" fn vec2_dot(lhs: Vector2F32, rhs: Vector2F32) -> f32 {
    return lhs.dot(rhs);
}

#[no_mangle]
pub extern "C" fn vec2_lerp(from: Vector2F32, to: Vector2F32, alpha: f32) -> Vector2F32 {
    return from.lerp(to, alpha);
}

#[no_mangle]
pub extern "C" fn vec2_new(x: f32, y: f32) -> Vector2F32 {
    return Vector2F32::new(x, y);
}

#[no_mangle]
pub extern "C" fn vec2_scalar(x: f32) -> Vector2F32 {
    return Vector2F32::from_scalar(x);
}

#[no_mangle]
pub extern "C" fn vec2_default() -> Vector2F32 {
    return Vector2F32::default();
}

#[no_mangle]
pub extern "C" fn vec2_from_vec3(i: Vector3F32) -> Vector2F32 {
    return Vector2F32::from(i);
}

#[no_mangle]
pub extern "C" fn vec2_from_vec4(i: Vector4F32) -> Vector2F32 {
    return Vector2F32::from(i);
}

//
// Vector3F32
//
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
pub extern "C" fn vec3_normalize(i: Vector3F32) -> Vector3F32 {
    return i.normalize();
}

#[no_mangle]
pub extern "C" fn vec3_abs(i: Vector3F32) -> Vector3F32 {
    return i.abs();
}

#[no_mangle]
pub extern "C" fn vec3_floor(i: Vector3F32) -> Vector3F32 {
    return i.floor();
}

#[no_mangle]
pub extern "C" fn vec3_ceil(i: Vector3F32) -> Vector3F32 {
    return i.ceil();
}

#[no_mangle]
pub extern "C" fn vec3_round(i: Vector3F32) -> Vector3F32 {
    return i.round();
}

#[no_mangle]
pub extern "C" fn vec3_saturate(i: Vector3F32) -> Vector3F32 {
    return i.saturate();
}

#[no_mangle]
pub extern "C" fn vec3_sqrt(i: Vector3F32) -> Vector3F32 {
    return i.sqrt();
}

#[no_mangle]
pub extern "C" fn vec3_sin(i: Vector3F32) -> Vector3F32 {
    return i.sin();
}

#[no_mangle]
pub extern "C" fn vec3_cos(i: Vector3F32) -> Vector3F32 {
    return i.cos();
}

#[no_mangle]
pub extern "C" fn vec3_tan(i: Vector3F32) -> Vector3F32 {
    return i.tan();
}

#[no_mangle]
pub extern "C" fn vec3_sign(i: Vector3F32) -> Vector3F32 {
    return i.sign();
}

#[no_mangle]
pub extern "C" fn vec3_fract(i: Vector3F32) -> Vector3F32 {
    return i.fract();
}

#[no_mangle]
pub extern "C" fn vec3_sum(i: Vector3F32) -> f32 {
    return i.sum();
}

#[no_mangle]
pub extern "C" fn vec3_magnitude(i: Vector3F32) -> f32 {
    return i.magnitude();
}

#[no_mangle]
pub extern "C" fn vec3_magnitude_sqr(i: Vector3F32) -> f32 {
    return i.magnitude_sqr();
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
pub extern "C" fn vec3_pow(lhs: Vector3F32, rhs: Vector3F32) -> Vector3F32 {
    return lhs.pow(rhs);
}

#[no_mangle]
pub extern "C" fn vec3_cross(lhs: Vector3F32, rhs: Vector3F32) -> Vector3F32 {
    return lhs.cross(rhs);
}

#[no_mangle]
pub extern "C" fn vec3_reflect(lhs: Vector3F32, rhs: Vector3F32) -> Vector3F32 {
    return lhs.reflect(rhs);
}

#[no_mangle]
pub extern "C" fn vec3_dot(lhs: Vector3F32, rhs: Vector3F32) -> f32 {
    return lhs.dot(rhs);
}

#[no_mangle]
pub extern "C" fn vec3_lerp(from: Vector3F32, to: Vector3F32, alpha: f32) -> Vector3F32 {
    return from.lerp(to, alpha);
}

#[no_mangle]
pub extern "C" fn vec3_new(x: f32, y: f32, z: f32) -> Vector3F32 {
    return Vector3F32::new(x, y, z);
}

#[no_mangle]
pub extern "C" fn vec3_scalar(x: f32) -> Vector3F32 {
    return Vector3F32::from_scalar(x);
}

#[no_mangle]
pub extern "C" fn vec3_default() -> Vector3F32 {
    return Vector3F32::default();
}

#[no_mangle]
pub extern "C" fn vec3_from_vec2(i: Vector2F32) -> Vector3F32 {
    return Vector3F32::from(i);
}

#[no_mangle]
pub extern "C" fn vec3_from_vec4(i: Vector4F32) -> Vector3F32 {
    return Vector3F32::from(i);
}

//
// Vector4F32
//
#[no_mangle]
pub extern "C" fn vec4_add(lhs: Vector4F32, rhs: Vector4F32) -> Vector4F32 {
    return lhs + rhs;
}

#[no_mangle]
pub extern "C" fn vec4_sub(lhs: Vector4F32, rhs: Vector4F32) -> Vector4F32 {
    return lhs - rhs;
}

#[no_mangle]
pub extern "C" fn vec4_mul(lhs: Vector4F32, rhs: Vector4F32) -> Vector4F32 {
    return lhs * rhs;
}

#[no_mangle]
pub extern "C" fn vec4_div(lhs: Vector4F32, rhs: Vector4F32) -> Vector4F32 {
    return lhs / rhs;
}

#[no_mangle]
pub extern "C" fn vec4_mul_mat4(lhs: Vector4F32, rhs: Matrix4x4F32) -> Vector4F32 {
    return lhs * rhs;
}

#[no_mangle]
pub extern "C" fn vec4_normalize(i: Vector4F32) -> Vector4F32 {
    return i.normalize();
}

#[no_mangle]
pub extern "C" fn vec4_abs(i: Vector4F32) -> Vector4F32 {
    return i.abs();
}

#[no_mangle]
pub extern "C" fn vec4_floor(i: Vector4F32) -> Vector4F32 {
    return i.floor();
}

#[no_mangle]
pub extern "C" fn vec4_ceil(i: Vector4F32) -> Vector4F32 {
    return i.ceil();
}

#[no_mangle]
pub extern "C" fn vec4_round(i: Vector4F32) -> Vector4F32 {
    return i.round();
}

#[no_mangle]
pub extern "C" fn vec4_saturate(i: Vector4F32) -> Vector4F32 {
    return i.saturate();
}

#[no_mangle]
pub extern "C" fn vec4_sqrt(i: Vector4F32) -> Vector4F32 {
    return i.sqrt();
}

#[no_mangle]
pub extern "C" fn vec4_sin(i: Vector4F32) -> Vector4F32 {
    return i.sin();
}

#[no_mangle]
pub extern "C" fn vec4_cos(i: Vector4F32) -> Vector4F32 {
    return i.cos();
}

#[no_mangle]
pub extern "C" fn vec4_tan(i: Vector4F32) -> Vector4F32 {
    return i.tan();
}

#[no_mangle]
pub extern "C" fn vec4_sign(i: Vector4F32) -> Vector4F32 {
    return i.sign();
}

#[no_mangle]
pub extern "C" fn vec4_fract(i: Vector4F32) -> Vector4F32 {
    return i.fract();
}

#[no_mangle]
pub extern "C" fn vec4_sum(i: Vector4F32) -> f32 {
    return i.sum();
}

#[no_mangle]
pub extern "C" fn vec4_magnitude(i: Vector4F32) -> f32 {
    return i.magnitude();
}

#[no_mangle]
pub extern "C" fn vec4_magnitude_sqr(i: Vector4F32) -> f32 {
    return i.magnitude_sqr();
}

#[no_mangle]
pub extern "C" fn vec4_min(lhs: Vector4F32, rhs: Vector4F32) -> Vector4F32 {
    return lhs.min(rhs);
}

#[no_mangle]
pub extern "C" fn vec4_max(lhs: Vector4F32, rhs: Vector4F32) -> Vector4F32 {
    return lhs.max(rhs);
}

#[no_mangle]
pub extern "C" fn vec4_pow(lhs: Vector4F32, rhs: Vector4F32) -> Vector4F32 {
    return lhs.pow(rhs);
}

#[no_mangle]
pub extern "C" fn vec4_dot(lhs: Vector4F32, rhs: Vector4F32) -> f32 {
    return lhs.dot(rhs);
}

#[no_mangle]
pub extern "C" fn vec4_lerp(from: Vector4F32, to: Vector4F32, alpha: f32) -> Vector4F32 {
    return from.lerp(to, alpha);
}

#[no_mangle]
pub extern "C" fn vec4_identity() -> Vector4F32 {
    return Vector4F32::identity();
}

#[no_mangle]
pub extern "C" fn vec4_new(x: f32, y: f32, z: f32, w: f32) -> Vector4F32 {
    return Vector4F32::new(x, y, z, w);
}

#[no_mangle]
pub extern "C" fn vec4_scalar(x: f32) -> Vector4F32 {
    return Vector4F32::from_scalar(x);
}

#[no_mangle]
pub extern "C" fn vec4_default() -> Vector4F32 {
    return Vector4F32::default();
}

#[no_mangle]
pub extern "C" fn vec4_from_vec2(i: Vector2F32) -> Vector4F32 {
    return Vector4F32::from(i);
}

#[no_mangle]
pub extern "C" fn vec4_from_vec3(i: Vector3F32) -> Vector4F32 {
    return Vector4F32::from(i);
}

//
// Vector2F64
//
#[no_mangle]
pub extern "C" fn dvec2_add(lhs: Vector2F64, rhs: Vector2F64) -> Vector2F64 {
    return lhs + rhs;
}

#[no_mangle]
pub extern "C" fn dvec2_sub(lhs: Vector2F64, rhs: Vector2F64) -> Vector2F64 {
    return lhs - rhs;
}

#[no_mangle]
pub extern "C" fn dvec2_mul(lhs: Vector2F64, rhs: Vector2F64) -> Vector2F64 {
    return lhs * rhs;
}

#[no_mangle]
pub extern "C" fn dvec2_div(lhs: Vector2F64, rhs: Vector2F64) -> Vector2F64 {
    return lhs / rhs;
}

#[no_mangle]
pub extern "C" fn dvec2_normalize(i: Vector2F64) -> Vector2F64 {
    return i.normalize();
}

#[no_mangle]
pub extern "C" fn dvec2_abs(i: Vector2F64) -> Vector2F64 {
    return i.abs();
}

#[no_mangle]
pub extern "C" fn dvec2_floor(i: Vector2F64) -> Vector2F64 {
    return i.floor();
}

#[no_mangle]
pub extern "C" fn dvec2_ceil(i: Vector2F64) -> Vector2F64 {
    return i.ceil();
}

#[no_mangle]
pub extern "C" fn dvec2_round(i: Vector2F64) -> Vector2F64 {
    return i.round();
}

#[no_mangle]
pub extern "C" fn dvec2_saturate(i: Vector2F64) -> Vector2F64 {
    return i.saturate();
}

#[no_mangle]
pub extern "C" fn dvec2_sqrt(i: Vector2F64) -> Vector2F64 {
    return i.sqrt();
}

#[no_mangle]
pub extern "C" fn dvec2_sin(i: Vector2F64) -> Vector2F64 {
    return i.sin();
}

#[no_mangle]
pub extern "C" fn dvec2_cos(i: Vector2F64) -> Vector2F64 {
    return i.cos();
}

#[no_mangle]
pub extern "C" fn dvec2_tan(i: Vector2F64) -> Vector2F64 {
    return i.tan();
}

#[no_mangle]
pub extern "C" fn dvec2_sign(i: Vector2F64) -> Vector2F64 {
    return i.sign();
}

#[no_mangle]
pub extern "C" fn dvec2_fract(i: Vector2F64) -> Vector2F64 {
    return i.fract();
}

#[no_mangle]
pub extern "C" fn dvec2_sum(i: Vector2F64) -> f64 {
    return i.sum();
}

#[no_mangle]
pub extern "C" fn dvec2_magnitude(i: Vector2F64) -> f64 {
    return i.magnitude();
}

#[no_mangle]
pub extern "C" fn dvec2_magnitude_sqr(i: Vector2F64) -> f64 {
    return i.magnitude_sqr();
}

#[no_mangle]
pub extern "C" fn dvec2_min(lhs: Vector2F64, rhs: Vector2F64) -> Vector2F64 {
    return lhs.min(rhs);
}

#[no_mangle]
pub extern "C" fn dvec2_max(lhs: Vector2F64, rhs: Vector2F64) -> Vector2F64 {
    return lhs.max(rhs);
}

#[no_mangle]
pub extern "C" fn dvec2_pow(lhs: Vector2F64, rhs: Vector2F64) -> Vector2F64 {
    return lhs.pow(rhs);
}

#[no_mangle]
pub extern "C" fn dvec2_dot(lhs: Vector2F64, rhs: Vector2F64) -> f64 {
    return lhs.dot(rhs);
}

#[no_mangle]
pub extern "C" fn dvec2_lerp(from: Vector2F64, to: Vector2F64, alpha: f64) -> Vector2F64 {
    return from.lerp(to, alpha);
}

#[no_mangle]
pub extern "C" fn dvec2_new(x: f64, y: f64) -> Vector2F64 {
    return Vector2F64::new(x, y);
}

#[no_mangle]
pub extern "C" fn dvec2_scalar(x: f64) -> Vector2F64 {
    return Vector2F64::from_scalar(x);
}

#[no_mangle]
pub extern "C" fn dvec2_default() -> Vector2F64 {
    return Vector2F64::default();
}

#[no_mangle]
pub extern "C" fn dvec2_from_vec3(i: Vector3F64) -> Vector2F64 {
    return Vector2F64::from(i);
}

#[no_mangle]
pub extern "C" fn dvec2_from_vec4(i: Vector4F64) -> Vector2F64 {
    return Vector2F64::from(i);
}

//
// Vector3F64
//
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
pub extern "C" fn dvec3_normalize(i: Vector3F64) -> Vector3F64 {
    return i.normalize();
}

#[no_mangle]
pub extern "C" fn dvec3_abs(i: Vector3F64) -> Vector3F64 {
    return i.abs();
}

#[no_mangle]
pub extern "C" fn dvec3_floor(i: Vector3F64) -> Vector3F64 {
    return i.floor();
}

#[no_mangle]
pub extern "C" fn dvec3_ceil(i: Vector3F64) -> Vector3F64 {
    return i.ceil();
}

#[no_mangle]
pub extern "C" fn dvec3_round(i: Vector3F64) -> Vector3F64 {
    return i.round();
}

#[no_mangle]
pub extern "C" fn dvec3_saturate(i: Vector3F64) -> Vector3F64 {
    return i.saturate();
}

#[no_mangle]
pub extern "C" fn dvec3_sqrt(i: Vector3F64) -> Vector3F64 {
    return i.sqrt();
}

#[no_mangle]
pub extern "C" fn dvec3_sin(i: Vector3F64) -> Vector3F64 {
    return i.sin();
}

#[no_mangle]
pub extern "C" fn dvec3_cos(i: Vector3F64) -> Vector3F64 {
    return i.cos();
}

#[no_mangle]
pub extern "C" fn dvec3_tan(i: Vector3F64) -> Vector3F64 {
    return i.tan();
}

#[no_mangle]
pub extern "C" fn dvec3_sign(i: Vector3F64) -> Vector3F64 {
    return i.sign();
}

#[no_mangle]
pub extern "C" fn dvec3_fract(i: Vector3F64) -> Vector3F64 {
    return i.fract();
}

#[no_mangle]
pub extern "C" fn dvec3_sum(i: Vector3F64) -> f64 {
    return i.sum();
}

#[no_mangle]
pub extern "C" fn dvec3_magnitude(i: Vector3F64) -> f64 {
    return i.magnitude();
}

#[no_mangle]
pub extern "C" fn dvec3_magnitude_sqr(i: Vector3F64) -> f64 {
    return i.magnitude_sqr();
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
pub extern "C" fn dvec3_pow(lhs: Vector3F64, rhs: Vector3F64) -> Vector3F64 {
    return lhs.pow(rhs);
}

#[no_mangle]
pub extern "C" fn dvec3_cross(lhs: Vector3F64, rhs: Vector3F64) -> Vector3F64 {
    return lhs.cross(rhs);
}

#[no_mangle]
pub extern "C" fn dvec3_reflect(lhs: Vector3F64, rhs: Vector3F64) -> Vector3F64 {
    return lhs.reflect(rhs);
}

#[no_mangle]
pub extern "C" fn dvec3_dot(lhs: Vector3F64, rhs: Vector3F64) -> f64 {
    return lhs.dot(rhs);
}

#[no_mangle]
pub extern "C" fn dvec3_lerp(from: Vector3F64, to: Vector3F64, alpha: f64) -> Vector3F64 {
    return from.lerp(to, alpha);
}

#[no_mangle]
pub extern "C" fn dvec3_new(x: f64, y: f64, z: f64) -> Vector3F64 {
    return Vector3F64::new(x, y, z);
}

#[no_mangle]
pub extern "C" fn dvec3_scalar(x: f64) -> Vector3F64 {
    return Vector3F64::from_scalar(x);
}

#[no_mangle]
pub extern "C" fn dvec3_default() -> Vector3F64 {
    return Vector3F64::default();
}

#[no_mangle]
pub extern "C" fn dvec3_from_vec2(i: Vector2F64) -> Vector3F64 {
    return Vector3F64::from(i);
}

#[no_mangle]
pub extern "C" fn dvec3_from_vec4(i: Vector4F64) -> Vector3F64 {
    return Vector3F64::from(i);
}

//
// Vector4F64
//
#[no_mangle]
pub extern "C" fn dvec4_add(lhs: Vector4F64, rhs: Vector4F64) -> Vector4F64 {
    return lhs + rhs;
}

#[no_mangle]
pub extern "C" fn dvec4_sub(lhs: Vector4F64, rhs: Vector4F64) -> Vector4F64 {
    return lhs - rhs;
}

#[no_mangle]
pub extern "C" fn dvec4_mul(lhs: Vector4F64, rhs: Vector4F64) -> Vector4F64 {
    return lhs * rhs;
}

#[no_mangle]
pub extern "C" fn dvec4_div(lhs: Vector4F64, rhs: Vector4F64) -> Vector4F64 {
    return lhs / rhs;
}

#[no_mangle]
pub extern "C" fn dvec4_mul_mat4(lhs: Vector4F64, rhs: Matrix4x4F64) -> Vector4F64 {
    return lhs * rhs;
}

#[no_mangle]
pub extern "C" fn dvec4_normalize(i: Vector4F64) -> Vector4F64 {
    return i.normalize();
}

#[no_mangle]
pub extern "C" fn dvec4_abs(i: Vector4F64) -> Vector4F64 {
    return i.abs();
}

#[no_mangle]
pub extern "C" fn dvec4_floor(i: Vector4F64) -> Vector4F64 {
    return i.floor();
}

#[no_mangle]
pub extern "C" fn dvec4_ceil(i: Vector4F64) -> Vector4F64 {
    return i.ceil();
}

#[no_mangle]
pub extern "C" fn dvec4_round(i: Vector4F64) -> Vector4F64 {
    return i.round();
}

#[no_mangle]
pub extern "C" fn dvec4_saturate(i: Vector4F64) -> Vector4F64 {
    return i.saturate();
}

#[no_mangle]
pub extern "C" fn dvec4_sqrt(i: Vector4F64) -> Vector4F64 {
    return i.sqrt();
}

#[no_mangle]
pub extern "C" fn dvec4_sin(i: Vector4F64) -> Vector4F64 {
    return i.sin();
}

#[no_mangle]
pub extern "C" fn dvec4_cos(i: Vector4F64) -> Vector4F64 {
    return i.cos();
}

#[no_mangle]
pub extern "C" fn dvec4_tan(i: Vector4F64) -> Vector4F64 {
    return i.tan();
}

#[no_mangle]
pub extern "C" fn dvec4_sign(i: Vector4F64) -> Vector4F64 {
    return i.sign();
}

#[no_mangle]
pub extern "C" fn dvec4_fract(i: Vector4F64) -> Vector4F64 {
    return i.fract();
}

#[no_mangle]
pub extern "C" fn dvec4_sum(i: Vector4F64) -> f64 {
    return i.sum();
}

#[no_mangle]
pub extern "C" fn dvec4_magnitude(i: Vector4F64) -> f64 {
    return i.magnitude();
}

#[no_mangle]
pub extern "C" fn dvec4_magnitude_sqr(i: Vector4F64) -> f64 {
    return i.magnitude_sqr();
}

#[no_mangle]
pub extern "C" fn dvec4_min(lhs: Vector4F64, rhs: Vector4F64) -> Vector4F64 {
    return lhs.min(rhs);
}

#[no_mangle]
pub extern "C" fn dvec4_max(lhs: Vector4F64, rhs: Vector4F64) -> Vector4F64 {
    return lhs.max(rhs);
}

#[no_mangle]
pub extern "C" fn dvec4_pow(lhs: Vector4F64, rhs: Vector4F64) -> Vector4F64 {
    return lhs.pow(rhs);
}

#[no_mangle]
pub extern "C" fn dvec4_dot(lhs: Vector4F64, rhs: Vector4F64) -> f64 {
    return lhs.dot(rhs);
}

#[no_mangle]
pub extern "C" fn dvec4_lerp(from: Vector4F64, to: Vector4F64, alpha: f64) -> Vector4F64 {
    return from.lerp(to, alpha);
}

#[no_mangle]
pub extern "C" fn dvec4_identity() -> Vector4F64 {
    return Vector4F64::identity();
}

#[no_mangle]
pub extern "C" fn dvec4_new(x: f64, y: f64, z: f64, w: f64) -> Vector4F64 {
    return Vector4F64::new(x, y, z, w);
}

#[no_mangle]
pub extern "C" fn dvec4_scalar(x: f64) -> Vector4F64 {
    return Vector4F64::from_scalar(x);
}

#[no_mangle]
pub extern "C" fn dvec4_default() -> Vector4F64 {
    return Vector4F64::default();
}

#[no_mangle]
pub extern "C" fn dvec4_from_vec2(i: Vector2F64) -> Vector4F64 {
    return Vector4F64::from(i);
}

#[no_mangle]
pub extern "C" fn dvec4_from_vec3(i: Vector3F64) -> Vector4F64 {
    return Vector4F64::from(i);
}

//
// Matrix4x4F32
//
#[no_mangle]
pub extern "C" fn mat4_mul(lhs: Matrix4x4F32, rhs: Matrix4x4F32) -> Matrix4x4F32 {
    return lhs * rhs;
}

#[no_mangle]
pub extern "C" fn mat4_mul_vec4(lhs: Matrix4x4F32, rhs: Vector4F32) -> Vector4F32 {
    return lhs * rhs;
}

#[no_mangle]
pub extern "C" fn mat4_inverse(i: Matrix4x4F32) -> Matrix4x4F32 {
    return i.inverse();
}

#[no_mangle]
pub extern "C" fn mat4_transpose(i: Matrix4x4F32) -> Matrix4x4F32 {
    return i.transpose();
}

#[no_mangle]
pub extern "C" fn mat4_translation(vector: Vector3F32) -> Matrix4x4F32 {
    return Matrix4x4F32::translation(vector);
}

#[no_mangle]
pub extern "C" fn mat4_rotation(euler: Vector3F32) -> Matrix4x4F32 {
    return Matrix4x4F32::rotation(euler);
}

#[no_mangle]
pub extern "C" fn mat4_rotate_x(degrees: f32) -> Matrix4x4F32 {
    return Matrix4x4F32::rotate_x(degrees);
}

#[no_mangle]
pub extern "C" fn mat4_rotate_y(degrees: f32) -> Matrix4x4F32 {
    return Matrix4x4F32::rotate_y(degrees);
}

#[no_mangle]
pub extern "C" fn mat4_rotate_z(degrees: f32) -> Matrix4x4F32 {
    return Matrix4x4F32::rotate_z(degrees);
}

#[no_mangle]
pub extern "C" fn mat4_perspective(
    fov_y: f32,
    aspect: f32,
    z_near: f32,
    z_far: f32,
) -> Matrix4x4F32 {
    return Matrix4x4F32::perspective(fov_y, aspect, z_near, z_far);
}

#[no_mangle]
pub extern "C" fn mat4_default() -> Matrix4x4F32 {
    return Matrix4x4F32::default();
}

#[no_mangle]
pub extern "C" fn mat4_identity() -> Matrix4x4F32 {
    return Matrix4x4F32::identity();
}

//
// Matrix4x4F64
//
#[no_mangle]
pub extern "C" fn dmat4_mul(lhs: Matrix4x4F64, rhs: Matrix4x4F64) -> Matrix4x4F64 {
    return lhs * rhs;
}

#[no_mangle]
pub extern "C" fn dmat4_mul_vec4(lhs: Matrix4x4F64, rhs: Vector4F64) -> Vector4F64 {
    return lhs * rhs;
}

#[no_mangle]
pub extern "C" fn dmat4_inverse(i: Matrix4x4F64) -> Matrix4x4F64 {
    return i.inverse();
}

#[no_mangle]
pub extern "C" fn dmat4_transpose(i: Matrix4x4F64) -> Matrix4x4F64 {
    return i.transpose();
}

#[no_mangle]
pub extern "C" fn dmat4_translation(vector: Vector3F64) -> Matrix4x4F64 {
    return Matrix4x4F64::translation(vector);
}

#[no_mangle]
pub extern "C" fn dmat4_rotation(euler: Vector3F64) -> Matrix4x4F64 {
    return Matrix4x4F64::rotation(euler);
}

#[no_mangle]
pub extern "C" fn dmat4_rotate_x(degrees: f64) -> Matrix4x4F64 {
    return Matrix4x4F64::rotate_x(degrees);
}

#[no_mangle]
pub extern "C" fn dmat4_rotate_y(degrees: f64) -> Matrix4x4F64 {
    return Matrix4x4F64::rotate_y(degrees);
}

#[no_mangle]
pub extern "C" fn dmat4_rotate_z(degrees: f64) -> Matrix4x4F64 {
    return Matrix4x4F64::rotate_z(degrees);
}

#[no_mangle]
pub extern "C" fn dmat4_perspective(
    fov_y: f64,
    aspect: f64,
    z_near: f64,
    z_far: f64,
) -> Matrix4x4F64 {
    return Matrix4x4F64::perspective(fov_y, aspect, z_near, z_far);
}

#[no_mangle]
pub extern "C" fn dmat4_default() -> Matrix4x4F64 {
    return Matrix4x4F64::default();
}

#[no_mangle]
pub extern "C" fn dmat4_identity() -> Matrix4x4F64 {
    return Matrix4x4F64::identity();
}
