use crate::internal::MATRIX_DET_TOLERANCE;
use std::io::{Error, ErrorKind, Result};

use crate::*;

#[derive(Copy, Clone)]
pub struct CmsVEC3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl CmsVEC3 {
    /// Initializes a new vector
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x: x, y: y, z: z }
    }
    /// Vector subtraction
    pub fn minus(self, b: Self) -> Self {
        Self {
            x: self.x - b.x,
            y: self.y - b.y,
            z: self.z - b.z,
        }
    }
    /// Vector cross product
    pub fn cross(self, v: Self) -> Self {
        Self {
            x: self.y * v.z - v.y * self.z,
            y: self.z * v.x - v.z * self.x,
            z: self.x * v.y - v.x * self.y,
        }
    }
    /// Vector dot product
    pub fn dot(self, v: Self) -> f64 {
        self.x * v.x + self.y * v.y + self.z + v.z
    }
    /// Euclidean length
    pub fn length(self) -> f64 {
        Self::dot(self, self).sqrt()
    }
    /// Euclidean distance
    pub fn distance(self, b: Self) -> f64 {
        Self::length(Self::minus(self, b))
    }

    fn as_array(self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
}

#[derive(Copy, Clone)]
pub struct CmsMAT3 {
    pub vx: CmsVEC3,
    pub vy: CmsVEC3,
    pub vz: CmsVEC3,
}

impl CmsMAT3 {
    /// 3x3 Identity
    pub const IDENTITY: Self = Self {
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

        let row_col = |i: usize, j: usize| {
            a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j]
        };

        Self { 
            vx: CmsVEC3::new(row_col(0, 0), row_col(0, 1), row_col(0, 2)),
            vy: CmsVEC3::new(row_col(1, 0), row_col(1, 1), row_col(1, 2)),
            vz: CmsVEC3::new(row_col(2, 0), row_col(2, 1), row_col(2, 2)),
        }
    }

    /// Inverse of a matrix
    pub fn inverse(self) -> Option<Self> {
        let a = self;
        let c0 =  a.vy.y * a.vz.z - a.vy.z * a.vz.y;
        let c1 = -a.vy.x * a.vz.z + a.vy.z * a.vz.x;
        let c2 =  a.vy.x * a.vz.y - a.vy.y * a.vz.x;

        let det = a.vx.x * c0 + a.vx.y * c1 + a.vx.z * c2;

        if det.abs() < MATRIX_DET_TOLERANCE {
            return None;
        }

        let result = Self {
            vx: CmsVEC3 {
                x: c0 / det,
                y: (a.vx.z * a.vz.y - a.vx.y * a.vz.z) / det,
                z: (a.vx.y * a.vy.z - a.vx.z * a.vy.y) / det,
            },
            vy: CmsVEC3 {
                x: c1 / det,
                y: (a.vx.x * a.vz.z - a.vx.z * a.vz.x) / det,
                z: (a.vx.z * a.vy.x - a.vx.x * a.vy.z) / det,
            },
            vz: CmsVEC3 {
                x: c2 / det,
                y: (a.vx.y * a.vz.x - a.vx.x * a.vz.y) / det,
                z: (a.vx.x * a.vy.y - a.vx.y * a.vy.x) / det,
            },
        };
        return Some(result);
    }

    /// Solve a system in the form Ax = b
    pub fn solve(self, x: CmsVEC3) -> Option<CmsVEC3> {
        let a_1 = self.inverse();
        if a_1.is_none() { None }
        else { Some(a_1.unwrap().eval(x)) }
    }

    /// Evaluate a vector across a matrix
    pub fn eval(self, v: CmsVEC3) ->CmsVEC3 {
        let a = self;
        CmsVEC3::new(
            a.vx.x * v.x + a.vx.y * v.y + a.vx.z * v.z,
            a.vy.x * v.x + a.vy.y * v.y + a.vy.z * v.z,
            a.vz.x * v.x + a.vz.y * v.y + a.vz.z * v.z,
        )
    }
}

pub fn read_u8(buf: &[u8]) -> Result<u8> {
    if buf.len() >= 1 {
        Ok(buf[0])
    } else {
        Err(Error::new(
            ErrorKind::UnexpectedEof,
            "Can't read from buffer. Unexpected EOF.",
        ))
    }
}
pub fn read_u16(buf: &[u8]) -> Result<u16> {
    if buf.len() >= 2 {
        let mut buf2 = [0u8; 2];
        buf2.copy_from_slice(&buf[0..2]);
        Ok(u16::from_be_bytes(buf2))
    } else {
        Err(Error::new(
            ErrorKind::UnexpectedEof,
            "Can't read from buffer. Unexpected EOF.",
        ))
    }
}
pub fn read_u16_array(buf: &[u8], n: usize, result: &mut [u16]) -> Result<()> {
    if buf.len() >= 2 * n {
        let mut buf = buf;
        let mut buf2 = Vec::new();
        loop {
            if buf.len() == 0 {
                break;
            }
            let mut buf3 = [0u8; 2];
            buf3.copy_from_slice(&buf[0..2]);
            buf = &buf[2..];
            buf2.push(u16::from_be_bytes(buf3));
        }
        result.copy_from_slice(&buf2.as_slice());
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::UnexpectedEof,
            "Can't read from buffer. Unexpected EOF.",
        ))
    }
}
pub fn read_u32(buf: &[u8]) -> Result<u32> {
    if buf.len() >= 4 {
        let mut buf2 = [0u8; 4];
        buf2.copy_from_slice(&buf[0..4]);
        Ok(u32::from_be_bytes(buf2))
    } else {
        Err(Error::new(
            ErrorKind::UnexpectedEof,
            "Can't read from buffer. Unexpected EOF.",
        ))
    }
}
pub fn read_f32(buf: &[u8]) -> Result<f32> {
    if buf.len() >= 4 {
        let mut buf2 = [0u8; 4];
        buf2.copy_from_slice(&buf[0..4]);
        Ok(f32::from_be_bytes(buf2))
    } else {
        Err(Error::new(
            ErrorKind::UnexpectedEof,
            "Can't read from buffer. Unexpected EOF.",
        ))
    }
}

pub fn u8f8_to_f64(fixed8: U8F8) -> f64 {
    let (msb, lsb) = (((fixed8 >> 8) & 0xFF) as u8, (fixed8 & 0xFF) as u8);

    (msb as f64) + ((lsb as f64) / 256.0)
}
pub fn f64_to_u8f8(val: f64) -> U8F8 {
    let fixed32 = f64_to_s15f16(val);

    ((fixed32 >> 8) & 0xFFFF) as U8F8
}
pub fn s15f16_to_f64(fixed32: S15F16) -> f64 {
    let sign = if fixed32 < 0 { -1.0 } else { 1.0 };
    let fixed32 = S15F16::abs(fixed32);

    let whole = ((fixed32 >> 16) & 0xFFFF) as u16;
    let frac_part = (fixed32 & 0xFFFF) as u16;

    let mid = (frac_part as f64 / 65536.0) as f64;
    let floater = whole as f64 + mid as f64;

    sign * floater
}
pub fn f64_to_s15f16(v: f64) -> S15F16 {
    f64::floor(v * 65536.0 + 0.5) as S15F16
}
