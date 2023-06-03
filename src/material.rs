use super::ray::Ray;
use super::vector::Vector3D;

pub trait Material {
    fn reflect(&self, ray: &Ray, length: f64, normal: &Vector3D) -> Ray;
    fn attenuation(&self) -> (f64, f64, f64);
}

#[derive(Clone, Copy)]
pub struct Metal {
    pub attenuation: (f64, f64, f64),
}

impl Default for Metal {
    fn default() -> Self {
        Self {
            attenuation: (1., 1., 1.),
        }
    }
}

impl Material for Metal {
    fn reflect(&self, ray: &Ray, length: f64, normal: &Vector3D) -> Ray {
        ray.reflect(length, normal)
    }
    fn attenuation(&self) -> (f64, f64, f64) {
        self.attenuation
    }
}

pub struct Glass {
    pub attenuation: (f64, f64, f64),
    pub rate: f64,
}

impl Default for Glass {
    fn default() -> Self {
        Self {
            attenuation: (1., 1., 1.),
            rate: 1.2,
        }
    }
}

impl Material for Glass {
    fn reflect(&self, ray: &Ray, length: f64, normal: &Vector3D) -> Ray {
        ray.refract(length, self.rate, normal)
    }
    fn attenuation(&self) -> (f64, f64, f64) {
        self.attenuation
    }
}
