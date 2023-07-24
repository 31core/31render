use crate::ray::Ray;
use crate::vector::Vector3D;
use rand::Rng;

#[derive(Default, Clone, Copy)]
pub struct Material {
    /** probability of reflection
     *
     * The probability of reflection is (reflect_rate), while the probability of refraction is (1 - reflect_rate)
     *
     */
    pub reflect_rate: f64,
    /** refractive index of glass */
    pub refract_index: f64,
    /** roughness of surface */
    pub fuzz: f64,
    pub attenuation: (f64, f64, f64),
    /** light intensity */
    pub emit: f64,
    pub is_light: bool,
}

impl Material {
    pub fn new_metal() -> Self {
        Self {
            reflect_rate: 1.,
            ..Default::default()
        }
    }
    #[allow(dead_code)]
    pub fn new_light(emit: f64) -> Self {
        Self {
            emit,
            ..Default::default()
        }
    }
    pub fn scatter(&self, ray: &Ray, length: f64, normal: &Vector3D) -> Ray {
        let p = rand::thread_rng().gen::<f64>();

        /* reflect */
        if p < self.reflect_rate {
            let mut ref_ray = ray.reflect(length, normal);
            ref_ray.direction += self.fuzz * Vector3D::new_random_unit();
            ref_ray.direction = ref_ray.direction.unit();
            ref_ray
        }
        /* refract */
        else {
            let mut ref_ray = {
                /* inject into the medium from air */
                if ray.direction.cdot(normal) < 0. {
                    ray.refract(length, self.refract_index, normal)
                }
                /* inject into air from the medium */
                else {
                    ray.refract(length, 1. / self.refract_index, normal)
                }
            };
            ref_ray.direction += self.fuzz * Vector3D::new_random_unit();
            ref_ray.direction = ref_ray.direction.unit();
            ref_ray
        }
    }
}
