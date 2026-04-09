use crate::geometry::hittable::{Hit, Hittable};
use crate::geometry::aabb::AABB;
use crate::math::ray::Ray;
use std::sync::Arc;

pub struct BVHNode {
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>,
    pub bbox: AABB,
}

impl BVHNode {
    pub fn new(mut objects: Vec<Arc<dyn Hittable>>) -> Arc<dyn Hittable> {
        let axis = rand::random::<usize>() % 3;

        objects.sort_by(|a, b| {
            let box_a = a.bounding_box().min;
            let box_b = b.bounding_box().min;
            
            let val_a = match axis {
                0 => box_a.x,
                1 => box_a.y,
                _ => box_a.z,
            };
            let val_b = match axis {
                0 => box_b.x,
                1 => box_b.y,
                _ => box_b.z,
            };

            val_a.partial_cmp(&val_b).unwrap()
        });

        let len = objects.len();
        if len == 1 {
            return objects[0].clone();
        } else if len == 2 {
            let left = objects[0].clone();
            let right = objects[1].clone();
            let bbox = AABB::surrounding_box(left.bounding_box(), right.bounding_box());
            return Arc::new(BVHNode { left, right, bbox });
        }

        let right_objects = objects.split_off(len / 2);
        let left = BVHNode::new(objects);
        let right = BVHNode::new(right_objects);
        let bbox = AABB::surrounding_box(left.bounding_box(), right.bounding_box());

        Arc::new(BVHNode { left, right, bbox })
    }
}

impl Hittable for BVHNode {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        if !self.bbox.intersect(ray) {
            return None;
        }

        let hit_left = self.left.intersect(ray);
        let hit_right = self.right.intersect(ray);

        match (hit_left, hit_right) {
            (Some(l), Some(r)) => if l.distance < r.distance { Some(l) } else { Some(r) },
            (Some(l), None) => Some(l),
            (None, Some(r)) => Some(r),
            (None, None) => None,
        }
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}