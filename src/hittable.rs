use super::ray::Ray;
use super::vec3::{Point3, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f32,
}

pub trait Hittable {
    fn hit(&self, r: Ray, tmin: f32, tmax: f32) -> Option<HitRecord>;
}
