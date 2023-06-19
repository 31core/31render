use crate::material::Material;
use std::collections::HashMap;

pub fn parse_mtl(mtl_content: &str) -> HashMap<String, Vec<Material>> {
    let mtl_content = mtl_content.replace('\n', " ");
    let tokens = mtl_content.split(' ').collect::<Vec<&str>>();
    let mut materials: HashMap<String, Vec<Material>> = HashMap::new();
    let mut current_mtl = String::new();

    let mut t = 0;
    while t < tokens.len() {
        if tokens[t] == "Ns" {
            let value = tokens[t + 1].parse().unwrap();
            let ns = Material::Ns(value);
            materials.get_mut(&current_mtl).unwrap().push(ns);
            t += 1;
        }
        if tokens[t] == "Ni" {
            let value = tokens[t + 1].parse().unwrap();
            let ns = Material::Ni(value);
            materials.get_mut(&current_mtl).unwrap().push(ns);
            t += 1;
        }
        if tokens[t] == "newmtl" {
            current_mtl = tokens[t + 1].parse().unwrap();
            materials.insert(current_mtl.clone(), Vec::new());
            t += 1;
        }
        t += 1;
    }
    materials
}
