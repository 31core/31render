use std::io::Result as IOResult;
use std::io::Write;

pub enum PPMType {
    P3,
    P6,
}

#[derive(Default, Debug, Clone)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

pub struct Image {
    width: usize,
    height: usize,
    pixels: Vec<Pixel>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Pixel::default(); width * height],
        }
    }
    pub fn save(&self, path: &str, ppm_type: PPMType) -> IOResult<()> {
        let mut f = std::fs::File::create(path)?;
        match ppm_type {
            PPMType::P3 => f.write_all("P3\n".as_bytes())?,
            PPMType::P6 => f.write_all("P6\n".as_bytes())?,
        }
        f.write_all(format!("{} {}\n", self.width, self.height).as_bytes())?;
        f.write_all("255\n".as_bytes())?;
        for pixel in &self.pixels {
            match ppm_type {
                PPMType::P3 => {
                    f.write_all(format!("{} {} {}\n", pixel.r, pixel.g, pixel.b).as_bytes())?;
                }
                PPMType::P6 => {
                    f.write_all(&[pixel.r as u8, pixel.g as u8, pixel.b as u8])?;
                }
            }
        }
        Ok(())
    }
    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: Pixel) {
        self.pixels[y * self.width + x] = pixel;
    }
}
