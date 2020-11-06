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
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let i = p.x().floor() as isize;
        let j = p.y().floor() as isize;
        let k = p.z().floor() as isize;

        let mut c = [[[0.0; 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let idx = (i + di) & 255;
                    let jdx = (j + dj) & 255;
                    let kdx = (k + dk) & 255;

                    c[di as usize][dj as usize][dk as usize] = self.rand_vec[self.perm_x
                        [idx as usize]
                        ^ self.perm_y[jdx as usize]
                        ^ self.perm_z[kdx as usize]];
                }
            }
        }

        Perlin::trilinear_interpolation(&c, uu, vv, ww)
    }

    fn trilinear_interpolation(c: &[[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += (i as f64 * u + (1 - i) as f64 * (1.0 - u))
                        * (j as f64 * v + (1 - j) as f64 * (1.0 - v))
                        * (k as f64 * w + (1 - k) as f64 * (1.0 - w))
                        * c[i][j][k];
                }
            }
        }

        accum
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
