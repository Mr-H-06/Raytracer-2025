use super::color::Color;
use super::hittable::HitRecord;
use super::pdf::{CosinePdf, SpherePdf};
//use super::onb;
use super::pdf::Pdf;
use super::ray::Ray;
use super::rtweekend;
use super::vec3::{self, Vec3};
use crate::texture::{SolidColor, Texture};
use std::sync::Arc;

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool;

    fn emitted(&self, _r_in: &Ray, _rec: &HitRecord, _u: f64, _v: f64, _p: vec3::Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        0.0
    }
}

#[derive(Clone)]
pub struct Lambertian<T: Texture> {
    pub albedo: T,
}

impl<T: Texture> Lambertian<T> {
    pub fn new_with_texture(a: T) -> Self {
        Self { albedo: a }
    }
}

impl Lambertian<SolidColor> {
    pub fn new(a: Color) -> Self {
        Self {
            albedo: SolidColor::new(a),
        }
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        srec.pdf = Arc::new(CosinePdf::new(rec.normal));
        srec.skip_pdf = false;
        true
    }

    fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cosine = vec3::dot(rec.normal, vec3::unit_vector(scattered.direction()));
        if cosine < 0.0 {
            0.0
        } else {
            cosine / rtweekend::PI
        }
    }
}

#[derive(Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(a: Color, f: f64) -> Self {
        Self {
            albedo: a,
            fuzz: if f < 1.0 { f } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.attenuation = self.albedo;
        srec.skip_pdf = true;
        let reflected = vec3::reflect(vec3::unit_vector(r_in.direction()), rec.normal);
        srec.skip_pdf_ray = Ray::new_with_time(
            rec.p,
            reflected + self.fuzz * vec3::random_in_unite_sphere(),
            r_in.time(),
        );
        true
    }
}

#[derive(Clone)]
pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            ir: index_of_refraction,
        }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.attenuation = Color::new(1.0, 1.0, 1.0);
        srec.skip_pdf = true;
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = vec3::unit_vector(r_in.direction());

        let cos_theta = vec3::dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract
            || Self::reflectance(cos_theta, refraction_ratio) > rtweekend::random_double()
        {
            vec3::reflect(unit_direction, rec.normal)
        } else {
            vec3::refract(unit_direction, rec.normal, refraction_ratio)
        };

        srec.skip_pdf_ray = Ray::new_with_time(rec.p, direction, r_in.time());
        true
    }
}

#[derive(Clone)]
pub struct DiffuseLight<T: Texture> {
    pub emit: T,
}

impl<T: Texture> DiffuseLight<T> {
    pub fn new(a: T) -> Self {
        Self { emit: a }
    }
}

impl DiffuseLight<SolidColor> {
    pub fn new_with_color(c: Color) -> Self {
        Self {
            emit: SolidColor::new(c),
        }
    }
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord, _srec: &mut ScatterRecord) -> bool {
        false
    }

    fn emitted(&self, _r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: vec3::Point3) -> Color {
        if rec.front_face {
            self.emit.value(u, v, p)
        } else {
            Color::default()
        }
    }
}

#[derive(Clone)]
pub struct Isotropic<T: Texture> {
    pub albedo: T,
}

impl<T: Texture> Isotropic<T> {
    pub fn new(a: T) -> Self {
        Self { albedo: a }
    }
}

impl Isotropic<SolidColor> {
    pub fn new_with_color(c: Color) -> Self {
        Self {
            albedo: SolidColor::new(c),
        }
    }
}

impl<T: Texture> Material for Isotropic<T> {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        srec.pdf = Arc::new(SpherePdf {});
        srec.skip_pdf = false;
        true
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        1.0 / (4.0 * rtweekend::PI)
    }
}

#[derive(Clone)]
pub struct NonePdf;

impl Pdf for NonePdf {
    fn value(&self, _direction: Vec3) -> f64 {
        0.0
    }

    fn generate(&self) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}

#[derive(Clone)]
pub struct ScatterRecord {
    pub attenuation: Color,
    pub pdf: Arc<dyn Pdf>,
    pub skip_pdf: bool,
    pub skip_pdf_ray: Ray,
}

impl Default for ScatterRecord {
    fn default() -> Self {
        Self {
            attenuation: Color::default(),
            pdf: Arc::new(NonePdf {}),
            skip_pdf: false,
            skip_pdf_ray: Ray::default(),
        }
    }
}
