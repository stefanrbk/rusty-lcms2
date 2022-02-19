#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct CmsVEC3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl CmsVEC3 {
    /// Initializes a new vector
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x: x, y: y, z: z }
    }
    /// Vector subtraction
    pub fn minus(&self, b: &Self) -> Self {
        Self {
            x: self.x - b.x,
            y: self.y - b.y,
            z: self.z - b.z,
        }
    }
    /// Vector cross product
    pub fn cross(&self, v: &Self) -> Self {
        Self {
            x: self.y * v.z - v.y * self.z,
            y: self.z * v.x - v.z * self.x,
            z: self.x * v.y - v.x * self.y,
        }
    }
    /// Vector dot product
    pub fn dot(&self, v: &Self) -> f64 {
        self.x * v.x + self.y * v.y + self.z + v.z
    }
    /// Euclidean length
    pub fn length(&self) -> f64 {
        Self::dot(self, self).sqrt()
    }
    /// Euclidean distance
    pub fn distance(&self, b: &Self) -> f64 {
        Self::length(&Self::minus(self, b))
    }

    pub(super) fn as_array(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
}

use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Neg};
impl Add for CmsVEC3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}
impl AddAssign for CmsVEC3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}
impl Sub for CmsVEC3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}
impl SubAssign for CmsVEC3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}
impl Mul for CmsVEC3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        self.cross(&other)
    }
}
impl MulAssign for CmsVEC3 {
    fn mul_assign(&mut self, other: Self) {
        let result = Self {
            x: self.y * other.z - other.y * self.z,
            y: self.z * other.x - other.z * self.x,
            z: self.x * other.y - other.x * self.y,
        };
        self.x = result.x;
        self.y = result.y;
        self.z = result.z;
    }
}
impl Neg for CmsVEC3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

use std::fmt::{Formatter, Result, Display};
impl Display for CmsVEC3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

use std::hash::{Hash, Hasher};
impl Hash for CmsVEC3 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        f64::to_be_bytes(self.x).hash(state);
        f64::to_be_bytes(self.y).hash(state);
        f64::to_be_bytes(self.z).hash(state);
        f64::to_be_bytes(420.69).hash(state);
    }
}
