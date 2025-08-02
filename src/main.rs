use color::Color;
use ray::Ray;
use vec3::{DoubleVec3, Point3};

pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod ray;
pub mod sphere;
pub mod vec3;

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = center - &ray.origin;
    let a = ray.direction.len_squared();
    let h = ray.direction.dot(&oc);
    let c = oc.len_squared() - radius * radius;
    let d = h * h - a * c;
    if d < 0.0 { -1.0 } else { (h - d.sqrt()) / a }
}

fn ray_color(ray: &Ray) -> Color {
    let t = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let n = (ray.at(t) - DoubleVec3::new(0.0, 0.0, -1.0)).unit();
        return Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0) * 0.5;
    }
    let unit_direction = ray.direction.clone().unit();
    let a = 0.5 * unit_direction.y + 1.0;
    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}

fn main() {
    // Image
    let aspec_ratio = 16.0 / 9.0;
    let image_width = 400;

    let image_height = (image_width as f64 / aspec_ratio) as i32;
    let image_height = if image_height < 1 { 1 } else { image_height };

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);
    let focal_point = Point3::new(0.0, 0.0, focal_length);

    let viewport_u = DoubleVec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = DoubleVec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = &viewport_u / image_width as f64;
    let pixel_delta_v = &viewport_v / image_height as f64;

    let mut viewport_upper_left = &camera_center - &focal_point;
    viewport_upper_left -= &viewport_u / 2.0;
    viewport_upper_left -= &viewport_v / 2.0;

    let pixel00_loc = &pixel_delta_u * 0.5 + &pixel_delta_v * 0.5 + &viewport_upper_left;

    let pb = indicatif::ProgressBar::new((image_width * image_height) as u64);

    println!("P3\n{image_width} {image_height}\n255");

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center = &pixel_delta_u * i as f64 + &pixel_delta_v * j as f64 + &pixel00_loc;
            let ray_direction = &pixel_center - &camera_center;
            let ray = Ray::new(camera_center.clone(), ray_direction);
            let pixel_color = ray_color(&ray);
            print!("{pixel_color}");
            pb.inc(1);
        }
    }
    pb.finish_with_message("done");
}
