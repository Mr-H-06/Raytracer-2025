use super::hittable;
use super::onb;
use super::rtweekend;
use super::vec3;
use std::sync::Arc;

pub trait Pdf: Send + Sync {
    fn value(&self, direction: vec3::Vec3) -> f64;
    fn generate(&self) -> vec3::Vec3;
}

pub struct SpherePdf;

impl Pdf for SpherePdf {
    fn value(&self, _direction: vec3::Vec3) -> f64 {
        1.0 / (4.0 * rtweekend::PI)
    }

    fn generate(&self) -> vec3::Vec3 {
        vec3::random_unit_vector()
    }
}

pub struct CosinePdf {
    uvw: onb::Onb,
}

impl CosinePdf {
    pub fn new(w: vec3::Vec3) -> Self {
        Self {
            uvw: onb::Onb::new_from_w(w),
        }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: vec3::Vec3) -> f64 {
        let cosine_theta = vec3::dot(vec3::unit_vector(direction), self.uvw.w());
        0.0_f64.max(cosine_theta / rtweekend::PI)
    }

    fn generate(&self) -> vec3::Vec3 {
        self.uvw.local_v(vec3::random_cosine_direction())
    }
}

pub struct HittablePdf {
    pub objects: Arc<dyn hittable::Hittable>,
    pub origin: vec3::Point3,
}

impl HittablePdf {
    pub fn new(objects: Arc<dyn hittable::Hittable>, origin: vec3::Point3) -> Arc<Self> {
        Arc::new(Self { objects, origin })
    }
}

impl Pdf for HittablePdf {
    fn value(&self, direction: vec3::Vec3) -> f64 {
        self.objects.pdf_value(self.origin, direction)
    }

    fn generate(&self) -> vec3::Vec3 {
        self.objects.random(self.origin)
    }
}

pub struct MixturePdf {
    pub p: [Arc<dyn Pdf>; 2],
}

impl MixturePdf {
    pub fn new(p0: Arc<dyn Pdf>, p1: Arc<dyn Pdf>) -> Self {
        Self { p: [p0, p1] }
    }
}

impl Pdf for MixturePdf {
    fn value(&self, direction: vec3::Vec3) -> f64 {
        0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
    }

    fn generate(&self) -> vec3::Vec3 {
        if rtweekend::random_double_range(0.0, 1.0) < 0.5 {
            self.p[0].generate()
        } else {
            self.p[1].generate()
        }
    }
}
