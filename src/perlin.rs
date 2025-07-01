use super::rtweekend;
use super::vec3::Point3;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    ranfloat: [f64; POINT_COUNT],
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
}

impl Default for Perlin {
    fn default() -> Self {
        let mut ranfloat = [0.0; POINT_COUNT];
        for val in ranfloat.iter_mut() {
            *val = rtweekend::random_double();
        }
        let mut perm_x = [0; POINT_COUNT];
        let mut perm_y = [0; POINT_COUNT];
        let mut perm_z = [0; POINT_COUNT];

        Self::perlin_generate_perm(&mut perm_x);
        Self::perlin_generate_perm(&mut perm_y);
        Self::perlin_generate_perm(&mut perm_z);

        Self {
            ranfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }
}

impl Perlin {
    pub fn noise(&self, p: Point3) -> f64 {
        let i = (((4.0 * p.x()) as i32) & 255) as usize;
        let j = (((4.0 * p.y()) as i32) & 255) as usize;
        let k = (((4.0 * p.z()) as i32) & 255) as usize;

        self.ranfloat[self.perm_x[i] as usize ^ self.perm_y[j] as usize ^ self.perm_z[k] as usize]
    }

    fn perlin_generate_perm(p: &mut [i32; POINT_COUNT]) {
        for (i, val) in p.iter_mut().enumerate() {
            *val = i as i32;
        }
        Self::permute(p);
    }

    fn permute(p: &mut [i32; POINT_COUNT]) {
        for i in (0..p.len()).rev() {
            let target = rtweekend::random_int(0, i as i32) as usize;
            p.swap(i, target);
        }
    }
}
