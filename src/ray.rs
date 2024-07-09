use crate::camera::Camera;
use crate::material::Material;
use glam::DVec3;
use rand::rngs::ThreadRng;
use rand::thread_rng;
use rand::Rng;
use rayon::prelude::*;
use std::ops::Range;

#[derive(Default, Debug)]
pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3,
}
impl Ray {
    pub fn at(&self, t: f64) -> DVec3 {
        self.direction.mul_add(DVec3::splat(t), self.origin)
    }
}

pub struct HitRecord {
    pub point: DVec3,
    pub normal: DVec3,
    pub time: f64,
    pub front_face: bool,
}

pub enum RayHittableEnum {
    Sphere(crate::shapes::Sphere),
}

impl RayHittableEnum {
    pub const fn get_material(&self) -> &Material {
        match self {
            RayHittableEnum::Sphere(s) => s.get_material(),
        }
    }

    fn hit(&self, ray: &Ray, range: Range<f64>) -> Option<HitRecord> {
        match self {
            RayHittableEnum::Sphere(s) => s.hit(ray, range),
        }
    }
}
fn random_vec3_clamp(rng: &mut ThreadRng, min: f64, max: f64) -> DVec3 {
    DVec3::new(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max),
    )
}
fn random_in_unit_sphere() -> DVec3 {
    let mut rng = thread_rng();
    loop {
        let random_vec = random_vec3_clamp(&mut rng, -1.0, 1.0);
        if random_vec.length_squared() < 1.0 {
            return random_vec;
        }
    }
}
pub fn random_unit_vector() -> DVec3 {
    random_in_unit_sphere().normalize()
}
fn random_on_hemisphere(hit_normal: &DVec3) -> DVec3 {
    let on_unit_sphere = random_unit_vector();
    if hit_normal.dot(on_unit_sphere) > 0.0 {
        // In the same hemisphere as the normal
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

fn random_in_unit_disk() -> DVec3 {
    let mut rng = thread_rng();
    loop {
        let p = DVec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn ray_color(ray: &Ray, max_bounces: u8, world: &Vec<RayHittableEnum>) -> DVec3 {
    world
        .iter()
        .rev()
        .find_map(|obj| {
            let range = 0.001..(f64::INFINITY);

            obj.hit(ray, range).and_then(|hit| {
                obj.get_material()
                    .on_ray_hit(ray, &hit)
                    .map(|calc_hit_data| {
                        let attenuation = calc_hit_data.attenuation;
                        let rebounce = calc_hit_data.rebounce;

                        // If we've exceeded the ray bounce limit, no more light is gathered.
                        if max_bounces <= 1 || attenuation == DVec3::ZERO {
                            return DVec3::ZERO;
                        }

                        attenuation * ray_color(&rebounce, max_bounces - 1, world)
                    })
                    .or(Some(DVec3::ZERO))
            })
        })
        .unwrap_or_else(|| {
            let unit = ray.direction.normalize_or_zero();
            let a = 0.5 * (unit.y + 1.0);
            (1.0 - a) * DVec3::ONE + a * DVec3::new(0.5, 0.7, 1.0)
        })
}

fn pixel_sample_square(pixel_delta_u: DVec3, pixel_delta_v: DVec3) -> DVec3 {
    let mut rng = thread_rng();
    let px = rng.gen_range(-0.5..0.5);
    let py = rng.gen_range(-0.5..0.5);
    px * pixel_delta_u + py * pixel_delta_v
}

fn defocus_disk_sample(camera_position: DVec3,defocus_disk_u: DVec3, defocus_disk_v:DVec3) -> DVec3 {
    let p = random_in_unit_disk();
     camera_position + (p.x * defocus_disk_u) + (p.y *defocus_disk_v)
}

pub fn create_rays(
    camera: &Camera,
    samples_per_square: usize,
) -> impl IndexedParallelIterator<Item = impl Iterator<Item = Ray>> {


    let pixel00_loc = camera.pixel_00_loc();
    let pixel_delta_u = camera.delta_pixel_u();
    let pixel_delta_v = camera.delta_pixel_v();
    let camera_width = camera.width as u32;
    let camera_height = camera.height as u32;
    let defocus_angle = camera.defocus_angle();

    let camera_position = camera.position;
    let defocus_disk_u = camera.defocus_disk_u();
    let defocus_disk_v = camera.defocus_disk_v();
    

    (0..(camera_height * camera_width))
        .into_par_iter()
        .map(move |compound_width_height| {

            let j = compound_width_height / camera_width;
            let i = compound_width_height % camera_width;
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);

            (0..samples_per_square).map(move |_| {
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
                }
            })
        })
}
