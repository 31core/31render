mod color;
mod cordinate;
mod material;
mod ray;
mod vector;

use cordinate::*;

fn main() {
    const SIZE_X: usize = 1920;
    const SIZE_Y: usize = 1080;
    let mut ppm = ppm::Image::new(SIZE_X, SIZE_Y);

    let view = Viewport::new(4, 2, SIZE_X, SIZE_Y);
    let mut objects: Vec<Box<dyn Object>> = Vec::new();
    let m = material::Glass::default();
    let s = Sphere::new(Point::new(0., 0., -1.), 0.5, m);
    objects.push(Box::new(s));
    let s = Sphere::new(Point::new(1.2, 0., -2.), 0.7, material::Metal::default());
    objects.push(Box::new(s));
    for y in 0..SIZE_Y {
        for x in 0..SIZE_X {
            let ray = view.get_ray(x, y);
            let color = ray.trace(&objects, 5);
            ppm.set_pixel(x, y, color.to_rgb());
        }
    }

    ppm.save("out.ppm", ppm::PPMType::P3).unwrap();
}
