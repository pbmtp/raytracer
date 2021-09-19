use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::tools::random_double;
use crate::vec3::{Color, Vec3};

use super::{Material, Scatter};

pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Dielectric {
        Dielectric { ref_idx }
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);

    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hr: &HitRecord) -> Scatter {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if hr.is_front() {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let unit_direction = ray.direction().to_unit_vector();
        let cos_theta = -unit_direction.dot(hr.get_normal()).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction =
            if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_double() {
                Vec3::reflect(&unit_direction, &hr.get_normal())
            } else {
                Vec3::refract(&unit_direction, &hr.get_normal(), refraction_ratio)
            };

        Scatter {
            attenuation,
            scattered: Some(Ray::new(hr.get_p(), direction, ray.time())),
        }
    }
}
