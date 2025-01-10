use crate::material::Material;
use std::collections::HashMap;

fn find_next_token(tokens: &[&str]) -> usize {
    let mut t = 1;
    while t < tokens.len() && tokens[t].is_empty() {
        t += 1;
    }
    t
}

pub fn parse_mtl(mtl_content: &str) -> HashMap<String, Vec<Material>> {
    let mtl_content = mtl_content.replace('\n', " ");
    let tokens = mtl_content.split(' ').collect::<Vec<&str>>();
    let mut materials: HashMap<String, Vec<Material>> = HashMap::new();
    let mut current_mtl = String::new();

    let mut t = 0;
    while t < tokens.len() {
        match tokens[t] {
            "Ns" => {
                t += find_next_token(&tokens[t..]);
                let value = tokens[t].parse().unwrap();
                let ns = Material::Ns(value);
                materials.get_mut(&current_mtl).unwrap().push(ns);
            }
            "Ni" => {
                t += find_next_token(&tokens[t..]);
                let value = tokens[t].parse().unwrap();
                let ns = Material::Ni(value);
                materials.get_mut(&current_mtl).unwrap().push(ns);
            }
            "Kd" => {
                t += find_next_token(&tokens[t..]);
                let r = tokens[t].parse().unwrap();

                t += find_next_token(&tokens[t..]);
                let g = tokens[t].parse().unwrap();

                t += find_next_token(&tokens[t..]);
                let b = tokens[t].parse().unwrap();
                let ns = Material::Kd(r, g, b);
                materials.get_mut(&current_mtl).unwrap().push(ns);
            }
            "d" => {
                t += find_next_token(&tokens[t..]);
                let d = tokens[t].parse().unwrap();

                let ns = Material::D(d);
                materials.get_mut(&current_mtl).unwrap().push(ns);
            }
            "newmtl" => {
                t += find_next_token(&tokens[t..]);
                current_mtl = tokens[t].parse().unwrap();
                materials.insert(current_mtl.clone(), Vec::new());
            }
            _ => {}
        }
        t += find_next_token(&tokens[t..]);
    }
    materials
}
