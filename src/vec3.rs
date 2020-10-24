#![allow(dead_code)]

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3(f32, f32, f32);

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    // constructors
    #[inline]
    pub fn zero() -> Vec3 {
        Vec3::default()
    }

    #[inline]
    pub fn new(r: f32, g: f32, b: f32) -> Vec3 {
        Vec3(r, g, b)
    }

    // getters
    #[inline]
    pub fn x(&self) -> f32 {
        self.0
    }
    #[inline]
    pub fn y(&self) -> f32 {
        self.1
    }
    #[inline]
    pub fn z(&self) -> f32 {
        self.2
    }

    #[inline]
    pub fn r(&self) -> f32 {
        self.0
    }
    #[inline]
    pub fn g(&self) -> f32 {
        self.1
    }
    #[inline]
    pub fn b(&self) -> f32 {
        self.2
    }

    // ops
    #[inline]
    pub fn dot(&self, other: &Vec3) -> f32 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    #[inline]
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    #[inline]
    pub fn length_squared(self) -> f32 {
        self.dot(&self)
    }

    #[inline]
    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn to_u8(self) -> [u8; 3] {
        fn f32tou8(f: f32) -> u8 {
            if f < 0.0f32 {
                0
            } else if f >= 1.0f32 {
                255
            } else {
                (255.999f32 * f) as u8
            }
        }

        [f32tou8(self.0), f32tou8(self.1), f32tou8(self.2)]
    }

    pub fn to_unit_vector(self) -> Vec3 {
        let l = self.length();

        Vec3::new(self.0 / l, self.1 / l, self.2 / l)
    }

    // TODO normalize
}

impl From<(f32, f32, f32)> for Vec3 {
    fn from(tuple: (f32, f32, f32)) -> Self {
        Self::new(tuple.0, tuple.1, tuple.2)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3::new(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self::new(self.0 + other.0, self.1 + other.1, self.2 + other.2);
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec3::new(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self::new(self.0 - other.0, self.1 - other.1, self.2 - other.2);
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Vec3::new(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Vec3::new(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        let v = Vec3::zero();
        assert_eq!((v.0, v.1, v.2), (0f32, 0f32, 0f32));
    }

    #[test]
    fn test_new_getters() {
        let v = Vec3::new(1.0f32, 2.0f32, 3.0f32);

        assert_eq!(v.x(), 1.0f32);
        assert_eq!(v.y(), 2.0f32);
        assert_eq!(v.z(), 3.0f32);

        assert_eq!(v.r(), 1.0f32);
        assert_eq!(v.g(), 2.0f32);
        assert_eq!(v.b(), 3.0f32);
    }

    #[test]
    fn test_add() {
        let z = Vec3::zero();
        let v1 = Vec3::new(1.0f32, 2.0f32, 3.0f32);
        let v2 = Vec3::new(1.0f32, 2.0f32, 3.0f32);
        assert_eq!(z + v1, v2);

        let v1 = Vec3::new(1.0f32, 2.0f32, 3.0f32);
        let v2 = Vec3::new(2.0f32, 3.0f32, 4.0f32);
        let v3 = Vec3::new(3.0f32, 5.0f32, 7.0f32);
        assert_eq!(v1 + v2, v3);
    }

    #[test]
    fn test_add_asign() {
        let mut z = Vec3::zero();
        let v1 = Vec3::new(1.0f32, 2.0f32, 3.0f32);
        let v2 = Vec3::new(1.0f32, 2.0f32, 3.0f32);
        z += v1;
        assert_eq!(z, v2);

        let mut v1 = Vec3::new(1.0f32, 2.0f32, 3.0f32);
        let v2 = Vec3::new(2.0f32, 3.0f32, 4.0f32);
        let v3 = Vec3::new(3.0f32, 5.0f32, 7.0f32);
        v1 += v2;
        assert_eq!(v1, v3);
    }

    #[test]
    fn test_sub() {
        let z = Vec3::zero();
        let v1 = Vec3::new(1.0f32, 2.0f32, 3.0f32);
        let v2 = Vec3::new(1.0f32, 2.0f32, 3.0f32);
        assert_eq!(v1 - v2, z);

        let v1 = Vec3::new(3.0f32, 5.0f32, 7.0f32);
        let v2 = Vec3::new(1.0f32, 2.0f32, 3.0f32);
        let v3 = Vec3::new(2.0f32, 3.0f32, 4.0f32);

        assert_eq!(v1 - v2, v3);
    }

    #[test]
    fn test_sub_assign() {
        let z = Vec3::zero();
        let mut v1 = Vec3::new(1.0f32, 2.0f32, 3.0f32);
        let v2 = Vec3::new(1.0f32, 2.0f32, 3.0f32);
        v1 -= v2;
        assert_eq!(v1, z);

        let mut v1 = Vec3::new(3.0f32, 5.0f32, 7.0f32);
        let v2 = Vec3::new(1.0f32, 2.0f32, 3.0f32);
        let v3 = Vec3::new(2.0f32, 3.0f32, 4.0f32);
        v1 -= v2;
        assert_eq!(v1, v3);
    }

    #[test]
    fn test_neg() {
        let v1 = Vec3::new(1.0f32, 2.0f32, 3.0f32);
        let v2 = Vec3::new(-1.0f32, -2.0f32, -3.0f32);
        assert_eq!(-v1, v2);

        let v1 = Vec3::new(1.0f32, 2.0f32, 3.0f32);
        let v2 = Vec3::new(-1.0f32, -2.0f32, -3.0f32);
        assert_eq!(-v2, v1);
    }

    #[test]
    fn test_length_squared() {
        let v1 = Vec3::new(1.0f32, 2.0f32, 3.0f32);
        let l2 = (1.0f32 * 1.0f32) + (2.0f32 * 2.0f32) + (3.0f32 * 3.0f32);
        assert_eq!(v1.length_squared(), l2);
    }

    #[test]
    fn test_length() {
        let v1 = Vec3::new(1.0f32, 2.0f32, 3.0f32);
        let l2 = 1.0f32 + (2.0f32 * 2.0f32) + (3.0f32 * 3.0f32);
        assert_eq!(v1.length(), l2.sqrt());
    }

    #[test]
    fn test_dot() {
        let v1 = Vec3::new(1.0f32, 2.0f32, 3.0f32);
        let v2 = Vec3::new(1.0f32, 2.0f32, 3.0f32);

        let dot = v1.0 * v2.0 + v1.1 * v2.1 + v1.2 * v2.2;
        assert_eq!(v1.dot(&v2), dot);
    }

    #[test]
    fn test_cross() {
        let v1 = Vec3::new(1.0f32, 2.0f32, 3.0f32);
        let v2 = Vec3::new(4.0f32, 5.0f32, 6.0f32);
        let v3 = Vec3::new(-3.0f32, 6.0f32, -3.0f32);
        assert_eq!(v1.cross(&v2), v3);

        let v1 = Vec3::new(1.0f32, 2.0f32, 3.0f32);
        let v2 = Vec3::new(4.0f32, 5.0f32, 6.0f32);
        let v3 = Vec3::new(3.0f32, -6.0f32, 3.0f32);
        assert_eq!(v2.cross(&v1), v3);

        let v1 = Vec3::new(1.0f32, 2.0f32, 3.0f32);
        let v2 = Vec3::new(1.0f32, 2.0f32, 3.0f32);
        assert_eq!(v1.cross(&v2), Vec3::zero());
    }

    #[test]
    fn test_mul() {
        let v1 = Vec3::new(1.0f32, 2.0f32, 3.0f32);
        let v2 = Vec3::new(2.0f32, 4.0f32, 6.0f32);
        assert_eq!(v1 * 2f32, v2);
        assert_eq!(2f32 * v1, v2);
    }

    #[test]
    fn test_mul_assign() {
        let mut v1 = Vec3::new(1.0f32, 2.0f32, 3.0f32);
        let v2 = Vec3::new(2.0f32, 4.0f32, 6.0f32);

        v1 *= 2f32;
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_div() {
        let v1 = Vec3::new(1.0f32, 2.0f32, 3.0f32);
        let v2 = Vec3::new(2.0f32, 4.0f32, 6.0f32);

        assert_eq!(v2 / 2f32, v1);
    }

    #[test]
    fn test_div_assign() {
        let mut v1 = Vec3::new(2.0f32, 4.0f32, 6.0f32);
        let v2 = Vec3::new(1.0f32, 2.0f32, 3.0f32);

        v1 /= 2f32;
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_to_u8() {
        let v1 = Vec3::new(0.5f32, -1.0f32, 0.75f32);
        let u1 = [127, 0, 191];
        assert_eq!(v1.to_u8(), u1);
    }

    #[test]
    fn test_to_unit_vector() {
        let v1 = Vec3::new(0.0f32, 4.0f32, 4.0f32);
        let v2 = Vec3::new(0.0f32, 0.70710677f32, 0.70710677f32);

        let l = v1.length();
        assert_eq!(v1 / l, v2);
    }
}
