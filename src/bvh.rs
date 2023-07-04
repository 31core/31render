use crate::coordinate::*;
use crate::ray::Ray;
use std::cmp::Ordering::*;
use std::rc::Rc;

fn get_aabb(objects: &[Rc<dyn Object>]) -> (Point, Point) {
    let mut aa = Point::new(objects[0].x_min(), objects[0].y_min(), objects[0].z_min());
    let mut bb = Point::new(objects[0].x_max(), objects[0].y_max(), objects[0].z_max());
    for o in &objects[1..] {
        if o.x_min() < aa.x() {
            aa.vector.x = o.x_min();
        }
        if o.y_min() < aa.y() {
            aa.vector.y = o.y_min();
        }
        if o.z_min() < aa.z() {
            aa.vector.z = o.z_min();
        }
        if o.x_max() > bb.x() {
            bb.vector.x = o.x_max();
        }
        if o.y_max() > bb.y() {
            bb.vector.y = o.y_max();
        }
        if o.z_max() > bb.z() {
            bb.vector.z = o.z_max();
        }
    }
    (aa, bb)
}

pub trait BoarderDedection {
    fn x_max(&self) -> f64;
    fn x_min(&self) -> f64;
    fn y_max(&self) -> f64;
    fn y_min(&self) -> f64;
    fn z_max(&self) -> f64;
    fn z_min(&self) -> f64;
}

#[derive(Default)]
pub struct BVHNode {
    pub nodes: Vec<BVHNode>,
    pub max: Point,
    pub min: Point,
    pub objects: Vec<Rc<dyn Object>>,
}

macro_rules! order2 {
    ($a: expr, $b: expr) => {
        if $a > $b {
            ($a, $b)
        } else {
            ($b, $a)
        }
    };
}

impl BVHNode {
    pub fn build(objects: &[Rc<dyn Object>], capability: usize) -> Self {
        let mut objects = objects.to_vec();
        let mut node = Self::default();
        (node.min, node.max) = get_aabb(&objects);
        if objects.len() < capability {
            node.objects = objects;
        } else {
            let x_len = node.max.x() - node.min.x();
            let y_len = node.max.y() - node.min.y();
            let z_len = node.max.z() - node.min.z();
            if x_len > y_len && x_len > z_len {
                objects.sort_by(|a, b| if a.x_max() > b.x_max() { Greater } else { Less });
            } else if y_len > z_len {
                objects.sort_by(|a, b| if a.y_max() > b.y_max() { Greater } else { Less });
            } else {
                objects.sort_by(|a, b| if a.z_max() > b.z_max() { Greater } else { Less });
            }

            let subnode = Self::build(&objects[0..objects.len() / 2], capability);
            node.nodes.push(subnode);
            let subnode = Self::build(&objects[objects.len() / 2..], capability);
            node.nodes.push(subnode);
        }
        node
    }
    pub fn hit(&self, ray: &Ray) -> bool {
        fn max3(a: f64, b: f64, c: f64) -> f64 {
            if a > b && a > c {
                a
            } else if b > c {
                b
            } else {
                c
            }
        }
        fn min3(a: f64, b: f64, c: f64) -> f64 {
            if a < b && a < c && a > 0. {
                a
            } else if b < c && b > 0. {
                b
            } else {
                c
            }
        }
        if ray.origin.x() > self.min.x()
            && ray.origin.x() < self.max.x()
            && ray.origin.y() > self.min.y()
            && ray.origin.y() < self.max.y()
            && ray.origin.z() > self.min.z()
            && ray.origin.z() < self.max.z()
        {
            return true;
        }
        let (x_far, x_near) = order2!(
            (self.max.x() - ray.origin.x()) / ray.direction.x,
            (self.min.x() - ray.origin.x()) / ray.direction.x
        );
        let (y_far, y_near) = order2!(
            (self.max.y() - ray.origin.y()) / ray.direction.y,
            (self.min.y() - ray.origin.y()) / ray.direction.y
        );
        let (z_far, z_near) = order2!(
            (self.max.z() - ray.origin.z()) / ray.direction.z,
            (self.min.z() - ray.origin.z()) / ray.direction.z
        );
        let t_max = max3(x_near, y_near, z_near);
        let t_min = min3(x_far, y_far, z_far);
        t_max >= 0. && t_min > 0. && t_max <= t_min
    }
    pub fn find_objects(&self, ray: &Ray) -> Vec<Rc<dyn Object>> {
        if !self.hit(ray) {
            return Vec::new();
        }
        if self.nodes.is_empty() {
            return self.objects.clone();
        }
        let mut objects = Vec::new();
        for bvh in &self.nodes {
            if bvh.hit(ray) {
                objects.extend(bvh.find_objects(ray));
            }
        }
        objects
    }
}
