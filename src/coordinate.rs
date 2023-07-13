use crate::bvh::BoarderDedection;
use crate::material::*;
use crate::ray::Ray;
use crate::vector::Vector3D;
use std::rc::Rc;

macro_rules! max {
    ($a: expr, $b: expr, $c: expr) => {
        if $a > $b && $a > $c {
            $a
        } else if $b > $c {
            $b
        } else {
            $c
        }
    };
}

macro_rules! min {
    ($a: expr, $b: expr, $c: expr) => {
        if $a < $b && $a < $c {
            $a
        } else if $b < $c {
            $b
        } else {
            $c
        }
    };
}

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

pub trait Object: crate::bvh::BoarderDedection {
    fn hit(&self, r: &Ray) -> Option<f64>;
    fn normal(&self, p: &Point) -> Vector3D;
    fn material(&self) -> Rc<dyn Material>;
}

#[derive(Clone, Debug, Default)]
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
    pub fn x(&self) -> f64 {
        self.vector.x
    }
    pub fn y(&self) -> f64 {
        self.vector.y
    }
    pub fn z(&self) -> f64 {
        self.vector.z
    }
    pub fn to_vec3d(&self, other: &Self) -> Vector3D {
        -self.vector + other.vector
    }
    pub fn from_vec3d(vector: Vector3D) -> Self {
        Self { vector }
    }
    pub fn from_obj(v: &obj::vertex::Vertex) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

pub struct Sphere {
    center: Point,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    #[allow(dead_code)]
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

impl BoarderDedection for Sphere {
    fn x_max(&self) -> f64 {
        self.center.x() + self.radius
    }
    fn x_min(&self) -> f64 {
        self.center.x() - self.radius
    }
    fn y_max(&self) -> f64 {
        self.center.y() + self.radius
    }
    fn y_min(&self) -> f64 {
        self.center.y() - self.radius
    }
    fn z_max(&self) -> f64 {
        self.center.z() + self.radius
    }
    fn z_min(&self) -> f64 {
        self.center.z() - self.radius
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
            /* if the light source is on the sphere */
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
    pub fn from_obj<M>(face: &obj::element::Face, material: M) -> Self
    where
        M: Material + 'static,
    {
        Self::new(
            Point::from_obj(&face.vertexes[0]),
            Point::from_obj(&face.vertexes[1]),
            Point::from_obj(&face.vertexes[2]),
            material,
        )
    }
    fn get_normal(&self) -> Vector3D {
        (self.p1.to_vec3d(&self.p2) * self.p1.to_vec3d(&self.p3)).unit()
    }
}

impl BoarderDedection for Triangle {
    fn x_max(&self) -> f64 {
        max!(self.p1.x(), self.p2.x(), self.p3.x())
    }
    fn x_min(&self) -> f64 {
        min!(self.p1.x(), self.p2.x(), self.p3.x())
    }
    fn y_max(&self) -> f64 {
        max!(self.p1.y(), self.p2.y(), self.p3.y())
    }
    fn y_min(&self) -> f64 {
        min!(self.p1.y(), self.p2.y(), self.p3.y())
    }
    fn z_max(&self) -> f64 {
        max!(self.p1.z(), self.p2.z(), self.p3.z())
    }
    fn z_min(&self) -> f64 {
        min!(self.p1.z(), self.p2.z(), self.p3.z())
    }
}

impl Object for Triangle {
    fn hit(&self, r: &Ray) -> Option<f64> {
        let vec_a = r.origin.to_vec3d(&self.p1);
        let vec_b = r.origin.to_vec3d(&self.p2);
        let vec_c = r.origin.to_vec3d(&self.p3);
        /*
        d = |x_a x_b x_c|
            |y_a y_b y_c|
            |z_a z_b z_c|
         */
        let d =
            vec_a.x * vec_b.y * vec_c.z + vec_b.x * vec_c.y * vec_a.z + vec_a.y * vec_b.z * vec_c.x
                - vec_c.x * vec_b.y * vec_a.z
                - vec_b.x * vec_a.y * vec_c.z
                - vec_c.y * vec_b.z * vec_a.x;
        let u = (r.direction.x * vec_b.y * vec_c.z
            + vec_b.x * vec_c.y * r.direction.z
            + r.direction.y * vec_b.z * vec_c.x
            - vec_c.x * vec_b.y * r.direction.z
            - vec_b.x * r.direction.y * vec_c.z
            - vec_c.y * vec_b.z * r.direction.x)
            / d;
        if u < 0. {
            return None;
        }
        let v = (vec_a.x * r.direction.y * vec_c.z
            + r.direction.x * vec_c.y * vec_a.z
            + vec_a.y * r.direction.z * vec_c.x
            - vec_c.x * r.direction.y * vec_a.z
            - r.direction.x * vec_a.y * vec_c.z
            - vec_c.y * r.direction.z * vec_a.x)
            / d;
        if v < 0. {
            return None;
        }
        let w = (vec_a.x * vec_b.y * r.direction.z
            + vec_b.x * r.direction.y * vec_a.z
            + vec_a.y * vec_b.z * r.direction.x
            - r.direction.x * vec_b.y * vec_a.z
            - vec_b.x * vec_a.y * r.direction.z
            - r.direction.y * vec_b.z * vec_a.x)
            / d;
        if w < 0. {
            return None;
        }
        plane_hit(r, &self.p1, &self.normal(&self.p1))
    }
    fn material(&self) -> Rc<dyn Material> {
        Rc::clone(&self.material)
    }
    fn normal(&self, _p: &Point) -> Vector3D {
        self.get_normal()
    }
}

pub struct Polygon {
    triangles: Vec<Triangle>,
    material: Rc<dyn Material>,
}

impl Polygon {
    pub fn new<M>(points: &[Point], material: M) -> Self
    where
        M: Material + Copy + 'static,
    {
        let mut triangles = Vec::new();
        let mut p = 1;
        while p + 1 < points.len() {
            triangles.push(Triangle::new(
                points[0].clone(),
                points[p].clone(),
                points[p + 1].clone(),
                material,
            ));
            p += 1;
        }
        Self {
            triangles,
            material: Rc::new(material),
        }
    }
    pub fn from_obj<M>(face: &obj::element::Face, material: M) -> Self
    where
        M: Material + Copy + 'static,
    {
        let mut points = Vec::new();
        for v in &face.vertexes {
            points.push(Point::from_obj(v));
        }
        Self::new(&points, material)
    }
}

impl BoarderDedection for Polygon {
    fn x_max(&self) -> f64 {
        let mut max = self.triangles[0].x_max();
        for tri in &self.triangles {
            if tri.x_max() > max {
                max = tri.x_max();
            }
        }
        max
    }
    fn x_min(&self) -> f64 {
        let mut min = self.triangles[0].x_min();
        for tri in &self.triangles {
            if tri.x_min() > min {
                min = tri.x_min();
            }
        }
        min
    }
    fn y_max(&self) -> f64 {
        let mut max = self.triangles[0].y_max();
        for tri in &self.triangles {
            if tri.y_max() > max {
                max = tri.y_max();
            }
        }
        max
    }
    fn y_min(&self) -> f64 {
        let mut min = self.triangles[0].y_min();
        for tri in &self.triangles {
            if tri.y_min() > min {
                min = tri.y_min();
            }
        }
        min
    }
    fn z_max(&self) -> f64 {
        let mut max = self.triangles[0].z_max();
        for tri in &self.triangles {
            if tri.z_max() > max {
                max = tri.z_max();
            }
        }
        max
    }
    fn z_min(&self) -> f64 {
        let mut min = self.triangles[0].z_min();
        for tri in &self.triangles {
            if tri.z_min() > min {
                min = tri.z_min();
            }
        }
        min
    }
}

impl Object for Polygon {
    fn hit(&self, r: &Ray) -> Option<f64> {
        for t in &self.triangles {
            if let Some(t) = t.hit(r) {
                return Some(t);
            }
        }
        None
    }
    fn material(&self) -> Rc<dyn Material> {
        Rc::clone(&self.material)
    }
    fn normal(&self, _p: &Point) -> Vector3D {
        self.triangles[0].get_normal()
    }
}

#[derive(Default)]
pub struct Viewport {
    pub width: f64,
    pub height: f64,
    pub pixel_x: usize,
    pub pixel_y: usize,

    pub origin: Point,
    pub at: Vector3D,
    pub top: Vector3D,
    pub left: Vector3D,

    unit: f64,
}

impl Viewport {
    /**
     * at: the direction of eye. (NOTE: this is not a unit vector, the module of `at` is focal distance.)
     */
    pub fn new(
        width: f64,
        height: f64,
        pixel_x: usize,
        pixel_y: usize,
        origin: Point,
        at: Vector3D,
        scale: f64,
    ) -> Self {
        let top = {
            let at_u = at.unit();
            let top = height / 2.
                * Vector3D::new(-at_u.x * at_u.y, -at_u.y.powi(2) + 1., -at_u.z * at_u.y).unit();
            let right = (at * top).unit();
            top + scale.atan() * top.module() * right.unit()
        };
        let left = width / 2. * -(at * top).unit();

        Self {
            width,
            height,
            pixel_x,
            pixel_y,
            origin,
            at,
            top,
            left,
            unit: if (width / pixel_x as f64) < height / pixel_y as f64 {
                width / pixel_x as f64
            } else {
                height / pixel_y as f64
            },
        }
    }
    pub fn get_ray_central(&self, x: usize, y: usize) -> Ray {
        let x = if x > self.pixel_x {
            (self.pixel_x / 2) as isize - (x - self.pixel_x) as isize
        } else {
            (self.pixel_x / 2) as isize - x as isize
        };
        let y = if y > self.pixel_y {
            (self.pixel_y / 2) as isize - (y - self.pixel_y) as isize
        } else {
            (self.pixel_y / 2) as isize - y as isize
        };
        let x_vec = x as f64 / (self.pixel_x as f64 / 2.) * self.left;
        let y_vec = y as f64 / (self.pixel_y as f64 / 2.) * self.top;
        let direction = self.at + x_vec + y_vec;
        Ray::new(self.origin.clone(), direction)
    }
    pub fn get_ray_random(&self, x: usize, y: usize) -> Ray {
        let x = if x > self.pixel_x {
            (self.pixel_x / 2) as isize - (x - self.pixel_x) as isize
        } else {
            (self.pixel_x / 2) as isize - x as isize
        };
        let y = if y > self.pixel_y {
            (self.pixel_y / 2) as isize - (y - self.pixel_y) as isize
        } else {
            (self.pixel_y / 2) as isize - y as isize
        };
        let x_vec = x as f64 / (self.pixel_x as f64 / 2.) * self.left;
        let y_vec = y as f64 / (self.pixel_y as f64 / 2.) * self.top;

        let direction = self.at + x_vec + y_vec + self.unit * Vector3D::new_random_unit();
        Ray::new(self.origin.clone(), direction)
    }
}

#[derive(Default)]
pub struct ViewportBuilder {
    width: f64,
    height: f64,
    pixel_x: usize,
    pixel_y: usize,

    origin: Point,
    at: Vector3D,
    scale: f64,
}

impl ViewportBuilder {
    pub fn build(self) -> Viewport {
        Viewport::new(
            self.width,
            self.height,
            self.pixel_x,
            self.pixel_y,
            self.origin,
            self.at,
            self.scale,
        )
    }
    pub fn at(mut self, at: Vector3D) -> Self {
        self.at = at;
        self
    }
    pub fn origin(mut self, origin: Point) -> Self {
        self.origin = origin;
        self
    }
    pub fn area(mut self, width: f64, height: f64) -> Self {
        self.width = width;
        self.height = height;
        self
    }
    pub fn size(mut self, pixel_x: usize, pixel_y: usize) -> Self {
        self.pixel_x = pixel_x;
        self.pixel_y = pixel_y;
        self
    }
    pub fn scale(mut self, scale: f64) -> Self {
        self.scale = scale;
        self
    }
}
