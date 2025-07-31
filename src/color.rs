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

impl std::ops::Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Color::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl std::ops::Add<Self> for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
