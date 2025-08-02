use crate::{
    ray::Ray,
    vec3::{DoubleVec3, Point3},
};

pub struct HitRecord {
    pub point: Point3,
    pub normal: DoubleVec3,
    pub time: f64,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}
