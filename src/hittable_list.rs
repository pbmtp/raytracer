use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

#[derive(Debug, Default)]
pub struct HittableList<T> {
    pub objects: Vec<Box<T>>,
}

impl<T: Hittable> HittableList<T> {
    pub fn new(object: T) -> HittableList<T> {
        HittableList::<T> {
            objects: vec![Box::new(object)],
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }
}

impl<T: Hittable> Hittable for HittableList<T> {
        fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let mut closest = None;
        let mut closest_so_far = tmax;

        for obj in self.objects.iter() {
            if let Some(hr) = obj.hit(r, tmin, closest_so_far) {
                closest_so_far = hr.get_t();
                closest = Some(hr);
            }
        }

        closest
    }
}
