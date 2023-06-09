use crate::ray::Ray;
use crate::vector::Vector3D;

pub trait Material {
    fn reflect(&self, ray: &Ray, length: f64, normal: &Vector3D) -> Option<Ray>;
    fn attenuation(&self) -> (f64, f64, f64);
}

#[derive(Clone, Copy)]
pub struct Metal {
    pub attenuation: (f64, f64, f64),
    pub fuzz: f64,
}

impl Default for Metal {
    fn default() -> Self {
        Self {
            attenuation: (1., 1., 1.),
            fuzz: 0.,
        }
    }
}

impl Material for Metal {
    fn reflect(&self, ray: &Ray, length: f64, normal: &Vector3D) -> Option<Ray> {
        let mut ref_ray = ray.reflect(length, normal);
        if self.fuzz == 0. {
            return Some(ref_ray);
        }
        ref_ray.direction += self.fuzz * Vector3D::new_random_unit();
        ref_ray.direction = ref_ray.direction.unit();
        Some(ref_ray)
    }
    fn attenuation(&self) -> (f64, f64, f64) {
        self.attenuation
    }
}

pub struct Glass {
    pub attenuation: (f64, f64, f64),
    pub rate: f64,
    pub fuzz: f64,
}

impl Default for Glass {
    fn default() -> Self {
        Self {
            attenuation: (1., 1., 1.),
            rate: 1.2,
            fuzz: 0.,
        }
    }
}

impl Material for Glass {
    fn reflect(&self, ray: &Ray, length: f64, normal: &Vector3D) -> Option<Ray> {
        let mut ref_ray = {
            if ray.direction.cdot(normal) < 0. {
                ray.refract(length, self.rate, normal)
            }
            /* inject into air from the medium */
            else {
                ray.refract(length, 1. / self.rate, normal)
            }
        };
        if self.fuzz == 0. {
            return Some(ref_ray);
        }
        ref_ray.direction += self.fuzz * Vector3D::new_random_unit();
        ref_ray.direction = ref_ray.direction.unit();
        Some(ref_ray)
    }
    fn attenuation(&self) -> (f64, f64, f64) {
        self.attenuation
    }
}

#[derive(Clone, Copy)]
pub struct Light {
    intensity: f64,
}

impl Light {
    #[allow(dead_code)]
    pub fn new(intensity: f64) -> Self {
        Self { intensity }
    }
}

impl Default for Light {
    fn default() -> Self {
        Self { intensity: 1. }
    }
}

impl Material for Light {
    fn reflect(&self, _ray: &Ray, _length: f64, _normal: &Vector3D) -> Option<Ray> {
        None
    }
    fn attenuation(&self) -> (f64, f64, f64) {
        (self.intensity, self.intensity, self.intensity)
    }
}
