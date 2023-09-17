use crate::camera::Camera;
use glam::DVec3;
use std::ops::Mul;
use std::ops::Range;

#[derive(Default, Debug)]
pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3,
}
impl Ray {
    pub fn at(self: &Self, t: f64) -> DVec3 {
        self.origin + t * self.direction
    }
}
pub trait RayHittable {
    fn hit(
        self: &Self,
        ray: &Ray,
        range: Range<f64>,
    ) -> Option<(
        f64,   // t
        DVec3, // point
        DVec3, // normal,
        bool,  // front_face
    )>;
}

pub fn ray_color(ray: Ray, world: &Vec<Box<dyn RayHittable>>) -> DVec3 {
    let normal_opt: Option<(f64, DVec3, DVec3, bool)> = world.iter().find_map(|obj| {
        let range = 0.0..(f64::INFINITY);
        obj.hit(&ray, range)
    });

    if let Some(hit_record) = normal_opt {
        return (hit_record.2 + DVec3::ONE).mul(0.5);
    }
    let unit = ray.direction.normalize_or_zero();
    let a = 0.5 * (unit.y + 1.0);
    return (1.0 - a) * DVec3::new(1.0, 1.0, 1.0) + a * DVec3::new(0.5, 0.7, 1.0);
}

pub fn create_rays(camera: &Camera) -> impl Iterator<Item = Ray> {
    let pixel00_loc = camera.pixel_00_loc();
    let pixel_delta_u = camera.delta_pixel_u();
    let pixel_delta_v = camera.delta_pixel_v();
    let camera_position = camera.position.clone();
    let camera_width = camera.width;
    let camera_height = camera.height;

    (0..camera_height).flat_map(move |j| {
        (0..camera_width).map(move |i| {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let direction = pixel_center - camera_position;
            Ray {
                origin: camera_position.clone(),
                direction,
            }
        })
    })
}
