using System.Runtime.InteropServices;

namespace PrismMath {
    //
    // Single precision types
    //
    public partial struct Vector2
    {
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = 2)]
        private float[] backing;
    }

    public partial struct Vector3
    {
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = 3)]
        private float[] backing;
    }

    public partial struct Vector4
    {
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = 4)]
        private float[] backing;
    }

    public partial struct Matrix4x4
    {
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = 4)]
        private float[] r0;

        [MarshalAs(UnmanagedType.ByValArray, SizeConst = 4)]
        private float[] r1;

        [MarshalAs(UnmanagedType.ByValArray, SizeConst = 4)]
        private float[] r2;

        [MarshalAs(UnmanagedType.ByValArray, SizeConst = 4)]
        private float[] r3;
    }

    //
    // Double precision types
    // 
    public partial struct DVector2
    {
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = 2)]
        private double[] backing;
    }

    public partial struct DVector3
    {
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = 3)]
        private double[] backing;
    }

    public partial struct DVector4
    {
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = 4)]
        private double[] backing;
    }

    public partial struct DMatrix4x4
    {
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = 4)]
        private double[] r0;

        [MarshalAs(UnmanagedType.ByValArray, SizeConst = 4)]
        private double[] r1;

        [MarshalAs(UnmanagedType.ByValArray, SizeConst = 4)]
        private double[] r2;

        [MarshalAs(UnmanagedType.ByValArray, SizeConst = 4)]
        private double[] r3;
    }
}