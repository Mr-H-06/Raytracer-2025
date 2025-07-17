use super::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{self, Point3, Vec3},
};
use std::sync::Arc;

#[derive(Clone)]
pub struct Triangle<M: Material> {
    p0: Point3,
    p1: Point3,
    p2: Point3,
    n0: Vec3,
    n1: Vec3,
    n2: Vec3,
    uv0: (f64, f64),
    uv1: (f64, f64),
    uv2: (f64, f64),
    mat: M,
    bbox: Aabb,
}

#[allow(clippy::too_many_arguments)]
impl<M: Material> Triangle<M> {
    pub fn new(
        p0: Point3,
        p1: Point3,
        p2: Point3,
        n0: Vec3,
        n1: Vec3,
        n2: Vec3,
        uv0: (f64, f64),
        uv1: (f64, f64),
        uv2: (f64, f64),
        mat: M,
    ) -> Self {
        let mut bbox = Aabb::new_with_point(&p0, &p1);
        bbox = Aabb::new_with_box(&bbox, &Aabb::new_with_point(&p0, &p2));
        Self {
            p0,
            p1,
            p2,
            n0,
            n1,
            n2,
            uv0,
            uv1,
            uv2,
            mat,
            bbox,
        }
    }
}

impl<M: Material + Clone + 'static> Hittable for Triangle<M> {
    fn hit(&self, r: &Ray, ray_t: &Interval, hit_record: &mut HitRecord) -> bool {
        let edge1 = self.p1 - self.p0;
        let edge2 = self.p2 - self.p0;
        let ray_cross_e2 = vec3::cross(r.direction, edge2);
        let det = vec3::dot(edge1, ray_cross_e2);

        if det.abs() < 1e-8 {
            return false;
        }

        let inv_det = 1.0 / det;
        let s = r.origin() - self.p0;
        let u = inv_det * vec3::dot(s, ray_cross_e2);

        if !(0.0..=1.0).contains(&u) {
            return false;
        }

        let s_cross_e1 = vec3::cross(s, edge1);
        let v = inv_det * vec3::dot(r.direction(), s_cross_e1);

        if v < 0.0 || u + v > 1.0 {
            return false;
        }

        let t = inv_det * vec3::dot(edge2, s_cross_e1);

        if !ray_t.surrounds(t) {
            return false;
        }

        hit_record.t = t;
        hit_record.p = r.at(t);

        let bary_u = u;
        let bary_v = v;
        let bary_w = 1.0 - bary_u - bary_v;

        let interpolated_normal = bary_w * self.n0 + bary_u * self.n1 + bary_v * self.n2;
        hit_record.set_face_normal(r, vec3::unit_vector(interpolated_normal));

        let interpolated_uv = bary_w * Vec3::new(self.uv0.0, self.uv0.1, 0.0)
            + bary_u * Vec3::new(self.uv1.0, self.uv1.1, 0.0)
            + bary_v * Vec3::new(self.uv2.0, self.uv2.1, 0.0);

        hit_record.u = interpolated_uv.x();
        hit_record.v = interpolated_uv.y();

        hit_record.mat = Some(Arc::new(self.mat.clone()));

        true
    }
    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
