#![allow(dead_code)]
pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod material;
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

    let r = (rtweekend::PI / 4.0).cos();
    let material_left: Rc<dyn material::Material> =
        Rc::new(material::Lambertian::new(color::Color::new(0.0, 0.0, 1.0)));
    let material_right: Rc<dyn material::Material> =
        Rc::new(material::Lambertian::new(color::Color::new(1.0, 0.0, 0.0)));
    world.add(Rc::new(Sphere::new(
        Point3::new(-r, 0.0, -1.0),
        r,
        material_left,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(r, 0.0, -1.0),
        r,
        material_right,
    )));

    // Camera
    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 90.0;

    // Render
    cam.render(&world);
}
