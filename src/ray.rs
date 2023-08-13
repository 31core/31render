use crate::bvh::BVHNode;
use crate::color::Color;
use crate::point::Point;
use crate::vector::*;

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
    pub fn trace(&self, bvh: &BVHNode, depth: usize) -> Color {
        if depth > 0 {
            if let Some((t, object)) = bvh.find_closest_hit(self) {
                let normal = object.normal(&self.point_at(t));

                if object.material().is_light {
                    let emit = object.material().emit;

                    let mut color = Color::new();
                    color.color_vec = Vector3D::from((emit, emit, emit));
                    return color;
                } else {
                    let ref_ray = object.material().scatter(self, t, &normal);
                    let mut color = ref_ray.trace(bvh, depth - 1);
                    color.apply_attenuate(object.material().attenuation);
                    return color;
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
        Point::from_vec3d(self.origin.point_vec + t * self.direction)
    }
}
