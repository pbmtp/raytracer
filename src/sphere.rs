use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Debug, Default)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let oc: Vec3 = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            let temp = (-half_b - root) / a;
            if temp > tmin && temp < tmax {
                let p = r.point_at(temp);
                let outward_normal = (p - self.center) / self.radius;

                let mut hr = HitRecord::new(p, Vec3::zero(), temp);
                hr.set_front_face(&r, outward_normal);

                return Some(hr);
            }

            let temp = (-half_b + root) / a;
            if temp > tmin && temp < tmax {
                let p = r.point_at(temp);
                let outward_normal = (p - self.center) / self.radius;

                let mut hr = HitRecord::new(p, Vec3::zero(), temp);
                hr.set_front_face(&r, outward_normal);

                return Some(hr);
            }
        }

        None
    }
}
