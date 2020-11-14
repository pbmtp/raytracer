use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::materials::Material;
use crate::ray::Ray;
use crate::sphere::Sphere;
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

        let p = r.at(root);
        let outward_normal = (p - self.center(r.time())) / self.radius;
        let (u, v) = Sphere::get_uv(&outward_normal);

        let mut hr = HitRecord::new(p, Vec3::zero(), root, u, v, &*self.material);
        hr.set_front_face(&r, outward_normal);

        Some(hr)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        let r = self.radius;

        let box0 = Aabb::new(
            self.center(time0) - Vec3::new(r, r, r),
            self.center(time0) + Vec3::new(r, r, r),
        );
        let box1 = Aabb::new(
            self.center(time1) - Vec3::new(r, r, r),
            self.center(time1) + Vec3::new(r, r, r),
        );

        Some(Aabb::surrounding_box(&box0, &box1))
    }
}
