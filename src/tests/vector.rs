#![allow(unused)]

mod vector2 {
    use crate::prelude::*;

    //
    // V2 only operations
    //
    fn test_v2_normalization() {
        let mut a = Vector2::from_scalar(2.0);

        // Test 4 times for sanity reasons!
        for _ in 0 .. 4 {
            assert_eq!(a.normalize().magnitude(), 1.0);
            a *= a;
        }
    }

    //
    // V2 and V3 operations
    //
}

mod vector3 {
    use crate::prelude::*;

    //
    // V3 only operations
    //
    fn test_v3_normalization() {
        let mut a = Vector3::from_scalar(1.0);

        // Test 4 times for sanity reasons!
        for _ in 0 .. 4 {
            assert_eq!(a.normalize().magnitude(), 1.0);
            a *= 2.0;
        }
    }

    //
    // V3 and V3 operations
    //
    #[test]
    fn test_v3_v3_addition() {
        let a = Vector3::from_array([0.75, 0.25, 0.5]);
        let b = Vector3::from_array([0.25, 0.75, 0.5]);

        assert_eq!(a + b, Vector3::from_array([1.0, 1.0, 1.0]))
    }

    #[test]
    fn test_v3_v3_subtraction() {
        let a = Vector3::from_array([2.0, 4.0, 8.0]);
        let b = Vector3::from_array([1.0, 3.0, 7.0]);

        assert_eq!(a - b, Vector3::from_array([1.0, 1.0, 1.0]))
    }

    #[test]
    fn test_v3_v3_multiply() {
        let a = Vector3::from_array([1.0, 2.0, 4.0]);
        let b = Vector3::from_array([2.0, 2.0, 2.0]);

        assert_eq!(a * b, Vector3::from_array([2.0, 4.0, 8.0]))
    }

    #[test]
    fn test_v3_v3_division() {
        let a = Vector3::from_array([2.0, 4.0, 8.0]);
        let b = Vector3::from_array([2.0, 2.0, 2.0]);

        assert_eq!(a / b, Vector3::from_array([1.0, 2.0, 4.0]))
    }

    #[test]
    fn test_v3_v3_cross() {
        let a = Vector3::from_array([1.0, 0.0, 0.0]);
        let b = Vector3::from_array([0.0, 1.0, 0.0]);

        assert_eq!(a.cross(b), Vector3::from_array([0.0, 0.0, 1.0]))
    }

    //
    // V3 and .0 operations
    //
    #[test]
    fn test_v3_addition() {
        assert_eq!(Vector3::from_array([0.0, 1.0, 2.0]) + 1.0, Vector3::from_array([1.0, 2.0, 3.0]))
    }

    #[test]
    fn test_v3_subtraction() {
        assert_eq!(Vector3::from_array([1.0, 2.0, 3.0]) - 1.0, Vector3::from_array([0.0, 1.0, 2.0]))
    }

    #[test]
    fn test_v3_multiply() {
        assert_eq!(Vector3::from_array([1.0, 2.0, 4.0]) * 2.0, Vector3::from_array([2.0, 4.0, 8.0]))
    }

    #[test]
    fn test_v3_division() {
        assert_eq!(Vector3::from_array([2.0, 4.0, 8.0]) / 2.0, Vector3::from_array([1.0, 2.0, 4.0]))
    }

    //
    // V3 from other vector tests
    //
    #[test]
    fn test_v3_from_v2() {
        let v2 = Vector2::from_array([1.0, 2.0]);
        let v3 = Vector3::from(v2);

        assert_eq!(v3, Vector3::from_array([1.0, 2.0, 0.0]))
    }

    #[test]
    fn test_v3_from_v4() {
        let v4 = Vector4::from_array([1.0, 2.0, 3.0, 4.0]);
        let v3 = Vector3::from(v4);

        assert_eq!(v3, Vector3::from_array([1.0, 2.0, 3.0]))
    }
}