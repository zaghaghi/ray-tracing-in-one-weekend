use std::{cmp::Ordering, sync::Arc};

use crate::hittable::{HitRecord, Hittable};

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new(objects: Vec<Arc<dyn Hittable>>) -> Self {
        Self { objects }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &crate::ray::Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        self.objects
            .iter()
            .filter_map(|object| object.hit(ray, ray_tmin, ray_tmax))
            .min_by(|x, y| {
                if x.time < y.time {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            })
    }
}
