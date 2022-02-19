use super::CmsVEC3 as CmsVEC3;

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
