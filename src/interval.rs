use std::f64;

#[derive(Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
        }
    }
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        f64::max(self.min, f64::min(self.max, x))
    }

    pub fn empty() -> &'static Interval {
        static EMPTY: Interval = Interval {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        };
        &EMPTY
    }

    pub fn universe() -> &'static Interval {
        static UNIVERSE: Interval = Interval {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
        };
        &UNIVERSE
    }
}
