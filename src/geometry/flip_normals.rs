use crate::camera::ray::Ray;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::vec3::{Point3, Vec3};

use super::aabb::Aabb;

pub struct FlipNormals<H: Hittable> {
    hittable: H,
}

impl<H: Hittable> FlipNormals<H> {
    pub fn new(hittable: H) -> Self {
        Self { hittable }
    }
}

impl<H: Hittable> Hittable for FlipNormals<H> {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        if let Some(hr) = self.hittable.hit(r, tmin, tmax) {
            return Some(hr.flip_normal());
        }

        None
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        self.hittable.bounding_box(time0, time1)
    }

    fn pdf_value(&self, origin: &Point3, v: &Vec3) -> f64 {
        self.hittable.pdf_value(origin, v)
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        self.hittable.random(origin)
    }
}
