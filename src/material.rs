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
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<Scatter> {
        let reflected = Vec3::reflect(&ray.direction, &hit.normal);
        let reflected = reflected.unit() + (Vec3::random_unit() * self.fuzz);
        let scattered = Ray::new(hit.point.clone(), reflected);
        if scattered.direction.dot(&hit.normal) > 0.0 {
            Some(Scatter {
                ray: scattered,
                attenuation: self.albedo.clone(),
            })
        } else {
            None
        }
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<Scatter> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if hit.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray.direction.unit();
        let cos_theta = (-unit_direction.clone()).dot(&hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refrect = ri * sin_theta > 1.0;
        let direction = if cannot_refrect {
            Vec3::reflect(&unit_direction, &hit.normal)
        } else {
            Vec3::refract(&unit_direction, &hit.normal, ri)
        };

        Some(Scatter {
            ray: Ray::new(hit.point.clone(), direction),
            attenuation,
        })
    }
}
