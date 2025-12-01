use serde::{Deserialize, Serialize};

/// 3D Vector for position and velocity
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag == 0.0 {
            Vector3::new(0.0, 0.0, 0.0)
        } else {
            Vector3::new(self.x / mag, self.y / mag, self.z / mag)
        }
    }

    #[allow(dead_code)]
    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector3) -> Self {
        Vector3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn add(&self, other: &Vector3) -> Self {
        Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}
