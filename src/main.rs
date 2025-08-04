use std::sync::Arc;

use camera::Camera;
use color::Color;
use hittable_list::HittableList;
use material::{Dielectric, Lambertian, Metal};
use sphere::Sphere;
use vec3::Point3;

pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod vec3;

fn main() {
    // World
    let mut world = HittableList::default();
    let ground = Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Lambertian::new(Color::new(0.8, 0.8, 0.0)),
    );

    let center = Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        Lambertian::new(Color::new(0.1, 0.2, 0.5)),
    );

    let left = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, Dielectric::new(1.5));

    let bubble = Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        Dielectric::new(1.0 / 1.5),
    );

    let right = Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        Metal::new(Color::new(0.8, 0.6, 0.2), 1.0),
    );

    world.add(Arc::new(ground));
    world.add(Arc::new(center));
    world.add(Arc::new(left));
    world.add(Arc::new(bubble));
    world.add(Arc::new(right));

    // Camera
    let camera = Camera::new(16.0 / 9.0, 800, 800);
    camera.render(Arc::new(world));
}
