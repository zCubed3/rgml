using System.Runtime.InteropServices;

using RGML.Internal;

namespace RGML
{
    public partial struct Vector2
    {
        public Vector2()
        {
            backing = new float[2] { 0, 0 };
        }

        public Vector2(float x, float y)
        {
            backing = new float[2] { x, y };
        }

        public Vector2(float x)
        {
            backing = new float[2] { x, x };
        }

        public override string ToString()
        {
            return $"{backing[0]}, {backing[1]}";
        }

        public float this[int i]
        {
            get
            {
                if (i < 0 || i > 1)
                    throw new ArgumentOutOfRangeException("Index was out of range!");

                return backing[i];
            }
            set
            {
                if (i < 0 || i > 1)
                    throw new ArgumentOutOfRangeException("Index was out of range!");

                backing[i] = value;
            }
        }

        public static Vector2 operator +(Vector2 lhs, Vector2 rhs) => Vector2F32.vec2_add(lhs, rhs);
        public static Vector2 operator -(Vector2 lhs, Vector2 rhs) => Vector2F32.vec2_sub(lhs, rhs);
        public static Vector2 operator *(Vector2 lhs, Vector2 rhs) => Vector2F32.vec2_mul(lhs, rhs);
        public static Vector2 operator /(Vector2 lhs, Vector2 rhs) => Vector2F32.vec2_div(lhs, rhs);

        public static implicit operator Vector2(Vector3 i) => Vector2F32.vec2_from_vec3(i);
        public static implicit operator Vector2(Vector4 i) => Vector2F32.vec2_from_vec4(i);

        public Vector2 Normalized => Vector2F32.vec2_normalize(this);
        public Vector2 Abs => Vector2F32.vec2_abs(this);
        public Vector2 Floor => Vector2F32.vec2_floor(this);
        public Vector2 Ceil => Vector2F32.vec2_ceil(this);
        public Vector2 Round => Vector2F32.vec2_round(this);
        public Vector2 Saturate => Vector2F32.vec2_saturate(this);
        public Vector2 Sqrt => Vector2F32.vec2_sqrt(this);
        public Vector2 Sin => Vector2F32.vec2_sin(this);
        public Vector2 Cos => Vector2F32.vec2_cos(this);
        public Vector2 Tan => Vector2F32.vec2_tan(this);
        public Vector2 Sign => Vector2F32.vec2_sign(this);
        public float Sum => Vector2F32.vec2_sum(this);
        public float Magnitude => Vector2F32.vec2_magnitude(this);
        public float MagnitudeSqr => Vector2F32.vec2_magnitude_sqr(this);
        public Vector2 Fract => Vector2F32.vec2_fract(this);

        public float[] Backing => backing;

        public Vector2 Min(Vector2 rhs) => Vector2F32.vec2_min(this, rhs);
        public Vector2 Max(Vector2 rhs) => Vector2F32.vec2_max(this, rhs);
        public Vector2 Pow(Vector2 rhs) => Vector2F32.vec2_pow(this, rhs);
        public float Dot(Vector2 rhs) => Vector2F32.vec2_dot(this, rhs);
        public Vector2 Lerp(Vector2 to, float alpha) => Vector2F32.vec2_lerp(this, to, alpha);
    }

    public partial struct DVector2
    {
        public DVector2()
        {
            backing = new double[2] { 0, 0 };
        }

        public DVector2(double x, double y)
        {
            backing = new double[2] { x, y };
        }

        public DVector2(double x)
        {
            backing = new double[2] { x, x };
        }

        public override string ToString()
        {
            return $"{backing[0]}, {backing[1]}";
        }

        public double this[int i]
        {
            get
            {
                if (i < 0 || i > 1)
                    throw new ArgumentOutOfRangeException("Index was out of range!");

                return backing[i];
            }
            set
            {
                if (i < 0 || i > 1)
                    throw new ArgumentOutOfRangeException("Index was out of range!");

                backing[i] = value;
            }
        }

        public static DVector2 operator +(DVector2 lhs, DVector2 rhs) => Vector2F64.dvec2_add(lhs, rhs);
        public static DVector2 operator -(DVector2 lhs, DVector2 rhs) => Vector2F64.dvec2_sub(lhs, rhs);
        public static DVector2 operator *(DVector2 lhs, DVector2 rhs) => Vector2F64.dvec2_mul(lhs, rhs);
        public static DVector2 operator /(DVector2 lhs, DVector2 rhs) => Vector2F64.dvec2_div(lhs, rhs);

        public static implicit operator DVector2(DVector3 i) => Vector2F64.dvec2_from_vec3(i);
        public static implicit operator DVector2(DVector4 i) => Vector2F64.dvec2_from_vec4(i);

        public DVector2 Normalized => Vector2F64.dvec2_normalize(this);
        public DVector2 Abs => Vector2F64.dvec2_abs(this);
        public DVector2 Floor => Vector2F64.dvec2_floor(this);
        public DVector2 Ceil => Vector2F64.dvec2_ceil(this);
        public DVector2 Round => Vector2F64.dvec2_round(this);
        public DVector2 Saturate => Vector2F64.dvec2_saturate(this);
        public DVector2 Sqrt => Vector2F64.dvec2_sqrt(this);
        public DVector2 Sin => Vector2F64.dvec2_sin(this);
        public DVector2 Cos => Vector2F64.dvec2_cos(this);
        public DVector2 Tan => Vector2F64.dvec2_tan(this);
        public DVector2 Sign => Vector2F64.dvec2_sign(this);
        public double Sum => Vector2F64.dvec2_sum(this);
        public double Magnitude => Vector2F64.dvec2_magnitude(this);
        public double MagnitudeSqr => Vector2F64.dvec2_magnitude_sqr(this);
        public DVector2 Fract => Vector2F64.dvec2_fract(this);

        public double[] Backing => backing;

        public DVector2 Min(DVector2 rhs) => Vector2F64.dvec2_min(this, rhs);
        public DVector2 Max(DVector2 rhs) => Vector2F64.dvec2_max(this, rhs);
        public DVector2 Pow(DVector2 rhs) => Vector2F64.dvec2_pow(this, rhs);
        public double Dot(DVector2 rhs) => Vector2F64.dvec2_dot(this, rhs);
        public DVector2 Lerp(DVector2 to, double alpha) => Vector2F64.dvec2_lerp(this, to, alpha);
    }
}