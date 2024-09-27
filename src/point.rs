use crate::vector::Vector3D;

#[derive(Clone, Copy, Debug, Default)]
pub struct Point {
    pub point_vec: Vector3D,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            point_vec: Vector3D::new(x, y, z),
        }
    }
    pub fn origin_point() -> Self {
        Self {
            point_vec: Vector3D::new(0., 0., 0.),
        }
    }
    pub fn x(&self) -> f64 {
        self.point_vec.x
    }
    pub fn y(&self) -> f64 {
        self.point_vec.y
    }
    pub fn z(&self) -> f64 {
        self.point_vec.z
    }
    pub fn set_x(&mut self, value: f64) {
        self.point_vec.x = value;
    }
    pub fn set_y(&mut self, value: f64) {
        self.point_vec.y = value;
    }
    pub fn set_z(&mut self, value: f64) {
        self.point_vec.z = value;
    }
    pub fn to_vec3d(self, other: &Self) -> Vector3D {
        -self.point_vec + other.point_vec
    }
    pub fn from_vec3d(point_vec: Vector3D) -> Self {
        Self { point_vec }
    }
    pub fn from_obj(v: &obj::vertex::Vertex) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}
