use crate::hittable::HitRecord;
use crate::ray::Ray;
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

        Scatter {
            attenuation: self.albedo.value(hr.get_u(), hr.get_v(), &hr.get_p()),
            scattered: Some(Ray::new(hr.get_p(), scatter_direction, ray.time())),
        }
    }
}
