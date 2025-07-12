use std::sync::Arc;

use super::aabb::Aabb;
use super::interval::Interval;
use super::material::Material;
use super::ray::Ray;
use super::rtweekend;
use super::vec3::{self, Point3, Vec3};

#[derive(Clone, Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Option<Arc<dyn Material>>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, ray_t: &Interval, hit_record: &mut HitRecord) -> bool;
    fn bounding_box(&self) -> &Aabb;
    fn pdf_value(&self, _origin: Point3, _direction: Vec3) -> f64 {
        0.0
    }
    fn random(&self, _origin: Point3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = vec3::dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub struct Translate<T: Hittable> {
    object: T,
    offset: Vec3,
    bbox: Aabb,
}

impl<T: Hittable> Translate<T> {
    pub fn new(object: T, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;
        Self {
            object,
            offset,
            bbox,
        }
    }
}

impl<T: Hittable> Hittable for Translate<T> {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let offset_r = Ray::new_with_time(r.origin() - self.offset, r.direction(), r.time());

        if !self.object.hit(&offset_r, ray_t, rec) {
            return false;
        }

        rec.p += self.offset;

        true
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

pub struct RotateY<T: Hittable> {
    object: T,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl<T: Hittable> RotateY<T> {
    pub fn new(p: T, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = p.bounding_box();
        let mut min = Point3::new(
            rtweekend::INFINITY,
            rtweekend::INFINITY,
            rtweekend::INFINITY,
        );
        let mut max = Point3::new(
            -rtweekend::INFINITY,
            -rtweekend::INFINITY,
            -rtweekend::INFINITY,
        );
        (0..2).for_each(|i| {
            (0..2).for_each(|j| {
                (0..2).for_each(|k| {
                    let x = i as f64 * bbox.x.max + (1 - i) as f64 * bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1 - j) as f64 * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1 - k) as f64 * bbox.z.min;
                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;
                    let tester = Vec3::new(newx, y, newz);
                    (0..3).for_each(|c| {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    })
                })
            })
        });
        let bbox = Aabb::new_with_point(&min, &max);
        Self {
            object: p,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl<T: Hittable> Hittable for RotateY<T> {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let mut origin = r.origin();
        let mut direction = r.direction();

        origin[0] = self.cos_theta * r.origin()[0] - self.sin_theta * r.origin()[2];
        origin[2] = self.sin_theta * r.origin()[0] + self.cos_theta * r.origin()[2];

        direction[0] = self.cos_theta * r.direction()[0] - self.sin_theta * r.direction()[2];
        direction[2] = self.sin_theta * r.direction()[0] + self.cos_theta * r.direction()[2];

        let rotated_r = Ray::new_with_time(origin, direction, r.time());

        if !self.object.hit(&rotated_r, ray_t, rec) {
            return false;
        }

        let mut p = rec.p;
        p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
        p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

        let mut normal = rec.normal;
        normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
        normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

        rec.p = p;
        rec.normal = normal;

        true
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
