use std::f64::consts::PI;

use crate::camera::ray::Ray;
use crate::hittable::HitRecord;
use crate::pdf::cosine::CosinePdf;
use crate::texture::{solid::SolidTexture, Texture};
use crate::vec3::Color;

use super::{Material, ScatterRecord};

pub struct Lambertian {
    pub albedo: Box<dyn Texture>,
}

impl From<Color> for Lambertian {
    fn from(color: Color) -> Self {
        Lambertian {
            albedo: Box::new(SolidTexture::from(color)),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hr: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord::diffuse(
            &self.albedo.value(hr.get_u(), hr.get_v(), &hr.get_p()),
            Box::new(CosinePdf::new(&hr.get_normal())),
        ))
    }

    fn scattering_pdf(&self, _ray: &Ray, hr: &HitRecord, scattered: &Ray) -> f64 {
        let cosine = hr.get_normal().dot(scattered.direction().to_unit_vector());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }
}
