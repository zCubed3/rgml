using System.Runtime.InteropServices;

using PrismMath.Internal;

namespace PrismMath
{
    public partial struct Vector3
    {
        public Vector3()
        {
            backing = new float[] { 0, 0, 0 };
        }

        public Vector3(float x, float y, float z)
        {
            backing = new float[] { x, y, z };
        }

        public override string ToString()
        {
            return $"{backing[0]}, {backing[1]}, {backing[2]}";
        }

        public float this[int i]
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

        public static Vector3 operator +(Vector3 lhs, Vector3 rhs) => Vector3F32.vec3_add(lhs, rhs);
        public static Vector3 operator -(Vector3 lhs, Vector3 rhs) => Vector3F32.vec3_sub(lhs, rhs);
        public static Vector3 operator *(Vector3 lhs, Vector3 rhs) => Vector3F32.vec3_mul(lhs, rhs);
        public static Vector3 operator /(Vector3 lhs, Vector3 rhs) => Vector3F32.vec3_div(lhs, rhs);

        public static implicit operator Vector3(Vector2 i) => Vector3F32.vec3_from_vec2(i);
        public static implicit operator Vector3(Vector4 i) => Vector3F32.vec3_from_vec4(i);

        public Vector3 Normalized => Vector3F32.vec3_normalize(this);
        public Vector3 Abs => Vector3F32.vec3_abs(this);
        public Vector3 Floor => Vector3F32.vec3_floor(this);
        public Vector3 Ceil => Vector3F32.vec3_ceil(this);
        public Vector3 Round => Vector3F32.vec3_round(this);
        public Vector3 Saturate => Vector3F32.vec3_saturate(this);
        public Vector3 Sqrt => Vector3F32.vec3_sqrt(this);
        public Vector3 Sin => Vector3F32.vec3_sin(this);
        public Vector3 Cos => Vector3F32.vec3_cos(this);
        public Vector3 Tan => Vector3F32.vec3_tan(this);
        public Vector3 Sign => Vector3F32.vec3_sign(this);
        public float Sum => Vector3F32.vec3_sum(this);
        public float Magnitude => Vector3F32.vec3_magnitude(this);
        public float MagnitudeSqr => Vector3F32.vec3_magnitude_sqr(this);

        public Vector3 Min(Vector3 rhs) => Vector3F32.vec3_min(this, rhs);
        public Vector3 Max(Vector3 rhs) => Vector3F32.vec3_max(this, rhs);
        public Vector3 Pow(Vector3 rhs) => Vector3F32.vec3_pow(this, rhs);
        public Vector3 Cross(Vector3 rhs) => Vector3F32.vec3_cross(this, rhs);
        public Vector3 Reflect(Vector3 rhs) => Vector3F32.vec3_reflect(this, rhs);
        public float Dot(Vector3 rhs) => Vector3F32.vec3_dot(this, rhs);
        public Vector3 Lerp(Vector3 to, float alpha) => Vector3F32.vec3_lerp(this, to, alpha);
    }

    public partial struct DVector3
    {
        public DVector3()
        {
            backing = new double[] { 0, 0, 0 };
        }

        public DVector3(double x, double y, double z)
        {
            backing = new double[] { x, y, z };
        }

        public override string ToString()
        {
            return $"{backing[0]}, {backing[1]}, {backing[2]}";
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

        public static DVector3 operator +(DVector3 lhs, DVector3 rhs) => Vector3F64.dvec3_add(lhs, rhs);
        public static DVector3 operator -(DVector3 lhs, DVector3 rhs) => Vector3F64.dvec3_sub(lhs, rhs);
        public static DVector3 operator *(DVector3 lhs, DVector3 rhs) => Vector3F64.dvec3_mul(lhs, rhs);
        public static DVector3 operator /(DVector3 lhs, DVector3 rhs) => Vector3F64.dvec3_div(lhs, rhs);

        public static implicit operator DVector3(DVector2 i) => Vector3F64.dvec3_from_vec2(i);
        public static implicit operator DVector3(DVector4 i) => Vector3F64.dvec3_from_vec4(i);

        public DVector3 Normalized => Vector3F64.dvec3_normalize(this);
        public DVector3 Abs => Vector3F64.dvec3_abs(this);
        public DVector3 Floor => Vector3F64.dvec3_floor(this);
        public DVector3 Ceil => Vector3F64.dvec3_ceil(this);
        public DVector3 Round => Vector3F64.dvec3_round(this);
        public DVector3 Saturate => Vector3F64.dvec3_saturate(this);
        public DVector3 Sqrt => Vector3F64.dvec3_sqrt(this);
        public DVector3 Sin => Vector3F64.dvec3_sin(this);
        public DVector3 Cos => Vector3F64.dvec3_cos(this);
        public DVector3 Tan => Vector3F64.dvec3_tan(this);
        public DVector3 Sign => Vector3F64.dvec3_sign(this);
        public double Sum => Vector3F64.dvec3_sum(this);
        public double Magnitude => Vector3F64.dvec3_magnitude(this);
        public double MagnitudeSqr => Vector3F64.dvec3_magnitude_sqr(this);

        public DVector3 Min(DVector3 rhs) => Vector3F64.dvec3_min(this, rhs);
        public DVector3 Max(DVector3 rhs) => Vector3F64.dvec3_max(this, rhs);
        public DVector3 Pow(DVector3 rhs) => Vector3F64.dvec3_pow(this, rhs);
        public DVector3 Cross(DVector3 rhs) => Vector3F64.dvec3_cross(this, rhs);
        public DVector3 Reflect(DVector3 rhs) => Vector3F64.dvec3_reflect(this, rhs);
        public double Dot(DVector3 rhs) => Vector3F64.dvec3_dot(this, rhs);
        public DVector3 Lerp(DVector3 to, double alpha) => Vector3F64.dvec3_lerp(this, to, alpha);
    }
}