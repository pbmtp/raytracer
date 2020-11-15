use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::materials::Isotropic;
use crate::ray::Ray;
use crate::tools::random_double;
use crate::vec3::Vec3;

pub struct ConstantMedium<H: Hittable> {
    pub boundary: H,
    pub density: f64,
    pub phase_function: Isotropic,
}

impl<H: Hittable> Hittable for ConstantMedium<H> {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        if let Some(hit1) = self.boundary.hit(&r, f64::MIN, f64::MAX) {
            if let Some(hit2) = self.boundary.hit(&r, hit1.get_t() + 0.0001, f64::MAX) {
                let mut t1 = hit1.get_t().max(tmin);
                let t2 = hit2.get_t().min(tmax);
                if t1 < t2 {
                    if t1 < 0.0 {
                        t1 = 0.0;
                    }
                    let ray_length = r.direction().length();
                    let distance_inside_boundary = (t2 - t1) * ray_length;
                    let hit_distance = (-1.0 / self.density) * random_double().ln();
                    if hit_distance <= distance_inside_boundary {
                        let t = t1 + hit_distance / ray_length;

                        return Some(HitRecord::new(
                            r.at(t),
                            Vec3::new(1.0, 0.0, 0.0),
                            t,
                            0.0,
                            0.0,
                            &self.phase_function,
                        ));
                    }
                }
            }
        }

        None
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        self.boundary.bounding_box(time0, time1)
    }
}
