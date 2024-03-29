mod bvh;
mod color;
mod material;
mod objects;
mod point;
mod ray;
mod render;
mod vector;
mod viewport;

use mtl::material::Material;
use obj::element::*;
use objects::*;
use point::Point;
use std::rc::Rc;
use viewport::ViewportBuilder;

const SIZE_X: usize = 1920;
const SIZE_Y: usize = 1080;

fn get_fuzz(materials: &[Material]) -> f64 {
    for mtl in materials {
        if let Material::Ns(ns) = mtl {
            return -0.001 * *ns + 1.; // map ns (from 0 to 1000) to fuzz (from 1 to 0)
        }
    }
    0.
}

fn main() -> std::io::Result<()> {
    let elements = obj::parser::parse_obj(&std::fs::read_to_string("untitled.obj")?);
    let mut objects: Vec<Rc<dyn Object>> = Vec::new();

    for element in &elements {
        if let Some(face) = element.downcast_ref::<Face>() {
            if face.vertexes.len() == 3 {
                let fuzz = get_fuzz(&face.materials);
                let mut metal = material::Material::new_metal();
                metal.fuzz = fuzz;
                objects.push(Rc::new(Triangle::from_obj(face, metal)));
            } else {
                let fuzz = get_fuzz(&face.materials);
                let mut metal = material::Material::new_metal();
                metal.fuzz = fuzz;
                objects.push(Rc::new(Polygon::from_obj(face, metal)));
            }
        }
    }

    let bvh = bvh::BVHNode::build(&objects, 10);

    let viewport = ViewportBuilder::default()
        .origin(Point::origin_point())
        .at(vector::Vector3D::new(0., 0., 1.))
        .size(SIZE_X, SIZE_Y)
        .area(4., 4. * 8. / 16.)
        .scale(0.)
        .build();

    let render = render::RenderBuilder::default()
        .viewport(viewport)
        .sample(1)
        .max_depth(10)
        .build();

    render.render(&bvh).save("out.ppm", ppm::PPMType::P6)?;
    Ok(())
}
