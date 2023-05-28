use super::vector::Vector3D;

pub struct Color {
    pub color: Vector3D,
}

impl Color {
    pub fn new() -> Self {
        Self {
            color: Vector3D::new(1., 1., 1.),
        }
    }
    pub fn to_rgb(&self) -> ppm::Pixel {
        ppm::Pixel::new(
            (255. * self.color.x) as u8,
            (255. * self.color.y) as u8,
            (255. * self.color.z) as u8,
        )
    }
}
