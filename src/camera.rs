use std::sync::Arc;

use crate::{
    color::Color,
    hittable::Hittable,
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Camera {
    image_width: i32,
    image_height: i32,
    samples_per_pixel: i32,
    max_depth: i32,
    pixel_samples_scale: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32, samples_per_pixel: i32) -> Self {
        let max_depth = 50;
        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;
        let image_height = (image_width as f64 / aspect_ratio) as i32;
        let image_height = if image_height < 1 { 1 } else { image_height };

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let center = Point3::new(0.0, 0.0, 0.0);
        let focal_point = Point3::new(0.0, 0.0, focal_length);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = &viewport_u / image_width as f64;
        let pixel_delta_v = &viewport_v / image_height as f64;

        let mut viewport_upper_left = &center - &focal_point;
        viewport_upper_left -= &viewport_u / 2.0;
        viewport_upper_left -= &viewport_v / 2.0;

        let pixel00_loc = &pixel_delta_u * 0.5 + &pixel_delta_v * 0.5 + &viewport_upper_left;
        Self {
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
            pixel_samples_scale,
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
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += Camera::ray_color(&ray, world.clone(), self.max_depth);
                }
                let pixel_color = pixel_color * self.pixel_samples_scale;
                print!("{pixel_color}");
                pb.inc(1);
            }
        }
        pb.finish_with_message("done");
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = Camera::sample_square();
        let pixel_sample = &self.pixel_delta_u * (i as f64 + offset.x)
            + &self.pixel_delta_v * (j as f64 + offset.y)
            + &self.pixel00_loc;

        let ray_direction = &pixel_sample - &self.center;
        Ray::new(self.center.clone(), ray_direction)
    }

    fn sample_square() -> Vec3 {
        Vec3::new(
            rand::random::<f64>() - 0.5,
            rand::random::<f64>() - 0.5,
            0.0,
        )
    }

    fn ray_color(ray: &Ray, world: Arc<dyn Hittable>, depth: i32) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        if let Some(hit) = world.hit(ray, &Interval::new(0.001, f64::INFINITY)) {
            return if let Some(scatter) = hit.material.scatter(ray, &hit) {
                scatter.attenuation * Camera::ray_color(&scatter.ray, world, depth - 1)
            } else {
                Color::new(0.0, 0.0, 0.0)
            };
        }
        let unit_direction = ray.direction.clone().unit();
        let a = 0.5 * unit_direction.y + 1.0;
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }
}
