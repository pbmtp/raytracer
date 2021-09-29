use crate::tools::random_double;
use crate::vec3::Vec3;

use super::Pdf;

pub struct MixturePdf<P0: Pdf, P1: Pdf> {
    p0: P0,
    p1: P1,
}

impl<P0: Pdf, P1: Pdf> MixturePdf<P0, P1> {
    pub fn new(p0: P0, p1: P1) -> Self {
        Self { p0, p1 }
    }
}

impl<P0: Pdf, P1: Pdf> Pdf for MixturePdf<P0, P1> {
    fn value(&self, direction: &Vec3) -> f64 {
        0.5 * self.p0.value(direction) + 0.5 * self.p1.value(direction)
    }

    fn generate(&self) -> Vec3 {
        if random_double() < 0.5 {
            self.p0.generate()
        } else {
            self.p1.generate()
        }
    }
}
