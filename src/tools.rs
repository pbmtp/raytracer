use rand::Rng;

use std::f64::consts::PI;

use crate::vec3::Vec3;

pub fn random_double_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();

    rng.gen_range(min, max)
}

pub fn random_double() -> f64 {
    random_double_range(0.0, 1.0)
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }

        return p;
    }
}

pub fn random_unit_vector() -> Vec3 {
    let a = random_double_range(0.0, 2.0 * PI);
    let z = random_double_range(-1.0, 1.0);
    let r = (1f64 - z * z).sqrt();

    Vec3::new(r * a.cos(), r * a.sin(), z)
}

pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(*normal) > 0.0 {
        // In the same hemisphere as the normal
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}
