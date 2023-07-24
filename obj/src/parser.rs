use crate::element::*;
use crate::vertex::*;
use mtl::material::Material;
use std::any::Any;
use std::collections::HashMap;

fn find_next_token(tokens: &[&str]) -> usize {
    let mut t = 1;
    while t < tokens.len() && tokens[t].is_empty() {
        t += 1;
    }
    t
}

fn is_keyword(param: &str) -> bool {
    let types = ["v", "f", "o"];
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
        .first()
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
                t += find_next_token(&tokens[t..]);
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
                t += find_next_token(&tokens[t..]);
                if t == tokens.len() || is_keyword(tokens[t]) {
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
            t += find_next_token(&tokens[t..]);
            mtllib = mtl::parser::parse_mtl(&std::fs::read_to_string(tokens[t]).unwrap());
        }
        if tokens[t] == "usemtl" {
            t += find_next_token(&tokens[t..]);
            usemtl = tokens[t].to_owned();
        }
        t += find_next_token(&tokens[t..]);
    }
    objects
}
