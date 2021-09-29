use rand::seq::SliceRandom;
use std::sync::Arc;

use crate::camera::ray::Ray;
use crate::geometry::aabb::Aabb;
use crate::materials::Material;
use crate::vec3::{Point3, Vec3};

#[derive(Clone)]
pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
    u: f64,
    v: f64,
    front_face: bool,
    pub material: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        p: Point3,
        normal: Vec3,
        t: f64,
        u: f64,
        v: f64,
        mat: Arc<dyn Material>,
    ) -> HitRecord {
        HitRecord {
            p,
            normal,
            t,
            u,
            v,
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

    pub fn is_front(&self) -> bool {
        self.front_face
    }

    pub fn get_p(&self) -> Point3 {
        self.p
    }

    pub fn get_normal(&self) -> Vec3 {
        self.normal
    }

    pub fn get_t(&self) -> f64 {
        self.t
    }

    pub fn get_u(&self) -> f64 {
        self.u
    }

    pub fn get_v(&self) -> f64 {
        self.v
    }

    pub fn translate(&mut self, offset: Vec3) {
        self.p += offset;
    }

    pub fn flip_normal(&self) -> HitRecord {
        HitRecord {
            front_face: !self.front_face,
            material: self.material.clone(),
            ..*self
        }
    }
}

pub trait Hittable: Sync {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;

    fn pdf_value(&self, _origin: &Point3, _v: &Vec3) -> f64 {
        0.0
    }

    fn random(&self, _origin: &Point3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}

impl Hittable for Vec<Box<dyn Hittable>> {
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

    fn pdf_value(&self, origin: &Point3, v: &Vec3) -> f64 {
        let weight = 1.0 / self.len() as f64;
        let mut sum = 0.0;

        for obj in self.iter() {
            sum += weight * obj.pdf_value(origin, v)
        }

        sum
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        self.choose(&mut rand::thread_rng())
            .map_or(Vec3::new(1.0, 0.0, 0.0), |obj| obj.random(origin))
    }
}

impl Hittable for &Vec<Box<dyn Hittable>> {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        (*self).hit(r, tmin, tmax)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        (*self).bounding_box(time0, time1)
    }

    fn pdf_value(&self, origin: &Point3, v: &Vec3) -> f64 {
        (*self).pdf_value(origin, v)
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        (*self).random(origin)
    }
}
