use crate::vertex::Vertex;
use mtl::material::Material;

#[derive(Default, Debug)]
pub struct Face {
    pub vertexes: Vec<Vertex>,
    pub materials: Vec<Material>,
}
