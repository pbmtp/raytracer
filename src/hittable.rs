use crate::aabb::Aabb;
use crate::materials::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
    p: Point3,
    normal: Vec3,
    t: f64,
    u: f64,
    v: f64,
    front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(p: Point3, normal: Vec3, t: f64, mat: &'a dyn Material) -> HitRecord<'a> {
        HitRecord {
            p,
            normal,
            t,
            u: 0.0,
            v: 0.0,
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

    pub fn is_front(self) -> bool {
        self.front_face
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
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;
}

impl Hittable for Vec<Box<dyn Hittable + Sync>> {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let mut closest = None;
        let mut closest_so_far = tmax;

        for obj in self.iter() {
            if let Some(hr) = obj.hit(r, tmin, closest_so_far) {
                closest_so_far = hr.get_t();
                closest = Some(hr);
            }
        }

        closest
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        if self.is_empty() {
            return None;
        }

        let mut result: Option<Aabb> = None;

        for obj in self.iter() {
            if let Some(b) = obj.bounding_box(time0, time1) {
                result = match result {
                    None => Some(b),
                    Some(r) => Some(Aabb::surrounding_box(&r, &b)),
                };
            } else {
                return None;
            }
        }

        result
    }
}
