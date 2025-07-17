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

pub struct RotateX<T: Hittable> {
    object: T,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl<T: Hittable> RotateX<T> {
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
                    let newy = cos_theta * y - sin_theta * z;
                    let newz = sin_theta * y + cos_theta * z;
                    let tester = Vec3::new(x, newy, newz);
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

impl<T: Hittable> Hittable for RotateX<T> {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let mut origin = r.origin();
        let mut direction = r.direction();

        origin[1] = self.cos_theta * r.origin()[1] - self.sin_theta * r.origin()[2];
        origin[2] = self.sin_theta * r.origin()[1] + self.cos_theta * r.origin()[2];

        direction[1] = self.cos_theta * r.direction()[1] - self.sin_theta * r.direction()[2];
        direction[2] = self.sin_theta * r.direction()[1] + self.cos_theta * r.direction()[2];

        let rotated_r = Ray::new_with_time(origin, direction, r.time());

        if !self.object.hit(&rotated_r, ray_t, rec) {
            return false;
        }

        let mut p = rec.p;
        p[1] = self.cos_theta * rec.p[1] + self.sin_theta * rec.p[2];
        p[2] = -self.sin_theta * rec.p[1] + self.cos_theta * rec.p[2];

        let mut normal = rec.normal;
        normal[1] = self.cos_theta * rec.normal[1] + self.sin_theta * rec.normal[2];
        normal[2] = -self.sin_theta * rec.normal[1] + self.cos_theta * rec.normal[2];

        rec.p = p;
        rec.normal = normal;

        true
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

#[derive(Clone)]
pub struct Scale<H: Hittable> {
    object: H,
    inv_scale: Vec3,
    scale: Vec3,
    bbox: Aabb,
}

impl<H: Hittable> Scale<H> {
    pub fn new(object: H, scale_vec: Vec3) -> Self {
        let inv_scale = Vec3::new(
            1.0 / scale_vec.x(),
            1.0 / scale_vec.y(),
            1.0 / scale_vec.z(),
        );

        let object_bbox = object.bounding_box();
        let min_p = object_bbox.x.min * scale_vec;
        let max_p = object_bbox.x.max * scale_vec;

        let bbox = Aabb::new_with_point(&min_p, &max_p);

        Self {
            object,
            inv_scale,
            scale: scale_vec,
            bbox,
        }
    }
}

impl<H: Hittable> Hittable for Scale<H> {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let origin = r.origin() * self.inv_scale;
        let direction = r.direction() * self.inv_scale;
        let scaled_r = Ray::new_with_time(origin, direction, r.time());

        if !self.object.hit(&scaled_r, ray_t, rec) {
            return false;
        }

        rec.p = rec.p * self.scale;

        let final_normal = vec3::unit_vector(rec.normal * self.inv_scale);
        rec.set_face_normal(r, final_normal);

        true
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
