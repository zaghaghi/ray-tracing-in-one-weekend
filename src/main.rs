use color::Color;

pub mod color;
pub mod vec3;

fn main() {
    let image_width = 256;
    let image_height = 256;
    let pb = indicatif::ProgressBar::new(image_width * image_height);

    println!("P3\n{image_width} {image_height}\n255");

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_color = Color::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.0f64,
            );

            print!("{pixel_color}");
            pb.inc(1);
        }
    }
    pb.finish_with_message("done");
}
