use std::f64::consts::PI;

use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::materials::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Box<dyn Material>,
}

impl Sphere {
    pub fn get_uv(p: &Point3) -> (f64, f64) {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;

        let u = phi / (2.0 * PI);
        let v = theta / PI;

        (u, v)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let oc: Vec3 = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < tmin || tmax < root {
            root = (-half_b + sqrtd) / a;
            if root < tmin || tmax < root {
                return None;
            }
        }

        let p = r.point_at(root);
        let outward_normal = (p - self.center) / self.radius;
        let (u, v) = Sphere::get_uv(&outward_normal);

        let mut hr = HitRecord::new(p, Vec3::zero(), root, u, v, &*self.material);
        hr.set_front_face(&r, outward_normal);

        return Some(hr);
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        let r = self.radius;

        Some(Aabb::new(
            self.center - Vec3::new(r, r, r),
            self.center + Vec3::new(r, r, r),
        ))
    }
}
