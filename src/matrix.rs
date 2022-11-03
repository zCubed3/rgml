#![allow(unused)]
#![allow(dead_code)]

use std::cmp::*;
use std::fmt::*;
use std::ops::*;

use crate::vector::Vector;
use super::real::RealNumber;

/// [Matrix] and [Vector] are very closely related!
/// Because of this, [Matrix] provides behavior to work with [Vector] types!
///
/// The underlying implementation of a [Matrix] is similar to a [Vector] except a 2D array instead of a 1D array
///
/// # Note:
///    Uncommon matrix types do not include determinant() and inverse() functions
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Matrix<T: RealNumber, const WIDTH: usize, const HEIGHT: usize> {
    /// The underlying array of the matrix, the matrix dereferences into this array
    pub underlying: [[T; WIDTH]; HEIGHT],
}

impl<T: RealNumber, const WIDTH: usize, const HEIGHT: usize> Matrix<T, WIDTH, HEIGHT> {
    pub fn from_array(array: [[T; WIDTH]; HEIGHT]) -> Self {
        Self { underlying: array }
    }

    /// Provides an identity matrix (this works best with evenly shaped [Matrix] types!)
    pub fn identity() -> Self {
        let mut array = [[T::default(); WIDTH]; HEIGHT];

        // Only diagonals are populated, therefore X and Y are the same!
        // We use WIDTH as our frame of reference
        for c in 0..WIDTH {
            // Depth check
            if c >= HEIGHT {
                break;
            }

            array[c][c] = T::real_get_one();
        }

        return Self { underlying: array };
    }

    /// Transposes the given matrix
    ///
    /// # Note:
    /// When transposing oddly shaped matrices (WIDTH != HEIGHT), the resulting matrix will flip WIDTH and HEIGHT
    /// Ex: Matrix<f32, 4, 3> will become Matrix<f32, 3, 4>
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
        &self.underlying
    }
}

impl<T: RealNumber, const WIDTH: usize, const HEIGHT: usize> DerefMut for Matrix<T, WIDTH, HEIGHT> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.underlying
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
        Self { underlying: [[T::default(); WIDTH]; HEIGHT] }
    }
}

//
// Math
//
macro_rules! real_op_assign {
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

macro_rules! real_op {
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

real_op_assign!(AddAssign, add_assign, +=);
real_op_assign!(SubAssign, sub_assign, -=);
real_op_assign!(MulAssign, mul_assign, *=);
real_op_assign!(DivAssign, div_assign, /=);

real_op!(Add, add, +=);
real_op!(Sub, sub, -=);
real_op!(Mul, mul, *=);
real_op!(Div, div, /=);

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

/// Contains commonly used [Matrix] aliases with additional implementations for ease of use
///
/// Implementations of Matrix inverse are from <https://github.com/g-truc/glm/blob/master/glm/detail/func_matrix.inl>
pub mod common {
    use crate::real::{Real, RealNumber};
    use crate::vector::common::{Vector3, Vector4};
    use crate::vector::Vector;

    use super::*;

    /// Default matrix 2x2 types
    pub type Matrix2x2 = Matrix<Real, 2, 2>;
    pub type Matrix2x2F32 = Matrix<f32, 2, 2>;
    pub type Matrix2x2F64 = Matrix<f64, 2, 2>;

    impl<T: RealNumber> Matrix<T, 2, 2> {
        #[inline]
        pub fn determinant(&self) -> T {
            self[0][0] * self[1][1] - self[0][1] * self[1][0]
        }

        pub fn inverse(&self) -> Self {
            let mut i = Self::default();
            let d = T::real_get_one() / self.determinant();

            i[0][0] = self[1][1];
            i[0][1] = -self[0][1];
            i[1][0] = -self[1][0];
            i[1][1] = self[0][0];

            i * d
        }
    }

    /// Default matrix 3x3 types
    pub type Matrix3x3 = Matrix<Real, 3, 3>;
    pub type Matrix3x3F32 = Matrix<f32, 3, 3>;
    pub type Matrix3x3F64 = Matrix<f64, 3, 3>;

    impl<T: RealNumber> Matrix<T, 3, 3> {
        #[inline]
        pub fn determinant(&self) -> T {
            self[0][0] * (self[1][1] * self[2][2] - self[2][1] * self[1][2]) -
                self[1][0] * (self[0][1] * self[2][2] - self[2][1] * self[0][2]) +
                self[2][0] * (self[0][1] * self[1][2] - self[1][1] * self[0][2])
        }

        pub fn inverse(&self) -> Self {
            let mut i = Self::default();
            let d = T::real_get_one() / self.determinant();

            i[0][0] = (self[1][1] * self[2][2] - self[2][1] * self[1][2]) * d;
            i[1][0] = -(self[1][0] * self[2][2] - self[2][0] * self[1][2]) * d;
            i[2][0] = (self[1][0] * self[2][1] - self[2][0] * self[1][1]) * d;
            i[0][1] = -(self[0][1] * self[2][2] - self[2][1] * self[0][2]) * d;
            i[1][1] = (self[0][0] * self[2][2] - self[2][0] * self[0][2]) * d;
            i[2][1] = -(self[0][0] * self[2][1] - self[2][0] * self[0][1]) * d;
            i[0][2] = (self[0][1] * self[1][2] - self[1][1] * self[0][2]) * d;
            i[1][2] = -(self[0][0] * self[1][2] - self[1][0] * self[0][2]) * d;
            i[2][2] = (self[0][0] * self[1][1] - self[1][0] * self[0][1]) * d;

            i
        }
    }

    /// Default matrix 4x4 types
    pub type Matrix4x4 = Matrix<Real, 4, 4>;
    pub type Matrix4x4F32 = Matrix<f32, 4, 4>;
    pub type Matrix4x4F64 = Matrix<f64, 4, 4>;

    impl<T: RealNumber> Matrix<T, 4, 4> {
        pub fn from_vectors(r0: Vector<T, 4>, r1: Vector<T, 4>, r2: Vector<T, 4>, r3: Vector<T, 4>) -> Self {
            Self { underlying: [*r0, *r1, *r2, *r3] }
        }

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

            let one = T::real_get_one();

            let sign_a = Vector::<T, 4>::new(one, -one, one, -one);
            let sign_b = -sign_a;

            let mut i = Self::from_vectors(inv0 * sign_a, inv1 * sign_b, inv2 * sign_a, inv3 * sign_b);
            let r0 = Vector::<T, 4>::new(i[0][0], i[1][0], i[2][0], i[3][0]);

            let dot0 = Vector::from_array(self[0]) * r0;
            let dot1 = (dot0[0] + dot0[1]) + (dot0[2] + dot0[3]);

            let d = one / dot1;

            return i * d;
        }

        pub fn perspective(fov_y: T, aspect: T, z_near: T, z_far: T) -> Self {
            let one = T::real_get_one();
            let two = one + one;

            let vertical_fov = fov_y * T::real_deg_to_rad();
            let half_fov = (fov_y / two);

            let mut m = Self::default();

            m[0][0] = one / (aspect * half_fov);
            m[1][1] = one / (half_fov);
            m[2][2] = -(z_far + z_near) / (z_far - z_near);
            m[2][3] = -one;
            m[3][2] = -(two * z_far * z_near) / (z_far - z_near);

            m
        }

        pub fn translate(translation: Vector<T, 3>) -> Self {
            let mut m = Self::identity();

            m[0][3] = translation[0];
            m[1][3] = translation[1];
            m[2][3] = translation[2];

            return m;
        }

        pub fn rotate_x(rotation: T) -> Self {
            let mut m = Self::identity();

            m[1] = [T::default(), rotation.real_cos(), -rotation.real_sin(), T::default()];
            m[2] = [T::default(), rotation.real_sin(), rotation.real_cos(), T::default()];

            return m;
        }

        pub fn rotate_y(rotation: T) -> Self {
            let mut m = Self::identity();

            m[0] = [rotation.real_cos(), T::default(), rotation.real_sin(), T::default()];
            m[2] = [-rotation.real_sin(), T::default(), rotation.real_cos(), T::default()];

            return m;
        }

        pub fn rotate_z(rotation: T) -> Self {
            let mut m = Self::identity();

            m[0] = [rotation.real_cos(), -rotation.real_sin(), T::default(), T::default()];
            m[1] = [rotation.real_sin(), rotation.real_cos(), T::default(), T::default()];

            return m;
        }

        pub fn rotation(euler: Vector<T, 3>) -> Self {
            let euler_rad = euler * T::real_deg_to_rad();
            Self::rotate_z(euler_rad[2]) * Self::rotate_y(euler_rad[1]) * Self::rotate_x(euler_rad[0])
        }

        pub fn look_at(direction: Vector<T, 3>) -> Self {
            let up = Vector::<T, 3>::new(T::default(), -T::real_get_one(), T::default());

            let r_right = direction.cross(up).normalize();
            let r_up = direction.cross(r_right).normalize();

            let mut m = Self::identity();

            m[0] = [r_right[0], r_right[1], r_right[2], T::default()];
            m[1] = [r_up[0], r_up[1], r_up[2], T::default()];
            m[2] = [direction[0], direction[1], direction[2], T::default()];

            m
        }

        pub fn transform_point(self, point: Vector<T, 3>) -> Vector<T, 3> {
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
}