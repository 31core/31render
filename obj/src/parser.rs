use crate::element::*;
use crate::vertex::*;
use std::any::Any;

fn get_v_index(param: &str) -> usize {
    param
        .split('/')
        .collect::<Vec<&str>>()
        .get(0)
        .unwrap()
        .parse::<usize>()
        .unwrap()
}

pub fn parse_obj(obj_content: &str) -> Vec<Box<dyn Any + 'static>> {
    let obj_content = obj_content.replace('\n', " ");
    let tokens = obj_content.split(' ').collect::<Vec<&str>>();
    let mut objects: Vec<Box<dyn Any + 'static>> = Vec::new();
    let mut vertexes = Vec::new();

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
            let mut last_vertex = 0;
            let mut f = Face::default();
            loop {
                t += 1;
                if !tokens[t].is_empty() {
                    match last_vertex {
                        0 => {
                            f.vertexes.0 = vertexes.get(get_v_index(tokens[t]) - 1).unwrap().clone()
                        }
                        1 => {
                            f.vertexes.1 = vertexes.get(get_v_index(tokens[t]) - 1).unwrap().clone()
                        }
                        2 => {
                            f.vertexes.2 = vertexes.get(get_v_index(tokens[t]) - 1).unwrap().clone()
                        }
                        _ => {}
                    }
                    last_vertex += 1;
                    if last_vertex == 3 {
                        break;
                    }
                }
            }
            objects.push(Box::new(f));
        }
        t += 1;
    }
    objects
}
