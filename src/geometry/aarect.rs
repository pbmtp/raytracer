use std::sync::Arc;

// Axis-Aligned Rectangles
use crate::geometry::aabb::Aabb;
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
    pub material: Arc<dyn Material>,
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
        let p = r.at(t);

        let mut hr = HitRecord::new(p, Vec3::zero(), t, u, v, self.material.clone());
        hr.set_front_face(r, outward_normal);

        Some(hr)
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

// X-Z Axis-Aligned Rectangle
pub struct XzRect {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
    pub material: Arc<dyn Material>,
}

impl Hittable for XzRect {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin().y()) / r.direction().y();
        if t < tmin || t > tmax {
            return None;
        }

        let x = r.origin().x() + t * r.direction().x();
        let z = r.origin().z() + t * r.direction().z();

        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let t = t;
        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        let p = r.at(t);

        let mut hr = HitRecord::new(p, Vec3::zero(), t, u, v, self.material.clone());
        hr.set_front_face(r, outward_normal);

        Some(hr)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        // The bounding box must have non-zero width in each dimension, so pad the Y
        // dimension a small amount.
        Some(Aabb::new(
            Point3::new(self.x0, self.k - 0.0001, self.z0),
            Point3::new(self.x1, self.k + 0.0001, self.z1),
        ))
    }
}

// Y-Z Axis-Aligned Rectangle
pub struct YzRect {
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
    pub material: Arc<dyn Material>,
}

impl Hittable for YzRect {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin().x()) / r.direction().x();
        if t < tmin || t > tmax {
            return None;
        }

        let y = r.origin().y() + t * r.direction().y();
        let z = r.origin().z() + t * r.direction().z();

        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (y - self.y0) / (self.y1 - self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let t = t;
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        let p = r.at(t);

        let mut hr = HitRecord::new(p, Vec3::zero(), t, u, v, self.material.clone());
        hr.set_front_face(r, outward_normal);

        Some(hr)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        // The bounding box must have non-zero width in each dimension, so pad the X
        // dimension a small amount.
        Some(Aabb::new(
            Point3::new(self.k - 0.0001, self.y0, self.z0),
            Point3::new(self.k + 0.0001, self.y0, self.z1),
        ))
    }
}
