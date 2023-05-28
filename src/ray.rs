use super::color::Color;
use super::cordinate::*;
use super::vector::*;

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
    pub fn trace<O>(&self, objects: &[O], depth: usize) -> Color
    where
        O: Object,
    {
        if depth > 0 {
            for object in objects {
                if let Some(t) = object.hit(self) {
                    let mut normal = object.normal(&self.point_at(t));
                    /* A point on a plane has two normal vectors, so we need to adjust normal vector. */
                    if self.direction.cdot(&normal) < 0. {
                        normal = -normal;
                    }
                    let ref_ray = self.reflect(t, &normal);

                    let mut color = ref_ray.trace(objects, depth - 1);
                    color.color = 0.5 * color.color;
                    return color;
                }
            }
        }
        let t = 0.5 * (self.direction.y + 1.);
        let mut color = Color::new();
        color.color = t * Vector3D::new(0.3, 0.5, 0.7);
        color.color += (1. - t) * Vector3D::new(1., 1., 1.);
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
    pub fn point_at(&self, t: f64) -> Point {
        Point::from_vec3d(self.origin.vector + t * self.direction)
    }
}
