use std::ops::Deref;

use crate::vec3::DoubleVec3;

pub struct Color(pub DoubleVec3);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self(DoubleVec3::new(r, g, b))
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r: u8 = (255.999 * self.0.x) as u8;
        let g: u8 = (255.999 * self.0.y) as u8;
        let b: u8 = (255.999 * self.0.z) as u8;

        write!(f, "{} {} {}\n", r, g, b)
    }
}

impl Deref for Color {
    type Target = DoubleVec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
