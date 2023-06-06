mod color;
mod cordinate;
mod material;
mod ray;
mod render;
mod vector;

use cordinate::*;
use std::rc::Rc;
fn main() {
    const SIZE_X: usize = 1920;
    const SIZE_Y: usize = 1080;

    let render = render::Render::new(4., 4. * 8. / 16., SIZE_X, SIZE_Y);
    let mut objects: Vec<Rc<dyn Object>> = Vec::new();

    /* left */
    let m = material::Glass {
        attenuation: (0.9, 0.9, 0.9),
        ..Default::default()
    };
    let s = Sphere::new(Point::new(-1., 0., -2.), 0.5, m);
    objects.push(Rc::new(s));

    /* central */
    let m = material::Metal {
        attenuation: (0.8, 0.4, 0.8),
        fuzz: 5.,
    };
    let s = Sphere::new(Point::new(0., 0., -2.), 0.5, m);
    objects.push(Rc::new(s));

    /* right */
    let m = material::Metal {
        attenuation: (0.8, 0.8, 0.8),
        fuzz: 0.7,
    };
    let s = Sphere::new(Point::new(1., 0., -2.), 0.5, m);
    objects.push(Rc::new(s));

    let m = material::Metal {
        attenuation: (0.8, 0.8, 0.8),
        ..Default::default()
    };
    let s = Sphere::new(Point::new(1.2, -520., -50.), 500., m);
    objects.push(Rc::new(s));

    render
        .rend(&objects)
        .save("out.ppm", ppm::PPMType::P3)
        .unwrap();
}
