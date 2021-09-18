use std::sync::Arc;

use crate::aabb::Aabb;
use crate::aarect::{XyRect, XzRect, YzRect};
use crate::hittable::{HitRecord, Hittable};
use crate::materials::{Lambertian, Material};
use crate::ray::Ray;
use crate::vec3::{Color, Point3};

pub struct Cube {
    p_min: Point3,
    p_max: Point3,
    sides: Vec<Box<dyn Hittable>>,
}

impl Cube {
    pub fn new(p0: Point3, p1: Point3, c: Color) -> Cube {
        let mut cube = Cube {
            p_min: p0,
            p_max: p1,
            sides: Vec::new(),
        };

        let mat: Arc<dyn Material> = Arc::new(Lambertian::from(c));

        cube.sides.push(Box::new(XyRect {
            x0: p0.x(),
            x1: p1.x(),
            y0: p0.y(),
            y1: p1.y(),
            k: p1.z(),
            material: mat.clone(),
        }));
        cube.sides.push(Box::new(XyRect {
            x0: p0.x(),
            x1: p1.x(),
            y0: p0.y(),
            y1: p1.y(),
            k: p0.z(),
            material: mat.clone(),
        }));

        cube.sides.push(Box::new(XzRect {
            x0: p0.x(),
            x1: p1.x(),
            z0: p0.z(),
            z1: p1.z(),
            k: p1.y(),
            material: mat.clone(),
        }));
        cube.sides.push(Box::new(XzRect {
            x0: p0.x(),
            x1: p1.x(),
            z0: p0.z(),
            z1: p1.z(),
            k: p0.y(),
            material: mat.clone(),
        }));

        cube.sides.push(Box::new(YzRect {
            y0: p0.y(),
            y1: p1.y(),
            z0: p0.z(),
            z1: p1.z(),
            k: p1.x(),
            material: mat.clone(),
        }));
        cube.sides.push(Box::new(YzRect {
            y0: p0.y(),
            y1: p1.y(),
            z0: p0.z(),
            z1: p1.z(),
            k: p0.x(),
            material: mat.clone(),
        }));

        cube
    }
}

impl Hittable for Cube {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        self.sides.hit(r, tmin, tmax)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(Aabb::new(self.p_min, self.p_max))
    }
}
