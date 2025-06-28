#![allow(dead_code)]
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod ray;
pub mod rtweekend;
pub mod sphere;
pub mod vec3;

use std::rc::Rc;

use color::Color;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use ray::Ray;
use sphere::Sphere;
use vec3::{Point3, Vec3};

use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::default();
    if world.hit(r, 0.0, rtweekend::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::one());
    }

    let unit_direction = vec3::unit_vector(r.direction);

    let t = 0.5 * (unit_direction.y + 1.0);

    // 当t为0时，白色，将t为1时，蓝色
    (1.0 - t) * Color::one() + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let path = std::path::Path::new("output/book1/image5.png");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    // Image config
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = ((image_width as f64) / ASPECT_RATIO) as u32;
    let image_height = if image_height < 1 { 1 } else { image_height };

    // World
    let mut world = HittableList::default();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera config
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let camera_center = Vec3::zero();

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left =
        camera_center - viewport_u / 2.0 - viewport_v / 2.0 - Vec3::new(0.0, 0.0, focal_length);
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // different from the book, we use image crate to create a .png image rather than outputting .ppm file, which is not widely used.
    // anyway, you may output any image format you like.
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);

    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((image_height * image_width) as u64)
    };

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel = img.get_pixel_mut(i, j);
            //let u: f64 = (i as f64) / ((width - 1) as f64);
            //let v: f64 = (j as f64) / ((height - 1) as f64);
            let ray_direction = pixel00_loc + i as f64 * pixel_delta_u + j as f64 * pixel_delta_v;
            let r = Ray::new(camera_center, ray_direction);
            let color = ray_color(&r, &world).to_u64();
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
