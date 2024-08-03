use crate::bounding_box::{self, BoundingBoxWrapped};
use crate::camera::Camera;
use crate::material::Material;
use crate::rng::get_rng;
use glam::{f32, Vec3A};
use rand::rngs::SmallRng;
use rand::Rng;
use rayon::prelude::*;
use std::ops::Range;

#[derive(Default, Debug)]
pub struct Ray {
    pub origin: Vec3A,
    pub direction: Vec3A,
    pub time: f32,
}
impl Ray {
    pub fn at(&self, t: f32) -> Vec3A {
        self.direction.mul_add(Vec3A::splat(t), self.origin)
    }
}

pub struct HitRecord<'a> {
    pub point: Vec3A,
    pub normal: Vec3A,
    pub time: f32,
    pub front_face: bool,
    pub material: Option<&'a Material>,
}

#[derive(Debug, Clone)]
pub enum RayHittableEnum {
    Sphere(crate::shapes::Sphere),
}

impl RayHittableEnum {
    pub fn hit(&self, ray: &Ray, range: &Range<f32>) -> Option<HitRecord> {
        match self {
            RayHittableEnum::Sphere(s) => s.hit(ray, range),
        }
    }
    pub fn bounding_box(
        &self,
        shutter_open_time: f32,
        shutter_close_time: f32,
    ) -> Option<bounding_box::BoundingBox> {
        match self {
            RayHittableEnum::Sphere(s) => s.bounding_box(shutter_open_time, shutter_close_time),
        }
    }
}

fn random_vec3_clamp(rng: &mut SmallRng, min: f32, max: f32) -> Vec3A {
    Vec3A::new(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max),
    )
}

fn random_in_unit_sphere() -> Vec3A {
    loop {
        let random_vec = random_vec3_clamp(crate::rng::get_rng(), -1.0, 1.0);
        if random_vec.length_squared() < 1.0 {
            return random_vec;
        }
    }
}
pub fn random_unit_vector() -> Vec3A {
    random_in_unit_sphere().normalize()
}
fn random_on_hemisphere(hit_normal: &Vec3A) -> Vec3A {
    let on_unit_sphere = random_unit_vector();
    if hit_normal.dot(on_unit_sphere) > 0.0 {
        // In the same hemisphere as the normal
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

fn random_in_unit_disk() -> Vec3A {
    let rng = get_rng();
    loop {
        let p = Vec3A::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

fn pixel_sample_square(pixel_delta_u: Vec3A, pixel_delta_v: Vec3A) -> Vec3A {
    let rng = get_rng();
    let px = rng.gen_range(-0.5..0.5);
    let py = rng.gen_range(-0.5..0.5);
    px * pixel_delta_u + py * pixel_delta_v
}

fn defocus_disk_sample(
    camera_position: Vec3A,
    defocus_disk_u: Vec3A,
    defocus_disk_v: Vec3A,
) -> Vec3A {
    let p = random_in_unit_disk();
    camera_position + (p.x * defocus_disk_u) + (p.y * defocus_disk_v)
}

pub fn create_rays<const SAMPLES_PER_SQUARE: usize>(
    camera: &Camera,
) -> impl IndexedParallelIterator<Item = (u32, [Ray; SAMPLES_PER_SQUARE])> {
    let pixel00_loc = camera.pixel_00_loc();
    let pixel_delta_u = camera.delta_pixel_u();
    let pixel_delta_v = camera.delta_pixel_v();
    let camera_width = camera.width as u32;
    let camera_height = camera.height as u32;
    let defocus_angle = camera.defocus_angle();

    let time0 = camera.open_time;
    let time1 = camera.close_time;
    let camera_position = camera.position;
    let defocus_disk_u = camera.defocus_disk_u();
    let defocus_disk_v = camera.defocus_disk_v();

    (0..(camera_height * camera_width))
        .into_par_iter()
        .map(move |compound_width_height| {
            let j = compound_width_height / camera_width;
            let i = compound_width_height % camera_width;
            let pixel_center =
                pixel00_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);

            assert!(SAMPLES_PER_SQUARE % 4 == 0);

            let rays: [Ray; SAMPLES_PER_SQUARE] = core::array::from_fn(|_| {
                let ray_origin = if defocus_angle <= 0.0 {
                    camera_position
                } else {
                    defocus_disk_sample(camera_position, defocus_disk_u, defocus_disk_v)
                };
                let pixel_sample = pixel_center + pixel_sample_square(pixel_delta_u, pixel_delta_v);
                let ray_direction = pixel_sample - ray_origin;

                Ray {
                    origin: ray_origin,
                    direction: ray_direction,
                    time: get_rng().gen_range(time0..time1),
                }
            });
            (compound_width_height, rays)
        })
}
