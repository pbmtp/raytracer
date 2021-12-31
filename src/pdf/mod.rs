use std::ops::Deref;

use crate::vec3::Vec3;

pub mod cosine;
pub mod hittable;
pub mod mixture;
pub mod sphere;

pub trait Pdf {
    fn value(&self, direction: &Vec3) -> f64;

    fn generate(&self) -> Vec3;
}

impl Pdf for Box<dyn Pdf> {
    fn value(&self, direction: &Vec3) -> f64 {
        self.deref().value(direction)
    }

    fn generate(&self) -> Vec3 {
        self.deref().generate()
    }
}
