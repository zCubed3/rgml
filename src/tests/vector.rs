#![allow(unused)]

mod vector2f32 {
    use crate::prelude::*;

    #[test]
    fn test_v2_normalization() {
        let original = Vector2F32::new(2.0, 2.0);
        let norm = original.normalize();

        assert_eq!(norm.magnitude().rl_round(), 1.0);
        assert_ne!(norm, original);
    }

    #[test]
    fn test_v2_add() {
        let a = Vector2F32::new(1.0, 0.0);
        let b = Vector2F32::new(0.0, 1.0);

        assert_eq!(a + b, Vector2F32::from_scalar(1.0));
    }

    #[test]
    fn test_v2_sub() {
        let a = Vector2F32::new(1.0, 2.0);
        let b = Vector2F32::new(2.0, 1.0);

        assert_eq!(a - b, Vector2F32::new(-1.0, 1.0));
    }

    #[test]
    fn test_v2_mul() {
        let a = Vector2F32::new(1.0, 2.0);
        let b = Vector2F32::new(2.0, 0.5);

        assert_eq!(a * b, Vector2F32::new(2.0, 1.0));
    }

    #[test]
    fn test_v2_div() {
        let a = Vector2F32::new(2.0, 2.0);
        let b = Vector2F32::new(2.0, 0.5);

        assert_eq!(a / b, Vector2F32::new(1.0, 4.0));
    }

    #[test]
    fn test_v2_pow() {
        let a = Vector2F32::new(2.0, 2.0);
        let b = Vector2F32::new(2.0, 4.0);

        assert_eq!(a.pow(b), Vector2F32::new(4.0, 16.0));
    }

    #[test]
    fn test_v2_from_scalar() {
        assert_eq!(Vector2F32::new(1.0, 1.0), Vector2F32::from_scalar(1.0));
    }

    #[test]
    fn test_v2_from_array() {
        assert_eq!(
            Vector2F32::new(2.0, 1.0),
            Vector2F32::from_array([2.0, 1.0])
        );
    }

    #[test]
    #[cfg(feature = "swizzle")]
    fn test_v2_swizzle() {
        let t = Vector2F32::new(1.0, 2.0);

        assert_ne!(t.xy(), t.yx());
        assert_ne!(t.rg(), t.gr());
        assert_ne!(t.uv(), t.vu());

        assert_ne!(t.xyx(), t.yxy());
        assert_ne!(t.rgr(), t.grg());
        assert_ne!(t.uvu(), t.vuv());

        assert_ne!(t.xyxy(), t.yxyx());
        assert_ne!(t.rgrg(), t.grgr());
        assert_ne!(t.uvuv(), t.vuvu());

        assert_eq!(t.xy(), t.uv());
        assert_eq!(t.xy(), t.rg());
    }
}

mod vector2f64 {
    use crate::prelude::*;

    #[test]
    fn test_v2_normalization() {
        let original = Vector2F64::new(2.0, 2.0);
        let norm = original.normalize();

        assert_eq!(norm.magnitude().rl_round(), 1.0);
        assert_ne!(norm, original);
    }

    #[test]
    fn test_v2_add() {
        let a = Vector2F64::new(1.0, 0.0);
        let b = Vector2F64::new(0.0, 1.0);

        assert_eq!(a + b, Vector2F64::from_scalar(1.0));
    }

    #[test]
    fn test_v2_sub() {
        let a = Vector2F64::new(1.0, 2.0);
        let b = Vector2F64::new(2.0, 1.0);

        assert_eq!(a - b, Vector2F64::new(-1.0, 1.0));
    }

    #[test]
    fn test_v2_mul() {
        let a = Vector2F64::new(1.0, 2.0);
        let b = Vector2F64::new(2.0, 0.5);

        assert_eq!(a * b, Vector2F64::new(2.0, 1.0));
    }

    #[test]
    fn test_v2_div() {
        let a = Vector2F64::new(2.0, 2.0);
        let b = Vector2F64::new(2.0, 0.5);

        assert_eq!(a / b, Vector2F64::new(1.0, 4.0));
    }

    #[test]
    fn test_v2_pow() {
        let a = Vector2F64::new(2.0, 2.0);
        let b = Vector2F64::new(2.0, 4.0);

        assert_eq!(a.pow(b), Vector2F64::new(4.0, 16.0));
    }

    #[test]
    fn test_v2_from_scalar() {
        assert_eq!(Vector2F64::new(1.0, 1.0), Vector2F64::from_scalar(1.0));
    }

    #[test]
    fn test_v2_from_array() {
        assert_eq!(
            Vector2F64::new(2.0, 1.0),
            Vector2F64::from_array([2.0, 1.0])
        );
    }

    #[test]
    #[cfg(feature = "swizzle")]
    fn test_v2_swizzle() {
        let t = Vector2F64::new(1.0, 2.0);

        assert_ne!(t.xy(), t.yx());
        assert_ne!(t.rg(), t.gr());
        assert_ne!(t.uv(), t.vu());

        assert_ne!(t.xyx(), t.yxy());
        assert_ne!(t.rgr(), t.grg());
        assert_ne!(t.uvu(), t.vuv());

        assert_ne!(t.xyxy(), t.yxyx());
        assert_ne!(t.rgrg(), t.grgr());
        assert_ne!(t.uvuv(), t.vuvu());

        assert_eq!(t.xy(), t.uv());
        assert_eq!(t.xy(), t.rg());
    }
}

mod vector3f32 {
    use crate::prelude::*;

    #[test]
    fn test_v3_normalization() {
        let original = Vector3F32::new(2.0, 2.0, 2.0);
        let norm = original.normalize();

        assert_eq!(norm.magnitude().rl_round(), 1.0);
        assert_ne!(norm, original);
    }

    #[test]
    fn test_v3_add() {
        let a = Vector3F32::new(1.0, 0.0, 0.5);
        let b = Vector3F32::new(0.0, 1.0, 0.5);

        assert_eq!(a + b, Vector3F32::from_scalar(1.0));
    }

    #[test]
    fn test_v3_sub() {
        let a = Vector3F32::new(1.0, 2.0, 1.0);
        let b = Vector3F32::new(2.0, 1.0, 1.0);

        assert_eq!(a - b, Vector3F32::new(-1.0, 1.0, 0.0));
    }

    #[test]
    fn test_v3_mul() {
        let a = Vector3F32::new(1.0, 2.0, 2.0);
        let b = Vector3F32::new(2.0, 0.5, 2.0);

        assert_eq!(a * b, Vector3F32::new(2.0, 1.0, 4.0));
    }

    #[test]
    fn test_v3_div() {
        let a = Vector3F32::new(2.0, 2.0, 2.0);
        let b = Vector3F32::new(2.0, 0.5, 4.0);

        assert_eq!(a / b, Vector3F32::new(1.0, 4.0, 0.5));
    }

    #[test]
    fn test_v3_pow() {
        let a = Vector3F32::new(2.0, 2.0, 2.0);
        let b = Vector3F32::new(2.0, 4.0, 8.0);

        assert_eq!(a.pow(b), Vector3F32::new(4.0, 16.0, 256.0));
    }

    #[test]
    fn test_v3_from_scalar() {
        assert_eq!(Vector3F32::new(1.0, 1.0, 1.0), Vector3F32::from_scalar(1.0));
    }

    #[test]
    fn test_v3_from_array() {
        assert_eq!(
            Vector3F32::new(2.0, 1.0, 0.0),
            Vector3F32::from_array([2.0, 1.0, 0.0])
        );
    }

    #[test]
    #[cfg(feature = "swizzle")]
    fn test_v3_swizzle() {
        let t = Vector3F32::new(1.0, 2.0, 3.0);

        assert_ne!(t.xy(), t.yx());
        assert_ne!(t.rg(), t.gr());
        assert_ne!(t.uv(), t.vu());

        assert_ne!(t.xyx(), t.yxy());
        assert_ne!(t.rgr(), t.grg());
        assert_ne!(t.uvu(), t.vuv());

        assert_ne!(t.xyxy(), t.yxyx());
        assert_ne!(t.rgrg(), t.grgr());
        assert_ne!(t.uvuv(), t.vuvu());

        assert_eq!(t.xy(), t.uv());
        assert_eq!(t.xy(), t.rg());
    }
}

mod vector3f64 {
    use crate::prelude::*;

    #[test]
    fn test_v3_normalization() {
        let original = Vector3F64::new(2.0, 2.0, 2.0);
        let norm = original.normalize();

        assert_eq!(norm.magnitude().rl_round(), 1.0);
        assert_ne!(norm, original);
    }

    #[test]
    fn test_v3_add() {
        let a = Vector3F64::new(1.0, 0.0, 0.5);
        let b = Vector3F64::new(0.0, 1.0, 0.5);

        assert_eq!(a + b, Vector3F64::from_scalar(1.0));
    }

    #[test]
    fn test_v3_sub() {
        let a = Vector3F64::new(1.0, 2.0, 1.0);
        let b = Vector3F64::new(2.0, 1.0, 1.0);

        assert_eq!(a - b, Vector3F64::new(-1.0, 1.0, 0.0));
    }

    #[test]
    fn test_v3_mul() {
        let a = Vector3F64::new(1.0, 2.0, 2.0);
        let b = Vector3F64::new(2.0, 0.5, 2.0);

        assert_eq!(a * b, Vector3F64::new(2.0, 1.0, 4.0));
    }

    #[test]
    fn test_v3_div() {
        let a = Vector3F64::new(2.0, 2.0, 2.0);
        let b = Vector3F64::new(2.0, 0.5, 4.0);

        assert_eq!(a / b, Vector3F64::new(1.0, 4.0, 0.5));
    }

    #[test]
    fn test_v3_pow() {
        let a = Vector3F64::new(2.0, 2.0, 2.0);
        let b = Vector3F64::new(2.0, 4.0, 8.0);

        assert_eq!(a.pow(b), Vector3F64::new(4.0, 16.0, 256.0));
    }

    #[test]
    fn test_v3_from_scalar() {
        assert_eq!(Vector3F64::new(1.0, 1.0, 1.0), Vector3F64::from_scalar(1.0));
    }

    #[test]
    fn test_v3_from_array() {
        assert_eq!(
            Vector3F64::new(2.0, 1.0, 0.0),
            Vector3F64::from_array([2.0, 1.0, 0.0])
        );
    }

    #[test]
    #[cfg(feature = "swizzle")]
    fn test_v3_swizzle() {
        let t = Vector3F64::new(1.0, 2.0, 3.0);

        assert_ne!(t.xy(), t.yx());
        assert_ne!(t.rg(), t.gr());
        assert_ne!(t.uv(), t.vu());

        assert_ne!(t.xyx(), t.yxy());
        assert_ne!(t.rgr(), t.grg());
        assert_ne!(t.uvu(), t.vuv());

        assert_ne!(t.xyxy(), t.yxyx());
        assert_ne!(t.rgrg(), t.grgr());
        assert_ne!(t.uvuv(), t.vuvu());

        assert_eq!(t.xy(), t.uv());
        assert_eq!(t.xy(), t.rg());
    }
}
