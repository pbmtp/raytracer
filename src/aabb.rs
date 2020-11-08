// Axis-Aligned Bounding Boxes
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::ops::Add;

#[derive(Copy, Clone)]
pub struct Aabb {
    minimum: Point3,
    maximum: Point3,
}

impl Aabb {
    pub fn new(min: Point3, max: Point3) -> Aabb {
        Aabb {
            minimum: min,
            maximum: max,
        }
    }

    pub fn min(self) -> Point3 {
        self.minimum
    }

    pub fn max(self) -> Point3 {
        self.maximum
    }

    pub fn hit(&self, r: &Ray, mut tmin: f64, mut tmax: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0f64 / r.direction()[a];
            let mut t0 = (self.min()[a] - r.origin()[a]) * inv_d;
            let mut t1 = (self.max()[a] - r.origin()[a]) * inv_d;

            if inv_d < 0.0f64 {
                let t = t0;
                t0 = t1;
                t1 = t;
            }

            tmin = t0.max(tmin);
            tmax = t1.min(tmax);
            if tmax <= tmin {
                return false;
            }
        }

        true
    }

    pub fn surrounding_box(box0: &Aabb, box1: &Aabb) -> Aabb {
        let small = Point3::new(
            box0.min().x().min(box1.min().x()),
            box0.min().y().min(box1.min().y()),
            box0.min().z().min(box1.min().z()),
        );

        let big = Point3::new(
            box0.max().x().max(box1.max().x()),
            box0.max().y().max(box1.max().y()),
            box0.max().z().max(box1.max().z()),
        );

        Aabb::new(small, big)
    }
}

impl Add<Vec3> for Aabb {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self {
        Aabb::new(self.minimum + rhs, self.maximum + rhs)
    }
}
