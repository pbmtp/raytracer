use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::tools::random_in_hemisphere; // random_unit_vector
use crate::vec3::Color;

#[derive(Clone, Copy, Debug)]
pub struct Scatter {
    attenuation: Color,
    scattered: Option<Ray>,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hr: &HitRecord) -> Scatter;
}

// Lambertian 
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(c: Color) -> Lambertian {
        Lambertian {
            albedo: c,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hr: &HitRecord) -> Scatter {
        // let scatter_direction = hr.get_normal() + random_unit_vector();
        let scatter_direction = random_in_hemisphere(&hr.get_normal());

        Scatter {
            attenuation: self.albedo,
            scattered: Some(Ray::new(hr.get_p(), scatter_direction)),
        }
    }
}
