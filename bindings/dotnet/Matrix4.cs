using System.Runtime.InteropServices;

using PrismMath.Internal;

namespace PrismMath
{
    public partial struct Matrix4x4
    {
        public static Matrix4x4 Identity => Matrix4x4F32.mat4_identity();
        public static Matrix4x4 Default => Matrix4x4F32.mat4_default();

        public Matrix4x4()
        {
            var def = Default;

            this.r0 = def.r0;
            this.r1 = def.r1;
            this.r2 = def.r2;
            this.r3 = def.r3;
        }

        public Matrix4x4(Vector4 r0, Vector4 r1, Vector4 r2, Vector4 r3)
        {
            this.r0 = r0.Backing;
            this.r1 = r1.Backing;
            this.r2 = r2.Backing;
            this.r3 = r3.Backing;
        }

        public override string ToString()
        {
            return $"{r0[0]}, {r0[1]}, {r0[2]}, {r0[3]}\n{r1[0]}, {r1[1]}, {r1[2]}, {r1[3]}\n{r2[0]}, {r2[1]}, {r2[2]}, {r2[3]}\n{r3[0]}, {r3[1]}, {r3[2]}, {r3[3]}";
        }

        public static Matrix4x4 operator *(Matrix4x4 lhs, Matrix4x4 rhs) => Matrix4x4F32.mat4_mul(lhs, rhs);
        public static Vector4 operator *(Matrix4x4 lhs, Vector4 rhs) => Matrix4x4F32.mat4_mul_vec4(lhs, rhs);

        public Matrix4x4 Inverse() => Matrix4x4F32.mat4_inverse(this);
        public Matrix4x4 Transpose() => Matrix4x4F32.mat4_transpose(this);

        public static Matrix4x4 Translation(Vector3 vector) => Matrix4x4F32.mat4_translation(vector);
        public static Matrix4x4 Rotation(Vector3 euler) => Matrix4x4F32.mat4_rotation(euler);
        public static Matrix4x4 RotateX(float degrees) => Matrix4x4F32.mat4_rotate_x(degrees);
        public static Matrix4x4 RotateY(float degrees) => Matrix4x4F32.mat4_rotate_y(degrees);
        public static Matrix4x4 RotateZ(float degrees) => Matrix4x4F32.mat4_rotate_z(degrees);
        public static Matrix4x4 Perspective(float fov_y, float aspect, float z_near, float z_far) => Matrix4x4F32.mat4_perspective(fov_y, aspect, z_near, z_far);
    }

    public partial struct DMatrix4x4
    {
        public static DMatrix4x4 Identity => Matrix4x4F64.dmat4_identity();
        public static DMatrix4x4 Default => Matrix4x4F64.dmat4_default();

        public DMatrix4x4()
        {
            var def = Default;

            this.r0 = def.r0;
            this.r1 = def.r1;
            this.r2 = def.r2;
            this.r3 = def.r3;
        }

        public DMatrix4x4(DVector4 r0, DVector4 r1, DVector4 r2, DVector4 r3)
        {
            this.r0 = r0.Backing;
            this.r1 = r1.Backing;
            this.r2 = r2.Backing;
            this.r3 = r3.Backing;
        }

        public override string ToString()
        {
            return $"{r0[0]}, {r0[1]}, {r0[2]}, {r0[3]}\n{r1[0]}, {r1[1]}, {r1[2]}, {r1[3]}\n{r2[0]}, {r2[1]}, {r2[2]}, {r2[3]}\n{r3[0]}, {r3[1]}, {r3[2]}, {r3[3]}";
        }

        public static DMatrix4x4 operator *(DMatrix4x4 lhs, DMatrix4x4 rhs) => Matrix4x4F64.dmat4_mul(lhs, rhs);
        public static DVector4 operator *(DMatrix4x4 lhs, DVector4 rhs) => Matrix4x4F64.dmat4_mul_vec4(lhs, rhs);

        public DMatrix4x4 Inverse() => Matrix4x4F64.dmat4_inverse(this);
        public DMatrix4x4 Transpose() => Matrix4x4F64.dmat4_transpose(this);

        public static DMatrix4x4 Translation(DVector3 vector) => Matrix4x4F64.dmat4_translation(vector);
        public static DMatrix4x4 Rotation(DVector3 euler) => Matrix4x4F64.dmat4_rotation(euler);
        public static DMatrix4x4 RotateX(float degrees) => Matrix4x4F64.dmat4_rotate_x(degrees);
        public static DMatrix4x4 RotateY(float degrees) => Matrix4x4F64.dmat4_rotate_y(degrees);
        public static DMatrix4x4 RotateZ(float degrees) => Matrix4x4F64.dmat4_rotate_z(degrees);
        public static DMatrix4x4 Perspective(float fov_y, float aspect, float z_near, float z_far) => Matrix4x4F64.dmat4_perspective(fov_y, aspect, z_near, z_far);
    }
}