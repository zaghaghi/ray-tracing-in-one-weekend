use std::ops::Deref;

use crate::{interval::Interval, vec3::Vec3};

pub struct Color(pub Vec3);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self(Vec3::new(r, g, b))
    }

    pub fn to_gamma_space(&self) -> Self {
        Self(Vec3::new(
            Self::linear_to_gamma(self.x),
            Self::linear_to_gamma(self.y),
            Self::linear_to_gamma(self.z),
        ))
    }

    fn linear_to_gamma(component: f64) -> f64 {
        if component > 0.0 {
            component.sqrt()
        } else {
            0.0
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        static INTENSITY: Interval = Interval {
            min: 0.000,
            max: 0.999,
        };

        let color = self.to_gamma_space();

        let r: u8 = (256.0 * INTENSITY.clamp(color.x)) as u8;
        let g: u8 = (256.0 * INTENSITY.clamp(color.y)) as u8;
        let b: u8 = (256.0 * INTENSITY.clamp(color.z)) as u8;

        write!(f, "{} {} {}\n", r, g, b)
    }
}

impl Deref for Color {
    type Target = Vec3;

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

impl std::ops::AddAssign<Self> for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.0.x += rhs.x;
        self.0.y += rhs.y;
        self.0.z += rhs.z;
    }
}
