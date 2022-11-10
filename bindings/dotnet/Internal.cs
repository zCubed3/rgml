//
// Generated at 2022-11-10 06:21:17.985565 by generate_ffi.py
//

using System.Runtime.InteropServices;

namespace PrismMath.Internal {
	internal class Vector2F32 {
		[DllImport("prism_math")]
		internal static extern Vector2 vec2_add(Vector2 lhs, Vector2 rhs);

		[DllImport("prism_math")]
		internal static extern Vector2 vec2_sub(Vector2 lhs, Vector2 rhs);

		[DllImport("prism_math")]
		internal static extern Vector2 vec2_mul(Vector2 lhs, Vector2 rhs);

		[DllImport("prism_math")]
		internal static extern Vector2 vec2_div(Vector2 lhs, Vector2 rhs);

		[DllImport("prism_math")]
		internal static extern Vector2 vec2_normalize(Vector2 i);

		[DllImport("prism_math")]
		internal static extern Vector2 vec2_abs(Vector2 i);

		[DllImport("prism_math")]
		internal static extern Vector2 vec2_floor(Vector2 i);

		[DllImport("prism_math")]
		internal static extern Vector2 vec2_ceil(Vector2 i);

		[DllImport("prism_math")]
		internal static extern Vector2 vec2_round(Vector2 i);

		[DllImport("prism_math")]
		internal static extern Vector2 vec2_saturate(Vector2 i);

		[DllImport("prism_math")]
		internal static extern Vector2 vec2_sqrt(Vector2 i);

		[DllImport("prism_math")]
		internal static extern Vector2 vec2_sin(Vector2 i);

		[DllImport("prism_math")]
		internal static extern Vector2 vec2_cos(Vector2 i);

		[DllImport("prism_math")]
		internal static extern Vector2 vec2_tan(Vector2 i);

		[DllImport("prism_math")]
		internal static extern Vector2 vec2_sign(Vector2 i);

		[DllImport("prism_math")]
		internal static extern Vector2 vec2_fract(Vector2 i);

		[DllImport("prism_math")]
		internal static extern float vec2_sum(Vector2 i);

		[DllImport("prism_math")]
		internal static extern float vec2_magnitude(Vector2 i);

		[DllImport("prism_math")]
		internal static extern float vec2_magnitude_sqr(Vector2 i);

		[DllImport("prism_math")]
		internal static extern Vector2 vec2_min(Vector2 lhs, Vector2 rhs);

		[DllImport("prism_math")]
		internal static extern Vector2 vec2_max(Vector2 lhs, Vector2 rhs);

		[DllImport("prism_math")]
		internal static extern Vector2 vec2_pow(Vector2 lhs, Vector2 rhs);

		[DllImport("prism_math")]
		internal static extern float vec2_dot(Vector2 lhs, Vector2 rhs);

		[DllImport("prism_math")]
		internal static extern Vector2 vec2_lerp(Vector2 from, Vector2 to, float alpha);

		[DllImport("prism_math")]
		internal static extern Vector2 vec2_new(float x, float y);

		[DllImport("prism_math")]
		internal static extern Vector2 vec2_scalar(float x);

		[DllImport("prism_math")]
		internal static extern Vector2 vec2_default();

		[DllImport("prism_math")]
		internal static extern Vector2 vec2_from_vec3(Vector3 i);

		[DllImport("prism_math")]
		internal static extern Vector2 vec2_from_vec4(Vector4 i);

		// Skeleton Bindings (these are placeholders, they may need to be manually fixed)
		/*
		public Vector2 Add(Vector2 rhs) => Vector2F32.vec2_add(this, rhs);
		public Vector2 Sub(Vector2 rhs) => Vector2F32.vec2_sub(this, rhs);
		public Vector2 Mul(Vector2 rhs) => Vector2F32.vec2_mul(this, rhs);
		public Vector2 Div(Vector2 rhs) => Vector2F32.vec2_div(this, rhs);
		public Vector2 Normalize() => Vector2F32.vec2_normalize(this);
		public Vector2 Abs() => Vector2F32.vec2_abs(this);
		public Vector2 Floor() => Vector2F32.vec2_floor(this);
		public Vector2 Ceil() => Vector2F32.vec2_ceil(this);
		public Vector2 Round() => Vector2F32.vec2_round(this);
		public Vector2 Saturate() => Vector2F32.vec2_saturate(this);
		public Vector2 Sqrt() => Vector2F32.vec2_sqrt(this);
		public Vector2 Sin() => Vector2F32.vec2_sin(this);
		public Vector2 Cos() => Vector2F32.vec2_cos(this);
		public Vector2 Tan() => Vector2F32.vec2_tan(this);
		public Vector2 Sign() => Vector2F32.vec2_sign(this);
		public Vector2 Fract() => Vector2F32.vec2_fract(this);
		public float Sum() => Vector2F32.vec2_sum(this);
		public float Magnitude() => Vector2F32.vec2_magnitude(this);
		public float Magnitude_sqr() => Vector2F32.vec2_magnitude_sqr(this);
		public Vector2 Min(Vector2 rhs) => Vector2F32.vec2_min(this, rhs);
		public Vector2 Max(Vector2 rhs) => Vector2F32.vec2_max(this, rhs);
		public Vector2 Pow(Vector2 rhs) => Vector2F32.vec2_pow(this, rhs);
		public float Dot(Vector2 rhs) => Vector2F32.vec2_dot(this, rhs);
		public Vector2 Lerp(Vector2 to, float alpha) => Vector2F32.vec2_lerp(this, to, alpha);
		public Vector2 New(float x, float y) => Vector2F32.vec2_new(x, y);
		public Vector2 Scalar(float x) => Vector2F32.vec2_scalar(x);
		public Vector2 Default() => Vector2F32.vec2_default();
		public Vector2 From_vec3(Vector3 i) => Vector2F32.vec2_from_vec3(i);
		public Vector2 From_vec4(Vector4 i) => Vector2F32.vec2_from_vec4(i);
		*/
	}
	internal class Vector3F32 {
		[DllImport("prism_math")]
		internal static extern Vector3 vec3_add(Vector3 lhs, Vector3 rhs);

		[DllImport("prism_math")]
		internal static extern Vector3 vec3_sub(Vector3 lhs, Vector3 rhs);

		[DllImport("prism_math")]
		internal static extern Vector3 vec3_mul(Vector3 lhs, Vector3 rhs);

		[DllImport("prism_math")]
		internal static extern Vector3 vec3_div(Vector3 lhs, Vector3 rhs);

		[DllImport("prism_math")]
		internal static extern Vector3 vec3_normalize(Vector3 i);

		[DllImport("prism_math")]
		internal static extern Vector3 vec3_abs(Vector3 i);

		[DllImport("prism_math")]
		internal static extern Vector3 vec3_floor(Vector3 i);

		[DllImport("prism_math")]
		internal static extern Vector3 vec3_ceil(Vector3 i);

		[DllImport("prism_math")]
		internal static extern Vector3 vec3_round(Vector3 i);

		[DllImport("prism_math")]
		internal static extern Vector3 vec3_saturate(Vector3 i);

		[DllImport("prism_math")]
		internal static extern Vector3 vec3_sqrt(Vector3 i);

		[DllImport("prism_math")]
		internal static extern Vector3 vec3_sin(Vector3 i);

		[DllImport("prism_math")]
		internal static extern Vector3 vec3_cos(Vector3 i);

		[DllImport("prism_math")]
		internal static extern Vector3 vec3_tan(Vector3 i);

		[DllImport("prism_math")]
		internal static extern Vector3 vec3_sign(Vector3 i);

		[DllImport("prism_math")]
		internal static extern Vector3 vec3_fract(Vector3 i);

		[DllImport("prism_math")]
		internal static extern float vec3_sum(Vector3 i);

		[DllImport("prism_math")]
		internal static extern float vec3_magnitude(Vector3 i);

		[DllImport("prism_math")]
		internal static extern float vec3_magnitude_sqr(Vector3 i);

		[DllImport("prism_math")]
		internal static extern Vector3 vec3_min(Vector3 lhs, Vector3 rhs);

		[DllImport("prism_math")]
		internal static extern Vector3 vec3_max(Vector3 lhs, Vector3 rhs);

		[DllImport("prism_math")]
		internal static extern Vector3 vec3_pow(Vector3 lhs, Vector3 rhs);

		[DllImport("prism_math")]
		internal static extern Vector3 vec3_cross(Vector3 lhs, Vector3 rhs);

		[DllImport("prism_math")]
		internal static extern Vector3 vec3_reflect(Vector3 lhs, Vector3 rhs);

		[DllImport("prism_math")]
		internal static extern float vec3_dot(Vector3 lhs, Vector3 rhs);

		[DllImport("prism_math")]
		internal static extern Vector3 vec3_lerp(Vector3 from, Vector3 to, float alpha);

		[DllImport("prism_math")]
		internal static extern Vector3 vec3_new(float x, float y, float z);

		[DllImport("prism_math")]
		internal static extern Vector3 vec3_scalar(float x);

		[DllImport("prism_math")]
		internal static extern Vector3 vec3_default();

		[DllImport("prism_math")]
		internal static extern Vector3 vec3_from_vec2(Vector2 i);

		[DllImport("prism_math")]
		internal static extern Vector3 vec3_from_vec4(Vector4 i);

		// Skeleton Bindings (these are placeholders, they may need to be manually fixed)
		/*
		public Vector3 Add(Vector3 rhs) => Vector3F32.vec3_add(this, rhs);
		public Vector3 Sub(Vector3 rhs) => Vector3F32.vec3_sub(this, rhs);
		public Vector3 Mul(Vector3 rhs) => Vector3F32.vec3_mul(this, rhs);
		public Vector3 Div(Vector3 rhs) => Vector3F32.vec3_div(this, rhs);
		public Vector3 Normalize() => Vector3F32.vec3_normalize(this);
		public Vector3 Abs() => Vector3F32.vec3_abs(this);
		public Vector3 Floor() => Vector3F32.vec3_floor(this);
		public Vector3 Ceil() => Vector3F32.vec3_ceil(this);
		public Vector3 Round() => Vector3F32.vec3_round(this);
		public Vector3 Saturate() => Vector3F32.vec3_saturate(this);
		public Vector3 Sqrt() => Vector3F32.vec3_sqrt(this);
		public Vector3 Sin() => Vector3F32.vec3_sin(this);
		public Vector3 Cos() => Vector3F32.vec3_cos(this);
		public Vector3 Tan() => Vector3F32.vec3_tan(this);
		public Vector3 Sign() => Vector3F32.vec3_sign(this);
		public Vector3 Fract() => Vector3F32.vec3_fract(this);
		public float Sum() => Vector3F32.vec3_sum(this);
		public float Magnitude() => Vector3F32.vec3_magnitude(this);
		public float Magnitude_sqr() => Vector3F32.vec3_magnitude_sqr(this);
		public Vector3 Min(Vector3 rhs) => Vector3F32.vec3_min(this, rhs);
		public Vector3 Max(Vector3 rhs) => Vector3F32.vec3_max(this, rhs);
		public Vector3 Pow(Vector3 rhs) => Vector3F32.vec3_pow(this, rhs);
		public Vector3 Cross(Vector3 rhs) => Vector3F32.vec3_cross(this, rhs);
		public Vector3 Reflect(Vector3 rhs) => Vector3F32.vec3_reflect(this, rhs);
		public float Dot(Vector3 rhs) => Vector3F32.vec3_dot(this, rhs);
		public Vector3 Lerp(Vector3 to, float alpha) => Vector3F32.vec3_lerp(this, to, alpha);
		public Vector3 New(float x, float y, float z) => Vector3F32.vec3_new(x, y, z);
		public Vector3 Scalar(float x) => Vector3F32.vec3_scalar(x);
		public Vector3 Default() => Vector3F32.vec3_default();
		public Vector3 From_vec2(Vector2 i) => Vector3F32.vec3_from_vec2(i);
		public Vector3 From_vec4(Vector4 i) => Vector3F32.vec3_from_vec4(i);
		*/
	}
	internal class Vector4F32 {
		[DllImport("prism_math")]
		internal static extern Vector4 vec4_add(Vector4 lhs, Vector4 rhs);

		[DllImport("prism_math")]
		internal static extern Vector4 vec4_sub(Vector4 lhs, Vector4 rhs);

		[DllImport("prism_math")]
		internal static extern Vector4 vec4_mul(Vector4 lhs, Vector4 rhs);

		[DllImport("prism_math")]
		internal static extern Vector4 vec4_div(Vector4 lhs, Vector4 rhs);

		[DllImport("prism_math")]
		internal static extern Vector4 vec4_mul_mat4(Vector4 lhs, Matrix4x4 rhs);

		[DllImport("prism_math")]
		internal static extern Vector4 vec4_normalize(Vector4 i);

		[DllImport("prism_math")]
		internal static extern Vector4 vec4_abs(Vector4 i);

		[DllImport("prism_math")]
		internal static extern Vector4 vec4_floor(Vector4 i);

		[DllImport("prism_math")]
		internal static extern Vector4 vec4_ceil(Vector4 i);

		[DllImport("prism_math")]
		internal static extern Vector4 vec4_round(Vector4 i);

		[DllImport("prism_math")]
		internal static extern Vector4 vec4_saturate(Vector4 i);

		[DllImport("prism_math")]
		internal static extern Vector4 vec4_sqrt(Vector4 i);

		[DllImport("prism_math")]
		internal static extern Vector4 vec4_sin(Vector4 i);

		[DllImport("prism_math")]
		internal static extern Vector4 vec4_cos(Vector4 i);

		[DllImport("prism_math")]
		internal static extern Vector4 vec4_tan(Vector4 i);

		[DllImport("prism_math")]
		internal static extern Vector4 vec4_sign(Vector4 i);

		[DllImport("prism_math")]
		internal static extern Vector4 vec4_fract(Vector4 i);

		[DllImport("prism_math")]
		internal static extern float vec4_sum(Vector4 i);

		[DllImport("prism_math")]
		internal static extern float vec4_magnitude(Vector4 i);

		[DllImport("prism_math")]
		internal static extern float vec4_magnitude_sqr(Vector4 i);

		[DllImport("prism_math")]
		internal static extern Vector4 vec4_min(Vector4 lhs, Vector4 rhs);

		[DllImport("prism_math")]
		internal static extern Vector4 vec4_max(Vector4 lhs, Vector4 rhs);

		[DllImport("prism_math")]
		internal static extern Vector4 vec4_pow(Vector4 lhs, Vector4 rhs);

		[DllImport("prism_math")]
		internal static extern float vec4_dot(Vector4 lhs, Vector4 rhs);

		[DllImport("prism_math")]
		internal static extern Vector4 vec4_lerp(Vector4 from, Vector4 to, float alpha);

		[DllImport("prism_math")]
		internal static extern Vector4 vec4_identity();

		[DllImport("prism_math")]
		internal static extern Vector4 vec4_new(float x, float y, float z, float w);

		[DllImport("prism_math")]
		internal static extern Vector4 vec4_scalar(float x);

		[DllImport("prism_math")]
		internal static extern Vector4 vec4_default();

		[DllImport("prism_math")]
		internal static extern Vector4 vec4_from_vec2(Vector2 i);

		[DllImport("prism_math")]
		internal static extern Vector4 vec4_from_vec3(Vector3 i);

		// Skeleton Bindings (these are placeholders, they may need to be manually fixed)
		/*
		public Vector4 Add(Vector4 rhs) => Vector4F32.vec4_add(this, rhs);
		public Vector4 Sub(Vector4 rhs) => Vector4F32.vec4_sub(this, rhs);
		public Vector4 Mul(Vector4 rhs) => Vector4F32.vec4_mul(this, rhs);
		public Vector4 Div(Vector4 rhs) => Vector4F32.vec4_div(this, rhs);
		public Vector4 Mul_mat4(Matrix4x4 rhs) => Vector4F32.vec4_mul_mat4(this, rhs);
		public Vector4 Normalize() => Vector4F32.vec4_normalize(this);
		public Vector4 Abs() => Vector4F32.vec4_abs(this);
		public Vector4 Floor() => Vector4F32.vec4_floor(this);
		public Vector4 Ceil() => Vector4F32.vec4_ceil(this);
		public Vector4 Round() => Vector4F32.vec4_round(this);
		public Vector4 Saturate() => Vector4F32.vec4_saturate(this);
		public Vector4 Sqrt() => Vector4F32.vec4_sqrt(this);
		public Vector4 Sin() => Vector4F32.vec4_sin(this);
		public Vector4 Cos() => Vector4F32.vec4_cos(this);
		public Vector4 Tan() => Vector4F32.vec4_tan(this);
		public Vector4 Sign() => Vector4F32.vec4_sign(this);
		public Vector4 Fract() => Vector4F32.vec4_fract(this);
		public float Sum() => Vector4F32.vec4_sum(this);
		public float Magnitude() => Vector4F32.vec4_magnitude(this);
		public float Magnitude_sqr() => Vector4F32.vec4_magnitude_sqr(this);
		public Vector4 Min(Vector4 rhs) => Vector4F32.vec4_min(this, rhs);
		public Vector4 Max(Vector4 rhs) => Vector4F32.vec4_max(this, rhs);
		public Vector4 Pow(Vector4 rhs) => Vector4F32.vec4_pow(this, rhs);
		public float Dot(Vector4 rhs) => Vector4F32.vec4_dot(this, rhs);
		public Vector4 Lerp(Vector4 to, float alpha) => Vector4F32.vec4_lerp(this, to, alpha);
		public Vector4 Identity() => Vector4F32.vec4_identity();
		public Vector4 New(float x, float y, float z, float w) => Vector4F32.vec4_new(x, y, z, w);
		public Vector4 Scalar(float x) => Vector4F32.vec4_scalar(x);
		public Vector4 Default() => Vector4F32.vec4_default();
		public Vector4 From_vec2(Vector2 i) => Vector4F32.vec4_from_vec2(i);
		public Vector4 From_vec3(Vector3 i) => Vector4F32.vec4_from_vec3(i);
		*/
	}
	internal class Vector2F64 {
		[DllImport("prism_math")]
		internal static extern DVector2 dvec2_add(DVector2 lhs, DVector2 rhs);

		[DllImport("prism_math")]
		internal static extern DVector2 dvec2_sub(DVector2 lhs, DVector2 rhs);

		[DllImport("prism_math")]
		internal static extern DVector2 dvec2_mul(DVector2 lhs, DVector2 rhs);

		[DllImport("prism_math")]
		internal static extern DVector2 dvec2_div(DVector2 lhs, DVector2 rhs);

		[DllImport("prism_math")]
		internal static extern DVector2 dvec2_normalize(DVector2 i);

		[DllImport("prism_math")]
		internal static extern DVector2 dvec2_abs(DVector2 i);

		[DllImport("prism_math")]
		internal static extern DVector2 dvec2_floor(DVector2 i);

		[DllImport("prism_math")]
		internal static extern DVector2 dvec2_ceil(DVector2 i);

		[DllImport("prism_math")]
		internal static extern DVector2 dvec2_round(DVector2 i);

		[DllImport("prism_math")]
		internal static extern DVector2 dvec2_saturate(DVector2 i);

		[DllImport("prism_math")]
		internal static extern DVector2 dvec2_sqrt(DVector2 i);

		[DllImport("prism_math")]
		internal static extern DVector2 dvec2_sin(DVector2 i);

		[DllImport("prism_math")]
		internal static extern DVector2 dvec2_cos(DVector2 i);

		[DllImport("prism_math")]
		internal static extern DVector2 dvec2_tan(DVector2 i);

		[DllImport("prism_math")]
		internal static extern DVector2 dvec2_sign(DVector2 i);

		[DllImport("prism_math")]
		internal static extern DVector2 dvec2_fract(DVector2 i);

		[DllImport("prism_math")]
		internal static extern double dvec2_sum(DVector2 i);

		[DllImport("prism_math")]
		internal static extern double dvec2_magnitude(DVector2 i);

		[DllImport("prism_math")]
		internal static extern double dvec2_magnitude_sqr(DVector2 i);

		[DllImport("prism_math")]
		internal static extern DVector2 dvec2_min(DVector2 lhs, DVector2 rhs);

		[DllImport("prism_math")]
		internal static extern DVector2 dvec2_max(DVector2 lhs, DVector2 rhs);

		[DllImport("prism_math")]
		internal static extern DVector2 dvec2_pow(DVector2 lhs, DVector2 rhs);

		[DllImport("prism_math")]
		internal static extern double dvec2_dot(DVector2 lhs, DVector2 rhs);

		[DllImport("prism_math")]
		internal static extern DVector2 dvec2_lerp(DVector2 from, DVector2 to, double alpha);

		[DllImport("prism_math")]
		internal static extern DVector2 dvec2_new(double x, double y);

		[DllImport("prism_math")]
		internal static extern DVector2 dvec2_scalar(double x);

		[DllImport("prism_math")]
		internal static extern DVector2 dvec2_default();

		[DllImport("prism_math")]
		internal static extern DVector2 dvec2_from_vec3(DVector3 i);

		[DllImport("prism_math")]
		internal static extern DVector2 dvec2_from_vec4(DVector4 i);

		// Skeleton Bindings (these are placeholders, they may need to be manually fixed)
		/*
		public DVector2 Add(DVector2 rhs) => Vector2F64.dvec2_add(this, rhs);
		public DVector2 Sub(DVector2 rhs) => Vector2F64.dvec2_sub(this, rhs);
		public DVector2 Mul(DVector2 rhs) => Vector2F64.dvec2_mul(this, rhs);
		public DVector2 Div(DVector2 rhs) => Vector2F64.dvec2_div(this, rhs);
		public DVector2 Normalize() => Vector2F64.dvec2_normalize(this);
		public DVector2 Abs() => Vector2F64.dvec2_abs(this);
		public DVector2 Floor() => Vector2F64.dvec2_floor(this);
		public DVector2 Ceil() => Vector2F64.dvec2_ceil(this);
		public DVector2 Round() => Vector2F64.dvec2_round(this);
		public DVector2 Saturate() => Vector2F64.dvec2_saturate(this);
		public DVector2 Sqrt() => Vector2F64.dvec2_sqrt(this);
		public DVector2 Sin() => Vector2F64.dvec2_sin(this);
		public DVector2 Cos() => Vector2F64.dvec2_cos(this);
		public DVector2 Tan() => Vector2F64.dvec2_tan(this);
		public DVector2 Sign() => Vector2F64.dvec2_sign(this);
		public DVector2 Fract() => Vector2F64.dvec2_fract(this);
		public double Sum() => Vector2F64.dvec2_sum(this);
		public double Magnitude() => Vector2F64.dvec2_magnitude(this);
		public double Magnitude_sqr() => Vector2F64.dvec2_magnitude_sqr(this);
		public DVector2 Min(DVector2 rhs) => Vector2F64.dvec2_min(this, rhs);
		public DVector2 Max(DVector2 rhs) => Vector2F64.dvec2_max(this, rhs);
		public DVector2 Pow(DVector2 rhs) => Vector2F64.dvec2_pow(this, rhs);
		public double Dot(DVector2 rhs) => Vector2F64.dvec2_dot(this, rhs);
		public DVector2 Lerp(DVector2 to, double alpha) => Vector2F64.dvec2_lerp(this, to, alpha);
		public DVector2 New(double x, double y) => Vector2F64.dvec2_new(x, y);
		public DVector2 Scalar(double x) => Vector2F64.dvec2_scalar(x);
		public DVector2 Default() => Vector2F64.dvec2_default();
		public DVector2 From_vec3(DVector3 i) => Vector2F64.dvec2_from_vec3(i);
		public DVector2 From_vec4(DVector4 i) => Vector2F64.dvec2_from_vec4(i);
		*/
	}
	internal class Vector3F64 {
		[DllImport("prism_math")]
		internal static extern DVector3 dvec3_add(DVector3 lhs, DVector3 rhs);

		[DllImport("prism_math")]
		internal static extern DVector3 dvec3_sub(DVector3 lhs, DVector3 rhs);

		[DllImport("prism_math")]
		internal static extern DVector3 dvec3_mul(DVector3 lhs, DVector3 rhs);

		[DllImport("prism_math")]
		internal static extern DVector3 dvec3_div(DVector3 lhs, DVector3 rhs);

		[DllImport("prism_math")]
		internal static extern DVector3 dvec3_normalize(DVector3 i);

		[DllImport("prism_math")]
		internal static extern DVector3 dvec3_abs(DVector3 i);

		[DllImport("prism_math")]
		internal static extern DVector3 dvec3_floor(DVector3 i);

		[DllImport("prism_math")]
		internal static extern DVector3 dvec3_ceil(DVector3 i);

		[DllImport("prism_math")]
		internal static extern DVector3 dvec3_round(DVector3 i);

		[DllImport("prism_math")]
		internal static extern DVector3 dvec3_saturate(DVector3 i);

		[DllImport("prism_math")]
		internal static extern DVector3 dvec3_sqrt(DVector3 i);

		[DllImport("prism_math")]
		internal static extern DVector3 dvec3_sin(DVector3 i);

		[DllImport("prism_math")]
		internal static extern DVector3 dvec3_cos(DVector3 i);

		[DllImport("prism_math")]
		internal static extern DVector3 dvec3_tan(DVector3 i);

		[DllImport("prism_math")]
		internal static extern DVector3 dvec3_sign(DVector3 i);

		[DllImport("prism_math")]
		internal static extern DVector3 dvec3_fract(DVector3 i);

		[DllImport("prism_math")]
		internal static extern double dvec3_sum(DVector3 i);

		[DllImport("prism_math")]
		internal static extern double dvec3_magnitude(DVector3 i);

		[DllImport("prism_math")]
		internal static extern double dvec3_magnitude_sqr(DVector3 i);

		[DllImport("prism_math")]
		internal static extern DVector3 dvec3_min(DVector3 lhs, DVector3 rhs);

		[DllImport("prism_math")]
		internal static extern DVector3 dvec3_max(DVector3 lhs, DVector3 rhs);

		[DllImport("prism_math")]
		internal static extern DVector3 dvec3_pow(DVector3 lhs, DVector3 rhs);

		[DllImport("prism_math")]
		internal static extern DVector3 dvec3_cross(DVector3 lhs, DVector3 rhs);

		[DllImport("prism_math")]
		internal static extern DVector3 dvec3_reflect(DVector3 lhs, DVector3 rhs);

		[DllImport("prism_math")]
		internal static extern double dvec3_dot(DVector3 lhs, DVector3 rhs);

		[DllImport("prism_math")]
		internal static extern DVector3 dvec3_lerp(DVector3 from, DVector3 to, double alpha);

		[DllImport("prism_math")]
		internal static extern DVector3 dvec3_new(double x, double y, double z);

		[DllImport("prism_math")]
		internal static extern DVector3 dvec3_scalar(double x);

		[DllImport("prism_math")]
		internal static extern DVector3 dvec3_default();

		[DllImport("prism_math")]
		internal static extern DVector3 dvec3_from_vec2(DVector2 i);

		[DllImport("prism_math")]
		internal static extern DVector3 dvec3_from_vec4(DVector4 i);

		// Skeleton Bindings (these are placeholders, they may need to be manually fixed)
		/*
		public DVector3 Add(DVector3 rhs) => Vector3F64.dvec3_add(this, rhs);
		public DVector3 Sub(DVector3 rhs) => Vector3F64.dvec3_sub(this, rhs);
		public DVector3 Mul(DVector3 rhs) => Vector3F64.dvec3_mul(this, rhs);
		public DVector3 Div(DVector3 rhs) => Vector3F64.dvec3_div(this, rhs);
		public DVector3 Normalize() => Vector3F64.dvec3_normalize(this);
		public DVector3 Abs() => Vector3F64.dvec3_abs(this);
		public DVector3 Floor() => Vector3F64.dvec3_floor(this);
		public DVector3 Ceil() => Vector3F64.dvec3_ceil(this);
		public DVector3 Round() => Vector3F64.dvec3_round(this);
		public DVector3 Saturate() => Vector3F64.dvec3_saturate(this);
		public DVector3 Sqrt() => Vector3F64.dvec3_sqrt(this);
		public DVector3 Sin() => Vector3F64.dvec3_sin(this);
		public DVector3 Cos() => Vector3F64.dvec3_cos(this);
		public DVector3 Tan() => Vector3F64.dvec3_tan(this);
		public DVector3 Sign() => Vector3F64.dvec3_sign(this);
		public DVector3 Fract() => Vector3F64.dvec3_fract(this);
		public double Sum() => Vector3F64.dvec3_sum(this);
		public double Magnitude() => Vector3F64.dvec3_magnitude(this);
		public double Magnitude_sqr() => Vector3F64.dvec3_magnitude_sqr(this);
		public DVector3 Min(DVector3 rhs) => Vector3F64.dvec3_min(this, rhs);
		public DVector3 Max(DVector3 rhs) => Vector3F64.dvec3_max(this, rhs);
		public DVector3 Pow(DVector3 rhs) => Vector3F64.dvec3_pow(this, rhs);
		public DVector3 Cross(DVector3 rhs) => Vector3F64.dvec3_cross(this, rhs);
		public DVector3 Reflect(DVector3 rhs) => Vector3F64.dvec3_reflect(this, rhs);
		public double Dot(DVector3 rhs) => Vector3F64.dvec3_dot(this, rhs);
		public DVector3 Lerp(DVector3 to, double alpha) => Vector3F64.dvec3_lerp(this, to, alpha);
		public DVector3 New(double x, double y, double z) => Vector3F64.dvec3_new(x, y, z);
		public DVector3 Scalar(double x) => Vector3F64.dvec3_scalar(x);
		public DVector3 Default() => Vector3F64.dvec3_default();
		public DVector3 From_vec2(DVector2 i) => Vector3F64.dvec3_from_vec2(i);
		public DVector3 From_vec4(DVector4 i) => Vector3F64.dvec3_from_vec4(i);
		*/
	}
	internal class Vector4F64 {
		[DllImport("prism_math")]
		internal static extern DVector4 dvec4_add(DVector4 lhs, DVector4 rhs);

		[DllImport("prism_math")]
		internal static extern DVector4 dvec4_sub(DVector4 lhs, DVector4 rhs);

		[DllImport("prism_math")]
		internal static extern DVector4 dvec4_mul(DVector4 lhs, DVector4 rhs);

		[DllImport("prism_math")]
		internal static extern DVector4 dvec4_div(DVector4 lhs, DVector4 rhs);

		[DllImport("prism_math")]
		internal static extern DVector4 dvec4_mul_mat4(DVector4 lhs, DMatrix4x4 rhs);

		[DllImport("prism_math")]
		internal static extern DVector4 dvec4_normalize(DVector4 i);

		[DllImport("prism_math")]
		internal static extern DVector4 dvec4_abs(DVector4 i);

		[DllImport("prism_math")]
		internal static extern DVector4 dvec4_floor(DVector4 i);

		[DllImport("prism_math")]
		internal static extern DVector4 dvec4_ceil(DVector4 i);

		[DllImport("prism_math")]
		internal static extern DVector4 dvec4_round(DVector4 i);

		[DllImport("prism_math")]
		internal static extern DVector4 dvec4_saturate(DVector4 i);

		[DllImport("prism_math")]
		internal static extern DVector4 dvec4_sqrt(DVector4 i);

		[DllImport("prism_math")]
		internal static extern DVector4 dvec4_sin(DVector4 i);

		[DllImport("prism_math")]
		internal static extern DVector4 dvec4_cos(DVector4 i);

		[DllImport("prism_math")]
		internal static extern DVector4 dvec4_tan(DVector4 i);

		[DllImport("prism_math")]
		internal static extern DVector4 dvec4_sign(DVector4 i);

		[DllImport("prism_math")]
		internal static extern DVector4 dvec4_fract(DVector4 i);

		[DllImport("prism_math")]
		internal static extern double dvec4_sum(DVector4 i);

		[DllImport("prism_math")]
		internal static extern double dvec4_magnitude(DVector4 i);

		[DllImport("prism_math")]
		internal static extern double dvec4_magnitude_sqr(DVector4 i);

		[DllImport("prism_math")]
		internal static extern DVector4 dvec4_min(DVector4 lhs, DVector4 rhs);

		[DllImport("prism_math")]
		internal static extern DVector4 dvec4_max(DVector4 lhs, DVector4 rhs);

		[DllImport("prism_math")]
		internal static extern DVector4 dvec4_pow(DVector4 lhs, DVector4 rhs);

		[DllImport("prism_math")]
		internal static extern double dvec4_dot(DVector4 lhs, DVector4 rhs);

		[DllImport("prism_math")]
		internal static extern DVector4 dvec4_lerp(DVector4 from, DVector4 to, double alpha);

		[DllImport("prism_math")]
		internal static extern DVector4 dvec4_identity();

		[DllImport("prism_math")]
		internal static extern DVector4 dvec4_new(double x, double y, double z, double w);

		[DllImport("prism_math")]
		internal static extern DVector4 dvec4_scalar(double x);

		[DllImport("prism_math")]
		internal static extern DVector4 dvec4_default();

		[DllImport("prism_math")]
		internal static extern DVector4 dvec4_from_vec2(DVector2 i);

		[DllImport("prism_math")]
		internal static extern DVector4 dvec4_from_vec3(DVector3 i);

		// Skeleton Bindings (these are placeholders, they may need to be manually fixed)
		/*
		public DVector4 Add(DVector4 rhs) => Vector4F64.dvec4_add(this, rhs);
		public DVector4 Sub(DVector4 rhs) => Vector4F64.dvec4_sub(this, rhs);
		public DVector4 Mul(DVector4 rhs) => Vector4F64.dvec4_mul(this, rhs);
		public DVector4 Div(DVector4 rhs) => Vector4F64.dvec4_div(this, rhs);
		public DVector4 Mul_mat4(DMatrix4x4 rhs) => Vector4F64.dvec4_mul_mat4(this, rhs);
		public DVector4 Normalize() => Vector4F64.dvec4_normalize(this);
		public DVector4 Abs() => Vector4F64.dvec4_abs(this);
		public DVector4 Floor() => Vector4F64.dvec4_floor(this);
		public DVector4 Ceil() => Vector4F64.dvec4_ceil(this);
		public DVector4 Round() => Vector4F64.dvec4_round(this);
		public DVector4 Saturate() => Vector4F64.dvec4_saturate(this);
		public DVector4 Sqrt() => Vector4F64.dvec4_sqrt(this);
		public DVector4 Sin() => Vector4F64.dvec4_sin(this);
		public DVector4 Cos() => Vector4F64.dvec4_cos(this);
		public DVector4 Tan() => Vector4F64.dvec4_tan(this);
		public DVector4 Sign() => Vector4F64.dvec4_sign(this);
		public DVector4 Fract() => Vector4F64.dvec4_fract(this);
		public double Sum() => Vector4F64.dvec4_sum(this);
		public double Magnitude() => Vector4F64.dvec4_magnitude(this);
		public double Magnitude_sqr() => Vector4F64.dvec4_magnitude_sqr(this);
		public DVector4 Min(DVector4 rhs) => Vector4F64.dvec4_min(this, rhs);
		public DVector4 Max(DVector4 rhs) => Vector4F64.dvec4_max(this, rhs);
		public DVector4 Pow(DVector4 rhs) => Vector4F64.dvec4_pow(this, rhs);
		public double Dot(DVector4 rhs) => Vector4F64.dvec4_dot(this, rhs);
		public DVector4 Lerp(DVector4 to, double alpha) => Vector4F64.dvec4_lerp(this, to, alpha);
		public DVector4 Identity() => Vector4F64.dvec4_identity();
		public DVector4 New(double x, double y, double z, double w) => Vector4F64.dvec4_new(x, y, z, w);
		public DVector4 Scalar(double x) => Vector4F64.dvec4_scalar(x);
		public DVector4 Default() => Vector4F64.dvec4_default();
		public DVector4 From_vec2(DVector2 i) => Vector4F64.dvec4_from_vec2(i);
		public DVector4 From_vec3(DVector3 i) => Vector4F64.dvec4_from_vec3(i);
		*/
	}
	internal class Matrix4x4F32 {
		[DllImport("prism_math")]
		internal static extern Matrix4x4 mat4_mul(Matrix4x4 lhs, Matrix4x4 rhs);

		[DllImport("prism_math")]
		internal static extern Vector4 mat4_mul_vec4(Matrix4x4 lhs, Vector4 rhs);

		[DllImport("prism_math")]
		internal static extern Matrix4x4 mat4_inverse(Matrix4x4 i);

		[DllImport("prism_math")]
		internal static extern Matrix4x4 mat4_transpose(Matrix4x4 i);

		[DllImport("prism_math")]
		internal static extern Matrix4x4 mat4_translation(Vector3 vector);

		[DllImport("prism_math")]
		internal static extern Matrix4x4 mat4_rotation(Vector3 euler);

		[DllImport("prism_math")]
		internal static extern Matrix4x4 mat4_rotate_x(float degrees);

		[DllImport("prism_math")]
		internal static extern Matrix4x4 mat4_rotate_y(float degrees);

		[DllImport("prism_math")]
		internal static extern Matrix4x4 mat4_rotate_z(float degrees);

		[DllImport("prism_math")]
		internal static extern Matrix4x4 mat4_perspective(float fov_y, float aspect, float z_near, float z_far);

		[DllImport("prism_math")]
		internal static extern Matrix4x4 mat4_default();

		[DllImport("prism_math")]
		internal static extern Matrix4x4 mat4_identity();

		// Skeleton Bindings (these are placeholders, they may need to be manually fixed)
		/*
		public Matrix4x4 Mul(Matrix4x4 rhs) => Matrix4x4F32.mat4_mul(this, rhs);
		public Vector4 Mul_vec4(Vector4 rhs) => Matrix4x4F32.mat4_mul_vec4(this, rhs);
		public Matrix4x4 Inverse() => Matrix4x4F32.mat4_inverse(this);
		public Matrix4x4 Transpose() => Matrix4x4F32.mat4_transpose(this);
		public Matrix4x4 Translation(Vector3 vector) => Matrix4x4F32.mat4_translation(vector);
		public Matrix4x4 Rotation(Vector3 euler) => Matrix4x4F32.mat4_rotation(euler);
		public Matrix4x4 Rotate_x(float degrees) => Matrix4x4F32.mat4_rotate_x(degrees);
		public Matrix4x4 Rotate_y(float degrees) => Matrix4x4F32.mat4_rotate_y(degrees);
		public Matrix4x4 Rotate_z(float degrees) => Matrix4x4F32.mat4_rotate_z(degrees);
		public Matrix4x4 Perspective(float fov_y, float aspect, float z_near, float z_far) => Matrix4x4F32.mat4_perspective(fov_y, aspect, z_near, z_far);
		public Matrix4x4 Default() => Matrix4x4F32.mat4_default();
		public Matrix4x4 Identity() => Matrix4x4F32.mat4_identity();
		*/
	}
	internal class Matrix4x4F64 {
		[DllImport("prism_math")]
		internal static extern DMatrix4x4 dmat4_mul(DMatrix4x4 lhs, DMatrix4x4 rhs);

		[DllImport("prism_math")]
		internal static extern DVector4 dmat4_mul_vec4(DMatrix4x4 lhs, DVector4 rhs);

		[DllImport("prism_math")]
		internal static extern DMatrix4x4 dmat4_inverse(DMatrix4x4 i);

		[DllImport("prism_math")]
		internal static extern DMatrix4x4 dmat4_transpose(DMatrix4x4 i);

		[DllImport("prism_math")]
		internal static extern DMatrix4x4 dmat4_translation(DVector3 vector);

		[DllImport("prism_math")]
		internal static extern DMatrix4x4 dmat4_rotation(DVector3 euler);

		[DllImport("prism_math")]
		internal static extern DMatrix4x4 dmat4_rotate_x(double degrees);

		[DllImport("prism_math")]
		internal static extern DMatrix4x4 dmat4_rotate_y(double degrees);

		[DllImport("prism_math")]
		internal static extern DMatrix4x4 dmat4_rotate_z(double degrees);

		[DllImport("prism_math")]
		internal static extern DMatrix4x4 dmat4_perspective(double fov_y, double aspect, double z_near, double z_far);

		[DllImport("prism_math")]
		internal static extern DMatrix4x4 dmat4_default();

		[DllImport("prism_math")]
		internal static extern DMatrix4x4 dmat4_identity();

		// Skeleton Bindings (these are placeholders, they may need to be manually fixed)
		/*
		public DMatrix4x4 Mul(DMatrix4x4 rhs) => Matrix4x4F64.dmat4_mul(this, rhs);
		public DVector4 Mul_vec4(DVector4 rhs) => Matrix4x4F64.dmat4_mul_vec4(this, rhs);
		public DMatrix4x4 Inverse() => Matrix4x4F64.dmat4_inverse(this);
		public DMatrix4x4 Transpose() => Matrix4x4F64.dmat4_transpose(this);
		public DMatrix4x4 Translation(DVector3 vector) => Matrix4x4F64.dmat4_translation(vector);
		public DMatrix4x4 Rotation(DVector3 euler) => Matrix4x4F64.dmat4_rotation(euler);
		public DMatrix4x4 Rotate_x(double degrees) => Matrix4x4F64.dmat4_rotate_x(degrees);
		public DMatrix4x4 Rotate_y(double degrees) => Matrix4x4F64.dmat4_rotate_y(degrees);
		public DMatrix4x4 Rotate_z(double degrees) => Matrix4x4F64.dmat4_rotate_z(degrees);
		public DMatrix4x4 Perspective(double fov_y, double aspect, double z_near, double z_far) => Matrix4x4F64.dmat4_perspective(fov_y, aspect, z_near, z_far);
		public DMatrix4x4 Default() => Matrix4x4F64.dmat4_default();
		public DMatrix4x4 Identity() => Matrix4x4F64.dmat4_identity();
		*/
	}
}
