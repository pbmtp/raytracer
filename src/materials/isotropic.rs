use std::f64::consts::PI;

use crate::camera::ray::Ray;
use crate::hittable::HitRecord;
use crate::pdf::sphere::SpherePdf;
use crate::texture::{solid::SolidTexture, Texture};
use crate::vec3::Color;

use super::{Material, ScatterRecord};

pub struct Isotropic {
    albedo: Box<dyn Texture>,
}

impl From<Color> for Isotropic {
    fn from(color: Color) -> Self {
        Isotropic {
            albedo: Box::new(SolidTexture::from(color)),
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, _ray: &Ray, hr: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord::diffuse(
            &self.albedo.value(hr.get_u(), hr.get_v(), &hr.get_p()),
            Box::new(SpherePdf {}),
        ))
    }

    fn scattering_pdf(&self, _ray: &Ray, _hr: &HitRecord, _scattered: &Ray) -> f64 {
        1.0 / (4.0 * PI)
    }
}
