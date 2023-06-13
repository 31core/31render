use crate::vertex::Vertex;

#[derive(Default, Debug)]
pub struct Face {
    pub vertexes: (Vertex, Vertex, Vertex),
}
