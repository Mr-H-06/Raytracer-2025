use super::aabb::Aabb;
use super::hittable::{HitRecord, Hittable};
use super::hittable_list::HittableList;
use super::interval::Interval;
use super::material::Material;
use super::ray::Ray;
use super::rtweekend;
use super::vec3::{self, Point3, Vec3};
use std::sync::Arc;

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    normal: Vec3,
    d: f64,
    mat: Arc<dyn Material>,
    bbox: Aabb,
    area: f64,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Self {
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
            area: n.length(),
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
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
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
        rec.mat = Some(Arc::clone(&self.mat));
        rec.set_face_normal(r, self.normal);
        true
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }

    fn pdf_value(&self, origin: Point3, direction: Vec3) -> f64 {
        let mut rec = HitRecord::default();
        if !self.hit(
            &Ray::new(origin, direction),
            &Interval::new(0.0001, f64::INFINITY),
            &mut rec,
        ) {
            return 0.0;
        }

        let distance_squared = rec.t * rec.t * direction.length_squared();
        let cosine = (vec3::dot(direction, rec.normal) / direction.length()).abs();

        distance_squared / (cosine * self.area)
    }

    fn random(&self, origin: Point3) -> Vec3 {
        let p =
            self.q + (rtweekend::random_double() * self.u) + (rtweekend::random_double() * self.v);
        p - origin
    }
}

pub fn make_box(a: Point3, b: Point3, mat: Arc<dyn Material>) -> Arc<HittableList> {
    // 返回一个包含两个对角顶点a和b的3D盒子（六个面）。

    let mut sides = HittableList::default();

    // 构造两个对角顶点，具有最小和最大的坐标。
    let min = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
    let max = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

    let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), min.y(), max.z()),
        dx,
        dy,
        Arc::clone(&mat),
    )));
    sides.add(Arc::new(Quad::new(
        Point3::new(max.x(), min.y(), max.z()),
        -dz,
        dy,
        Arc::clone(&mat),
    )));
    sides.add(Arc::new(Quad::new(
        Point3::new(max.x(), min.y(), min.z()),
        -dx,
        dy,
        Arc::clone(&mat),
    )));
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), min.y(), min.z()),
        dz,
        dy,
        Arc::clone(&mat),
    )));
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), max.y(), max.z()),
        dx,
        -dz,
        Arc::clone(&mat),
    )));
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), min.y(), min.z()),
        dx,
        dz,
        Arc::clone(&mat),
    )));

    Arc::new(sides)
}
