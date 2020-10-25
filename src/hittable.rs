use super::ray::Ray;
use super::vec3::{Point3, Vec3};

#[derive(Clone, Copy, Debug, Default)]
pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f32,
    front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f32) -> HitRecord {
        HitRecord {
            p,
            normal,
            t,
            front_face: true,
        }
    }

    pub fn set_front_face(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }

    pub fn get_p(self) -> Point3 {
        self.p
    }

    pub fn get_normal(self) -> Vec3 {
        self.normal
    }

    pub fn get_t(self) -> f32 {
        self.t
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord>;
}
