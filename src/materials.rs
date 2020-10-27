use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Scatter {
    pub attenuation: Color,
    pub scattered: Option<Ray>,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hr: &HitRecord) -> Scatter;
}

// Lambertian
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hr: &HitRecord) -> Scatter {
        // let scatter_direction = hr.get_normal() + Vec3::random_unit_vector();
        let scatter_direction = Vec3::random_in_hemisphere(&hr.get_normal());

        Scatter {
            attenuation: self.albedo,
            scattered: Some(Ray::new(hr.get_p(), scatter_direction)),
        }
    }
}

// Metal
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        let f = if fuzz < 1.0 {
            fuzz
        } else {
            1.0
        };
        Metal { albedo, fuzz: f }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hr: &HitRecord) -> Scatter {
        let reflected = ray.direction().reflect(&hr.get_normal());
        let scattered = Ray::new(
            hr.get_p(),
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        );

        Scatter {
            attenuation: self.albedo,
            scattered: if scattered.direction().dot(hr.get_normal()) > 0.0 {
                Some(scattered)
            } else {
                None
            },
        }
    }
}
