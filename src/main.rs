use std::sync::Arc;

use camera::Camera;
use hittable_list::HittableList;
use sphere::Sphere;
use vec3::Point3;

pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod ray;
pub mod sphere;
pub mod vec3;

fn main() {
    // World
    let mut world = HittableList::default();
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let camera = Camera::new(16.0 / 9.0, 400);
    camera.render(Arc::new(world));
}
