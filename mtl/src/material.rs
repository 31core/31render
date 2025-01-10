#[derive(Clone, Copy, Debug)]
pub enum Material {
    Ns(f64),
    Ni(f64),
    Kd(f64, f64, f64),
    D(f64),
}
