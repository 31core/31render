use super::color::Color;
use super::coordinate::*;
use std::rc::Rc;

pub struct Render {
    viewport: Viewport,
    sample: usize,
    max_depth: usize,
}

impl Render {
    pub fn rend(&self, objects: &[Rc<dyn Object>]) -> ppm::Image {
        let mut image = ppm::Image::new(self.viewport.pixel_x, self.viewport.pixel_y);

        for y in 0..self.viewport.pixel_y {
            for x in 0..self.viewport.pixel_x {
                if self.sample > 1 {
                    let mut color = Color::new();
                    color.color_vec = super::vector::Vector3D::new(0., 0., 0.);
                    for _ in 0..self.sample {
                        let ray = self.viewport.get_ray_random(x, y);
                        color.color_vec += ray.trace(objects, self.max_depth).color_vec;
                    }
                    color.color_vec = color.color_vec / self.sample as f64;
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

#[derive(Default)]
pub struct RenderBuilder {
    viewport: Viewport,
    sample: usize,
    max_depth: usize,
}

impl RenderBuilder {
    pub fn sample(mut self, sample: usize) -> Self {
        self.sample = sample;
        self
    }
    pub fn max_depth(mut self, max_depth: usize) -> Self {
        self.max_depth = max_depth;
        self
    }
    pub fn viewport(mut self, width: f64, height: f64, pixel_x: usize, pixel_y: usize) -> Self {
        self.viewport = Viewport::new(width, height, pixel_x, pixel_y);
        self
    }
    pub fn build(self) -> Render {
        Render {
            viewport: self.viewport,
            sample: self.sample,
            max_depth: self.max_depth,
        }
    }
}
