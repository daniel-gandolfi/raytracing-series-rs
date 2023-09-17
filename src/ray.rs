use crate::camera::Camera;
use glam::DVec3;
use rand::Rng;
use std::ops::Mul;
use std::ops::Range;
use rand::thread_rng;

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

pub fn ray_color(ray: &Ray, world: &Vec<Box<dyn RayHittable>>) -> DVec3 {
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
fn pixel_sample_square(
    pixel_delta_u: DVec3,
    pixel_delta_v: DVec3
) -> DVec3 {
    let mut rng = thread_rng();
    let px = -0.5 + rng.gen_range(0.0..1.0);
    let py = -0.5 + rng.gen_range(0.0..1.0);
    px * pixel_delta_u + py *pixel_delta_v
}

pub fn create_rays(camera: &Camera, samples_per_square: usize) -> impl Iterator<Item = Ray> {
    let pixel00_loc = camera.pixel_00_loc();
    let pixel_delta_u = camera.delta_pixel_u();
    let pixel_delta_v = camera.delta_pixel_v();
    let camera_position = camera.position.clone();
    let camera_width = camera.width;
    let camera_height = camera.height;

    (0..camera_height).flat_map(move |j| {
        (0..camera_width).flat_map(move |i| {

            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            (0..samples_per_square).into_iter().map(move |_| {
                pixel_center + pixel_sample_square(
                    pixel_delta_u,
                    pixel_delta_v
                )
            }).map(move |pixel_sample|{   
                let ray_origin = camera_position.clone();
                let ray_direction = pixel_sample - ray_origin;

                Ray {
                    origin: ray_origin,
                    direction: ray_direction,
                }
            })
        })
    })
}
