use crate::camera::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec3::{Color, Vec3};

use super::{Material, ScatterRecord};

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        let f = if fuzz < 1.0 { fuzz } else { 1.0 };
        Metal { albedo, fuzz: f }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hr: &HitRecord) -> Option<ScatterRecord> {
        let reflected = Vec3::reflect(&ray.direction().to_unit_vector(), &hr.get_normal());
        let specular_ray = Ray::new(
            hr.get_p(),
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
            ray.time(),
        );

        Some(ScatterRecord::specular(&self.albedo, &specular_ray))
    }
}
