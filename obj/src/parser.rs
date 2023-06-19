use crate::element::*;
use crate::vertex::*;
use mtl::material::Material;
use std::any::Any;
use std::collections::HashMap;

fn is_type(param: &str) -> bool {
    let types = ["v", "f"];
    for i in types {
        if param == i {
            return true;
        }
    }
    false
}

fn get_v_index(param: &str) -> usize {
    param
        .split('/')
        .collect::<Vec<&str>>()
        .get(0)
        .unwrap()
        .parse::<usize>()
        .unwrap()
}

pub fn parse_obj(obj_content: &str) -> Vec<Box<dyn Any>> {
    let obj_content = obj_content.replace('\n', " ");
    let tokens = obj_content.split(' ').collect::<Vec<&str>>();
    let mut objects: Vec<Box<dyn Any>> = Vec::new();
    let mut vertexes = Vec::new();
    let mut usemtl = String::new();
    let mut mtllib: HashMap<String, Vec<Material>> = std::collections::HashMap::new();

    let mut t = 0;
    while t < tokens.len() {
        if tokens[t] == "v" {
            let mut last_vertex = 0;
            let mut v = Vertex::default();
            loop {
                t += 1;
                if !tokens[t].is_empty() {
                    match last_vertex {
                        0 => v.x = tokens[t].parse::<f64>().unwrap(),
                        1 => v.y = tokens[t].parse::<f64>().unwrap(),
                        2 => v.z = tokens[t].parse::<f64>().unwrap(),
                        _ => {}
                    }
                    last_vertex += 1;
                    if last_vertex == 3 {
                        break;
                    }
                }
            }
            vertexes.push(v);
        }
        if tokens[t] == "f" {
            let mut f = Face::default();
            if !usemtl.is_empty() {
                f.materials = mtllib.get(&usemtl).unwrap().clone();
            }
            loop {
                t += 1;
                if t == tokens.len() || is_type(tokens[t]) {
                    t -= 1;
                    break;
                }
                if !tokens[t].is_empty() {
                    f.vertexes
                        .push(vertexes[get_v_index(tokens[t]) - 1].clone());
                }
            }
            objects.push(Box::new(f));
        }
        if tokens[t] == "mtllib" {
            mtllib = mtl::parser::parse_mtl(&std::fs::read_to_string(&tokens[t + 1]).unwrap());
            t += 1;
        }
        if tokens[t] == "usemtl" {
            usemtl = tokens[t + 1].to_owned();
            t += 1;
        }
        t += 1;
    }
    objects
}
