use super::material::*;
use super::ray::Ray;
use super::vector::Vector3D;
use rand::Rng;
use std::rc::Rc;

/**
 * Check if a ray hit a plane
*/
fn plane_hit(ray: &Ray, p: &Point, normal: &Vector3D) -> Option<f64> {
    let ap = ray.origin.to_vec3d(p);
    let t_n = -ap.cdot(normal);
    let t = -t_n / ray.direction.cdot(normal);
    if t > 0. {
        Some(t)
    } else {
        None
    }
}

/**
 * Check if a point in a triangle
*/
fn point_in_triangle(p: &Point, p1: &Point, p2: &Point, p3: &Point) -> bool {
    let ap = p1.to_vec3d(p);
    let vec_a = p1.to_vec3d(p2);
    let vec_b = p1.to_vec3d(p3);

    /*
    d = |x_a x_b|
        |y_a y_b|
    */
    let d = vec_a.x * vec_b.y - vec_a.y * vec_b.x;
    if d != 0. {
        let u = (ap.x * vec_b.y - ap.y * vec_b.x) / d;
        let v = (vec_a.x * ap.y - vec_a.y * ap.x) / d;
        return u >= 0. && v >= 0. && u + v <= 1.;
    }
    /*
    d = |y_a y_b|
        |z_a z_b|
    */
    let d = vec_a.y * vec_b.z - vec_a.z * vec_b.y;
    if d != 0. {
        let u = (ap.y * vec_b.z - ap.z * vec_b.y) / d;
        let v = (vec_a.y * ap.z - vec_a.z * ap.y) / d;
        return u >= 0. && v >= 0. && u + v <= 1.;
    }
    /*
    d = |x_a x_b|
        |z_a z_b|
    */
    let d = vec_a.x * vec_b.z - vec_a.z * vec_b.x;
    let u = (ap.x * vec_b.z - ap.z * vec_b.x) / d;
    let v = (vec_a.x * ap.z - vec_a.z * ap.x) / d;
    u >= 0. && v >= 0. && u + v <= 1.
}

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
    pub fn origin_point() -> Self {
        Self {
            vector: Vector3D::new(0., 0., 0.),
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

pub struct Plane {
    point: Point,
    normal: Vector3D,
    material: Rc<dyn Material>,
}

impl Plane {
    pub fn new<M>(point: Point, normal: Vector3D, material: M) -> Self
    where
        M: Material + 'static,
    {
        Self {
            point,
            normal: normal.unit(),
            material: Rc::new(material),
        }
    }
}

impl Object for Plane {
    fn hit(&self, r: &Ray) -> Option<f64> {
        plane_hit(r, &self.point, &self.normal)
    }
    fn normal(&self, _p: &Point) -> Vector3D {
        self.normal
    }
    fn material(&self) -> Rc<dyn Material> {
        Rc::clone(&self.material)
    }
}

pub struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
    material: Rc<dyn Material>,
}

impl Triangle {
    pub fn new<M>(p1: Point, p2: Point, p3: Point, material: M) -> Self
    where
        M: Material + 'static,
    {
        Self {
            p1,
            p2,
            p3,
            material: Rc::new(material),
        }
    }
    fn get_normal(&self) -> Vector3D {
        (self.p1.to_vec3d(&self.p2) * self.p1.to_vec3d(&self.p3)).unit()
    }
}

impl Object for Triangle {
    fn hit(&self, r: &Ray) -> Option<f64> {
        plane_hit(r, &self.p1, &self.get_normal())
            .filter(|&t| point_in_triangle(&r.point_at(t), &self.p1, &self.p2, &self.p3))
    }
    fn material(&self) -> Rc<dyn Material> {
        Rc::clone(&self.material)
    }
    fn normal(&self, _p: &Point) -> Vector3D {
        self.get_normal()
    }
}

pub struct Parallelogram {
    p_top: Point,
    p_side1: Point,
    p_side2: Point,
    material: Rc<dyn Material>,
}

impl Parallelogram {
    pub fn new<M>(p_top: Point, p_side1: Point, p_side2: Point, material: M) -> Self
    where
        M: Material + 'static,
    {
        Self {
            p_top,
            p_side1,
            p_side2,
            material: Rc::new(material),
        }
    }
    fn get_normal(&self) -> Vector3D {
        (self.p_top.to_vec3d(&self.p_side1) * self.p_top.to_vec3d(&self.p_side2)).unit()
    }
}

impl Object for Parallelogram {
    fn hit(&self, r: &Ray) -> Option<f64> {
        match plane_hit(r, &self.p_top, &self.get_normal()) {
            Some(t) => {
                let p_bottom = Point::from_vec3d(
                    self.p_top.to_vec3d(&self.p_side1)
                        + self.p_top.to_vec3d(&self.p_side2)
                        + Point::origin_point().to_vec3d(&self.p_top),
                );
                if point_in_triangle(&r.point_at(t), &self.p_top, &self.p_side1, &self.p_side2)
                    || point_in_triangle(&r.point_at(t), &p_bottom, &self.p_side1, &self.p_side2)
                {
                    Some(t)
                } else {
                    None
                }
            }
            None => None,
        }
    }
    fn material(&self) -> Rc<dyn Material> {
        Rc::clone(&self.material)
    }
    fn normal(&self, _p: &Point) -> Vector3D {
        self.get_normal()
    }
}

#[derive(Default)]
pub struct Viewport {
    pub width: f64,
    pub height: f64,
    pub pixel_x: usize,
    pub pixel_y: usize,
    pub focal: f64,
}

impl Viewport {
    pub fn new(width: f64, height: f64, pixel_x: usize, pixel_y: usize) -> Self {
        Self {
            width,
            height,
            pixel_x,
            pixel_y,
            focal: -1.,
        }
    }
    pub fn get_ray_central(&self, x: usize, y: usize) -> Ray {
        let mut start = Vector3D::new(-self.width * 0.5, self.height * 0.5, self.focal);

        let x_unit = self.width / self.pixel_x as f64;
        let y_unit = self.height / self.pixel_y as f64;
        start.x += x_unit * x as f64 + 0.5 * x_unit;
        start.y -= y_unit * y as f64 + 0.5 * y_unit;
        let direction = start - Vector3D::new(0., 0., 0.);
        Ray::new(Point::origin_point(), direction)
    }
    pub fn get_ray_random(&self, x: usize, y: usize) -> Ray {
        let mut start = Vector3D::new(-self.width * 0.5, self.height * 0.5, self.focal);
        let x_unit = self.width / self.pixel_x as f64;
        let y_unit = self.height / self.pixel_y as f64;

        let mut rng = rand::thread_rng();
        start.x += x_unit * x as f64 + rng.gen_range(0.0..x_unit);
        start.y -= y_unit * y as f64 + rng.gen_range(0.0..y_unit);
        let direction = start - Vector3D::new(0., 0., 0.);
        Ray::new(Point::origin_point(), direction)
    }
}
