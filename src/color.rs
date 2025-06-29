use super::vec3::Vec3;
use crate::interval::Interval;
use image::Rgb;

pub type Color = Vec3;

const INTENSITY: Interval = Interval {
    min: 0.0,
    max: 0.999,
};

pub fn linear_to_gamma(liner_component: f64) -> f64 {
    if liner_component > 0.0 {
        liner_component.sqrt()
    } else {
        0.0
    }
}

impl Color {
    //pub fn to_u64(self) -> (u64, u64, u64) {
    //    let x = (self.x * 255.999) as u64;
    //    let y = (self.y * 255.999) as u64;
    //    let z = (self.z * 255.999) as u64;
    //    (x, y, z)
    //}

    pub fn to_color(&self, samples_per_pixel: usize) -> Rgb<u8> {
        let r = self.x;
        let g = self.y;
        let b = self.z;

        let scale = 1.0 / samples_per_pixel as f64;
        let r = scale * r;
        let g = scale * g;
        let b = scale * b;

        let r = linear_to_gamma(r);
        let g = linear_to_gamma(g);
        let b = linear_to_gamma(b);

        Rgb([
            (256.0 * INTENSITY.clamp(r)) as u8,
            (256.0 * INTENSITY.clamp(g)) as u8,
            (256.0 * INTENSITY.clamp(b)) as u8,
        ])
    }
}
