use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, Point3};

pub mod dielectric;
pub mod diffuse_light;
pub mod isotropic;
pub mod lambertian;
pub mod metal;

#[derive(Clone, Copy, Debug)]
pub struct Scatter {
    pub attenuation: Color,
    pub scattered: Option<Ray>,
}

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hr: &HitRecord) -> Scatter;

    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::zero()
    }
}
