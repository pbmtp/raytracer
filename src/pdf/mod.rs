use crate::vec3::Vec3;

pub mod cosine;
pub mod hittable;
pub mod mixture;

pub trait Pdf {
    fn value(&self, direction: &Vec3) -> f64;

    fn generate(&self) -> Vec3;
}
