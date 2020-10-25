use super::hittable::{HitRecord, Hittable};
use super::ray::Ray;
use super::vec3::Vec3;

#[derive(Debug, Default)]
pub struct HittableList<T> {
    pub objects: Vec<T>,
}

impl<T: Hittable> HittableList<T> {
    pub fn new(object: T) -> HittableList<T> {
        HittableList::<T> {
            objects: vec![object],
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: T) {
        self.objects.push(object);
    }
}

impl<T: Hittable> Hittable for HittableList<T> {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let mut rec = HitRecord::new(Vec3::zero(), Vec3::zero(), 0.0);
        let mut hit_anything = false;
        let mut closest_so_far = tmax;

        for obj in self.objects.iter() {
            if let Some(hr) = obj.hit(r, tmin, closest_so_far) {
                hit_anything = true;
                closest_so_far = hr.get_t();
                rec = hr;
            }
        }
        if hit_anything {
            Some(rec)
        } else {
            None
        }
    }
}
