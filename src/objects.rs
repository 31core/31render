use crate::bvh::BoarderDedection;
use crate::material::*;
use crate::point::Point;
use crate::ray::Ray;
use crate::vector::Vector3D;

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

pub trait Object: BoarderDedection {
    fn hit(&self, r: &Ray) -> Option<f64>;
    fn normal(&self, p: &Point) -> Vector3D;
    fn material(&self) -> Material;
}

pub struct Sphere {
    center: Point,
    radius: f64,
    material: Material,
}

impl Sphere {
    #[allow(dead_code)]
    pub fn new(center: Point, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
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
    fn material(&self) -> Material {
        self.material
    }
}

pub struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
    material: Material,
    normal_vec_cache: Vector3D,
}

impl Triangle {
    pub fn new(p1: Point, p2: Point, p3: Point, material: Material) -> Self {
        Self {
            p1,
            p2,
            p3,
            material,
            normal_vec_cache: (p1.to_vec3d(&p2) * p1.to_vec3d(&p3)).unit(),
        }
    }
    pub fn from_obj(face: &obj::element::Face, material: Material) -> Self {
        Self::new(
            Point::from_obj(&face.vertexes[0]),
            Point::from_obj(&face.vertexes[1]),
            Point::from_obj(&face.vertexes[2]),
            material,
        )
    }
    fn get_normal(&self) -> Vector3D {
        self.normal_vec_cache
    }
}

impl BoarderDedection for Triangle {
    fn x_max(&self) -> f64 {
        self.p1.x().max(self.p2.x().max(self.p3.x()))
    }
    fn x_min(&self) -> f64 {
        self.p1.x().min(self.p2.x().min(self.p3.x()))
    }
    fn y_max(&self) -> f64 {
        self.p1.y().max(self.p2.y().max(self.p3.y()))
    }
    fn y_min(&self) -> f64 {
        self.p1.y().min(self.p2.y().min(self.p3.y()))
    }
    fn z_max(&self) -> f64 {
        self.p1.z().max(self.p2.z().max(self.p3.z()))
    }
    fn z_min(&self) -> f64 {
        self.p1.z().min(self.p2.z().min(self.p3.z()))
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
    fn material(&self) -> Material {
        self.material
    }
    fn normal(&self, _p: &Point) -> Vector3D {
        self.get_normal()
    }
}

pub struct Polygon {
    triangles: Vec<Triangle>,
    material: Material,
}

impl Polygon {
    pub fn new(points: &[Point], material: Material) -> Self {
        let mut triangles = Vec::new();
        let mut p = 1;
        while p + 1 < points.len() {
            triangles.push(Triangle::new(
                points[0],
                points[p],
                points[p + 1],
                material,
            ));
            p += 1;
        }
        Self {
            triangles,
            material,
        }
    }
    pub fn from_obj(face: &obj::element::Face, material: Material) -> Self {
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
    fn material(&self) -> Material {
        self.material
    }
    fn normal(&self, _p: &Point) -> Vector3D {
        self.triangles[0].get_normal()
    }
}
