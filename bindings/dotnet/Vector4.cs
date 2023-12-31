﻿using System.Runtime.InteropServices;

using RGML.Internal;

namespace RGML
{
    public partial struct Vector4
    {
        public Vector4()
        {
            backing = new float[4] { 0, 0, 0, 0 };
        }

        public Vector4(float x)
        {
            backing = new float[4] { x, x, x, x };
        }

        public Vector4(float x, float y, float z, float w)
        {
            backing = new float[4] { x, y, z, w };
        }

        public override string ToString()
        {
            return $"{backing[0]}, {backing[1]}, {backing[2]}, {backing[3]}";
        }

        public float this[int i]
        {
            get
            {
                if (i < 0 || i > 3)
                    throw new ArgumentOutOfRangeException("Index was out of range!");

                return backing[i];
            }
            set
            {
                if (i < 0 || i > 3)
                    throw new ArgumentOutOfRangeException("Index was out of range!");

                backing[i] = value;
            }
        }

        public static Vector4 operator +(Vector4 lhs, Vector4 rhs) => Vector4F32.vec4_add(lhs, rhs);
        public static Vector4 operator -(Vector4 lhs, Vector4 rhs) => Vector4F32.vec4_sub(lhs, rhs);
        public static Vector4 operator *(Vector4 lhs, Vector4 rhs) => Vector4F32.vec4_mul(lhs, rhs);
        public static Vector4 operator /(Vector4 lhs, Vector4 rhs) => Vector4F32.vec4_div(lhs, rhs);

        public static Vector4 operator *(Vector4 lhs, Matrix4x4 rhs) => Vector4F32.vec4_mul_mat4(lhs, rhs);

        public static implicit operator Vector4(Vector2 i) => Vector4F32.vec4_from_vec2(i);
        public static implicit operator Vector4(Vector3 i) => Vector4F32.vec4_from_vec3(i);

        public Vector4 Normalized => Vector4F32.vec4_normalize(this);
        public Vector4 Abs => Vector4F32.vec4_abs(this);
        public Vector4 Floor => Vector4F32.vec4_floor(this);
        public Vector4 Ceil => Vector4F32.vec4_ceil(this);
        public Vector4 Round => Vector4F32.vec4_round(this);
        public Vector4 Saturate => Vector4F32.vec4_saturate(this);
        public Vector4 Sqrt => Vector4F32.vec4_sqrt(this);
        public Vector4 Sin => Vector4F32.vec4_sin(this);
        public Vector4 Cos => Vector4F32.vec4_cos(this);
        public Vector4 Tan => Vector4F32.vec4_tan(this);
        public Vector4 Sign => Vector4F32.vec4_sign(this);
        public float Sum => Vector4F32.vec4_sum(this);
        public float Magnitude => Vector4F32.vec4_magnitude(this);
        public float MagnitudeSqr => Vector4F32.vec4_magnitude_sqr(this);
        public Vector4 Fract => Vector4F32.vec4_fract(this);

        public float[] Backing => backing;

        public static Vector4 Identity => Vector4F32.vec4_identity();

        public Vector4 Min(Vector4 rhs) => Vector4F32.vec4_min(this, rhs);
        public Vector4 Max(Vector4 rhs) => Vector4F32.vec4_max(this, rhs);
        public Vector4 Pow(Vector4 rhs) => Vector4F32.vec4_pow(this, rhs);
        public float Dot(Vector4 rhs) => Vector4F32.vec4_dot(this, rhs);
        public Vector4 Lerp(Vector4 to, float alpha) => Vector4F32.vec4_lerp(this, to, alpha);
    }

    public partial struct DVector4
    {
        public DVector4()
        {
            backing = new double[4] { 0, 0, 0, 0 };
        }

        public DVector4(double x)
        {
            backing = new double[4] { x, x, x, x };
        }

        public DVector4(double x, double y, double z, double w)
        {
            backing = new double[4] { x, y, z, w };
        }

        public override string ToString()
        {
            return $"{backing[0]}, {backing[1]}, {backing[2]}, {backing[3]}";
        }

        public double this[int i]
        {
            get
            {
                if (i < 0 || i > 2)
                    throw new ArgumentOutOfRangeException("Index was out of range!");

                return backing[i];
            }
            set
            {
                if (i < 0 || i > 2)
                    throw new ArgumentOutOfRangeException("Index was out of range!");

                backing[i] = value;
            }
        }

        public static DVector4 operator +(DVector4 lhs, DVector4 rhs) => Vector4F64.dvec4_add(lhs, rhs);
        public static DVector4 operator -(DVector4 lhs, DVector4 rhs) => Vector4F64.dvec4_sub(lhs, rhs);
        public static DVector4 operator *(DVector4 lhs, DVector4 rhs) => Vector4F64.dvec4_mul(lhs, rhs);
        public static DVector4 operator /(DVector4 lhs, DVector4 rhs) => Vector4F64.dvec4_div(lhs, rhs);

        public static DVector4 operator *(DVector4 lhs, DMatrix4x4 rhs) => Vector4F64.dvec4_mul_mat4(lhs, rhs);

        public static implicit operator DVector4(DVector2 i) => Vector4F64.dvec4_from_vec2(i);
        public static implicit operator DVector4(DVector3 i) => Vector4F64.dvec4_from_vec3(i);

        public DVector4 Normalized => Vector4F64.dvec4_normalize(this);
        public DVector4 Abs => Vector4F64.dvec4_abs(this);
        public DVector4 Floor => Vector4F64.dvec4_floor(this);
        public DVector4 Ceil => Vector4F64.dvec4_ceil(this);
        public DVector4 Round => Vector4F64.dvec4_round(this);
        public DVector4 Saturate => Vector4F64.dvec4_saturate(this);
        public DVector4 Sqrt => Vector4F64.dvec4_sqrt(this);
        public DVector4 Sin => Vector4F64.dvec4_sin(this);
        public DVector4 Cos => Vector4F64.dvec4_cos(this);
        public DVector4 Tan => Vector4F64.dvec4_tan(this);
        public DVector4 Sign => Vector4F64.dvec4_sign(this);
        public double Sum => Vector4F64.dvec4_sum(this);
        public double Magnitude => Vector4F64.dvec4_magnitude(this);
        public double MagnitudeSqr => Vector4F64.dvec4_magnitude_sqr(this);
        public DVector4 Fract => Vector4F64.dvec4_fract(this);

        public double[] Backing => backing;

        public static DVector4 Identity => Vector4F64.dvec4_identity();


        public DVector4 Min(DVector4 rhs) => Vector4F64.dvec4_min(this, rhs);
        public DVector4 Max(DVector4 rhs) => Vector4F64.dvec4_max(this, rhs);
        public DVector4 Pow(DVector4 rhs) => Vector4F64.dvec4_pow(this, rhs);
        public double Dot(DVector4 rhs) => Vector4F64.dvec4_dot(this, rhs);
        public DVector4 Lerp(DVector4 to, double alpha) => Vector4F64.dvec4_lerp(this, to, alpha);
    }
}