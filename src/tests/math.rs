#![allow(unused)]

mod vector3 {
    use crate::math::vector3::*;

    //
    // V3 only operations
    //
    fn test_v3_normalization() {
        let mut a = Vector3::from_single(1.0f32);

        // Test 4 times for sanity reasons!
        for _ in 0 .. 4 {
            assert_eq!(a.normalize().magnitude(), 1f32);
            a *= 2f32;
        }
    }

    //
    // V3 and V3 operations
    //
    #[test]
    fn test_v3_v3_addition() {
        let a = Vector3::from_array([0.75f32, 0.25f32, 0.5f32]);
        let b = Vector3::from_array([0.25f32, 0.75f32, 0.5f32]);

        assert_eq!(a + b, Vector3::from_array([1f32, 1f32, 1f32]))
    }

    #[test]
    fn test_v3_v3_subtraction() {
        let a = Vector3::from_array([2f32, 4f32, 8f32]);
        let b = Vector3::from_array([1f32, 3f32, 7f32]);

        assert_eq!(a - b, Vector3::from_array([1f32, 1f32, 1f32]))
    }

    #[test]
    fn test_v3_v3_multiply() {
        let a = Vector3::from_array([1f32, 2f32, 4f32]);
        let b = Vector3::from_array([2f32, 2f32, 2f32]);

        assert_eq!(a * b, Vector3::from_array([2f32, 4f32, 8f32]))
    }

    #[test]
    fn test_v3_v3_division() {
        let a = Vector3::from_array([2f32, 4f32, 8f32]);
        let b = Vector3::from_array([2f32, 2f32, 2f32]);

        assert_eq!(a / b, Vector3::from_array([1f32, 2f32, 4f32]))
    }

    #[test]
    fn test_v3_v3_cross() {
        let a = Vector3::from_array([1f32, 0f32, 0f32]);
        let b = Vector3::from_array([0f32, 1f32, 0f32]);

        assert_eq!(a.cross(b), Vector3::from_array([0f32, 0f32, 1f32]))
    }

    //
    // V3 and F32 operations
    //
    #[test]
    fn test_v3_f32_addition() {
        assert_eq!(Vector3::from_array([0f32, 1f32, 2f32]) + 1f32, Vector3::from_array([1f32, 2f32, 3f32]))
    }

    #[test]
    fn test_v3_f32_subtraction() {
        assert_eq!(Vector3::from_array([1f32, 2f32, 3f32]) - 1f32, Vector3::from_array([0f32, 1f32, 2f32]))
    }

    #[test]
    fn test_v3_f32_multiply() {
        assert_eq!(Vector3::from_array([1f32, 2f32, 4f32]) * 2f32, Vector3::from_array([2f32, 4f32, 8f32]))
    }

    #[test]
    fn test_v3_f32_division() {
        assert_eq!(Vector3::from_array([2f32, 4f32, 8f32]) / 2f32, Vector3::from_array([1f32, 2f32, 4f32]))
    }
}