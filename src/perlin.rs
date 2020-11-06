use crate::tools::{random_double, random_usize_range};
use crate::vec3::Point3;

const PERLIN_SIZE: usize = 256;

pub struct Perlin {
    rand_vec: Vec<f64>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Perlin {
        Perlin {
            rand_vec: Perlin::generate_rand_vec(),
            perm_x: Perlin::generate_permutation(),
            perm_y: Perlin::generate_permutation(),
            perm_z: Perlin::generate_permutation(),
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let i = (4.0 * p.x()) as isize & 255;
        let j = (4.0 * p.y()) as isize & 255;
        let k = (4.0 * p.z()) as isize & 255;

        self.rand_vec[self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]]
    }

    fn generate_rand_vec() -> Vec<f64> {
        let mut p = Vec::with_capacity(PERLIN_SIZE);

        for _ in 0..PERLIN_SIZE {
            p.push(random_double());
        }

        p
    }

    fn generate_permutation() -> Vec<usize> {
        let mut v = Vec::new();

        for i in 0..PERLIN_SIZE {
            v.push(i);
        }

        Perlin::permute(&mut v, PERLIN_SIZE);

        v
    }

    fn permute(p: &mut Vec<usize>, n: usize) {
        for i in (0..n).rev() {
            let target = random_usize_range(0, i + 1);
            p.swap(i, target);
        }
    }
}
