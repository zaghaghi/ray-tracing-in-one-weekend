use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Color,
}

pub trait Material {
    fn scatter(&self, _ray: &Ray, _hit: &HitRecord) -> Option<Scatter> {
        None
    }
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<Scatter> {
        let scatter_direction = &hit.normal + &Vec3::random_unit();
        let scatter_direction = if scatter_direction.near_zero() {
            hit.normal.clone()
        } else {
            scatter_direction
        };
        let scattered = Ray::new(hit.point.clone(), scatter_direction);
        Some(Scatter {
            ray: scattered,
            attenuation: self.albedo.clone(),
        })
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<Scatter> {
        let reflected = Vec3::reflect(&ray.direction, &hit.normal);
        let scattered = Ray::new(hit.point.clone(), reflected);
        Some(Scatter {
            ray: scattered,
            attenuation: self.albedo.clone(),
        })
    }
}
