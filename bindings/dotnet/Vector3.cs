using System.Runtime.InteropServices;

namespace PrismMath {
    public struct Vector3 {
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = 3)]
        private float[] backing;

        public Vector3(float x, float y, float z) 
        {
            backing = new float[3] { x, y, z };
        }

        public float this[int i]
        {
            get 
            {
                if (i > 2 || i < 0)
                    throw new System.IndexOutOfRangeException("Vector indexer was out of range!");

                return backing[i]; 
            }
            set 
            { 
                if (i > 2 || i < 0)
                    throw new System.IndexOutOfRangeException("Vector indexer was out of range!");

                backing[i] = value; 
            }
        }

        public static Vector3 operator +(Vector3 lhs, Vector3 rhs) => Vector3Internals.vec3_add(lhs, rhs);
    }

    internal class Vector3Internals {
        [DllImport("prism_math")]
        internal static extern Vector3 vec3_new(float x, float y, float z);

        [DllImport("prism_math")]
        internal static extern Vector3 vec3_zero();
    
        [DllImport("prism_math")]
        internal static extern Vector3 vec3_add(Vector3 lhs, Vector3 rhs);
    }
}