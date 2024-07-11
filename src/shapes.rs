use crate::material::Material;
use crate::ray::{HitRecord, Ray};
use glam::DVec3;
use std::ops::Range;

pub struct Sphere {
    pub center: DVec3,
    pub radius: f64,
    pub material: Material,
    pub velocity: DVec3
}

impl Sphere {
    pub const fn get_material(&self) -> &Material {
        &self.material
    }

    fn center(&self, time: f32) -> DVec3 {
        return self.velocity.mul_add(DVec3::splat(f64::from(time)), self.center)
    }
    pub fn hit(&self, ray: &Ray, range: Range<f64>) -> Option<HitRecord> {
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
            Option::Some(HitRecord {
                time: hit_t,
                point: hit_point,
                normal: if front_face { hit_normal } else { -hit_normal },
                front_face,
            })
        }
    }
}
