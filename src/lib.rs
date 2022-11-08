#[allow(clippy::needless_return)]
pub mod real;

pub mod vector;
pub mod matrix;

pub mod common;

pub mod ffi;

pub mod prelude {
    //
    // Matrix types
    //
    pub use matrix::common::Matrix2x2;
    pub use matrix::common::Matrix2x2F32;
    pub use matrix::common::Matrix2x2F64;
    pub use matrix::common::Matrix3x3;
    pub use matrix::common::Matrix3x3F32;
    pub use matrix::common::Matrix3x3F64;
    pub use matrix::common::Matrix4x4;
    pub use matrix::common::Matrix4x4F32;
    pub use matrix::common::Matrix4x4F64;

    //
    // Real
    //
    pub use real::Real;
    pub use real::RealNumber;

    //
    // Vector types
    //
    pub use vector::common::Vector2;
    pub use vector::common::Vector2F32;
    pub use vector::common::Vector2F64;
    pub use vector::common::Vector3;
    pub use vector::common::Vector3F32;
    pub use vector::common::Vector3F64;
    pub use vector::common::Vector4;
    pub use vector::common::Vector4F32;
    pub use vector::common::Vector4F64;

    use crate::matrix;
    use crate::real;
    use crate::vector;
}

#[cfg(test)]
mod tests;