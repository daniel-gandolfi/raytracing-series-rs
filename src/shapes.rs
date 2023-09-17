use crate::ray::Ray;
use crate::ray::RayHittable;
use glam::DVec3;
use std::ops::Range;

pub struct Sphere {
    pub center: DVec3,
    pub radius: f64,
}

impl RayHittable for Sphere {
    fn hit(self: &Self, ray: &Ray, range: Range<f64>) -> Option<(f64, DVec3, DVec3, bool)> {
        let center = self.center;
        let oc = ray.origin - center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            Option::None
        } else {
            let sqrtd = discriminant.sqrt();
            let mut root = (-half_b - sqrtd) / a;
            if !range.contains(&root) && &range.start != &root {
                root = (-half_b + sqrtd) / a;
                if !range.contains(&root) && &range.start != &root {
                    return Option::None;
                }
            }

            let hit_t = root;
            let hit_point = ray.at(hit_t);
            let hit_normal = (hit_point - center) / self.radius;
            let front_face = ray.direction.dot(hit_normal) < 0.0;
            Option::Some((
                hit_t,
                hit_point,
                if front_face { hit_normal } else { -hit_normal },
                front_face,
            ))
        }
    }
}
