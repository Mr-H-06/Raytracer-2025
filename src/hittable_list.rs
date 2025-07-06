use super::hittable::{HitRecord, Hittable};
use super::ray::Ray;
use super::rtweekend;
use super::vec3;
use crate::aabb::Aabb;
use crate::interval::Interval;
use std::rc::Rc;

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,

    bbox: Aabb,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList::default()
    }

    pub fn new_with_object(object: Rc<dyn Hittable>) -> Self {
        Self {
            objects: vec![object],
            bbox: Aabb::default(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.bbox = Aabb::new_with_box(&self.bbox, object.bounding_box());
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            if object.hit(r, &Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }

    fn pdf_value(&self, origin: vec3::Point3, direction: vec3::Vec3) -> f64 {
        let weight = 1.0 / self.objects.len() as f64;
        let mut sum = 0.0;

        for object in self.objects.iter() {
            sum += weight * object.pdf_value(origin, direction);
        }

        sum
    }

    fn random(&self, origin: vec3::Point3) -> vec3::Vec3 {
        let int_size = self.objects.len() as i32;
        self.objects[rtweekend::random_int(0, int_size - 1) as usize].random(origin)
    }
}
