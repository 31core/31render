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
        /* calculate SAH cost */
        fn calculate_cost(aa: &Point, bb: &Point, objects: usize) -> f64 {
            (((aa.x() - bb.x()) * (aa.y() - bb.y())).abs()
                + ((aa.x() - bb.x()) * (aa.z() - bb.z())).abs()
                + ((aa.y() - bb.y()) * (aa.z() - bb.z())).abs())
                * objects as f64
        }
        let mut objects = objects.to_vec();
        let mut node = Self::default();
        (node.min, node.max) = get_aabb(&objects);
        if objects.len() <= capability {
            node.objects = objects;
        } else {
            let mut cost = 0.;
            let mut left_objects = None;
            let mut right_objects = None;

            objects.sort_by(|a, b| {
                if (a.x_min() + a.x_max()) / 2. > (b.x_min() + b.x_max()) / 2. {
                    Greater
                } else {
                    Less
                }
            });
            for i in 1..objects.len() {
                let (aa, bb) = get_aabb(&objects[..i]);
                let mut this_cost = calculate_cost(&aa, &bb, i);
                let (aa, bb) = get_aabb(&objects[i..]);
                this_cost += calculate_cost(&aa, &bb, objects.len() - i);
                if this_cost < cost || cost == 0. {
                    cost = this_cost;
                    left_objects = Some(&objects[..i]);
                    right_objects = Some(&objects[i..]);
                }
            }

            let mut objects = objects.clone();
            objects.sort_by(|a, b| {
                if (a.y_min() + a.y_max()) / 2. > (b.y_min() + b.y_max()) / 2. {
                    Greater
                } else {
                    Less
                }
            });
            for i in 1..objects.len() {
                let (aa, bb) = get_aabb(&objects[..i]);
                let mut this_cost = calculate_cost(&aa, &bb, i);
                let (aa, bb) = get_aabb(&objects[i..]);
                this_cost += calculate_cost(&aa, &bb, objects.len() - i);
                if this_cost < cost {
                    cost = this_cost;
                    left_objects = Some(&objects[..i]);
                    right_objects = Some(&objects[i..]);
                }
            }

            let mut objects = objects.clone();
            objects.sort_by(|a, b| {
                if (a.z_min() + a.z_max()) / 2. > (b.z_min() + b.z_max()) / 2. {
                    Greater
                } else {
                    Less
                }
            });
            for i in 1..objects.len() {
                let (aa, bb) = get_aabb(&objects[..i]);
                let mut this_cost = calculate_cost(&aa, &bb, i);
                let (aa, bb) = get_aabb(&objects[i..]);
                this_cost += calculate_cost(&aa, &bb, objects.len() - i);
                if this_cost < cost {
                    cost = this_cost;
                    left_objects = Some(&objects[..i]);
                    right_objects = Some(&objects[i..]);
                }
            }

            let subnode = Self::build(left_objects.unwrap(), capability);
            node.nodes.push(subnode);
            let subnode = Self::build(right_objects.unwrap(), capability);
            node.nodes.push(subnode);
        }
        node
    }

    /**
     * Check if a ray hits this AABB Box.
     */
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

    /**
     * Search the whole tree to find the closest object to hit.
     */
    pub fn find_closest_hit(&self, ray: &Ray) -> Option<(f64, Rc<dyn Object>)> {
        if !self.hit(ray) {
            return None;
        }
        if self.nodes.is_empty() {
            return find_closest_hit(ray, &self.objects);
        }

        let mut closest_hit = None;
        for bvh in &self.nodes {
            if bvh.hit(ray) {
                let hit = bvh.find_closest_hit(ray);
                if closest_hit.is_none() {
                    closest_hit = hit;
                } else if let Some((t, _)) = &hit {
                    if *t < closest_hit.clone().unwrap().0 {
                        closest_hit = hit;
                    }
                }
            }
        }
        closest_hit
    }
}
