use std::sync::Arc;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::Point3,
};

pub struct Sphere<T: Material> {
    pub center: Point3,
    pub radius: f64,
    pub material: Arc<T>,
}

impl<T> Sphere<T>
where
    T: Material,
{
    pub fn new(center: Point3, radius: f64, material: T) -> Self {
        Self {
            center,
            radius: f64::max(radius, 0.0),
            material: Arc::new(material),
        }
    }
}

impl<T> Hittable for Sphere<T>
where
    T: Material + 'static,
{
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<crate::hittable::HitRecord> {
        let oc = &self.center - &ray.origin;
        let a = ray.direction.len_squared();
        let h = ray.direction.dot(&oc);
        let c = oc.len_squared() - self.radius * self.radius;

        let d = h * h - a * c;
        if d < 0.0 {
            return None;
        }

        let dsqrt = d.sqrt();

        let root = (h - dsqrt) / a;
        let root = if !interval.surrounds(root) {
            (h + dsqrt) / a
        } else {
            root
        };

        if !interval.surrounds(root) {
            return None;
        }

        let time = root;
        let point = ray.at(time);
        let normal = (&point - &self.center) / self.radius;
        Some(HitRecord::new(
            point,
            time,
            normal,
            &ray,
            self.material.clone(),
        ))
    }
}
