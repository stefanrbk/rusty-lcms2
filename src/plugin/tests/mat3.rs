use super::*;

#[test]
fn test_mat3_is_identity_with_exact() {
    let matrix = Mat3 {
        vx: Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        vy: Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        vz: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    };
    assert!(matrix == Mat3::IDENTITY);
    assert!(matrix.is_identity());
}

#[test]
fn test_mat3_is_identity_with_close() {
    // is_identity checks that each value is within (1/65535) of the proper value.
    // 1/65535 is ~ 1.5259e-5.
    let matrix = Mat3 {
        vx: Vec3 {
            x: 1.0 + 1.4e-5,
            y: 0.0 - 1.0e-10,
            z: 0.0 + 1.0e-10,
        },
        vy: Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        vz: Vec3 {
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
    let matrix = Mat3 {
        vx: Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0 + 1.0e-4,
        },
        vy: Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        vz: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    };
    assert!(!matrix.is_identity());
}

#[test]
fn test_mat3_multiply_identity_with_identity_equals_identity() {
    let ident = Mat3::IDENTITY;

    let result = ident.per(ident);

    assert!(result.is_identity());
}

#[test]
fn test_mat3_multiply_matrix_with_identity_equals_original_matrix() {
    let matrix = Mat3 {
        vx: Vec3 {
            x: 5.0,
            y: 6.0,
            z: -4.0,
        },
        vy: Vec3 {
            x: 7.0,
            y: 1.0,
            z: 1.0,
        },
        vz: Vec3 {
            x: -2.0,
            y: -9.0,
            z: 8.0,
        },
    };
    let ident = Mat3::IDENTITY;

    let result = matrix * ident;

    assert!(matrix == result);

    let result = ident * matrix;

    assert!(matrix == result);
}

#[test]
fn test_mat3_multiply_skew_with_scale_equals_scaled_skew() {
    let skew = Mat3 {
        vx: Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        vy: Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        vz: Vec3 {
            x: 4.0,
            y: -1.0,
            z: 1.0,
        },
    };
    let scale = Mat3 {
        vx: Vec3 {
            x: 3.0,
            y: 0.0,
            z: 0.0,
        },
        vy: Vec3 {
            x: 0.0,
            y: 3.0,
            z: 0.0,
        },
        vz: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    };
    let expected = Mat3 {
        vx: Vec3 {
            x: 3.0,
            y: 0.0,
            z: 0.0,
        },
        vy: Vec3 {
            x: 0.0,
            y: 3.0,
            z: 0.0,
        },
        vz: Vec3 {
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
    let matrix = Mat3 {
        vx: Vec3 {
            x: 2.0,
            y: 0.0,
            z: 0.0,
        },
        vy: Vec3 {
            x: 0.0,
            y: 2.0,
            z: 0.0,
        },
        vz: Vec3 {
            x: 4.0,
            y: -1.0,
            z: 1.0,
        },
    };
    let expected = Mat3 {
        vx: Vec3 {
            x: 0.5,
            y: 0.0,
            z: 0.0,
        },
        vy: Vec3 {
            x: 0.0,
            y: 0.5,
            z: 0.0,
        },
        vz: Vec3 {
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
    let matrix = Mat3 {
        vx: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        vy: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        vz: Vec3 {
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
    let vector = Vec3 {
        x: -2.0,
        y: 3.0,
        z: 1.0,
    };
    let matrix = Mat3 {
        vx: Vec3 {
            x: 2.0,
            y: 0.0,
            z: 0.0,
        },
        vy: Vec3 {
            x: 0.0,
            y: 2.0,
            z: 0.0,
        },
        vz: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    };
    let expected = Vec3 {
        x: -1.0,
        y: 1.5,
        z: 1.0,
    };

    let actual = matrix.solve(vector);

    assert_eq!(expected, actual.unwrap());
}

#[test]
fn test_mat3_solve_vector_with_invalid_matrix() {
    let vector = Vec3 {
        x: -2.0,
        y: 3.0,
        z: 1.0,
    };
    let matrix = Mat3 {
        vx: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        vy: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        vz: Vec3 {
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
        Vec3 {
            x: 5.0,
            y: 6.0,
            z: -4.0,
        },
        Vec3 {
            x: 7.0,
            y: 1.0,
            z: 1.0,
        },
        Vec3 {
            x: -2.0,
            y: -9.0,
            z: 8.0,
        },
    ];
    let expected = Mat3 {
        vx: Vec3 {
            x: 5.0,
            y: 6.0,
            z: -4.0,
        },
        vy: Vec3 {
            x: 7.0,
            y: 1.0,
            z: 1.0,
        },
        vz: Vec3 {
            x: -2.0,
            y: -9.0,
            z: 8.0,
        },
    };
    let actual: Mat3 = array.into();

    assert_eq!(expected, actual);
}

#[test]
fn test_mat3_from_f64_array() {
    let array = [5.0, 6.0, -4.0, 7.0, 1.0, 1.0, -2.0, -9.0, 8.0];
    let expected = Mat3 {
        vx: Vec3 {
            x: 5.0,
            y: 6.0,
            z: -4.0,
        },
        vy: Vec3 {
            x: 7.0,
            y: 1.0,
            z: 1.0,
        },
        vz: Vec3 {
            x: -2.0,
            y: -9.0,
            z: 8.0,
        },
    };
    let actual: Mat3 = array.into();

    assert_eq!(expected, actual);
}

#[test]
fn test_mat3_into_vec3_array() {
    let matrix = Mat3 {
        vx: Vec3 {
            x: 5.0,
            y: 6.0,
            z: -4.0,
        },
        vy: Vec3 {
            x: 7.0,
            y: 1.0,
            z: 1.0,
        },
        vz: Vec3 {
            x: -2.0,
            y: -9.0,
            z: 8.0,
        },
    };
    let expected = [
        Vec3 {
            x: 5.0,
            y: 6.0,
            z: -4.0,
        },
        Vec3 {
            x: 7.0,
            y: 1.0,
            z: 1.0,
        },
        Vec3 {
            x: -2.0,
            y: -9.0,
            z: 8.0,
        },
    ];
    let actual: [Vec3; 3] = matrix.into();

    assert_eq!(expected, actual);
}

#[test]
fn test_mat3_into_f64_array() {
    let matrix = Mat3 {
        vx: Vec3 {
            x: 5.0,
            y: 6.0,
            z: -4.0,
        },
        vy: Vec3 {
            x: 7.0,
            y: 1.0,
            z: 1.0,
        },
        vz: Vec3 {
            x: -2.0,
            y: -9.0,
            z: 8.0,
        },
    };
    let expected = [5.0, 6.0, -4.0, 7.0, 1.0, 1.0, -2.0, -9.0, 8.0];
    let actual: [f64; 9] = matrix.into();

    assert_eq!(expected, actual);
}

#[test]
fn test_mat3_multiply_assign_skew_with_scale_equals_scaled_skew() {
    let mut matrix = Mat3 {
        vx: Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        vy: Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        vz: Vec3 {
            x: 4.0,
            y: -1.0,
            z: 1.0,
        },
    };
    let scale = Mat3 {
        vx: Vec3 {
            x: 3.0,
            y: 0.0,
            z: 0.0,
        },
        vy: Vec3 {
            x: 0.0,
            y: 3.0,
            z: 0.0,
        },
        vz: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    };
    let expected = Mat3 {
        vx: Vec3 {
            x: 3.0,
            y: 0.0,
            z: 0.0,
        },
        vy: Vec3 {
            x: 0.0,
            y: 3.0,
            z: 0.0,
        },
        vz: Vec3 {
            x: 12.0,
            y: -3.0,
            z: 1.0,
        },
    };

    matrix *= scale;

    assert_eq!(expected, matrix);
}
