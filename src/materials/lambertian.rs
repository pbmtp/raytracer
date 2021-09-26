use std::f64::consts::PI;

use crate::camera::ray::Ray;
use crate::hittable::HitRecord;
use crate::texture::{solid::SolidTexture, Texture};
use crate::vec3::{Color, Vec3};

use super::{Material, Scatter};

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
    fn scatter(&self, ray: &Ray, hr: &HitRecord) -> Scatter {
        let mut scatter_direction = hr.get_normal() + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hr.get_normal();
        }

        let attenuation = self.albedo.value(hr.get_u(), hr.get_v(), &hr.get_p());
        let scattered = Ray::new(hr.get_p(), scatter_direction.to_unit_vector(), ray.time());
        let pdf = hr.get_normal().dot(scattered.direction()) / PI;

        Scatter {
            attenuation,
            scattered: Some(scattered),
            pdf,
        }
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
