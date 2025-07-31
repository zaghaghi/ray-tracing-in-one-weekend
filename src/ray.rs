use crate::vec3::{DoubleVec3, Point3};

#[derive(Default)]
pub struct Ray {
    pub origin: Point3,
    pub direction: DoubleVec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: DoubleVec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        &self.origin + &(&self.direction * t)
    }
}
