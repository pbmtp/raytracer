use std::f64::consts::PI;

use crate::onb::OrthoNormalBasis;
use crate::vec3::Vec3;

use super::Pdf;

pub struct CosinePdf {
    uvw: OrthoNormalBasis,
}

impl CosinePdf {
    pub fn new(w: &Vec3) -> Self {
        Self {
            uvw: OrthoNormalBasis::from(*w),
        }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        let cosine = direction.to_unit_vector().dot(*self.uvw.w());

        if cosine <= 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }

    fn generate(&self) -> Vec3 {
        self.uvw.local(&Vec3::random_cosine_direction())
    }
}
