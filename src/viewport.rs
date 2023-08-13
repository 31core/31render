use crate::point::Point;
use crate::ray::Ray;
use crate::vector::Vector3D;

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
