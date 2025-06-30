use super::vec3::{Point3, Vec3};
#[derive(Debug, Copy, Clone, Default)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    tm: f64,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            origin,
            direction,
            tm: 0.0,
        }
    }

    pub fn new_with_time(origin: Point3, direction: Vec3, tm: f64) -> Self {
        Self {
            origin,
            direction,
            tm,
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn time(&self) -> f64 {
        self.tm
    }
}
