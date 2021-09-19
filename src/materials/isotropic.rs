use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::texture::{solid::SolidTexture, Texture};
use crate::vec3::{Color, Vec3};

use super::{Material, Scatter};

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
    fn scatter(&self, ray: &Ray, hr: &HitRecord) -> Scatter {
        Scatter {
            attenuation: self.albedo.value(hr.get_u(), hr.get_v(), &hr.get_p()),
            scattered: Some(Ray::new(
                hr.get_p(),
                Vec3::random_in_unit_sphere(),
                ray.time(),
            )),
        }
    }
}
