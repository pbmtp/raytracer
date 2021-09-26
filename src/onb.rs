use crate::vec3::Vec3;

// Orthonormal Basis

pub struct OrthoNormalBasis {
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl OrthoNormalBasis {
    /// Get a reference to the ortho normal bases's u.
    pub fn u(&self) -> &Vec3 {
        &self.u
    }

    /// Get a reference to the ortho normal bases's v.
    pub fn v(&self) -> &Vec3 {
        &self.v
    }

    /// Get a reference to the ortho normal bases's w.
    pub fn w(&self) -> &Vec3 {
        &self.w
    }

    pub fn local(&self, a: &Vec3) -> Vec3 {
        a.x() * self.u + a.y() * self.v + a.z() * self.w
    }
}

impl From<Vec3> for OrthoNormalBasis {
    fn from(v: Vec3) -> Self {
        let w = v.to_unit_vector();

        // x axis or y axis ?
        let a = if w.x().abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };

        let v = w.cross(a).to_unit_vector();
        let u = w.cross(v);

        OrthoNormalBasis { u, v, w }
    }
}
