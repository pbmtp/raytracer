use std::cmp::Ordering;

use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::tools::random_usize_range;

pub struct BvhNode {
    bbox: Aabb,
    left: Option<Box<dyn Hittable>>,
    right: Option<Box<dyn Hittable>>,
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        if !self.bbox.hit(r, tmin, tmax) {
            return None;
        }

        let hit_left = match &self.left {
            None => None,
            Some(hr) => hr.hit(r, tmin, tmax),
        };
        let hit_right = match &self.right {
            None => None,
            Some(hr) => hr.hit(r, tmin, tmax),
        };

        match (hit_left, hit_right) {
            (None, None) => None,
            (None, Some(hr)) => Some(hr),
            (Some(hr), None) => Some(hr),
            (Some(left_hr), Some(right_hr)) => {
                if left_hr.get_t() < right_hr.get_t() {
                    Some(left_hr)
                } else {
                    Some(right_hr)
                }
            }
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(self.bbox)
    }
}

fn get_aabbs(
    left: &Box<dyn Hittable>,
    right: &Box<dyn Hittable>,
    time0: f64,
    time1: f64,
) -> (Aabb, Aabb) {
    let left_box = left.bounding_box(time0, time1);
    let right_box = right.bounding_box(time0, time1);

    let left_box = match left_box {
        Some(bounding_box) => bounding_box,
        None => panic!("Hittable with no bounding boxes are not supported"),
    };
    let right_box = match right_box {
        Some(bounding_box) => bounding_box,
        None => panic!("Hittable with no bounding boxes are not supported"),
    };

    (left_box, right_box)
}

fn compare_box_by_x_axis(
    left: &Box<dyn Hittable>,
    right: &Box<dyn Hittable>,
    time0: f64,
    time1: f64,
) -> Ordering {
    let (left_box, right_box) = get_aabbs(left, right, time0, time1);

    // Should never get a NaN here. Panic if we do
    left_box
        .min()
        .x()
        .partial_cmp(&right_box.min().x())
        .unwrap()
}

fn compare_box_by_y_axis(
    left: &Box<dyn Hittable>,
    right: &Box<dyn Hittable>,
    time0: f64,
    time1: f64,
) -> Ordering {
    let (left_box, right_box) = get_aabbs(left, right, time0, time1);

    // Should never get a NaN here. Panic if we do
    left_box
        .min()
        .y()
        .partial_cmp(&right_box.min().y())
        .unwrap()
}

fn compare_box_by_z_axis(
    left: &Box<dyn Hittable>,
    right: &Box<dyn Hittable>,
    time0: f64,
    time1: f64,
) -> Ordering {
    let (left_box, right_box) = get_aabbs(left, right, time0, time1);

    // Should never get a NaN here. Panic if we do
    left_box
        .min()
        .z()
        .partial_cmp(&right_box.min().y())
        .unwrap()
}

impl BvhNode {
    pub fn new(objects: Vec<Box<dyn Hittable>>, time0: f64, time1: f64) -> BvhNode {
        let axis = random_usize_range(0, 2);
        let compare = match axis {
            0 => compare_box_by_x_axis,
            1 => compare_box_by_y_axis,
            _ => compare_box_by_z_axis,
        };

        let mut objs: Vec<Box<dyn Hittable>> = objects;
        objs.sort_unstable_by(|a, b| compare(a, b, time0, time1));

        let len = objs.len();

        let (left, right) = match len {
            0 => (None, None),
            1 => (Some(objs.remove(0)), None),
            2 => (Some(objs.remove(0)), Some(objs.remove(0))),
            _ => {
                let mid = len / 2;

                let left_objs = objs.drain(0..mid).collect();
                let right_objs = objs;

                let left: Box<dyn Hittable> = Box::new(BvhNode::new(left_objs, time0, time1));
                let right: Box<dyn Hittable> = Box::new(BvhNode::new(right_objs, time0, time1));

                (Some(left), Some(right))
            }
        };

        let bbox = match (&left, &right) {
            (Some(left_objs), Some(right_objs)) => Some(Aabb::surrounding_box(
                &left_objs.bounding_box(time0, time1).unwrap(),
                &right_objs.bounding_box(time0, time1).unwrap(),
            )),
            (Some(left_objs), None) => left_objs.bounding_box(time0, time1),
            (None, Some(right_objs)) => right_objs.bounding_box(time0, time1),
            (None, None) => None,
        };

        let bbox = match bbox {
            Some(bbox) => bbox,
            None => panic!("Geometries with no bounding boxes are not supported"),
        };

        BvhNode { bbox, left, right }
    }
}
