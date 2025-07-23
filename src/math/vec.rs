use super::{ClosedAdd, ClosedMul};


/// Static vector with a number of elements
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T> {
    /// Create a new vector from coordinates
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T: ClosedMul + ClosedAdd + Copy> Vector3<T> {
    pub fn dot(&self, other: &Self) -> T {
        self.x*other.x + self.y*other.y + self.z*other.z
    }
}
