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
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();
        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;
        let mut c = [[[0.0; 2]; 2]; 2];
        (0..2).for_each(|di| {
            (0..2).for_each(|dj| {
                (0..2).for_each(|dk| {
                    c[di][dj][dk] = self.ranfloat[self.perm_x[((i + di as i32) & 255) as usize]
                        as usize
                        ^ self.perm_y[((j + dj as i32) & 255) as usize] as usize
                        ^ self.perm_z[((k + dk as i32) & 255) as usize] as usize];
                })
            })
        });
        Self::trilinear_interp(&c, u, v, w)
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

    fn trilinear_interp(c: &[[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        (0..2).for_each(|i| {
            (0..2).for_each(|j| {
                (0..2).for_each(|k| {
                    accum += (i as f64 * u + (1 - i) as f64 * (1.0 - u))
                        * (j as f64 * v + (1 - j) as f64 * (1.0 - v))
                        * (k as f64 * w + (1 - k) as f64 * (1.0 - w))
                        * c[i][j][k];
                })
            })
        });
        accum
    }
}
