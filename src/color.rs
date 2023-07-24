use crate::vector::Vector3D;

pub struct Color {
    pub color_vec: Vector3D,
}

impl Color {
    pub fn new() -> Self {
        Self {
            color_vec: Vector3D::new(1., 1., 1.),
        }
    }
    pub fn apply_attenuate(&mut self, attenuation: (f64, f64, f64)) {
        self.color_vec.x *= attenuation.0;
        self.color_vec.y *= attenuation.1;
        self.color_vec.z *= attenuation.2;
    }
    pub fn to_rgb(&self) -> ppm::Pixel {
        ppm::Pixel::new(
            (255. * self.color_vec.x) as u8,
            (255. * self.color_vec.y) as u8,
            (255. * self.color_vec.z) as u8,
        )
    }
}
