use crate::vec3::{Color, Point3};

use super::Texture;

// Uniform Colored texture
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
