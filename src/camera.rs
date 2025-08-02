use std::sync::Arc;

use crate::{
    color::Color,
    hittable::Hittable,
    interval::Interval,
    ray::Ray,
    vec3::{DoubleVec3, Point3},
};

pub struct Camera {
    image_width: i32,
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: DoubleVec3,
    pixel_delta_v: DoubleVec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32) -> Self {
        let image_height = (image_width as f64 / aspect_ratio) as i32;
        let image_height = if image_height < 1 { 1 } else { image_height };

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let center = Point3::new(0.0, 0.0, 0.0);
        let focal_point = Point3::new(0.0, 0.0, focal_length);

        let viewport_u = DoubleVec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = DoubleVec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = &viewport_u / image_width as f64;
        let pixel_delta_v = &viewport_v / image_height as f64;

        let mut viewport_upper_left = &center - &focal_point;
        viewport_upper_left -= &viewport_u / 2.0;
        viewport_upper_left -= &viewport_v / 2.0;

        let pixel00_loc = &pixel_delta_u * 0.5 + &pixel_delta_v * 0.5 + &viewport_upper_left;
        Self {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: Arc<dyn Hittable>) {
        let pb = indicatif::ProgressBar::new((self.image_width * self.image_height) as u64);

        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let pixel_center = &self.pixel_delta_u * i as f64
                    + &self.pixel_delta_v * j as f64
                    + &self.pixel00_loc;
                let ray_direction = &pixel_center - &self.center;
                let ray = Ray::new(self.center.clone(), ray_direction);
                let pixel_color = Camera::ray_color(&ray, world.clone());
                print!("{pixel_color}");
                pb.inc(1);
            }
        }
        pb.finish_with_message("done");
    }

    fn ray_color(ray: &Ray, world: Arc<dyn Hittable>) -> Color {
        if let Some(hit) = world.hit(ray, &Interval::new(0.0, f64::INFINITY)) {
            return Color::new(hit.normal.x + 1.0, hit.normal.y + 1.0, hit.normal.z + 1.0) * 0.5;
        }
        let unit_direction = ray.direction.clone().unit();
        let a = 0.5 * unit_direction.y + 1.0;
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }
}
