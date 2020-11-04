use crate::vec3::{Color, Point3};

pub trait Texture: Sync {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
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
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.color
    }
}
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
        // https://github.com/RayTracing/raytracing.github.io/issues/663
        let sines = (p.x() * 10.0).sin() * (p.y() * 10.0).sin() * (p.z() * 10.0).sin();
        // let sines = (u * 10.0).sin() * (v * 10.0).sin() * (p.z() * 10.0).sin();

        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
