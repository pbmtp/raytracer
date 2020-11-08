#![allow(dead_code)]

use std::iter::Sum;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use crate::tools::{random_double, random_double_range};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    e: [f64; 3],
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    // constructors
    #[inline]
    pub fn zero() -> Vec3 {
        Vec3::default()
    }

    #[inline]
    pub fn new(r: f64, g: f64, b: f64) -> Vec3 {
        Vec3 { e: [r, g, b] }
    }

    #[inline]
    pub fn random() -> Vec3 {
        Vec3 {
            e: [random_double(), random_double(), random_double()],
        }
    }

    #[inline]
    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3 {
            e: [
                random_double_range(min, max),
                random_double_range(min, max),
                random_double_range(min, max),
            ],
        }
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
        Vec3::random_in_unit_sphere().to_unit_vector()
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if in_unit_sphere.dot(*normal) > 0.0 {
            // In the same hemisphere as the normal
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    // getters
    #[inline]
    pub fn x(&self) -> f64 {
        self.e[0]
    }
    #[inline]
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    #[inline]
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    #[inline]
    pub fn r(&self) -> f64 {
        self.e[0]
    }
    #[inline]
    pub fn g(&self) -> f64 {
        self.e[1]
    }
    #[inline]
    pub fn b(&self) -> f64 {
        self.e[2]
    }

    // ops
    #[inline]
    pub fn dot(&self, other: Vec3) -> f64 {
        self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2]
    }

    #[inline]
    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e[1] * other.e[2] - self.e[2] * other.e[1],
            self.e[2] * other.e[0] - self.e[0] * other.e[2],
            self.e[0] * other.e[1] - self.e[1] * other.e[0],
        )
    }

    #[inline]
    pub fn length_squared(self) -> f64 {
        self.dot(self)
    }

    #[inline]
    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn clamp(self, min: f64, max: f64) -> Vec3 {
        Vec3::new(
            self.e[0].clamp(min, max),
            self.e[1].clamp(min, max),
            self.e[2].clamp(min, max),
        )
    }

    pub fn to_u8(self) -> [u8; 3] {
        let c = self.clamp(0.0, 1.0) * 255.999f64;

        [c.e[0] as u8, c.e[1] as u8, c.e[2] as u8]
    }

    pub fn to_u8_avg(self, samples_per_pixel: u32) -> [u8; 3] {
        let mut c = self.clone();
        c /= samples_per_pixel as f64;
        c = c.clamp(0.0, 0.999) * 256f64;

        [c.e[0] as u8, c.e[1] as u8, c.e[2] as u8]
    }

    pub fn to_u8_avg_gamma2(self, samples_per_pixel: u32) -> [u8; 3] {
        let mut c = self.clone();
        c /= samples_per_pixel as f64;

        let r = c.e[0].sqrt().clamp(0.0, 0.999) * 256f64;
        let g = c.e[1].sqrt().clamp(0.0, 0.999) * 256f64;
        let b = c.e[2].sqrt().clamp(0.0, 0.999) * 256f64;

        [r as u8, g as u8, b as u8]
    }

    pub fn to_unit_vector(self) -> Vec3 {
        let l = self.length();

        Vec3::new(self.e[0] / l, self.e[1] / l, self.e[2] / l)
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - 2.0 * v.dot(*n) * *n
    }

    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = -uv.dot(*n);
        let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * *n;

        r_out_perp + r_out_parallel
    }

    pub fn near_zero(self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        let s = 1E-8f64;
        self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s
    }

    // TODO normalize
}

/*
impl From<(f64, f64, f64)> for Vec3 {
    fn from(tuple: (f64, f64, f64)) -> Self {
        Self::new(tuple.0, tuple.1, tuple.2)
    }
}
*/

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3::new(
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2],
        )
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, rhs: f64) -> Self {
        Vec3::new(self.e[0] + rhs, self.e[1] + rhs, self.e[2] + rhs)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self::new(
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2],
        );
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec3::new(
            self.e[0] - other.e[0],
            self.e[1] - other.e[1],
            self.e[2] - other.e[2],
        )
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self::new(
            self.e[0] - other.e[0],
            self.e[1] - other.e[1],
            self.e[2] - other.e[2],
        );
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] * other.e[0],
            self.e[1] * other.e[1],
            self.e[2] * other.e[2],
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Vec3::new(self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self * rhs.e[0], self * rhs.e[1], self * rhs.e[2])
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Vec3::new(self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.e[0] /= rhs;
        self.e[1] /= rhs;
        self.e[2] /= rhs;
    }
}

impl<'a> Sum<&'a Self> for Vec3 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::new(0.0, 0.0, 0.0), |a, b| {
            Self::new(a.e[0] + b.e[0], a.e[1] + b.e[1], a.e[2] + b.e[2])
        })
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &f64 {
        &self.e[i]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.e[i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        let v = Vec3::zero();
        assert_eq!((v.e[0], v.e[1], v.e[2]), (0f64, 0f64, 0f64));
    }

    #[test]
    fn test_new_getters() {
        let v = Vec3::new(1.0f64, 2.0f64, 3.0f64);

        assert_eq!(v.x(), 1.0f64);
        assert_eq!(v.y(), 2.0f64);
        assert_eq!(v.z(), 3.0f64);

        assert_eq!(v.r(), 1.0f64);
        assert_eq!(v.g(), 2.0f64);
        assert_eq!(v.b(), 3.0f64);
    }

    #[test]
    fn test_add() {
        let z = Vec3::zero();
        let v1 = Vec3::new(1.0f64, 2.0f64, 3.0f64);
        let v2 = Vec3::new(1.0f64, 2.0f64, 3.0f64);
        assert_eq!(z + v1, v2);

        let v1 = Vec3::new(1.0f64, 2.0f64, 3.0f64);
        let v2 = Vec3::new(2.0f64, 3.0f64, 4.0f64);
        let v3 = Vec3::new(3.0f64, 5.0f64, 7.0f64);
        assert_eq!(v1 + v2, v3);
    }

    #[test]
    fn test_add_asign() {
        let mut z = Vec3::zero();
        let v1 = Vec3::new(1.0f64, 2.0f64, 3.0f64);
        let v2 = Vec3::new(1.0f64, 2.0f64, 3.0f64);
        z += v1;
        assert_eq!(z, v2);

        let mut v1 = Vec3::new(1.0f64, 2.0f64, 3.0f64);
        let v2 = Vec3::new(2.0f64, 3.0f64, 4.0f64);
        let v3 = Vec3::new(3.0f64, 5.0f64, 7.0f64);
        v1 += v2;
        assert_eq!(v1, v3);
    }

    #[test]
    fn test_sub() {
        let z = Vec3::zero();
        let v1 = Vec3::new(1.0f64, 2.0f64, 3.0f64);
        let v2 = Vec3::new(1.0f64, 2.0f64, 3.0f64);
        assert_eq!(v1 - v2, z);

        let v1 = Vec3::new(3.0f64, 5.0f64, 7.0f64);
        let v2 = Vec3::new(1.0f64, 2.0f64, 3.0f64);
        let v3 = Vec3::new(2.0f64, 3.0f64, 4.0f64);

        assert_eq!(v1 - v2, v3);
    }

    #[test]
    fn test_sub_assign() {
        let z = Vec3::zero();
        let mut v1 = Vec3::new(1.0f64, 2.0f64, 3.0f64);
        let v2 = Vec3::new(1.0f64, 2.0f64, 3.0f64);
        v1 -= v2;
        assert_eq!(v1, z);

        let mut v1 = Vec3::new(3.0f64, 5.0f64, 7.0f64);
        let v2 = Vec3::new(1.0f64, 2.0f64, 3.0f64);
        let v3 = Vec3::new(2.0f64, 3.0f64, 4.0f64);
        v1 -= v2;
        assert_eq!(v1, v3);
    }

    #[test]
    fn test_neg() {
        let v1 = Vec3::new(1.0f64, 2.0f64, 3.0f64);
        let v2 = Vec3::new(-1.0f64, -2.0f64, -3.0f64);
        assert_eq!(-v1, v2);

        let v1 = Vec3::new(1.0f64, 2.0f64, 3.0f64);
        let v2 = Vec3::new(-1.0f64, -2.0f64, -3.0f64);
        assert_eq!(-v2, v1);
    }

    #[test]
    fn test_length_squared() {
        let v1 = Vec3::new(1.0f64, 2.0f64, 3.0f64);
        let l2 = (1.0f64 * 1.0f64) + (2.0f64 * 2.0f64) + (3.0f64 * 3.0f64);
        assert_eq!(v1.length_squared(), l2);
    }

    #[test]
    fn test_length() {
        let v1 = Vec3::new(1.0f64, 2.0f64, 3.0f64);
        let l2 = 1.0f64 + (2.0f64 * 2.0f64) + (3.0f64 * 3.0f64);
        assert_eq!(v1.length(), l2.sqrt());
    }

    #[test]
    fn test_dot() {
        let v1 = Vec3::new(1.0f64, 2.0f64, 3.0f64);
        let v2 = Vec3::new(1.0f64, 2.0f64, 3.0f64);

        let dot = v1.e[0] * v2.e[0] + v1.e[1] * v2.e[1] + v1.e[2] * v2.e[2];
        assert_eq!(v1.dot(v2), dot);
    }

    #[test]
    fn test_cross() {
        let v1 = Vec3::new(1.0f64, 2.0f64, 3.0f64);
        let v2 = Vec3::new(4.0f64, 5.0f64, 6.0f64);
        let v3 = Vec3::new(-3.0f64, 6.0f64, -3.0f64);
        assert_eq!(v1.cross(v2), v3);

        let v1 = Vec3::new(1.0f64, 2.0f64, 3.0f64);
        let v2 = Vec3::new(4.0f64, 5.0f64, 6.0f64);
        let v3 = Vec3::new(3.0f64, -6.0f64, 3.0f64);
        assert_eq!(v2.cross(v1), v3);

        let v1 = Vec3::new(1.0f64, 2.0f64, 3.0f64);
        let v2 = Vec3::new(1.0f64, 2.0f64, 3.0f64);
        assert_eq!(v1.cross(v2), Vec3::zero());
    }

    #[test]
    fn test_mul() {
        let v1 = Vec3::new(1.0f64, 2.0f64, 3.0f64);
        let v2 = Vec3::new(2.0f64, 4.0f64, 6.0f64);
        let v3 = Vec3::new(2.0f64, 8.0f64, 18.0f64);

        assert_eq!(v1 * v2, v3);
        assert_eq!(v2 * v1, v3);
    }

    #[test]
    fn test_mul_f64() {
        let v1 = Vec3::new(1.0f64, 2.0f64, 3.0f64);
        let v2 = Vec3::new(2.0f64, 4.0f64, 6.0f64);
        assert_eq!(v1 * 2f64, v2);
        assert_eq!(2f64 * v1, v2);
    }

    #[test]
    fn test_mul_assign() {
        let mut v1 = Vec3::new(1.0f64, 2.0f64, 3.0f64);
        let v2 = Vec3::new(2.0f64, 4.0f64, 6.0f64);

        v1 *= 2f64;
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_div() {
        let v1 = Vec3::new(1.0f64, 2.0f64, 3.0f64);
        let v2 = Vec3::new(2.0f64, 4.0f64, 6.0f64);

        assert_eq!(v2 / 2f64, v1);
    }

    #[test]
    fn test_div_assign() {
        let mut v1 = Vec3::new(2.0f64, 4.0f64, 6.0f64);
        let v2 = Vec3::new(1.0f64, 2.0f64, 3.0f64);

        v1 /= 2f64;
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_to_u8() {
        let v1 = Vec3::new(0.5f64, -1.0f64, 0.75f64);
        let u1 = [127, 0, 191];
        assert_eq!(v1.to_u8(), u1);
    }

    #[test]
    fn test_to_unit_vector() {
        let v1 = Vec3::new(0.0f64, 4.0f64, 4.0f64);
        let v2 = Vec3::new(0.0f64, 0.7071067811865475, 0.7071067811865475);

        let l = v1.length();
        assert_eq!(v1 / l, v2);
    }

    #[test]
    fn test_clamp() {
        let v1 = Vec3::new(-42.0f64, 0.5f64, 1.1f64);
        let v2 = Vec3::new(0.0f64, 0.5f64, 1.0f64);

        assert_eq!(v1.clamp(0.0, 1.0), v2);
    }

    #[test]
    fn test_index() {
        let v1 = Vec3::new(1.0f64, 2.0f64, 3.0f64);
        assert_eq!(v1[0], 1.0f64);
        assert_eq!(v1[1], 2.0f64);
        assert_eq!(v1[2], 3.0f64);
    }

    #[test]
    fn test_index_mut() {
        let mut v1 = Vec3::zero();
        let v2 = Vec3::new(1.0f64, 2.0f64, 3.0f64);

        v1[0] = 1.0f64;
        v1[1] = 2.0f64;
        v1[2] = 3.0f64;

        assert_eq!(v1, v2);
    }
}
