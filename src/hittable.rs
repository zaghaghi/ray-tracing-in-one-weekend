use crate::{
    ray::Ray,
    vec3::{DoubleVec3, Point3},
};

pub struct HitRecord {
    pub point: Point3,
    pub normal: DoubleVec3,
    pub time: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(hit_point: Point3, hit_time: f64, outward_normal: DoubleVec3, ray: &Ray) -> Self {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            point: hit_point,
            normal,
            time: hit_time,
            front_face,
        }
    }
}
pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}
