use std::sync::Arc;

use crate::{
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
    pub time: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        hit_point: Point3,
        hit_time: f64,
        outward_normal: Vec3,
        ray: &Ray,
        material: Arc<dyn Material>,
    ) -> Self {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            point: hit_point,
            normal,
            material,
            time: hit_time,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord>;
}
