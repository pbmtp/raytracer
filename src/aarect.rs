// Axis-Aligned Rectangles
use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::materials::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

// X-Y Axis-Aligned Rectangle
pub struct XyRect {
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
    pub material: Box<dyn Material>,
}

impl Hittable for XyRect {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin().z()) / r.direction().z();
        if t < tmin || t > tmax {
            return None;
        }

        let x = r.origin().x() + t * r.direction().x();
        let y = r.origin().y() + t * r.direction().y();

        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);
        let t = t;
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        let p = r.point_at(t);

        let mut hr = HitRecord::new(p, Vec3::zero(), t, u, v, &*self.material);
        hr.set_front_face(&r, outward_normal);

        return Some(hr);
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        // The bounding box must have non-zero width in each dimension, so pad the Z
        // dimension a small amount.
        Some(Aabb::new(
            Point3::new(self.x0, self.y0, self.k - 0.0001),
            Point3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}
