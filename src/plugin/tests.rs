use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

use super::*;

// CmsVEC3 Tests

fn assert_vec3(expected: &CmsVEC3, actual: &CmsVEC3) {
    assert!(
        expected.x == actual.x,
        "Expected x to be {} but was {}",
        expected.x,
        actual.x
    );
    assert!(
        expected.y == actual.y,
        "Expected y to be {} but was {}",
        expected.y,
        actual.y
    );
    assert!(
        expected.z == actual.z,
        "Expected z to be {} but was {}",
        expected.z,
        actual.z
    );
}

#[test]
fn test_vec3_new() {
    let (x, y, z) = (1.0, 69.0, 420.0);

    let value = CmsVEC3::new(x, y, z);

    assert_vec3(&CmsVEC3 { x: x, y: y, z: z }, &value);
}

#[test]
fn test_vec3_minus() {
    let left = CmsVEC3::new(-1.0, 42.0, 421.0);
    let right = CmsVEC3::new(-2.0, -27.0, 1.0);

    let result = left.minus(&right);

    assert_vec3(&CmsVEC3::new(1.0, 69.0, 420.0), &result);
}

#[test]
fn test_vec3_cross() {
    let left = CmsVEC3::new(2.0, 3.0, 4.0);
    let right = CmsVEC3::new(3.0, 4.0, 2.0);

    let result = left.cross(&right);

    assert_vec3(&CmsVEC3::new(-10.0, 8.0, -1.0), &result);
}

#[test]
fn test_vec3_dot() {
    let left = CmsVEC3::new(2.0, 3.0, 4.0);
    let right = CmsVEC3::new(3.0, 4.0, 2.0);

    let result = left.dot(&right);

    assert_eq!(26.0, result);
}

#[test]
fn test_vec3_length() {
    let value = CmsVEC3::new(2.0, 3.0, 4.0);

    let result = value.length();

    assert_eq!(5.3851648071345040312507104915403, result);
}

#[test]
fn test_vec3_distance() {
    let left = CmsVEC3::new(1.0, 69.0, 420.0);
    let right = CmsVEC3::new(2.0, 3.0, 4.0);

    let result = left.distance(&right);

    assert_eq!(421.2042259996924579764305420548, result);
}

#[test]
fn test_vec3_as_array() {
    let value = CmsVEC3::new(1.0, 69.0, 420.0);

    let result = value.as_array();

    assert_eq!([1.0, 69.0, 420.0], result);
}

#[test]
fn test_vec3_add() {
    let left = CmsVEC3::new(1.0, 69.0, 420.0);
    let right = CmsVEC3::new(-2.0, -27.0, 1.0);

    let result = left + right;

    assert_vec3(&CmsVEC3::new(-1.0, 42.0, 421.0), &result);
}

#[test]
fn test_vec3_add_assign() {
    let mut left = CmsVEC3::new(1.0, 69.0, 420.0);
    let right = CmsVEC3::new(-2.0, -27.0, 1.0);

    left += right;

    assert_vec3(&CmsVEC3::new(-1.0, 42.0, 421.0), &left);
}

#[test]
fn test_vec3_sub() {
    let left = CmsVEC3::new(-1.0, 42.0, 421.0);
    let right = CmsVEC3::new(-2.0, -27.0, 1.0);

    let result = left - right;

    assert_vec3(&CmsVEC3::new(1.0, 69.0, 420.0), &result);
}

#[test]
fn test_vec3_sub_assign() {
    let mut left = CmsVEC3::new(-1.0, 42.0, 421.0);
    let right = CmsVEC3::new(-2.0, -27.0, 1.0);

    left -= right;

    assert_vec3(&CmsVEC3::new(1.0, 69.0, 420.0), &left);
}

#[test]
fn test_vec3_mul() {
    let left = CmsVEC3::new(2.0, 3.0, 4.0);
    let right = CmsVEC3::new(3.0, 4.0, 2.0);

    let result = left * right;

    assert_vec3(&CmsVEC3::new(-10.0, 8.0, -1.0), &result);
}

#[test]
fn test_vec3_mul_assign() {
    let mut left = CmsVEC3::new(2.0, 3.0, 4.0);
    let right = CmsVEC3::new(3.0, 4.0, 2.0);

    left *= right;

    assert_vec3(&CmsVEC3::new(-10.0, 8.0, -1.0), &left);
}

#[test]
fn test_vec3_neg() {
    let value = CmsVEC3::new(1.0, 69.0, 420.0);

    let result = -value;

    assert_eq!(CmsVEC3::new(-1.0, -69.0, -420.0), result);
}

#[test]
fn test_vec3_fmt() {
    let actual = format!("The vector is {}", CmsVEC3::new(-1.0, 0.5, 69.0));

    let expected = "The vector is (-1, 0.5, 69)";

    assert_eq!(expected, actual);
}

#[test]
fn test_vec3_hash_same_for_same_input() {
    let hash1 = calculate_hash(&CmsVEC3::new(1.0, 69.0, 420.0));
    let hash2 = calculate_hash(&CmsVEC3::new(1.0, 69.0, 420.0));

    assert_eq!(hash1, hash2);
}

#[test]
fn test_vec3_hash_same_for_same_input_with_nan() {
    let hash1 = calculate_hash(&CmsVEC3::new(f64::NAN, 69.0, 420.0));
    let hash2 = calculate_hash(&CmsVEC3::new(f64::NAN, 69.0, 420.0));

    assert_eq!(hash1, hash2);
}

#[test]
fn test_vec3_hash_different_for_different_input() {
    let hash1 = calculate_hash(&CmsVEC3::new(1.0, 69.0, 420.0));
    let hash2 = calculate_hash(&CmsVEC3::new(2.0, 3.0, 4.0));

    assert_ne!(hash1, hash2);
}

#[test]
fn test_identity_is_identity() {
    let ident = CmsMAT3::IDENTITY;
    assert!(
        ident.vx.x == 1.0
            && ident.vx.y == 0.0
            && ident.vx.z == 0.0
            && ident.vy.x == 0.0
            && ident.vy.y == 1.0
            && ident.vy.z == 0.0
            && ident.vz.x == 0.0
            && ident.vz.y == 0.0
            && ident.vz.z == 1.0
    );
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}
