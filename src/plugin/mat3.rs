use std::ops::MulAssign;
use std::ops::Mul;
use super::Vec3;
use crate::internal::MATRIX_DET_TOLERANCE;

#[derive(Copy, Clone, PartialEq, Debug, Default, Hash)]
pub struct Mat3 {
    pub vx: Vec3,
    pub vy: Vec3,
    pub vz: Vec3,
}

impl Mat3 {
    /// 3x3 Identity
    pub const IDENTITY: Self = Self {
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

    fn close_enough(a: f64, b: f64) -> bool {
        (b - a).abs() < (1.0 / 65535.0)
    }

    fn as_array(self) -> [[f64; 3]; 3] {
        [self.vx.as_array(), self.vy.as_array(), self.vz.as_array()]
    }

    pub fn is_identity(self) -> bool {
        let value = self.as_array();
        let identity = Self::IDENTITY.as_array();

        for i in 0..3 {
            for j in 0..3 {
                if !Self::close_enough(value[i][j], identity[i][j]) {
                    return false;
                }
            }
        }
        return true;
    }

    /// Multiply two matrices
    pub fn per(self, b: Self) -> Self {
        let a = self.as_array();
        let b = b.as_array();

        let row_col =
            |i: usize, j: usize| a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j];

        Self {
            vx: Vec3::new(row_col(0, 0), row_col(0, 1), row_col(0, 2)),
            vy: Vec3::new(row_col(1, 0), row_col(1, 1), row_col(1, 2)),
            vz: Vec3::new(row_col(2, 0), row_col(2, 1), row_col(2, 2)),
        }
    }

    /// Inverse of a matrix
    pub fn inverse(self) -> Option<Self> {
        let a = self;
        let c0 = a.vy.y * a.vz.z - a.vy.z * a.vz.y;
        let c1 = -a.vy.x * a.vz.z + a.vy.z * a.vz.x;
        let c2 = a.vy.x * a.vz.y - a.vy.y * a.vz.x;

        let det = a.vx.x * c0 + a.vx.y * c1 + a.vx.z * c2;

        if det.abs() < MATRIX_DET_TOLERANCE {
            return None;
        }

        let result = Self {
            vx: Vec3 {
                x: c0 / det,
                y: (a.vx.z * a.vz.y - a.vx.y * a.vz.z) / det,
                z: (a.vx.y * a.vy.z - a.vx.z * a.vy.y) / det,
            },
            vy: Vec3 {
                x: c1 / det,
                y: (a.vx.x * a.vz.z - a.vx.z * a.vz.x) / det,
                z: (a.vx.z * a.vy.x - a.vx.x * a.vy.z) / det,
            },
            vz: Vec3 {
                x: c2 / det,
                y: (a.vx.y * a.vz.x - a.vx.x * a.vz.y) / det,
                z: (a.vx.x * a.vy.y - a.vx.y * a.vy.x) / det,
            },
        };
        return Some(result);
    }

    /// Solve a system in the form Ax = b
    pub fn solve(self, x: Vec3) -> Option<Vec3> {
        let a_1 = self.inverse();
        if a_1.is_none() {
            None
        } else {
            Some(a_1.unwrap().eval(x))
        }
    }

    /// Evaluate a vector across a matrix
    pub fn eval(self, v: Vec3) -> Vec3 {
        let a = self;
        Vec3::new(
            a.vx.x * v.x + a.vx.y * v.y + a.vx.z * v.z,
            a.vy.x * v.x + a.vy.y * v.y + a.vy.z * v.z,
            a.vz.x * v.x + a.vz.y * v.y + a.vz.z * v.z,
        )
    }
}

impl From<[Vec3; 3]> for Mat3 {
    fn from(array: [Vec3; 3]) -> Self {
        Self {
            vx: array[0],
            vy: array[1],
            vz: array[2],
        }
    }
}

impl From<[f64; 9]> for Mat3 {
    fn from(array: [f64; 9]) -> Self {
        Self {
            vx: Vec3::new(array[0], array[1], array[2]),
            vy: Vec3::new(array[3], array[4], array[5]),
            vz: Vec3::new(array[6], array[7], array[8]),
        }
    }
}

impl From<Mat3> for [Vec3; 3] {
    fn from(value: Mat3) -> Self {
        [value.vx, value.vy, value.vz]
    }
}

impl From<Mat3> for [f64; 9] {
    fn from(value: Mat3) -> Self {
        [
            value.vx.x, value.vx.y, value.vx.z,
            value.vy.x, value.vy.y, value.vy.z,
            value.vz.x, value.vz.y, value.vz.z
        ]
    }
}

impl Mul for Mat3 {
    type Output = Mat3;
    fn mul(self, other: Self) -> Self {
        self.per(other)
    }
}

impl MulAssign for Mat3 {
    fn mul_assign(&mut self, other: Self) {
        let matrix = self.per(other);

        self.vx.x = matrix.vx.x;
        self.vx.y = matrix.vx.y;
        self.vx.z = matrix.vx.z;

        self.vy.x = matrix.vy.x;
        self.vy.y = matrix.vy.y;
        self.vy.z = matrix.vy.z;

        self.vz.x = matrix.vz.x;
        self.vz.y = matrix.vz.y;
        self.vz.z = matrix.vz.z;
    }
}
