use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Point3;

pub struct RotateY<H: Hittable> {
    hittable: H,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Option<Aabb>,
}

impl<H: Hittable> RotateY<H> {
    pub fn new(hittable: H, angle: f64) -> RotateY<H> {
        let radians = angle.to_radians();

        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        if let Some(bbox) = hittable.bounding_box(0.0, 1.0) {
            let mut min = Point3::max();
            let mut max = Point3::min();

            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x = i as f64 * bbox.max().x() + (1.0 - i as f64) * bbox.min().x();
                        let y = j as f64 * bbox.max().y() + (1.0 - j as f64) * bbox.min().y();
                        let z = k as f64 * bbox.max().z() + (1.0 - k as f64) * bbox.min().z();

                        let newx = cos_theta * x + sin_theta * z;
                        let newz = -sin_theta * x + cos_theta * z;

                        let tester = Point3::new(newx, y, newz);

                        for c in 0..3 {
                            min[c] = min[c].min(tester[c]);
                            max[c] = max[c].max(tester[c]);
                        }
                    }
                }
            }

            return RotateY {
                hittable,
                sin_theta,
                cos_theta,
                bbox: Some(Aabb::new(min, max)),
            };
        }

        RotateY {
            hittable,
            sin_theta,
            cos_theta,
            bbox: None,
        }
    }
}

impl<H: Hittable> Hittable for RotateY<H> {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let mut origin = r.origin();
        let mut direction = r.direction();

        origin[0] = self.cos_theta * r.origin()[0] - self.sin_theta * r.origin()[2];
        origin[2] = self.sin_theta * r.origin()[0] + self.cos_theta * r.origin()[2];

        direction[0] = self.cos_theta * r.direction()[0] - self.sin_theta * r.direction()[2];
        direction[2] = self.sin_theta * r.direction()[0] + self.cos_theta * r.direction()[2];

        let rotated_r = Ray::new(origin, direction, r.time());

        if let Some(hr) = self.hittable.hit(&rotated_r, tmin, tmax) {
            let mut p = hr.get_p();
            let mut normal = hr.get_normal();

            p[0] = self.cos_theta * hr.get_p()[0] + self.sin_theta * hr.get_p()[2];
            p[2] = -self.sin_theta * hr.get_p()[0] + self.cos_theta * hr.get_p()[2];

            normal[0] = self.cos_theta * hr.get_normal()[0] + self.sin_theta * hr.get_normal()[2];
            normal[2] = -self.sin_theta * hr.get_normal()[0] + self.cos_theta * hr.get_normal()[2];

            let mut hr_ret =
                HitRecord::new(p, normal, hr.get_t(), hr.get_u(), hr.get_v(), hr.material);
            hr_ret.set_front_face(&rotated_r, normal);

            return Some(hr_ret);
        }

        None
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        self.bbox
    }
}
