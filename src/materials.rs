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
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
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

// Metal
pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Metal {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hr: &HitRecord) -> Scatter {
        let reflected = Vec3::unit_vector(ray.direction()).reflect(hr.get_normal());
        let attenuation = albedo;
        let scattered = Ray::new(rec.p, reflected);

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
