use crate::camera::ray::Ray;
use crate::geometry::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::vec3::Vec3;

pub struct Translate<H: Hittable> {
    hittable: H,
    offset: Vec3,
}

impl<H: Hittable> Translate<H> {
    pub fn new(hittable: H, offset: Vec3) -> Translate<H> {
        Translate { hittable, offset }
    }
}

impl<H: Hittable> Hittable for Translate<H> {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let moved_ray = Ray::new(r.origin() - self.offset, r.direction(), r.time());

        if let Some(mut hr) = self.hittable.hit(&moved_ray, tmin, tmax) {
            hr.translate(self.offset);

            return Some(hr);
        }

        None
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        if let Some(b) = self.hittable.bounding_box(time0, time1) {
            return Some(b + self.offset);
        }

        None
    }
}
