#![allow(dead_code)]
mod ray;
mod vec3;
use ray::Ray;
use vec3::{Color, Vec3};

use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

fn ray_color(r: Ray) -> Color {
    let unit_direction = Vec3::unit_vector(r.direction);

    let t = 0.5 * (unit_direction.y + 1.0);

    (1.0 - t) * Color::one() + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let path = std::path::Path::new("output/book1/image2.png");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    // Image config
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    let width: u32 = 400;
    let height: u32 = ((width as f64) / ASPECT_RATIO) as u32;

    // Camera config
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::zero();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, -viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // different from the book, we use image crate to create a .png image rather than outputting .ppm file, which is not widely used.
    // anyway, you may output any image format you like.
    let mut img: RgbImage = ImageBuffer::new(width, height);

    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };

    for j in (0..height).rev() {
        for i in 0..width {
            let pixel = img.get_pixel_mut(i, j);
            let u: f64 = (i as f64) / ((width - 1) as f64);
            let v: f64 = (j as f64) / ((height - 1) as f64);
            let direction = lower_left_corner + u * horizontal + v * vertical - origin;
            let ray = Ray::new(origin, direction);
            let color = ray_color(ray).to_u64();
            *pixel = image::Rgb([color.0 as u8, color.1 as u8, color.2 as u8]);
        }
        progress.inc(1);
    }
    progress.finish();

    println!(
        "Output image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    img.save(path).expect("Cannot save the image to the file");
}
