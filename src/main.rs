fn main() {
    let image_width = 256;
    let image_height = 256;
    let pb = indicatif::ProgressBar::new(image_width * image_height);

    println!("P3\n{image_width} {image_height}\n255");

    for j in 0..image_height {
        for i in 0..image_width {
            let ir = (255.999f32 * i as f32 / (image_width - 1) as f32) as u8;
            let ig = (255.999f32 * j as f32 / (image_height - 1) as f32) as u8;
            let ib = 0.0f32;

            println!("{ir} {ig} {ib}");
            pb.inc(1);
        }
    }
    pb.finish_with_message("done");
}
