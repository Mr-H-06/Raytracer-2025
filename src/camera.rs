use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

use super::color::Color;
use super::hittable::{HitRecord, Hittable};
use super::interval::Interval;
use super::material;
use super::pdf;
use super::pdf::{HittablePdf, Pdf};
use super::ray::Ray;
use super::rtweekend;
use super::vec3::{self, Point3, Vec3};
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: usize,
    pub max_depth: i32,
    pub background: Color,
    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    image_height: u32,
    sqrt_spp: usize,
    recip_sqrt_spp: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn render(&mut self, world: &dyn Hittable, lights: &dyn Hittable) {
        self.initialize();

        let path = std::path::Path::new("output/book3/image12.png");
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

        let mut img: RgbImage = ImageBuffer::new(self.image_width, self.image_height);

        let progress = if option_env!("CI").unwrap_or_default() == "true" {
            ProgressBar::hidden()
        } else {
            ProgressBar::new((self.image_height * self.image_width) as u64)
        };

        // different from the book, we use image crate to create a .png image rather than outputting .ppm file, which is not widely used.
        // anyway, you may output any image format you like.
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color = Color::default();

                /*for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, self.max_depth, world);
                }*/
                for s_j in 0..self.sqrt_spp {
                    for s_i in 0..self.sqrt_spp {
                        let r = self.get_ray(i, j, s_i as u32, s_j as u32);
                        pixel_color += self.ray_color(&r, self.max_depth, world, lights);
                    }
                }

                let pixel = img.get_pixel_mut(i, j);
                *pixel = pixel_color.to_color(self.samples_per_pixel);
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

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.sqrt_spp = (self.samples_per_pixel as f64).sqrt() as usize;
        self.recip_sqrt_spp = 1.0 / (self.sqrt_spp as f64);

        self.center = self.lookfrom;

        // 确认视口的大小。
        let theta = rtweekend::degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        self.w = vec3::unit_vector(self.lookfrom - self.lookat);
        self.u = vec3::unit_vector(vec3::cross(self.vup, self.w));
        self.v = vec3::cross(self.w, self.u);

        // 计算水平和垂直视口边缘上的向量。
        let viewport_u = self.u * viewport_width;
        let viewport_v = -self.v * viewport_height;

        // 计算从像素到像素的水平和垂直增量向量。
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // 计算左上角像素的位置。
        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - (0.5 * viewport_u) - (0.5 * viewport_v);
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
        let defocus_radius =
            self.focus_dist * rtweekend::degrees_to_radians(self.defocus_angle / 2.0).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn ray_color(&self, r: &Ray, depth: i32, world: &dyn Hittable, lights: &dyn Hittable) -> Color {
        let mut rec = HitRecord::default();

        if depth <= 0 {
            return Color::default();
        }

        if !world.hit(r, &Interval::new(0.001, rtweekend::INFINITY), &mut rec) {
            return self.background;
        }
        if let Some(mat) = rec.mat.clone() {
            let mut srec = material::ScatterRecord::default();
            let color_from_emission = mat.emitted(r, &rec, rec.u, rec.v, rec.p);
            if !mat.scatter(r, &rec, &mut srec) {
                return color_from_emission;
            }
            if srec.skip_pdf {
                return srec.attenuation
                    * self.ray_color(&srec.skip_pdf_ray, depth - 1, world, lights);
            }
            let light_pdf = HittablePdf::new(lights, rec.p);
            let mixed_pdf = pdf::MixturePdf::new(&light_pdf, &*srec.pdf);

            let scattered = Ray::new_with_time(rec.p, mixed_pdf.generate(), r.time());
            let pdf = mixed_pdf.value(scattered.direction());

            let scattering_pdf = mat.scattering_pdf(r, &rec, &scattered);

            let color_from_scatter = (srec.attenuation
                * scattering_pdf
                * self.ray_color(&scattered, depth - 1, world, lights))
                / pdf;

            color_from_emission + color_from_scatter
        } else {
            Color::default()
        }
    }

    fn get_ray(&self, i: u32, j: u32, s_i: u32, s_j: u32) -> Ray {
        let pixel_center =
            self.pixel00_loc + i as f64 * self.pixel_delta_u + j as f64 * self.pixel_delta_v;
        let pixel_sample = pixel_center + self.pixel_sample_square(s_i, s_j);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = rtweekend::random_double();

        Ray::new_with_time(ray_origin, ray_direction, ray_time)
    }

    fn pixel_sample_square(&self, s_i: u32, s_j: u32) -> Vec3 {
        let px = -0.5 + self.recip_sqrt_spp * (s_i as f64 + rtweekend::random_double());
        let py = -0.5 + self.recip_sqrt_spp * (s_j as f64 + rtweekend::random_double());
        px * self.pixel_delta_u + py * self.pixel_delta_v
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = vec3::random_in_unit_disk();
        self.center + p.x() * self.defocus_disk_u + p.y() * self.defocus_disk_v
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            image_height: 0,
            samples_per_pixel: 10,
            max_depth: 10,
            background: Color::default(),
            vfov: 90.0,
            lookfrom: Point3::new(0.0, 0.0, -1.0),
            lookat: Point3::new(0.0, 0.0, 0.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            sqrt_spp: 10.0_f64.sqrt() as usize,
            recip_sqrt_spp: 1.0 / (10.0_f64.sqrt()),
            center: Point3::default(),
            pixel00_loc: Point3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
            u: Vec3::default(),
            v: Vec3::default(),
            w: Vec3::default(),
            defocus_disk_u: Vec3::default(),
            defocus_disk_v: Vec3::default(),
        }
    }
}
