use std::f64::consts::PI;

use crate::vec3::Vec3;

use super::Pdf;

pub struct SpherePdf {}

impl Pdf for SpherePdf {
    fn value(&self, _direction: &Vec3) -> f64 {
        1.0 / (4.0 * PI)
    }

    fn generate(&self) -> Vec3 {
        Vec3::random_unit_vector()
    }
}
