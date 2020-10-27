use crate::materials::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
    p: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(p: Point3, normal: Vec3, t: f64, mat: &'a dyn Material) -> HitRecord<'a> {
        HitRecord {
            p,
            normal,
            t,
            front_face: true,
            material: mat,
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

    pub fn get_t(self) -> f64 {
        self.t
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
}
