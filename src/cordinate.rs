use super::material::*;
use super::ray::Ray;
use super::vector::Vector3D;
use std::rc::Rc;

pub trait Object {
    fn hit(&self, r: &Ray) -> Option<f64>;
    fn normal(&self, p: &Point) -> Vector3D;
    fn material(&self) -> Rc<dyn Material>;
}

#[derive(Clone, Debug)]
pub struct Point {
    pub vector: Vector3D,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            vector: Vector3D::new(x, y, z),
        }
    }
    pub fn to_vec3d(&self, other: &Self) -> Vector3D {
        -self.vector + other.vector
    }
    pub fn from_vec3d(vector: Vector3D) -> Self {
        Self { vector }
    }
}

pub struct Sphere {
    center: Point,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new<M>(center: Point, radius: f64, material: M) -> Self
    where
        M: Material + 'static,
    {
        Self {
            center,
            radius,
            material: Rc::new(material),
        }
    }
}

impl Object for Sphere {
    fn hit(&self, ray: &Ray) -> Option<f64> {
        let ca = self.center.to_vec3d(&ray.origin);
        let t_d = -ca.cdot(&ray.direction);

        let distance = (ca + t_d * ray.direction).module();

        if t_d > 0. && distance <= self.radius {
            let t = t_d - (self.radius.powi(2) - distance.powi(2)).sqrt();
            if t > 0. {
                Some(t)
            }
            /* if the lighht source is on the sphere */
            else {
                let t = t_d + (self.radius.powi(2) - distance.powi(2)).sqrt();
                Some(t)
            }
        } else {
            None
        }
    }
    fn normal(&self, p: &Point) -> Vector3D {
        self.center.to_vec3d(p).unit()
    }
    fn material(&self) -> Rc<dyn Material> {
        Rc::clone(&self.material)
    }
}

pub struct Viewport {
    pub width: usize,
    pub height: usize,
    pub pixel_x: usize,
    pub pixel_y: usize,
    pub focal: f64,
}

impl Viewport {
    pub fn new(width: usize, height: usize, pixel_x: usize, pixel_y: usize) -> Self {
        Self {
            width,
            height,
            pixel_x,
            pixel_y,
            focal: -1.,
        }
    }
    pub fn get_ray(&self, x: usize, y: usize) -> Ray {
        let mut start = Vector3D::new(
            -(self.width as f64) * 0.5,
            self.height as f64 * 0.5,
            self.focal,
        );
        start.x += self.width as f64 / self.pixel_x as f64 * x as f64;
        start.y -= self.height as f64 / self.pixel_y as f64 * y as f64;
        let direction = start - Vector3D::new(0., 0., 0.);
        Ray::new(Point::new(0., 0., 0.), direction)
    }
}
