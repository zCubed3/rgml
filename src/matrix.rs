#![allow(unused)]
#![allow(dead_code)]

use std::cmp::*;
use std::fmt::*;
use std::ops::*;

use crate::vector::Vector;
use super::real::RealNumber;

/// A matrix type. Commonly used for 3D graphics and rendering
///
/// At an underlying level, [Matrix] is an equivalent to using `[[T; W]; H]`
///
/// # Note:
///   * [Matrix<T, 2, 2>] and [Matrix<T, 3, 3>] types include determinant() functions
///   * [Matrix<T, 2, 2>], [Matrix<T, 3, 3>], and [Matrix<T, 4, 4>] types include inverse() functions
///
///   Implementations of Matrix inverse and determinant are from [GLM](<https://github.com/g-truc/glm/blob/master/glm/detail/func_matrix.inl>)
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Matrix<T: RealNumber, const WIDTH: usize, const HEIGHT: usize> {
    /// The underlying array of the matrix, the matrix dereferences into this array
    pub backing: [[T; WIDTH]; HEIGHT],
}

impl<T: RealNumber, const WIDTH: usize, const HEIGHT: usize> Matrix<T, WIDTH, HEIGHT> {
    pub fn from_array(array: [[T; WIDTH]; HEIGHT]) -> Self {
        Self { backing: array }
    }

    pub fn from_vectors(array: [Vector<T, WIDTH>; HEIGHT]) -> Self {
        let mut prod = Self::default();

        for r in 0..HEIGHT {
            prod[r] = *array[r];
        }

        return prod;
    }

    /// Provides an identity matrix for the given [Matrix] type
    pub fn identity() -> Self {
        let mut array = [[T::default(); WIDTH]; HEIGHT];

        // Only diagonals are populated, therefore X and Y are the same!
        // We use WIDTH as our frame of reference
        for c in 0..WIDTH {
            // Depth check
            if c >= HEIGHT {
                break;
            }

            array[c][c] = T::ONE;
        }

        return Self { backing: array };
    }

    /// Transposes the given matrix
    ///
    /// # Note:
    ///   When transposing oddly shaped matrices (WIDTH != HEIGHT), the resulting matrix will flip WIDTH and HEIGHT
    ///
    ///   Ex: [Matrix<f32, 4, 3>] will become [Matrix<f32, 3, 4>]
    pub fn transpose(&self) -> Matrix<T, HEIGHT, WIDTH> {
        let mut m = Matrix::<T, HEIGHT, WIDTH>::default();

        for y in 0..WIDTH {
            for x in 0..HEIGHT {
                m[x][y] = self[y][x];
            }
        }

        return m;
    }
}

//
// Deref
//
impl<T: RealNumber, const WIDTH: usize, const HEIGHT: usize> Deref for Matrix<T, WIDTH, HEIGHT> {
    type Target = [[T; WIDTH]; HEIGHT];

    fn deref(&self) -> &Self::Target {
        &self.backing
    }
}

impl<T: RealNumber, const WIDTH: usize, const HEIGHT: usize> DerefMut for Matrix<T, WIDTH, HEIGHT> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.backing
    }
}

//
// Formatters
//
impl<T: RealNumber, const WIDTH: usize, const HEIGHT: usize> Display for Matrix<T, WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..HEIGHT {
            if y != 0 {
                writeln!(f).expect("Failed to write!");
            }

            write!(f, "[").expect("Failed to write!");

            for x in 0..WIDTH {
                write!(f, "{}", self[y][x]).expect("Failed to write!");

                if x != WIDTH - 1 {
                    write!(f, ", ").expect("Failed to write!");
                }
            }

            write!(f, "]").expect("Failed to write!");
        }

        Ok(())
    }
}

//
// Default
//
impl<T: RealNumber, const WIDTH: usize, const HEIGHT: usize> Default for Matrix<T, WIDTH, HEIGHT> {
    fn default() -> Self {
        Self { backing: [[T::default(); WIDTH]; HEIGHT] }
    }
}

//
// Math
//
macro_rules! rl_op_assign {
    ($op:ident, $func:ident, $call:tt) => {
        impl<T: RealNumber, const WIDTH: usize, const HEIGHT: usize> $op<T> for Matrix<T, WIDTH, HEIGHT> {
            fn $func(&mut self, rhs: T) {
                for y in 0 .. HEIGHT {
                    for x in 0 .. WIDTH {
                        self[x][y] $call rhs;
                    }
                }
            }
        }
    };
}

macro_rules! rl_op {
    ($op:ident, $func:ident, $call:tt) => {
        impl<T: RealNumber, const WIDTH: usize, const HEIGHT: usize> $op<T> for Matrix<T, WIDTH, HEIGHT> {
            type Output = Self;

            fn $func(self, rhs: T) -> Self::Output {
                let mut prod = self;

                for y in 0 .. HEIGHT {
                    for x in 0 .. WIDTH {
                        prod[x][y] $call rhs;
                    }
                }

                prod
            }
        }
    };
}

rl_op_assign!(AddAssign, add_assign, +=);
rl_op_assign!(SubAssign, sub_assign, -=);
rl_op_assign!(MulAssign, mul_assign, *=);
rl_op_assign!(DivAssign, div_assign, /=);

rl_op!(Add, add, +=);
rl_op!(Sub, sub, -=);
rl_op!(Mul, mul, *=);
rl_op!(Div, div, /=);

// Matrix * Matrix
impl<T: RealNumber, const WIDTH: usize, const HEIGHT: usize> Mul<Self> for Matrix<T, WIDTH, HEIGHT> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut m = Self::default();

        for y in 0..HEIGHT {
            let r = Vector::from_array(self[y]);
            for x in 0..WIDTH {
                let mut c = Vector::<T, WIDTH>::default();

                for y2 in 0..HEIGHT {
                    c[y2] = rhs[y2][x];
                }

                m[y][x] = (r * c).sum();
            }
        }

        m
    }
}

//
// Common matrix types
//
impl<T: RealNumber> Matrix<T, 2, 2> {
    #[inline]
    /// Returns the determinant of this [Matrix<T, 2, 2>]
    pub fn determinant(&self) -> T {
        return self[0][0] * self[1][1] - self[0][1] * self[1][0];
    }

    /// Returns the inverse of this [Matrix<T, 2, 2>]
    pub fn inverse(&self) -> Self {
        let mut i = Self::default();
        let d = T::ONE / self.determinant();

        i[0][0] = self[1][1];
        i[0][1] = -self[0][1];
        i[1][0] = -self[1][0];
        i[1][1] = self[0][0];

        return i * d;
    }
}

impl<T: RealNumber> Matrix<T, 3, 3> {
    #[inline]
    /// Returns the determinant of this [Matrix<T, 3, 3>]
    pub fn determinant(&self) -> T {
        return
            self[0][0] * (self[1][1] * self[2][2] - self[2][1] * self[1][2]) -
            self[1][0] * (self[0][1] * self[2][2] - self[2][1] * self[0][2]) +
            self[2][0] * (self[0][1] * self[1][2] - self[1][1] * self[0][2]);
    }

    /// Returns the inverse of this [Matrix<T, 3, 3>]
    pub fn inverse(&self) -> Self {
        let mut i = Self::default();
        let d = T::ONE / self.determinant();

        i[0][0] = (self[1][1] * self[2][2] - self[2][1] * self[1][2]) * d;
        i[1][0] = -(self[1][0] * self[2][2] - self[2][0] * self[1][2]) * d;
        i[2][0] = (self[1][0] * self[2][1] - self[2][0] * self[1][1]) * d;
        i[0][1] = -(self[0][1] * self[2][2] - self[2][1] * self[0][2]) * d;
        i[1][1] = (self[0][0] * self[2][2] - self[2][0] * self[0][2]) * d;
        i[2][1] = -(self[0][0] * self[2][1] - self[2][0] * self[0][1]) * d;
        i[0][2] = (self[0][1] * self[1][2] - self[1][1] * self[0][2]) * d;
        i[1][2] = -(self[0][0] * self[1][2] - self[1][0] * self[0][2]) * d;
        i[2][2] = (self[0][0] * self[1][1] - self[1][0] * self[0][1]) * d;

        return i;
    }
}

impl<T: RealNumber> Matrix<T, 4, 4> {
    /// Returns the inverse of this [Matrix<T, 4, 4>]
    pub fn inverse(&self) -> Self {
        let coef00 = self[2][2] * self[3][3] - self[3][2] * self[2][3];
        let coef02 = self[1][2] * self[3][3] - self[3][2] * self[1][3];
        let coef03 = self[1][2] * self[2][3] - self[2][2] * self[1][3];

        let coef04 = self[2][1] * self[3][3] - self[3][1] * self[2][3];
        let coef06 = self[1][1] * self[3][3] - self[3][1] * self[1][3];
        let coef07 = self[1][1] * self[2][3] - self[2][1] * self[1][3];

        let coef08 = self[2][1] * self[3][2] - self[3][1] * self[2][2];
        let coef10 = self[1][1] * self[3][2] - self[3][1] * self[1][2];
        let coef11 = self[1][1] * self[2][2] - self[2][1] * self[1][2];

        let coef12 = self[2][0] * self[3][3] - self[3][0] * self[2][3];
        let coef14 = self[1][0] * self[3][3] - self[3][0] * self[1][3];
        let coef15 = self[1][0] * self[2][3] - self[2][0] * self[1][3];

        let coef16 = self[2][0] * self[3][2] - self[3][0] * self[2][2];
        let coef18 = self[1][0] * self[3][2] - self[3][0] * self[1][2];
        let coef19 = self[1][0] * self[2][2] - self[2][0] * self[1][2];

        let coef20 = self[2][0] * self[3][1] - self[3][0] * self[2][1];
        let coef22 = self[1][0] * self[3][1] - self[3][0] * self[1][1];
        let coef23 = self[1][0] * self[2][1] - self[2][0] * self[1][1];

        let fac0 = Vector::<T, 4>::new(coef00, coef00, coef02, coef03);
        let fac1 = Vector::<T, 4>::new(coef04, coef04, coef06, coef07);
        let fac2 = Vector::<T, 4>::new(coef08, coef08, coef10, coef11);
        let fac3 = Vector::<T, 4>::new(coef12, coef12, coef14, coef15);
        let fac4 = Vector::<T, 4>::new(coef16, coef16, coef18, coef19);
        let fac5 = Vector::<T, 4>::new(coef20, coef20, coef22, coef23);

        let vec0 = Vector::<T, 4>::new(self[1][0], self[0][0], self[0][0], self[0][0]);
        let vec1 = Vector::<T, 4>::new(self[1][1], self[0][1], self[0][1], self[0][1]);
        let vec2 = Vector::<T, 4>::new(self[1][2], self[0][2], self[0][2], self[0][2]);
        let vec3 = Vector::<T, 4>::new(self[1][3], self[0][3], self[0][3], self[0][3]);

        let inv0 = (vec1 * fac0 - vec2 * fac1 + vec3 * fac2);
        let inv1 = (vec0 * fac0 - vec2 * fac3 + vec3 * fac4);
        let inv2 = (vec0 * fac1 - vec1 * fac3 + vec3 * fac5);
        let inv3 = (vec0 * fac2 - vec1 * fac4 + vec2 * fac5);

        let one = T::ONE;

        let sign_a = Vector::<T, 4>::new(one, -one, one, -one);
        let sign_b = -sign_a;

        let mut i = Self::from_vectors([inv0 * sign_a, inv1 * sign_b, inv2 * sign_a, inv3 * sign_b]);
        let r0 = Vector::<T, 4>::new(i[0][0], i[1][0], i[2][0], i[3][0]);

        let dot0 = Vector::from_array(self[0]) * r0;
        let dot1 = (dot0[0] + dot0[1]) + (dot0[2] + dot0[3]);

        let d = one / dot1;

        return i * d;
    }

    /// Constructs a perspective projection matrix
    pub fn perspective(fov_y: T, aspect: T, z_near: T, z_far: T) -> Self {
        let one = T::ONE;
        let two = one + one;

        let vertical_fov = (fov_y / two).rl_to_radians();
        let half_fov = vertical_fov.rl_tan();

        let mut m = Self::default();

        m[0][0] = one / (aspect * half_fov);
        m[1][1] = one / (half_fov);
        m[2][2] = -(z_far + z_near) / (z_far - z_near);
        m[2][3] = -one;
        m[3][2] = -(two * z_far * z_near) / (z_far - z_near);

        m.transpose()
    }

    /// Constructs a translation matrix
    pub fn translation(translation: Vector<T, 3>) -> Self {
        let mut m = Self::identity();

        m[0][3] = translation[0];
        m[1][3] = translation[1];
        m[2][3] = translation[2];

        return m;
    }

    /// Constructs an X axis rotation matrix
    pub fn rotate_x(rotation: T) -> Self {
        let mut m = Self::identity();

        m[1] = [T::default(), rotation.rl_cos(), -rotation.rl_sin(), T::default()];
        m[2] = [T::default(), rotation.rl_sin(), rotation.rl_cos(), T::default()];

        return m;
    }

    /// Constructs a Y axis rotation matrix
    pub fn rotate_y(rotation: T) -> Self {
        let mut m = Self::identity();

        m[0] = [rotation.rl_cos(), T::default(), rotation.rl_sin(), T::default()];
        m[2] = [-rotation.rl_sin(), T::default(), rotation.rl_cos(), T::default()];

        return m;
    }

    /// Constructs a Z axis rotation matrix
    pub fn rotate_z(rotation: T) -> Self {
        let mut m = Self::identity();

        m[0] = [rotation.rl_cos(), -rotation.rl_sin(), T::default(), T::default()];
        m[1] = [rotation.rl_sin(), rotation.rl_cos(), T::default(), T::default()];

        return m;
    }

    /// Constructs a rotation matrix
    pub fn rotation(euler: Vector<T, 3>) -> Self {
        let euler_rad = euler * T::DEG2RAD;
        Self::rotate_z(euler_rad[2]) * Self::rotate_y(euler_rad[1]) * Self::rotate_x(euler_rad[0])
    }

    /// Constructs a look at view matrix
    pub fn look_at(direction: Vector<T, 3>) -> Self {
        let up = Vector::<T, 3>::new(T::default(), -T::ONE, T::default());

        let r_right = direction.cross(up).normalize();
        let r_up = direction.cross(r_right).normalize();

        let mut m = Self::identity();

        m[0] = [r_right[0], r_right[1], r_right[2], T::default()];
        m[1] = [r_up[0], r_up[1], r_up[2], T::default()];
        m[2] = [direction[0], direction[1], direction[2], T::default()];

        m
    }

    /// Projects a [Vector<T, 3>] by this matrix
    pub fn project_point(self, point: Vector<T, 3>) -> Vector<T, 3> {
        let mut reference = Vector::<T, 3>::default();

        reference[0] = point[0] * self[0][0] + point[1] * self[0][0] + point[2] * self[0][2] + self[0][3];
        reference[1] = point[0] * self[1][0] + point[1] * self[1][0] + point[2] * self[1][2] + self[1][3];
        reference[2] = point[0] * self[2][0] + point[1] * self[2][0] + point[2] * self[2][2] + self[2][3];

        let w: T = point[0] * self[3][0] + point[1] * self[3][1] + point[2] * self[3][2] + self[3][3];

        reference[0] /= w;
        reference[1] /= w;
        reference[2] /= w;

        return reference;
    }
}

/// Matrix * Vector
/// From: <https://github.com/g-truc/glm/blob/master/glm/detail/type_mat4x4.inl>
impl<T: RealNumber> Mul<Vector<T, 4>> for Matrix<T, 4, 4> {
    type Output = Vector<T, 4>;

    fn mul(self, rhs: Vector<T, 4>) -> Self::Output {
        Vector::<T, 4>::new(
            rhs[0] * self[0][0] + rhs[1] * self[0][1] + rhs[2] * self[0][2] + rhs[3] * self[0][3],
            rhs[0] * self[1][0] + rhs[1] * self[1][1] + rhs[2] * self[1][2] + rhs[3] * self[1][3],
            rhs[0] * self[2][0] + rhs[1] * self[2][1] + rhs[2] * self[2][2] + rhs[3] * self[2][3],
            rhs[0] * self[3][0] + rhs[1] * self[3][1] + rhs[2] * self[3][2] + rhs[3] * self[3][3],
        )
    }
}

/// Vector * Matrix
/// From: <https://github.com/g-truc/glm/blob/master/glm/detail/type_mat4x4.inl>
impl<T: RealNumber> Mul<Matrix<T, 4, 4>> for Vector<T, 4> {
    type Output = Self;

    fn mul(self, rhs: Matrix<T, 4, 4>) -> Self::Output {
        Vector::<T, 4>::new(
            self[0] * rhs[0][0] + self[1] * rhs[0][1] + self[2] * rhs[0][2] + self[3] * rhs[0][3],
            self[0] * rhs[1][0] + self[1] * rhs[1][1] + self[2] * rhs[1][2] + self[3] * rhs[1][3],
            self[0] * rhs[2][0] + self[1] * rhs[2][1] + self[2] * rhs[2][2] + self[3] * rhs[2][3],
            self[0] * rhs[3][0] + self[1] * rhs[3][1] + self[2] * rhs[3][2] + self[3] * rhs[3][3],
        )
    }
}

/// Contains commonly used [Matrix] aliases with additional implementations for ease of use
pub mod common {
    use crate::real::Real;
    use super::*;

    // Default matrix 2x2 types
    /// [Matrix<T, 2, 2>] backed by [Real]
    pub type Matrix2x2 = Matrix<Real, 2, 2>;
    /// [Matrix<T, 2, 2>] backed by [f32]
    pub type Matrix2x2F32 = Matrix<f32, 2, 2>;
    /// [Matrix<T, 2, 2>] backed by [f64]
    pub type Matrix2x2F64 = Matrix<f64, 2, 2>;

    // Default matrix 3x3 types
    /// [Matrix<T, 3, 3>] backed by [Real]
    pub type Matrix3x3 = Matrix<Real, 3, 3>;
    /// [Matrix<T, 3, 3>] backed by [f32]
    pub type Matrix3x3F32 = Matrix<f32, 3, 3>;
    /// [Matrix<T, 3, 3>] backed by [f64]
    pub type Matrix3x3F64 = Matrix<f64, 3, 3>;

    // Default matrix 4x4 types

    /// [Matrix<T, 4, 4>] backed by [Real]
    pub type Matrix4x4 = Matrix<Real, 4, 4>;
    /// [Matrix<T, 4, 4>] backed by [f32]
    pub type Matrix4x4F32 = Matrix<f32, 4, 4>;
    /// [Matrix<T, 4, 4>] backed by [f64]
    pub type Matrix4x4F64 = Matrix<f64, 4, 4>;
}