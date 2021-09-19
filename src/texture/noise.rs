use crate::vec3::{Color, Point3};

use super::{perlin::Perlin, Texture};

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
