#![allow(clippy::needless_return)]

/// Defines "real" numbers, which are used within the math library
pub mod real;

/// Defines arbitrary vector types backed by "real"
pub mod vector;

/// Defines matrix types backed by "real"
pub mod matrix;

/// Defines common real, vector, and matrix types
pub mod common;

/// [Auto generated](https://github.com/zCubed3/rgml/blob/main/scripts/codegen/generate_ffi.py) FFI glue for C/C++ support
pub mod ffi;

/// Prelude which contains the most commonly used features
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