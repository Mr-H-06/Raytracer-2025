use super::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn to_u64(self) -> (u64, u64, u64) {
        let x = (self.x * 255.999) as u64;
        let y = (self.y * 255.999) as u64;
        let z = (self.z * 255.999) as u64;
        (x, y, z)
    }
}
