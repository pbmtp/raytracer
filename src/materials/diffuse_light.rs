use crate::camera::ray::Ray;
use crate::hittable::HitRecord;
use crate::texture::{solid::SolidTexture, Texture};
use crate::vec3::{Color, Point3};

use super::{Material, Scatter};

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
            pdf: 1.0,
        }
    }

    fn emitted(&self, _ray: &Ray, hr: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        if hr.is_front() {
            self.emit.value(u, v, p)
        } else {
            Color::zero()
        }
    }
}
