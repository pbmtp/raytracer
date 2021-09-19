use crate::vec3::{Color, Point3};

use super::{solid::SolidTexture, Texture};

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
