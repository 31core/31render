mod color;
mod cordinate;
mod ray;
mod vector;

use cordinate::*;

fn main() {
    const SIZE_X: usize = 1920;
    const SIZE_Y: usize = 1080;
    let mut ppm = ppm::Image::new(SIZE_X, SIZE_Y);

    let view = Viewport::new(4, 2, SIZE_X, SIZE_Y);
    let mut objects = Vec::new();
    let s = Sphere::new(Point::new(0., 1., -3.), 0.5);
    objects.push(s);
    let s = Sphere::new(Point::new(-1., 0., -1.), 0.5);
    objects.push(s);
    for y in 0..SIZE_Y {
        for x in 0..SIZE_X {
            let ray = view.get_ray(x, y);
            let color = ray.trace(&objects, 5);
            ppm.set_pixel(x, y, color.to_rgb());
        }
    }

    ppm.save("out.ppm", ppm::PPMType::P3).unwrap();
}
