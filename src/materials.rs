use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::texture::{SolidTexture, Texture};
use crate::tools::random_double;
use crate::vec3::{Color, Point3, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Scatter {
    pub attenuation: Color,
    pub scattered: Option<Ray>,
}

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hr: &HitRecord) -> Scatter;

    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::zero()
    }
}

// Lambertian
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

// Metal
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
    fn scatter(&self, ray: &Ray, hr: &HitRecord) -> Scatter {
        let reflected = Vec3::reflect(&ray.direction().to_unit_vector(), &hr.get_normal());
        let scattered = Ray::new(
            hr.get_p(),
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
            ray.time(),
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

// Dielectric
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

// DiffuseLight
pub struct DiffuseLight {
    pub emit: Box<dyn Texture>,
}

impl From<Color> for DiffuseLight {
    fn from(color: Color) -> Self {
        DiffuseLight {
            emit: Box::new(SolidTexture::from(color)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray: &Ray, _hr: &HitRecord) -> Scatter {
        Scatter {
            attenuation: Color::zero(),
            scattered: None,
        }
    }

    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.emit.value(u, v, p)
    }
}

// Isotropic
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
