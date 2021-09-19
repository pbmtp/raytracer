use crate::vec3::{Color, Point3};

pub mod checker;
pub mod solid;
pub mod noise;
pub mod image;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

