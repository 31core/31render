use super::color::Color;
use super::cordinate::*;
use std::rc::Rc;

pub struct Render {
    viewport: Viewport,
    sample: usize,
    max_depth: usize,
}

impl Render {
    pub fn new(width: f64, height: f64, pixel_x: usize, pixel_y: usize) -> Self {
        Render {
            viewport: Viewport::new(width, height, pixel_x, pixel_y),
            sample: 10,
            max_depth: 10,
        }
    }
    pub fn rend(&self, objects: &[Rc<dyn Object>]) -> ppm::Image {
        let mut image = ppm::Image::new(self.viewport.pixel_x, self.viewport.pixel_y);

        for y in 0..self.viewport.pixel_y {
            for x in 0..self.viewport.pixel_x {
                if self.sample > 1 {
                    let ray = self.viewport.get_ray_random(x, y);
                    let mut color = Color::new();
                    color.color_vec = super::vector::Vector3D::new(0., 0., 0.);
                    for i in 0..self.sample {
                        color.color_vec += ray.trace(objects, self.max_depth).color_vec;
                        if i > 0 {
                            color.color_vec = color.color_vec / 2.;
                        }
                    }
                    image.set_pixel(x, y, color.to_rgb());
                } else {
                    let ray = self.viewport.get_ray_central(x, y);
                    let color = ray.trace(objects, self.max_depth);
                    image.set_pixel(x, y, color.to_rgb());
                }
            }
        }
        image
    }
}
