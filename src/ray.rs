use crate::color::Color;
use crate::coordinate::*;
use crate::vector::*;
use std::rc::Rc;

fn find_closest_hit(ray: &Ray, objects: &[Rc<dyn Object>]) -> Option<(f64, Rc<dyn Object>)> {
    let mut closest = 0.;
    let mut closest_object = None;
    for object in objects {
        if let Some(t) = object.hit(ray) {
            if closest == 0. || t < closest {
                closest = t;
                closest_object = Some((t, Rc::clone(object)));
            }
        }
    }
    closest_object
}

#[derive(Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector3D,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector3D) -> Self {
        Self {
            origin,
            direction: direction.unit(),
        }
    }
    /**
     * Do ray tracing
     */
    pub fn trace(&self, objects: &[Rc<dyn Object>], depth: usize) -> Color {
        if depth > 0 {
            if let Some((t, object)) = find_closest_hit(self, objects) {
                let normal = object.normal(&self.point_at(t));

                match object.material().reflect(self, t, &normal) {
                    Some(ref_ray) => {
                        let mut color = ref_ray.trace(objects, depth - 1);
                        color.attenuate(object.material().attenuation());
                        return color;
                    }
                    /* light source */
                    None => {
                        let mut color = Color::new();
                        color.color_vec = Vector3D::from(object.material().attenuation());
                        return color;
                    }
                }
            }
        }

        let mut color = Color::new();
        color.color_vec = Vector3D::new(0., 0., 0.);
        color
    }
    /**
     * Reflect a ray
     */
    pub fn reflect(&self, length: f64, normal: &Vector3D) -> Self {
        let t = -2. * self.direction.cdot(normal);
        let direction = t * *normal + self.direction;
        Self {
            origin: self.point_at(length),
            direction,
        }
    }
    /**
     * Refract a ray
     */
    pub fn refract(&self, length: f64, rate: f64, normal: &Vector3D) -> Self {
        let t = -self.direction.cdot(normal);
        let c = self.direction + t * *normal;
        let direction = c / rate - t * *normal;
        Self {
            origin: self.point_at(length),
            direction,
        }
    }
    pub fn point_at(&self, t: f64) -> Point {
        Point::from_vec3d(self.origin.vector + t * self.direction)
    }
}
