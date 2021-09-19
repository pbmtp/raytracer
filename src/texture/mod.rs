use crate::vec3::{Color, Point3};

pub mod checker;
pub mod image;
pub mod noise;
pub mod perlin;
pub mod solid;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}
