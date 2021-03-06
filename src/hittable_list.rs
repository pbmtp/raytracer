use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::camera::ray::Ray;

#[derive(Debug, Default)]
pub struct HittableList<T>
where
    T: Hittable,
{
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

    fn bounding_box(&self) -> Option<Aabb> {
        if self.objects.is_empty() {
            return None;
        }

        let mut result: Option<Aabb> = None;

        for obj in self.objects.iter() {
            if let Some(b) = obj.bounding_box() {
                result = match result {
                    None => Some(b),
                    Some(r) => Some(Aabb::surrounding_box(&r, &b)),
                };
            } else {
                return None;
            }
        }

        result
    }
}
