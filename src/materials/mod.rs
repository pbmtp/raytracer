use crate::camera::ray::Ray;
use crate::hittable::HitRecord;
use crate::pdf::Pdf;
use crate::vec3::{Color, Point3};

pub mod dielectric;
pub mod diffuse_light;
pub mod isotropic;
pub mod lambertian;
pub mod metal;

pub enum ScatterRecord {
    Specular {
        attenuation: Color,
        ray: Ray,
    },
    Diffuse {
        attenuation: Color,
        pdf: Box<dyn Pdf>,
    },
}

impl ScatterRecord {
    pub fn specular(attenuation: &Color, ray: &Ray) -> ScatterRecord {
        ScatterRecord::Specular {
            attenuation: *attenuation,
            ray: *ray,
        }
    }

    pub fn diffuse(attenuation: &Color, pdf: Box<dyn Pdf>) -> ScatterRecord {
        ScatterRecord::Diffuse {
            attenuation: *attenuation,
            pdf,
        }
    }
}

pub trait Material: Send + Sync {
    fn emitted(&self, _ray: &Ray, _hr: &HitRecord, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::zero()
    }

    fn scatter(&self, _ray: &Ray, _hr: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn scattering_pdf(&self, _ray: &Ray, _hr: &HitRecord, _scattered: &Ray) -> f64 {
        1.0
    }
}
