use crate::vec3::{Point3, Vec3};
#[derive(Clone, Copy, Debug)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(o: Point3, d: Vec3) -> Ray {
        Ray {
            origin: o,
            direction: d,
        }
    }

    pub fn direction(self) -> Vec3 {
        self.direction
    }

    pub fn origin(self) -> Point3 {
        self.origin
    }

    pub fn point_at(self, t: f64) -> Vec3 {
        self.origin + (self.direction * t)
    }
}
