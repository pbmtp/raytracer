use image::GenericImageView;

use crate::perlin::Perlin;
use crate::vec3::{Color, Point3};

const BYTES_PER_PIXEL: usize = 3;

// Generic trait
pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

// Uniform Color
pub struct SolidTexture {
    color: Color,
}

impl SolidTexture {
    pub fn new(r: f64, g: f64, b: f64) -> SolidTexture {
        SolidTexture {
            color: Color::new(r, g, b),
        }
    }
}

impl From<Color> for SolidTexture {
    fn from(color: Color) -> Self {
        Self::new(color.r(), color.g(), color.b())
    }
}

impl Texture for SolidTexture {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.color
    }
}

// Checker texture (alternating between two textures)
pub struct CheckerTexture {
    pub odd: Box<dyn Texture>,
    pub even: Box<dyn Texture>,
}

impl From<(Color, Color)> for CheckerTexture {
    fn from(tuple: (Color, Color)) -> Self {
        CheckerTexture {
            odd: Box::new(SolidTexture::from(tuple.0)),
            even: Box::new(SolidTexture::from(tuple.1)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();

        // https://github.com/RayTracing/raytracing.github.io/issues/663
        // let sines = (u * 10.0).sin() * (v * 10.0).sin() * (p.z() * 10.0).sin();

        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

// NoiseTexture using Perlin as noise source
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new() -> NoiseTexture {
        NoiseTexture {
            noise: Perlin::new(),
            scale: 1.0,
        }
    }
}

impl From<f64> for NoiseTexture {
    fn from(scale: f64) -> Self {
        NoiseTexture {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        Color::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (self.scale * p.z() + 10.0 * self.noise.turb(p, 7)).sin())
    }
}

// Mapped image texture
pub struct ImageTexture {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl ImageTexture {
    pub fn new(filename: &str) -> ImageTexture {
        let img = image::open(filename).expect("Failed to load image");

        let (width, height) = img.dimensions();
        let pixels = img.as_flat_samples_u8().expect("Failed to convert image");
        let mut data = Vec::new();

        data.extend_from_slice(pixels.as_slice());

        ImageTexture {
            data,
            width: width as usize,
            height: height as usize,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point3) -> Color {
        if self.data.is_empty() {
            return Color::new(0.0, 1.0, 1.0);
        }

        // Clamp input texture coordinates to [0,1] x [1,0]
        let uc = u.clamp(0.0, 1.0);
        let vc = 1.0 - v.clamp(0.0, 1.0); // Flip V to image coordinates

        let mut i = (uc * self.width as f64) as usize;
        let mut j = (vc * self.height as f64) as usize;

        // Clamp integer mapping, since actual coordinates should be less than 1.0
        i = i.clamp(0, self.width - 1);
        j = j.clamp(0, self.height - 1);

        let idx = BYTES_PER_PIXEL * i + BYTES_PER_PIXEL * self.width * j;

        let r = self.data[idx] as f64 / 255.0;
        let g = self.data[idx + 1] as f64 / 255.0;
        let b = self.data[idx + 2] as f64 / 255.0;
        Color::new(r, g, b)
    }
}
