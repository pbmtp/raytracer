use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::materials::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct MovingSphere {
    pub center0: Point3,
    pub center1: Point3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub material: Box<dyn Material>,
}

impl MovingSphere {
    pub fn center(&self, time: f64) -> Point3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let oc: Vec3 = r.origin() - self.center(r.time());
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            let temp = (-half_b - root) / a;
            if temp > tmin && temp < tmax {
                let p = r.point_at(temp);
                let outward_normal = (p - self.center(r.time())) / self.radius;

                let mut hr = HitRecord::new(p, Vec3::zero(), temp, &*self.material);
                hr.set_front_face(&r, outward_normal);

                return Some(hr);
            }

            let temp = (-half_b + root) / a;
            if temp > tmin && temp < tmax {
                let p = r.point_at(temp);
                let outward_normal = (p - self.center(r.time())) / self.radius;

                let mut hr = HitRecord::new(p, Vec3::zero(), temp, &*self.material);
                hr.set_front_face(&r, outward_normal);

                return Some(hr);
            }
        }

        None
    }

    // FIXME handle time
    fn bounding_box(&self) -> Option<Aabb> {
        let r = self.radius;

        Some(Aabb::new(
            self.center(0.0) - Vec3::new(r, r, r),
            self.center(0.0) + Vec3::new(r, r, r),
        ))
    }
}
