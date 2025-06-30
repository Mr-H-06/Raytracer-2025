use std::rc::Rc;

use super::hittable::{HitRecord, Hittable};
use super::material::Material;
use super::ray::Ray;
use super::vec3::{self, Point3, Vec3};
use crate::interval::Interval;

pub struct Sphere {
    center: Ray,
    radius: f64,
    mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(static_center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            center: Ray::new(static_center, Vec3::zero()),
            radius,
            mat: material,
        }
    }

    pub fn new_with_center2(
        center1: Point3,
        center2: Point3,
        radius: f64,
        material: Rc<dyn Material>,
    ) -> Self {
        Self {
            center: Ray::new(center1, center2 - center1),
            radius,
            mat: material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval, hit_record: &mut HitRecord) -> bool {
        let current_center = self.center.at(r.time());
        let oc = current_center - r.origin();
        let a = r.direction().length_squared();
        let h = vec3::dot(r.direction(), oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if root <= ray_t.min || ray_t.max <= root {
            root = (h + sqrtd) / a;
            if root <= ray_t.min || ray_t.max <= root {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.p = r.at(hit_record.t);
        let outward_normal = (hit_record.p - current_center) / self.radius;
        hit_record.set_face_normal(r, outward_normal);
        hit_record.mat = Some(Rc::clone(&self.mat));

        true
    }
}
