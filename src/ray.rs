use crate::vec3::{Point3, Vec3};
#[derive(Clone, Copy, Debug)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
    tm: f64,
}

impl Ray {
    pub fn new(o: Point3, d: Vec3, time: f64) -> Ray {
        Ray {
            origin: o,
            direction: d,
            tm: time,
        }
    }

    pub fn direction(self) -> Vec3 {
        self.direction
    }

    pub fn origin(self) -> Point3 {
        self.origin
    }

    pub fn time(self) -> f64 {
        self.tm
    }

    pub fn point_at(self, t: f64) -> Vec3 {
        self.origin + (self.direction * t)
    }
}
