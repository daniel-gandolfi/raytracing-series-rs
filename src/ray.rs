use crate::camera::Camera;
use crate::material::Material;
use glam::DVec3;
use rand::rngs::ThreadRng;
use rand::thread_rng;
use rand::Rng;
use std::ops::Range;

#[derive(Default, Debug)]
pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3,
}
impl Ray {
    pub fn at(&self, t: f64) -> DVec3 {
        self.origin + t * self.direction
    }
}

pub struct HitRecord {
    pub point: DVec3,
    pub normal: DVec3,
    pub time: f64,
    pub front_face: bool,
}
pub trait RayHittable {
    fn get_material(&self) -> &Material;
    fn hit(&self, ray: &Ray, range: Range<f64>) -> Option<HitRecord>;
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

pub fn ray_color(ray: &Ray, max_bounces: u8, world: &Vec<Box<dyn RayHittable>>) -> DVec3 {
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
            (1.0 - a) * DVec3::new(1.0, 1.0, 1.0) + a * DVec3::new(0.5, 0.7, 1.0)
        })
}

fn pixel_sample_square(pixel_delta_u: DVec3, pixel_delta_v: DVec3) -> DVec3 {
    let mut rng = thread_rng();
    let px = -0.5 + rng.gen_range(0.0..1.0);
    let py = -0.5 + rng.gen_range(0.0..1.0);
    px * pixel_delta_u + py * pixel_delta_v
}

fn defocus_disk_sample(camera: &Camera) -> DVec3 {
    let p = random_in_unit_disk();
    camera.position + (p.x * camera.defocus_disk_u()) + (p.y * camera.defocus_disk_v())
}

pub fn create_rays(camera: &Camera, samples_per_square: usize) -> impl Iterator<Item = Ray> {
    let ray_origin = if camera.defocus_angle() <= 0.0 {
        camera.position
    } else {
        defocus_disk_sample(camera)
    };

    let pixel00_loc = camera.pixel_00_loc();
    let pixel_delta_u = camera.delta_pixel_u();
    let pixel_delta_v = camera.delta_pixel_v();
    let camera_width = camera.width;
    let camera_height = camera.height;

    (0..camera_height).flat_map(move |j| {
        (0..camera_width).flat_map(move |i| {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            (0..samples_per_square)
                .map(move |_| pixel_center + pixel_sample_square(pixel_delta_u, pixel_delta_v))
                .map(move |pixel_sample| {
                    let ray_direction = pixel_sample - ray_origin;

                    Ray {
                        origin: ray_origin,
                        direction: ray_direction,
                    }
                })
        })
    })
}
