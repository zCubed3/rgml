using System.Runtime.InteropServices;

namespace PrismMath {
    public struct Vector3 {
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = 3)]
        private float[] backing;

        public Vector3()
        {
            backing = new float[3] { 0F, 0F, 0F };
        }

        public Vector3(float scalar)
        {
            backing = new float[3] { scalar, scalar, scalar };
        }

        public Vector3(float x, float y, float z) 
        {
            backing = new float[3] { x, y, z };
        }

        public float this[int i]
        {
            get 
            {
                if (backing == null)
                    throw new System.NullReferenceException("Backing was null! Something has seriously gone wrong!");

                if (i > 2 || i < 0)
                    throw new System.IndexOutOfRangeException("Vector indexer was out of range!");

                return backing[i]; 
            }
            set 
            {
                if (backing == null)
                    throw new System.NullReferenceException("Backing was null! Something has seriously gone wrong!");

                if (i > 2 || i < 0)
                    throw new System.IndexOutOfRangeException("Vector indexer was out of range!");

                backing[i] = value; 
            }
        }

        public float X
        {
            get => this[0];
            set => this[0] = value;
        }

        public float Y
        {
            get => this[1];
            set => this[1] = value;
        }

        public float Z
        {
            get => this[2];
            set => this[2] = value;
        }

        public static Vector3 operator +(Vector3 lhs, Vector3 rhs) => Vector3Internals.vec3_add(lhs, rhs);
        public static Vector3 operator -(Vector3 lhs, Vector3 rhs) => Vector3Internals.vec3_sub(lhs, rhs);
        public static Vector3 operator *(Vector3 lhs, Vector3 rhs) => Vector3Internals.vec3_mul(lhs, rhs);
        public static Vector3 operator /(Vector3 lhs, Vector3 rhs) => Vector3Internals.vec3_div(lhs, rhs);

        public Vector3 Cross(Vector3 rhs) => Vector3Internals.vec3_cross(this, rhs);
        public Vector3 Min(Vector3 rhs) => Vector3Internals.vec3_min(this, rhs);
        public Vector3 Max(Vector3 rhs) => Vector3Internals.vec3_max(this, rhs);
        public float Dot(Vector3 rhs) => Vector3Internals.vec3_dot(this, rhs);
        public Vector3 Abs() => Vector3Internals.vec3_abs(this);
        public Vector3 Sin() => Vector3Internals.vec3_sin(this);
        public Vector3 Cos() => Vector3Internals.vec3_cos(this);
        public Vector3 Tan() => Vector3Internals.vec3_tan(this);
        public float Sum() => Vector3Internals.vec3_sum(this);
        public float Magnitude() => Vector3Internals.vec3_magnitude(this);
        public float MagnitudeSqr() => Vector3Internals.vec3_magnitude_sqr(this);
        public Vector3 Saturate() => Vector3Internals.vec3_saturate(this);
        public Vector3 Sign() => Vector3Internals.vec3_sign(this);
        public Vector3 Lerp(Vector3 to, float alpha) => Vector3Internals.vec3_lerp(this, to, alpha);
        public Vector3 Floor() => Vector3Internals.vec3_floor(this);
        public Vector3 Ceil() => Vector3Internals.vec3_ceil(this);
        public Vector3 Round() => Vector3Internals.vec3_round(this);
        public Vector3 Sqrt() => Vector3Internals.vec3_sqrt(this);
        public Vector3 Pow(Vector3 exp) => Vector3Internals.vec3_pow(this, exp);
        public Vector3 Step(Vector3 edge) => Vector3Internals.vec3_step(this, edge);

        public override string ToString()
        {
            return $"{X}, {Y}, {Z}";
        }
    }

    internal class Vector3Internals {
        [DllImport("prism_math")]
        internal static extern Vector3 vec3_new(float x, float y, float z);

        [DllImport("prism_math")]
        internal static extern Vector3 vec3_zero();

        [DllImport("prism_math")]
        internal static extern Vector3 vec3_add(Vector3 lhs, Vector3 rhs);

        [DllImport("prism_math")]
        internal static extern Vector3 vec3_sub(Vector3 lhs, Vector3 rhs);

        [DllImport("prism_math")]
        internal static extern Vector3 vec3_mul(Vector3 lhs, Vector3 rhs);

        [DllImport("prism_math")]
        internal static extern Vector3 vec3_div(Vector3 lhs, Vector3 rhs);

        [DllImport("prism_math")]
        internal static extern Vector3 vec3_cross(Vector3 lhs, Vector3 rhs);

        [DllImport("prism_math")]
        internal static extern Vector3 vec3_min(Vector3 lhs, Vector3 rhs);

        [DllImport("prism_math")]
        internal static extern Vector3 vec3_max(Vector3 lhs, Vector3 rhs);

        [DllImport("prism_math")]
        internal static extern float vec3_dot(Vector3 lhs, Vector3 rhs);

        [DllImport("prism_math")]
        internal static extern Vector3 vec3_normalize(Vector3 v);

        [DllImport("prism_math")]
        internal static extern Vector3 vec3_abs(Vector3 v);

        [DllImport("prism_math")]
        internal static extern Vector3 vec3_sin(Vector3 v);

        [DllImport("prism_math")]
        internal static extern Vector3 vec3_cos(Vector3 v);

        [DllImport("prism_math")]
        internal static extern Vector3 vec3_tan(Vector3 v);

        [DllImport("prism_math")]
        internal static extern float vec3_sum(Vector3 v);

        [DllImport("prism_math")]
        internal static extern float vec3_magnitude(Vector3 v);

        [DllImport("prism_math")]
        internal static extern float vec3_magnitude_sqr(Vector3 v);

        [DllImport("prism_math")]
        internal static extern Vector3 vec3_saturate(Vector3 v);

        [DllImport("prism_math")]
        internal static extern Vector3 vec3_sign(Vector3 v);

        [DllImport("prism_math")]
        internal static extern Vector3 vec3_lerp(Vector3 from, Vector3 to, float alpha);

        [DllImport("prism_math")]
        internal static extern Vector3 vec3_step(Vector3 lhs, Vector3 rhs);

        [DllImport("prism_math")]
        internal static extern Vector3 vec3_pow(Vector3 lhs, Vector3 rhs);

        [DllImport("prism_math")]
        internal static extern Vector3 vec3_floor(Vector3 v);

        [DllImport("prism_math")]
        internal static extern Vector3 vec3_ceil(Vector3 v);

        [DllImport("prism_math")]
        internal static extern Vector3 vec3_round(Vector3 v);

        [DllImport("prism_math")]
        internal static extern Vector3 vec3_sqrt(Vector3 v);
    }
}