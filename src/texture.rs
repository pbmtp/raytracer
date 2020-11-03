use crate::vec3::{Color, Point3};

pub trait Texture: Sync {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color;
}

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
    fn value(&self, _u: f32, _v: f32, _p: &Point3) -> Color {
        self.color
    }
}
pub struct CheckerTexture {
    pub odd: Box<dyn Texture>,
    pub even: Box<dyn Texture>,
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color {
        let sines = (p.x() * 10.0).sin() * (p.y() * 10.0).sin() * (p.z() * 10.0).sin();

        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
