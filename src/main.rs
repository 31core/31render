mod bvh;
mod color;
mod material;
mod objects;
mod point;
mod ray;
mod render;
mod script;
mod vector;
mod viewport;

use clap::Parser;
use mtl::material::Material;
use obj::element::*;
use objects::*;
use script::*;
use std::io::Result as IOResult;
use std::rc::Rc;
use viewport::ViewportBuilder;

#[derive(Parser)]
struct Args {
    /// Script path
    #[arg(short = 'S')]
    script: String,
    /// Output path
    #[arg(short)]
    output: String,
    /// Samping
    #[arg(short, default_value_t = 100)]
    sampling: usize,
    /// Max depth
    #[arg(short = 'd', default_value_t = 10)]
    max_depth: usize,
}

fn get_fuzz(materials: &[Material]) -> f64 {
    for mtl in materials {
        if let Material::Ns(ns) = mtl {
            return -0.001 * *ns + 1.; // map ns (from 0 to 1000) to fuzz (from 1 to 0)
        }
    }
    0.
}

fn get_attenuation(materials: &[Material]) -> (f64, f64, f64) {
    for mtl in materials {
        if let Material::Kd(r, g, b) = mtl {
            return (*r, *g, *b);
        }
    }

    (1., 1., 1.)
}

fn load_obj(objects: &mut Vec<Rc<dyn Object>>, obj_file: &str) -> IOResult<()> {
    let elements = obj::parser::parse_obj(&std::fs::read_to_string(obj_file)?);

    for element in &elements {
        if let Some(face) = element.downcast_ref::<Face>() {
            if face.vertexes.len() == 3 {
                let mut metal = material::Material::new_metal();
                metal.fuzz = get_fuzz(&face.materials);
                metal.attenuation = get_attenuation(&face.materials);

                objects.push(Rc::new(Triangle::from_obj(face, metal)));
            } else {
                let mut metal = material::Material::new_metal();
                metal.fuzz = get_fuzz(&face.materials);
                metal.attenuation = get_attenuation(&face.materials);

                objects.push(Rc::new(Polygon::from_obj(face, metal)));
            }
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let script = script::Script::parse(&std::fs::read_to_string(args.script)?);

    let mut objects: Vec<Rc<dyn Object>> = Vec::new();

    for ins in &script.instructions {
        if let Instruction::LoadObj(obj_file) = ins {
            load_obj(&mut objects, obj_file)?;
        }
    }

    let bvh = bvh::BVHNode::build(&objects, 20);

    let viewport = ViewportBuilder::default()
        .origin(script.get_camera())
        .at(script.get_camera_at())
        .size(script.get_size())
        .area(4., 4. * 8. / 16.)
        .scale(script.get_camera_scale())
        .build();

    let render = render::RenderBuilder::default()
        .viewport(viewport)
        .sample(args.sampling)
        .max_depth(args.max_depth)
        .build();

    render.render(&bvh).save(&args.output, ppm::PPMType::P6)?;
    Ok(())
}
