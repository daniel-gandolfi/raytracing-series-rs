use crate::bounding_box::{BoundingBox, BoundingBoxWrapped};
use crate::material::Material;
use crate::ray::{HitRecord, Ray};
use glam::{FloatExt, Vec3A};
use std::ops::Range;

#[derive(Debug, Clone, derive_more::Constructor)]
pub struct Sphere {
    pub material: Material,
    pub center: Vec3A,
    pub velocity: Vec3A,
    pub radius: f32,
}

impl Sphere {
    fn center(&self, time: f32) -> Vec3A {
        return self.velocity.mul_add(Vec3A::splat(time), self.center);
    }
    pub fn hit(&self, ray: &Ray, range: &Range<f32>) -> Option<HitRecord> {
        let center = self.center(ray.time);
        let oc = ray.origin - center;
        let half_b = oc.dot(ray.direction);

        /*
         * Consider the ray is heading somewhere, eg to (2,2,2) with origin  (1,1,1)
         * Now lets take the sphere with center (3,3,3): from the ray origin perspective the sphere center is (1,1,1)
         *   so they have the same "direction" from the ray origin perspective.
         * If this conditition is false we can skip a couple of checks
         */
        let does_ray_have_same_direction_as_center_from_ray_origin_perspective = -half_b < 0.0;

        if does_ray_have_same_direction_as_center_from_ray_origin_perspective {
            return None;
        }
        let a = ray.direction.length_squared();
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            Option::None
        } else {
            let sqrtd = discriminant.sqrt();
            let mut root = (-half_b - sqrtd) / a;
            if !range.contains(&root) && range.start != root {
                root = (-half_b + sqrtd) / a;
                if !range.contains(&root) && range.start != root {
                    return Option::None;
                }
            }

            let hit_t = root;
            let hit_point = ray.at(hit_t);
            let hit_normal = (hit_point - center) / self.radius;
            let front_face = ray.direction.dot(hit_normal) < 0.0;
            let (u, v) = get_sphere_uv(&hit_normal);
            Option::Some(HitRecord {
                u,
                v,
                time: hit_t,
                point: hit_point,
                normal: if front_face { hit_normal } else { -hit_normal },
                front_face,
                material: Some(&self.material),
            })
        }
    }
}

pub fn get_sphere_uv(point: &Vec3A) -> (f32, f32) {
    const HALF_PI: f32 = std::f32::consts::PI / 2.0;
    let phi = (-point.z).atan2(point.x);
    let theta = point.y.asin();
    assert!(!phi.is_nan());
    assert!(!theta.is_nan());
    let u = phi.remap(-HALF_PI, HALF_PI, 0.0, 1.0);
    let v = (theta).remap(-HALF_PI, HALF_PI, 0.0, 1.0);

    (u, v)
}

impl BoundingBoxWrapped for Sphere {
    fn bounding_box(
        &self,
        shutter_open_time: f32,
        shutter_close_time: f32,
    ) -> Option<crate::bounding_box::BoundingBox> {
        let radius = Vec3A::splat(self.radius);
        let open_position = (self.center - radius, self.center + radius);
        let center_on_shutter_close = self.velocity.mul_add(
            Vec3A::splat(shutter_close_time - shutter_open_time),
            self.center,
        );
        let close_position = (
            center_on_shutter_close - radius,
            center_on_shutter_close + radius,
        );

        Some(BoundingBox::new(
            open_position
                .0
                .min(open_position.1)
                .min(close_position.0)
                .min(close_position.1),
            open_position
                .0
                .max(open_position.1)
                .max(close_position.0)
                .max(close_position.1),
        ))
    }
}
