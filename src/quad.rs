use super::aabb::Aabb;
use super::hittable::{HitRecord, Hittable};
use super::material::Material;
use super::vec3::{Point3, Vec3};
use crate::vec3;
use std::rc::Rc;

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    normal: Vec3,
    d: f64,
    mat: Rc<dyn Material>,
    bbox: Aabb,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Rc<dyn Material>) -> Self {
        let n = vec3::cross(u, v);
        let normal = vec3::unit_vector(n);
        let mut res = Self {
            q,
            u,
            v,
            w: n / n.length_squared(),
            normal,
            d: vec3::dot(normal, q),
            mat,
            bbox: Aabb::default(),
        };
        res.set_bounding_box();
        res
    }
    pub fn set_bounding_box(&mut self) {
        let bbox_diagonal1 = Aabb::new_with_point(&self.q, &(self.q + self.u + self.v));
        let bbox_diagnoal2 = Aabb::new_with_point(&(self.q + self.u), &(self.q + self.v));
        self.bbox = Aabb::new_with_box(&bbox_diagonal1, &bbox_diagnoal2);
    }

    pub fn is_interior(&self, a: f64, b: f64, rec: &mut HitRecord) -> bool {
        if !(0.0..=1.0).contains(&a) || !(0.0..=1.0).contains(&b) {
            return false;
        }
        rec.u = a;
        rec.v = b;
        true
    }
}

impl Hittable for Quad {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: &crate::interval::Interval,
        rec: &mut HitRecord,
    ) -> bool {
        let denom = vec3::dot(self.normal, r.direction);

        if denom.abs() < 1e-8 {
            return false;
        }
        let t = (self.d - vec3::dot(self.normal, r.origin)) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = vec3::dot(self.w, vec3::cross(planar_hitpt_vector, self.v));
        let beta = vec3::dot(self.w, vec3::cross(self.u, planar_hitpt_vector));
        if !self.is_interior(alpha, beta, rec) {
            return false;
        }

        rec.t = t;
        rec.p = intersection;
        rec.mat = Some(Rc::clone(&self.mat));
        rec.set_face_normal(r, self.normal);
        true
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
