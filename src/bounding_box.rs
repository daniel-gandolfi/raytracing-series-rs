use glam::Vec3A;

use crate::ray::{HitRecord, Ray, RayHittableEnum};

pub trait BoundingBoxWrapped {
    fn bounding_box(&self, shutter_open_time: f32, shutter_close_time: f32) -> Option<BoundingBox>;
}

#[derive(PartialEq, Debug, Clone, Copy, derive_more::Constructor)]
pub struct BoundingBox {
    pub min: Vec3A,
    pub max: Vec3A,
}

impl BoundingBox {
    pub fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> bool {
        let a_intersection = (self.min - ray.origin) / ray.direction;
        let b_intersection = (self.max - ray.origin) / ray.direction;

        let min_intersection = a_intersection.min(b_intersection);
        let max_intersection = a_intersection.max(b_intersection);

        let int_or_max_limit = max_intersection.max(Vec3A::splat(tmax));
        let int_or_min_limit = min_intersection.min(Vec3A::splat(tmin));

        return int_or_max_limit.cmpgt(int_or_min_limit).all();
    }
}

#[derive(Clone)]
pub enum BVHChildren<'a, const CHILD_ARR_LEN: usize> {
    RayHittables(&'a [RayHittableEnum]),
    Nodes([Box<BVHNode<'a, CHILD_ARR_LEN>>; 2]),
}
#[derive(Clone)]
pub struct BVHNode<'a, const CHILD_ARR_LEN: usize> {
    bb_cache: Option<BoundingBox>,
    children: BVHChildren<'a, CHILD_ARR_LEN>,
}

pub fn map_iterator_to_minmax_points(
    bounding_boxes: impl Iterator<Item = Option<BoundingBox>>,
) -> (Vec3A, Vec3A) {
    bounding_boxes
        .flatten()
        .flat_map(|bounding_box| [bounding_box.min, bounding_box.max] as [Vec3A; 2])
        .fold((Vec3A::ZERO, Vec3A::ZERO), |acc, border| {
            (acc.0.min(border), acc.1.max(border))
        })
}
impl<const CHILD_ARR_LEN: usize> BVHNode<'_, CHILD_ARR_LEN> {
    pub fn new(
        sorted_items: &[RayHittableEnum],
        shutter_open_time: f32,
        shutter_close_time: f32,
    ) -> BVHNode<'_, CHILD_ARR_LEN> {
        if sorted_items.len() > CHILD_ARR_LEN {
            let midpoint = sorted_items.len() / 2;
            let (left, right) = sorted_items.split_at(midpoint);
            let children = [
                Box::new(BVHNode::new(left, shutter_open_time, shutter_close_time)),
                Box::new(BVHNode::new(right, shutter_open_time, shutter_close_time)),
            ];
            let mut node = BVHNode {
                bb_cache: None,
                children: BVHChildren::Nodes(children),
            };
            node.bounding_box(shutter_open_time, shutter_close_time);
            return node;
        } else {
            assert!(sorted_items.len() <= CHILD_ARR_LEN);
            let mut node = BVHNode {
                bb_cache: None,
                children: BVHChildren::RayHittables(sorted_items),
            };
            node.bounding_box(shutter_open_time, shutter_close_time);
            return node;
        }
    }
    pub fn hit(&self, ray: &Ray, range: &core::ops::Range<f32>) -> Option<HitRecord> {
        let bbox_hit = self
            .bb_cache
            .map(|bbox| bbox.hit(ray, range.start, range.end));

        // None -> undeterministic/non-computable, theere is an unboundable object inside
        // Some(true ) -> hit
        // Some(false) -> no hit
        let skip_children = matches!(bbox_hit, Some(false));

        if skip_children {
            return None;
        }

        match &self.children {
            BVHChildren::Nodes(children) => {
                let [left_node, right_node] = children;

                left_node
                    .hit(ray, range)
                    .or_else(|| right_node.hit(ray, range))
            }
            BVHChildren::RayHittables(children) => {
                children.iter().find_map(|child| child.hit(ray, range))
            }
        }
    }
}
impl<const CHILDREN_ARR_LEN: usize> BVHNode<'_, CHILDREN_ARR_LEN> {
    fn bounding_box(
        &mut self,
        shutter_open_time: f32,
        shutter_close_time: f32,
    ) -> Option<BoundingBox> {
        let cache = self.bb_cache;
        if cache.is_some() {
            return cache;
        }

        let children = &mut self.children;
        let minmax = match children {
            BVHChildren::RayHittables(children) => {
                assert!(children.len() <= CHILDREN_ARR_LEN);
                map_iterator_to_minmax_points(
                    children
                        .iter()
                        .map(|item| item.bounding_box(shutter_open_time, shutter_close_time)),
                )
            }
            BVHChildren::Nodes(children) => map_iterator_to_minmax_points(
                children
                    .iter_mut()
                    .map(|item| item.bounding_box(shutter_open_time, shutter_close_time)),
            ),
        };

        let bb = Some(BoundingBox {
            min: minmax.0,
            max: minmax.1,
        });
        self.bb_cache = bb;
        return self.bb_cache;
    }
}
