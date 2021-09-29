use crate::vec3::Vec3;
use crate::{hittable::Hittable, vec3::Point3};

use super::Pdf;

pub struct HittablePdf<H: Hittable> {
    hittable: H,
    origin: Point3,
}

impl<H: Hittable> HittablePdf<H> {
    pub fn new(hittable: H, origin: Point3) -> HittablePdf<H> {
        Self { hittable, origin }
    }
}

impl<H: Hittable> Pdf for HittablePdf<H> {
    fn value(&self, direction: &Vec3) -> f64 {
        self.hittable.pdf_value(&self.origin, direction)
    }

    fn generate(&self) -> Vec3 {
        self.hittable.random(&self.origin)
    }
}
