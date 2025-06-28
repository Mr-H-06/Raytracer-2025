#![allow(dead_code)]
pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod ray;
pub mod rtweekend;
pub mod sphere;
pub mod vec3;

use std::rc::Rc;

use crate::camera::Camera;
use hittable_list::HittableList;
use sphere::Sphere;
use vec3::Point3;

fn main() {
    // World
    let mut world = HittableList::default();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.render(&world);
}
