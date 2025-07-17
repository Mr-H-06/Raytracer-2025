#![allow(dead_code)]
pub mod aabb;
pub mod bvh;
pub mod camera;
pub mod color;
pub mod constant_medium;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod material;
pub mod model;
pub mod onb;
pub mod pdf;
pub mod perlin;
pub mod quad;
pub mod ray;
pub mod rtw_stb_image;
pub mod rtweekend;
pub mod sphere;
pub mod texture;
pub mod triangle;
pub mod vec3;

use std::sync::Arc;

//use crate::model::load_model;
use crate::bvh::BvhNode;
use crate::camera::Camera;
use crate::hittable::{Hittable, RotateX, RotateY, Scale, Translate};
use crate::model::load_model;
use crate::texture::{ImageTexture, NoiseTexture};
use crate::vec3::{Point3, Vec3};
use color::Color;
use hittable_list::HittableList;
use material::{Dielectric, DiffuseLight, Lambertian, Metal};
use quad::Quad;
use sphere::Sphere;

fn final_scene(image_width: u32, samples_per_pixel: usize, max_depth: i32) {
    let mut boxes1 = HittableList::default();
    let ground = Lambertian::new(Color::new(0.48, 0.83, 0.53));

    let boxes_per_side = 20;
    (0..boxes_per_side).for_each(|i| {
        (0..boxes_per_side).for_each(|j| {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rtweekend::random_double_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Arc::new(quad::make_box(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            )));
        });
    });

    let mut world = HittableList::default();

    world.add(Arc::new(BvhNode::new(&mut boxes1)));

    let light = DiffuseLight::new_with_color(Color::new(7.0, 7.0, 7.0));
    world.add(Arc::new(Quad::new(
        Point3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light.clone(),
    )));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_material = Lambertian::new(Color::new(0.7, 0.3, 0.1));
    world.add(Arc::new(Sphere::new_with_center2(
        center1,
        center2,
        50.0,
        sphere_material.clone(),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Dielectric::new(1.5),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Metal::new(Color::new(0.8, 0.8, 0.9), 1.0),
    )));

    let boundary = Sphere::new(Point3::new(360.0, 150.0, 145.0), 70.0, Dielectric::new(1.5));
    world.add(Arc::new(boundary.clone()));
    world.add(Arc::new(constant_medium::ConstantMedium::new_with_color(
        boundary.clone(),
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));
    let boundary = Sphere::new(Point3::new(0.0, 0.0, 0.0), 5000.0, Dielectric::new(1.5));
    world.add(Arc::new(constant_medium::ConstantMedium::new_with_color(
        boundary.clone(),
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    )));

    let emat = Lambertian::new_with_texture(ImageTexture::new("earthmap.jpg"));
    world.add(Arc::new(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));
    let pertext = NoiseTexture::new(0.2);
    world.add(Arc::new(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Lambertian::new_with_texture(pertext),
    )));

    /*let mut boxes2 = HittableList::default();
    let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
    let ns = 1000;
    (0..ns).for_each(|_| {
        boxes2.add(Arc::new(Sphere::new(
            Point3::random_range(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    });

    world.add(Arc::new(hittable::Translate::new(
        hittable::RotateY::new(BvhNode::new(&mut boxes2), 15.0),
        Vec3::new(-100.0, 270.0, 395.0),
    )));*/

    let mut lights = HittableList::default();
    lights.add(Arc::new(Quad::new(
        Point3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light,
    )));
    //lights.add(boundary);

    let mut cam = Camera::default();

    cam.aspect_ratio = 1.0;
    cam.image_width = image_width;
    cam.samples_per_pixel = samples_per_pixel;
    cam.max_depth = max_depth;
    cam.background = Color::default();

    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(478.0, 278.0, -600.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(Arc::new(world), Arc::new(lights));
}

fn attempt(image_width: u32, samples_per_pixel: usize, max_depth: i32) {
    let mut world = HittableList::new();

    //let ground = Lambertian::new_with_texture(ImageTexture::new("wood.jpg"));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, 0.0, 2.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        Lambertian::new(Color::new(0.2, 0.2, 0.2)), //ground
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, 0.0, -1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        Lambertian::new(Color::new(0.8, 0.8, 0.7)), //wall
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, 0.0, 2.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        Dielectric::new(1.5), //left
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(2.0, 0.0, -2.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        Dielectric::new(1.5), //right
    )));

    let model_material = Lambertian::new(Color::new(0.8, 0.85, 0.9));
    let mut model_triangles = load_model("images/2/week_6.obj", model_material);
    let model_bvh = BvhNode::new(&mut model_triangles);

    let target_factor = 8.0;
    let scaled_model = Scale::new(
        model_bvh,
        Vec3::new(target_factor, target_factor, target_factor),
    );
    let rotated_model = RotateY::new(scaled_model, 30.0);
    let final_bbox = rotated_model.bounding_box();
    let target_position = Point3::new(2.5, 0.0, 2.5);
    let translation_vec = target_position
        - Vec3::new(
            (final_bbox.x.min + final_bbox.x.max) / 2.0,
            final_bbox.y.min,
            (final_bbox.z.min + final_bbox.z.max) / 2.0,
        );
    let final_model = Translate::new(rotated_model, translation_vec);
    world.add(Arc::new(final_model));

    let light_material = DiffuseLight::new_with_color(Color::new(10.0, 10.0, 10.0));

    world.add(Arc::new(Quad::new(
        Point3::new(-0.5, 3.0, -0.5),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        light_material.clone(),
    )));
    let mut lights = HittableList::default();
    lights.add(Arc::new(Quad::new(
        Point3::new(-0.5, 3.0, -0.5),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        light_material.clone(),
    )));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = image_width;
    cam.samples_per_pixel = samples_per_pixel;
    cam.max_depth = max_depth;
    cam.background = Color::new(0.35, 0.4, 0.5);

    cam.vfov = 28.0;
    cam.lookfrom = Point3::new(0.0, 1.5, 4.0);
    cam.lookat = Point3::new(0.0, 0.5, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.5;
    cam.focus_dist = (cam.lookfrom - cam.lookat).length();

    cam.render(Arc::new(world), Arc::new(lights));
}

fn scene(image_width: u32, samples_per_pixel: usize, max_depth: i32) {
    let mut world = HittableList::new();
    let mut lights = HittableList::default();

    let ground = Lambertian::new_with_texture(ImageTexture::new("wood.jpg"));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, 0.0, 2.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        ground,
        //Lambertian::new(Color::new(0.2, 0.2, 0.2)), //ground
    )));

    let wall = Lambertian::new_with_texture(ImageTexture::new("back.jpg"));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, 0.0, -2.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        wall,
        //Lambertian::new(Color::new(0.2, 0.2, 0.2)), //wall
    )));
    /*lights.add(Arc::new(Quad::new(
        Point3::new(-2.0, 0.0, -1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        Metal::new(Color::new(1.0, 1.0, 1.0), 0.2), //wall
    )));*/

    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, 0.0, 2.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        Dielectric::new(1.5), //left
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(2.0, 0.0, -2.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        Dielectric::new(1.5), //right
    )));
    //--------------------------------------------------------------------------
    let model_material = Lambertian::new(Color::new(0.8, 0.85, 0.9));
    let mut model_triangles = load_model("images/2/week_6.obj", model_material);
    let model_bvh = BvhNode::new(&mut model_triangles);
    let target_factor = 10.0;
    let scaled_model = Scale::new(
        model_bvh,
        Vec3::new(target_factor, target_factor, target_factor),
    );
    let rotated_model = RotateY::new(scaled_model, 30.0);

    let final_bbox = rotated_model.bounding_box();
    let target_position = Point3::new(-1.3, -0.55, -1.3);
    let translation_vec = target_position
        - Vec3::new(
            (final_bbox.x.min + final_bbox.x.max) / 2.0,
            final_bbox.y.min,
            (final_bbox.z.min + final_bbox.z.max) / 2.0,
        );
    let final_model = Translate::new(rotated_model, translation_vec);
    world.add(Arc::new(final_model));
    //--------------------------------------------------------------------------
    let model_material = Lambertian::new(Color::new(0.8, 0.85, 0.9));
    let mut model_triangles = load_model("images/3/Cactus.obj", model_material);
    let model_bvh = BvhNode::new(&mut model_triangles);
    let target_factor = 0.11;
    let scaled_model = Scale::new(
        model_bvh,
        Vec3::new(target_factor, target_factor, target_factor),
    );
    let rotated_model = RotateX::new(scaled_model, 90.0);
    let rotated_model = RotateY::new(rotated_model, 55.0);

    let final_bbox = rotated_model.bounding_box();
    let target_position = Point3::new(1.7, -0.3, -1.3);
    let translation_vec = target_position
        - Vec3::new(
            (final_bbox.x.min + final_bbox.x.max) / 2.0,
            final_bbox.y.min,
            (final_bbox.z.min + final_bbox.z.max) / 2.0,
        );
    let final_model = Translate::new(rotated_model, translation_vec);
    world.add(Arc::new(final_model));
    //--------------------------------------------------------------------------
    let model_material = Lambertian::new(Color::new(0.8, 0.85, 0.9));
    let mut model_triangles = load_model("images/4/coke.obj", model_material);
    let model_bvh = BvhNode::new(&mut model_triangles);
    let target_factor = 0.1;
    let scaled_model = Scale::new(
        model_bvh,
        Vec3::new(target_factor, target_factor, target_factor),
    );
    let rotated_model = RotateY::new(scaled_model, -25.0);

    let final_bbox = rotated_model.bounding_box();
    let target_position = Point3::new(1.5, 0.2, 0.4);
    let translation_vec = target_position
        - Vec3::new(
            (final_bbox.x.min + final_bbox.x.max) / 2.0,
            final_bbox.y.min,
            (final_bbox.z.min + final_bbox.z.max) / 2.0,
        );
    let final_model = Translate::new(rotated_model, translation_vec);
    world.add(Arc::new(final_model));
    //--------------------------------------------------------------------------
    let model_material = Lambertian::new(Color::new(0.8, 0.85, 0.9));
    let mut model_triangles = load_model("images/4/coke.obj", model_material);
    let model_bvh = BvhNode::new(&mut model_triangles);
    let target_factor = 0.1;
    let scaled_model = Scale::new(
        model_bvh,
        Vec3::new(target_factor, target_factor, target_factor),
    );
    let rotated_model = RotateY::new(scaled_model, 25.0);

    let final_bbox = rotated_model.bounding_box();
    let target_position = Point3::new(1.3, 0.2, 0.2);
    let translation_vec = target_position
        - Vec3::new(
            (final_bbox.x.min + final_bbox.x.max) / 2.0,
            final_bbox.y.min,
            (final_bbox.z.min + final_bbox.z.max) / 2.0,
        );
    let final_model = Translate::new(rotated_model, translation_vec);
    world.add(Arc::new(final_model));
    //--------------------------------------------------------------------------
    let model_material = Lambertian::new(Color::new(0.8, 0.85, 0.9));
    let mut model_triangles = load_model("images/5/6.obj", model_material);
    let model_bvh = BvhNode::new(&mut model_triangles);

    let target_factor = 0.004;
    let scaled_model = Scale::new(
        model_bvh,
        Vec3::new(target_factor, target_factor, target_factor),
    );
    let rotated_model = scaled_model; //RotateY::new(scaled_model, -25.0);

    let final_bbox = rotated_model.bounding_box();
    let target_position = Point3::new(0.08, -0.8, 0.05);
    let translation_vec = target_position
        - Vec3::new(
            (final_bbox.x.min + final_bbox.x.max) / 2.0,
            final_bbox.y.min,
            (final_bbox.z.min + final_bbox.z.max) / 2.0,
        );
    let final_model = Translate::new(rotated_model, translation_vec);
    world.add(Arc::new(final_model));

    world.add(Arc::new(quad::make_box(
        Point3::new(-1.6, 0.0, 0.5),
        Point3::new(-1.3, 0.3, 0.2),
        Metal::new(Color::new(0.4, 0.4, 0.45), 0.0),
        //Lambertian::new(Color::new(0.1, 0.2, 0.5)),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-1.45, 0.5, 0.35),
        0.2,
        Dielectric::new(1.4),
    )));
    let light_material = DiffuseLight::new_with_color(Color::new(10.0, 10.0, 10.0));

    world.add(Arc::new(Quad::new(
        Point3::new(-0.5, 3.0, -0.5),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        light_material.clone(),
    )));
    lights.add(Arc::new(Quad::new(
        Point3::new(-0.5, 3.0, -0.5),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        light_material.clone(),
    )));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = image_width;
    cam.samples_per_pixel = samples_per_pixel;
    cam.max_depth = max_depth;
    cam.background = Color::new(0.21, 0.27, 0.31);

    cam.vfov = 28.0;
    cam.lookfrom = Point3::new(0.0, 1.5, 4.0);
    cam.lookat = Point3::new(0.0, 0.5, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.5;
    cam.focus_dist = (cam.lookfrom - cam.lookat).length();

    cam.render(Arc::new(world), Arc::new(lights));
}

fn main() {
    scene(600, 300, 20);
    //attempt(400, 100, 10);
    //attempt(800, 500, 50);
    //final_scene(800, 100, 10);
}
