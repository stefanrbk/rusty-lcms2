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
fn test_vec3_from_f64_array() {
    let array = [1.0, 69.0, 420.0];

    let expected = CmsVEC3::new(1.0, 69.0, 420.0);

    let actual: CmsVEC3 = array.into();

    assert_eq!(expected, actual);
}

#[test]
fn test_vec3_into_f64_array() {
    let vector = CmsVEC3::new(1.0, 69.0, 420.0);

    let expected = [1.0, 69.0, 420.0];

    let actual: [f64; 3] = vector.into();

    assert_eq!(expected, actual);
}

#[test]
fn test_mat3_is_identity_with_exact() {
    let matrix = CmsMAT3 {
        vx: CmsVEC3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        vy: CmsVEC3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        vz: CmsVEC3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    };
    assert!(matrix == CmsMAT3::IDENTITY);
    assert!(matrix.is_identity());
}

#[test]
fn test_mat3_is_identity_with_close() {
    // is_identity checks that each value is within (1/65535) of the proper value.
    // 1/65535 is ~ 1.5259e-5.
    let matrix = CmsMAT3 {
        vx: CmsVEC3 {
            x: 1.0 + 1.4e-5,
            y: 0.0 - 1.0e-10,
            z: 0.0 + 1.0e-10,
        },
        vy: CmsVEC3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        vz: CmsVEC3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    };
    assert!(matrix.is_identity());
}

#[test]
fn test_mat3_is_identity_fails() {
    // is_identity checks that each value is within (1/65535) of the proper value.
    // 1/65535 is ~ 1.5259e-5.
    let matrix = CmsMAT3 {
        vx: CmsVEC3 {
            x: 1.0,
            y: 0.0,
            z: 0.0 + 1.0e-4,
        },
        vy: CmsVEC3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        vz: CmsVEC3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    };
    assert!(!matrix.is_identity());
}

#[test]
fn test_mat3_multiply_identity_with_identity_equals_identity() {
    let ident = CmsMAT3::IDENTITY;

    let result = ident.per(ident);

    assert!(result.is_identity());
}

#[test]
fn test_mat3_multiply_matrix_with_identity_equals_original_matrix() {
    let matrix = CmsMAT3 {
        vx: CmsVEC3 {
            x: 5.0,
            y: 6.0,
            z: -4.0,
        },
        vy: CmsVEC3 {
            x: 7.0,
            y: 1.0,
            z: 1.0,
        },
        vz: CmsVEC3 {
            x: -2.0,
            y: -9.0,
            z: 8.0,
        },
    };
    let ident = CmsMAT3::IDENTITY;

    let result = matrix * ident;

    assert!(matrix == result);

    let result = ident * matrix;

    assert!(matrix == result);
}

#[test]
fn test_mat3_multiply_skew_with_scale_equals_scaled_skew() {
    let skew = CmsMAT3 {
        vx: CmsVEC3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        vy: CmsVEC3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        vz: CmsVEC3 {
            x: 4.0,
            y: -1.0,
            z: 1.0,
        },
    };
    let scale = CmsMAT3 {
        vx: CmsVEC3 {
            x: 3.0,
            y: 0.0,
            z: 0.0,
        },
        vy: CmsVEC3 {
            x: 0.0,
            y: 3.0,
            z: 0.0,
        },
        vz: CmsVEC3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    };
    let expected = CmsMAT3 {
        vx: CmsVEC3 {
            x: 3.0,
            y: 0.0,
            z: 0.0,
        },
        vy: CmsVEC3 {
            x: 0.0,
            y: 3.0,
            z: 0.0,
        },
        vz: CmsVEC3 {
            x: 12.0,
            y: -3.0,
            z: 1.0,
        },
    };

    let actual = skew * scale;

    assert_eq!(expected, actual);
}

#[test]
fn test_mat3_invert_returns_inverted_matrix() {
    let matrix = CmsMAT3 {
        vx: CmsVEC3 {
            x: 2.0,
            y: 0.0,
            z: 0.0,
        },
        vy: CmsVEC3 {
            x: 0.0,
            y: 2.0,
            z: 0.0,
        },
        vz: CmsVEC3 {
            x: 4.0,
            y: -1.0,
            z: 1.0,
        },
    };
    let expected = CmsMAT3 {
        vx: CmsVEC3 {
            x: 0.5,
            y: 0.0,
            z: 0.0,
        },
        vy: CmsVEC3 {
            x: 0.0,
            y: 0.5,
            z: 0.0,
        },
        vz: CmsVEC3 {
            x: -2.0,
            y: 0.5,
            z: 1.0,
        },
    };

    let actual = matrix.inverse().unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn test_mat3_invert_with_determinant_too_small_returns_none() {
    let matrix = CmsMAT3 {
        vx: CmsVEC3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        vy: CmsVEC3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        vz: CmsVEC3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
    };

    let actual = matrix.inverse();

    assert!(actual.is_none());
}

#[test]
fn test_mat3_solve_vector_with_valid_matrix() {
    let vector = CmsVEC3 {
        x: -2.0,
        y: 3.0,
        z: 1.0,
    };
    let matrix = CmsMAT3 {
        vx: CmsVEC3 {
            x: 2.0,
            y: 0.0,
            z: 0.0,
        },
        vy: CmsVEC3 {
            x: 0.0,
            y: 2.0,
            z: 0.0,
        },
        vz: CmsVEC3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    };
    let expected = CmsVEC3 {
        x: -1.0,
        y: 1.5,
        z: 1.0,
    };

    let actual = matrix.solve(vector);

    assert_eq!(expected, actual.unwrap());
}

#[test]
fn test_mat3_solve_vector_with_invalid_matrix() {
    let vector = CmsVEC3 {
        x: -2.0,
        y: 3.0,
        z: 1.0,
    };
    let matrix = CmsMAT3 {
        vx: CmsVEC3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        vy: CmsVEC3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        vz: CmsVEC3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
    };

    let actual = matrix.solve(vector);

    assert!(actual.is_none());
}

#[test]
fn test_mat3_from_vec3_array() {
    let array = [
        CmsVEC3 {
            x: 5.0,
            y: 6.0,
            z: -4.0,
        },
        CmsVEC3 {
            x: 7.0,
            y: 1.0,
            z: 1.0,
        },
        CmsVEC3 {
            x: -2.0,
            y: -9.0,
            z: 8.0,
        }
    ];
    let expected = CmsMAT3 {
        vx: CmsVEC3 {
            x: 5.0,
            y: 6.0,
            z: -4.0,
        },
        vy: CmsVEC3 {
            x: 7.0,
            y: 1.0,
            z: 1.0,
        },
        vz: CmsVEC3 {
            x: -2.0,
            y: -9.0,
            z: 8.0,
        },
    };
    let actual:CmsMAT3 = array.into();

    assert_eq!(expected, actual);
}

#[test]
fn test_mat3_from_f64_array() {
    let array = [
        5.0,
        6.0,
        -4.0,
        7.0,
        1.0,
        1.0,
        -2.0,
        -9.0,
        8.0
    ];
    let expected = CmsMAT3 {
        vx: CmsVEC3 {
            x: 5.0,
            y: 6.0,
            z: -4.0,
        },
        vy: CmsVEC3 {
            x: 7.0,
            y: 1.0,
            z: 1.0,
        },
        vz: CmsVEC3 {
            x: -2.0,
            y: -9.0,
            z: 8.0,
        },
    };
    let actual:CmsMAT3 = array.into();

    assert_eq!(expected, actual);
}

#[test]
fn test_mat3_into_vec3_array() {
    let matrix = CmsMAT3 {
        vx: CmsVEC3 {
            x: 5.0,
            y: 6.0,
            z: -4.0,
        },
        vy: CmsVEC3 {
            x: 7.0,
            y: 1.0,
            z: 1.0,
        },
        vz: CmsVEC3 {
            x: -2.0,
            y: -9.0,
            z: 8.0,
        },
    };
    let expected = [
        CmsVEC3 {
            x: 5.0,
            y: 6.0,
            z: -4.0,
        },
        CmsVEC3 {
            x: 7.0,
            y: 1.0,
            z: 1.0,
        },
        CmsVEC3 {
            x: -2.0,
            y: -9.0,
            z: 8.0,
        }
    ];
    let actual: [CmsVEC3; 3] = matrix.into();

    assert_eq!(expected, actual);
}

#[test]
fn test_mat3_into_f64_array() {
    let matrix = CmsMAT3 {
        vx: CmsVEC3 {
            x: 5.0,
            y: 6.0,
            z: -4.0,
        },
        vy: CmsVEC3 {
            x: 7.0,
            y: 1.0,
            z: 1.0,
        },
        vz: CmsVEC3 {
            x: -2.0,
            y: -9.0,
            z: 8.0,
        },
    };
    let expected = [
        5.0,
        6.0,
        -4.0,
        7.0,
        1.0,
        1.0,
        -2.0,
        -9.0,
        8.0
    ];
    let actual: [f64; 9] = matrix.into();

    assert_eq!(expected, actual);
}

#[test]
fn test_mat3_multiply_assign_skew_with_scale_equals_scaled_skew() {
    let mut matrix = CmsMAT3 {
        vx: CmsVEC3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        vy: CmsVEC3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        vz: CmsVEC3 {
            x: 4.0,
            y: -1.0,
            z: 1.0,
        },
    };
    let scale = CmsMAT3 {
        vx: CmsVEC3 {
            x: 3.0,
            y: 0.0,
            z: 0.0,
        },
        vy: CmsVEC3 {
            x: 0.0,
            y: 3.0,
            z: 0.0,
        },
        vz: CmsVEC3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    };
    let expected = CmsMAT3 {
        vx: CmsVEC3 {
            x: 3.0,
            y: 0.0,
            z: 0.0,
        },
        vy: CmsVEC3 {
            x: 0.0,
            y: 3.0,
            z: 0.0,
        },
        vz: CmsVEC3 {
            x: 12.0,
            y: -3.0,
            z: 1.0,
        },
    };

    matrix *= scale;

    assert_eq!(expected, matrix);
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}
