use crate::{point::Point, vector::Vector3D};

const DEFAULT_WIDTH: usize = 1920;
const DEFAULT_HEIGHT: usize = 1080;

#[derive(Debug)]
pub enum Instruction {
    Camera {
        x: f64,
        y: f64,
        z: f64,
    },
    CameraAt {
        x: f64,
        y: f64,
        z: f64,
    },
    CameraScale(f64),
    Size {
        width: usize,
        height: usize,
    },
    LoadObj(String),
    LoadMtl(String),
    AddSphere {
        x: f64,
        y: f64,
        z: f64,
        raius: f64,
        material: String,
    },
}

#[derive(Default, Debug)]
pub struct Script {
    pub instructions: Vec<Instruction>,
}

impl Script {
    pub fn parse(script_src: &str) -> Script {
        let mut script = Script::default();

        for ins in script_src.split('\n') {
            let line = ins.split(' ').collect::<Vec<&str>>();
            match line[0] {
                "camera" => script.instructions.push(Instruction::Camera {
                    x: line[1].parse().unwrap(),
                    y: line[2].parse().unwrap(),
                    z: line[3].parse().unwrap(),
                }),
                "camera-at" => script.instructions.push(Instruction::CameraAt {
                    x: line[1].parse().unwrap(),
                    y: line[2].parse().unwrap(),
                    z: line[3].parse().unwrap(),
                }),
                "camera-scale" => script
                    .instructions
                    .push(Instruction::CameraScale(line[1].parse().unwrap())),
                "size" => script.instructions.push(Instruction::Size {
                    width: line[1].parse().unwrap(),
                    height: line[2].parse().unwrap(),
                }),
                "load-obj" => script
                    .instructions
                    .push(Instruction::LoadObj(line[1].to_owned())),
                "load-mtl" => script
                    .instructions
                    .push(Instruction::LoadMtl(line[1].to_owned())),
                "add-sphere" => script.instructions.push(Instruction::AddSphere {
                    x: line[1].parse().unwrap(),
                    y: line[2].parse().unwrap(),
                    z: line[3].parse().unwrap(),
                    raius: line[4].parse().unwrap(),
                    material: line[5].to_owned(),
                }),
                _ => {}
            }
        }

        script
    }
    pub fn get_camera(&self) -> Point {
        for i in &self.instructions {
            if let Instruction::Camera { x, y, z } = i {
                return Point::new(*x, *y, *z);
            }
        }

        Point::origin_point()
    }
    pub fn get_camera_at(&self) -> Vector3D {
        for i in &self.instructions {
            if let Instruction::CameraAt { x, y, z } = i {
                return Vector3D::new(*x, *y, *z);
            }
        }

        Vector3D::new(0., 0., -1.)
    }
    pub fn get_camera_scale(&self) -> f64 {
        for i in &self.instructions {
            if let Instruction::CameraScale(scale) = i {
                return *scale;
            }
        }

        0.
    }
    pub fn get_size(&self) -> (usize, usize) {
        for i in &self.instructions {
            if let Instruction::Size { width, height } = i {
                return (*width, *height);
            }
        }

        (DEFAULT_WIDTH, DEFAULT_HEIGHT)
    }
}
