use crate::camera::ray::Ray;
use crate::hittable::HitRecord;
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
    pub pdf: f64,
}

pub trait Material: Send + Sync {
    fn scatter(&self, _ray: &Ray, _hr: &HitRecord) -> Scatter {
        Scatter {
            attenuation: Color::zero(),
            scattered: None,
            pdf: 1.0,
        }
    }

    fn scattering_pdf(&self, _ray: &Ray, _hr: &HitRecord, _scattered: &Ray) -> f64 {
        1.0
    }

    fn emitted(&self, _ray: &Ray, _hr: &HitRecord, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::zero()
    }
}
