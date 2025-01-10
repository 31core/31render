use std::io::{Result as IOResult, Write};

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
                    f.write_all(&[pixel.r, pixel.g, pixel.b])?;
                }
            }
        }
        Ok(())
    }
    pub fn load(&mut self, path: &str) -> IOResult<()> {
        let data = std::fs::read(path)?;

        let datas = data.split(|b| *b == b'\n').collect::<Vec<&[u8]>>();
        let ppm_type = String::from_utf8(datas[0].to_vec()).unwrap();
        if ppm_type == "P3" {
            let mut pixel = 0;
            for i in &datas[3..] {
                if i.is_empty() {
                    continue;
                }
                let rgb_str = String::from_utf8(i.to_vec()).unwrap();
                let rgb = rgb_str.split(' ').collect::<Vec<&str>>();
                let r = rgb[0].parse::<u8>().unwrap();
                let g = rgb[1].parse::<u8>().unwrap();
                let b = rgb[2].parse::<u8>().unwrap();
                self.pixels[pixel] = Pixel::new(r, g, b);
                pixel += 1;
            }
        } else if ppm_type == "P6" {
            /* bytes in pixels may include 0x0a (\n), so we need to merge these parts */
            let mut ppm_data: Vec<u8> = Vec::new();
            for i in &datas[3..] {
                ppm_data.extend(*i);
                ppm_data.push(b'\n');
            }
            ppm_data.pop();

            let mut rgb = Vec::new();
            let mut pixel = 0;
            for i in ppm_data {
                rgb.push(i);
                if rgb.len() == 3 {
                    self.pixels[pixel] = Pixel::new(rgb[0], rgb[1], rgb[2]);
                    rgb.clear();
                    pixel += 1;
                }
            }
        }
        Ok(())
    }
    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: Pixel) {
        self.pixels[y * self.width + x] = pixel;
    }
}
