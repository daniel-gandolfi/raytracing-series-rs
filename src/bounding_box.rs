use glam::DVec3;

use crate::ray::{HitRecord, Ray, RayHittableEnum};

pub trait BoundingBoxWrapped {
    fn bounding_box(
        self: &Self,
        shutter_open_time: f32,
        shutter_close_time: f32,
    ) -> Option<BoundingBox>;
}

#[derive(PartialEq, Debug, Clone, Copy, derive_more::Constructor)]
pub struct BoundingBox {
    pub min: DVec3,
    pub max: DVec3,
}

impl BoundingBox {
    pub fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> bool {
        let a_intersection = (self.min - ray.origin) / ray.direction;
        let b_intersection = (self.max - ray.origin) / ray.direction;

        let min_intersection = a_intersection.min(b_intersection);
        let max_intersection = a_intersection.max(b_intersection);

        let int_or_max_limit = max_intersection.max(DVec3::splat(tmax));
        let int_or_min_limit = min_intersection.min(DVec3::splat(tmin));

        return int_or_max_limit.cmpgt(int_or_min_limit).all();
    }
}
struct BVHLeafData<'a> {
    bb_cache: Option<BoundingBox>,
    children: &'a [RayHittableEnum],
}

struct BVHNodeData<'a> {
    bb_cache: Option<BoundingBox>,
    children: Box<[BVHNode<'a>; 2]>,
}

pub enum BVHNode<'a> {
    Leaf(BVHLeafData<'a>),
    Node(BVHNodeData<'a>),
}

pub fn map_iterator_to_minmax_points(
    bounding_boxes: impl Iterator<Item = Option<BoundingBox>>,
) -> (DVec3, DVec3) {
    bounding_boxes
        .filter(Option::is_some)
        .map(Option::unwrap)
        .flat_map(|bounding_box| [bounding_box.min, bounding_box.max] as [DVec3; 2])
        .fold((DVec3::ZERO, DVec3::ZERO), |acc, border| {
            (acc.0.min(border), acc.1.max(border))
        })
}
impl BVHNode<'_> {
    pub fn new(
        sorted_items: &[RayHittableEnum],
        max_items_in_node: usize,
        shutter_open_time: f32,
        shutter_close_time: f32,
    ) -> BVHNode {
        if sorted_items.len() > max_items_in_node {
            let midpoint = sorted_items.len() / 2;
            let (left, right) = sorted_items.split_at(midpoint);
            let children = Box::new([
                BVHNode::new(
                    left,
                    max_items_in_node,
                    shutter_open_time,
                    shutter_close_time,
                ),
                BVHNode::new(
                    right,
                    max_items_in_node,
                    shutter_open_time,
                    shutter_close_time,
                ),
            ]);
            return BVHNode::Node(BVHNodeData {
                bb_cache: None,
                children,
            });
        } else {
            return BVHNode::Leaf(BVHLeafData {
                bb_cache: None,
                children: sorted_items,
            });
        }
    }
    pub fn hit(&self, ray: &Ray, range: &core::ops::Range<f64>) -> Option<HitRecord> {
        match self {
            BVHNode::Node(data) => {
                let [left_node, right_node] = data.children.as_ref();
                left_node
                    .hit(ray, range)
                    .or_else(|| right_node.hit(ray, range))
            }
            BVHNode::Leaf(data) => {
                for i in 0..data.children.len() {
                    let hit = data.children.get(i)?.hit(ray, range);
                    if hit.is_some() {
                        return hit;
                    }
                }
                return None;
            }
        }
    }
}
impl BVHNode<'_> {
    fn bounding_box(
        &mut self,
        shutter_open_time: f32,
        shutter_close_time: f32,
    ) -> Option<BoundingBox> {
        let cache = match self {
            BVHNode::Leaf(data) => data.bb_cache,
            BVHNode::Node(data) => data.bb_cache,
        };
        if cache.is_some() {
            return cache;
        }
        let minmax = match self {
            BVHNode::Leaf(data) => map_iterator_to_minmax_points(
                data.children
                    .iter()
                    .map(|item| item.bounding_box(shutter_open_time, shutter_close_time)),
            ),
            BVHNode::Node(data) => map_iterator_to_minmax_points(
                data.children
                    .iter_mut()
                    .map(|item| item.bounding_box(shutter_open_time, shutter_close_time)),
            ),
        };

        let bb = Some(BoundingBox {
            min: minmax.0,
            max: minmax.1,
        });
        return match self {
            BVHNode::Leaf(data) => {
                data.bb_cache = bb;
                data.bb_cache
            }
            BVHNode::Node(data) => {
                data.bb_cache = bb;
                data.bb_cache
            }
        };
    }
}
