// A 3D vector.
// Custom struct so we can have our dot products and what not.
// Could use a crate for this, but the point is to learn!

use std::fmt;
use std::ops::{Add, Sub, Neg};

pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {

    // Constructor
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self{x, y, z}
    }

    // Gets the magnitude of the vector
    pub fn mag(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}


// Operator overloading
impl Add for &Vector3 {
    type Output = Vector3;

    fn add(self, other: Self) -> Self::Output {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for &Vector3 {
    type Output = Vector3;

    fn sub(self, other: Self) -> Self::Output {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self, other: Self) -> Self::Output {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}


// to string
impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.2e}, {:.2e}, {:.2e})", self.x, self.y, self.z)
    }
}