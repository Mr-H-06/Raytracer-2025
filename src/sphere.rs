use super::hittable::{HitRecord, Hittable};
use super::material::Material;
use super::onb;
use super::ray::Ray;
use super::rtweekend;
use super::vec3::{self, Point3, Vec3};
use crate::aabb::Aabb;
use crate::interval::Interval;
use std::sync::Arc;

#[derive(Clone)]
pub struct Sphere<T: Material> {
    center: Ray,
    radius: f64,
    mat: T,

    bbox: Aabb,
}

impl<T: Material> Sphere<T> {
    pub fn new(static_center: Point3, radius: f64, material: T) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        Self {
            center: Ray::new(static_center, Vec3::zero()),
            radius,
            mat: material,

            bbox: Aabb::new_with_point(&(static_center - rvec), &(static_center + rvec)),
        }
    }

    pub fn new_with_center2(center1: Point3, center2: Point3, radius: f64, material: T) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        let box1 = Aabb::new_with_point(&(center1 - rvec), &(center1 + rvec));
        let box2 = Aabb::new_with_point(&(center2 - rvec), &(center2 + rvec));
        Self {
            center: Ray::new(center1, center2 - center1),
            radius,
            mat: material,

            bbox: Aabb::new_with_box(&box1, &box2),
        }
    }

    fn get_sphere_uv(p: Point3) -> (f64, f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + rtweekend::PI;
        (phi / (2.0 * rtweekend::PI), theta / rtweekend::PI)
    }

    fn random_to_sphere(radius: f64, distance_squared: f64) -> Vec3 {
        let r1 = rtweekend::random_double();
        let r2 = rtweekend::random_double();
        let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);

        let phi = 2.0 * rtweekend::PI * r1;
        let x = phi.cos() * (1.0 - z * z).sqrt();
        let y = phi.sin() * (1.0 - z * z).sqrt();

        Vec3::new(x, y, z)
    }
}

impl<T: Material + Clone + 'static> Hittable for Sphere<T> {
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
        (hit_record.u, hit_record.v) = Self::get_sphere_uv(outward_normal);
        hit_record.mat = Some(Arc::new(self.mat.clone()) as Arc<dyn Material>);

        true
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }

    fn pdf_value(&self, origin: Point3, direction: Vec3) -> f64 {
        let mut rec = HitRecord::default();
        if !self.hit(
            &Ray::new(origin, direction),
            &Interval::new(0.001, rtweekend::INFINITY),
            &mut rec,
        ) {
            return 0.0;
        }

        let cos_theta_max = (1.0
            - self.radius * self.radius / (self.center.at(0.0) - origin).length_squared())
        .sqrt();
        let solid_angle = 2.0 * rtweekend::PI * (1.0 - cos_theta_max);

        1.0 / solid_angle
    }

    fn random(&self, origin: Point3) -> Vec3 {
        let direction = self.center.at(0.0) - origin;
        let distance_squared = direction.length_squared();
        let uvw = onb::Onb::new_from_w(direction);
        uvw.local_v(Self::random_to_sphere(self.radius, distance_squared))
    }
}
