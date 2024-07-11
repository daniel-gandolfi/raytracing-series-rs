#![feature(thread_local)]
#![feature(iter_array_chunks)]
#![feature(iter_collect_into)]
#![feature(test)]
#![feature(const_fn_floating_point_arithmetic)]

extern crate test;

use glam::DVec3;
use indicatif::*;
mod camera;
mod material;
mod ppm_renderer;
mod ray;
mod rng;
mod shapes;
mod world_creation;
use crate::camera::Camera;
use crate::world_creation::heavy_rand_world::create_world;
use ppm_renderer::RayTracingRenderer;
use ray::{create_rays, ray_color};
use rayon::iter::ParallelIterator;
use rayon::prelude::*;

fn create_camera() -> Camera {
    const WIDTH: u16 = 1200_u16;

    const LOOKFROM: DVec3 = DVec3::new(13.0, 2.0, 3.0);
    const LOOKAT: DVec3 = DVec3::new(0.0, 0.0, 0.0);
    const VUP: DVec3 = DVec3::new(0.0, 1.0, 0.0);
    const FOV: f32 = 20.0;
    Camera::new(
        LOOKFROM,
        LOOKAT,
        VUP,
        WIDTH,
        FOV,
        16.0 / 9.0,
        0.0,
        10.0,
        0.0,
        1.0,
    )
}
const SAMPLES_PER_PIXEL: usize = 500;
const MAX_RAY_BOUNCES: u8 = 50;
const COLOR_CLAMP_UPPER: DVec3 = DVec3::splat(0.99999999999);
const RAY_SAMPLE_SCALE_FACTOR: DVec3 = DVec3::splat(1.0 / SAMPLES_PER_PIXEL as f64);

fn main() -> std::io::Result<()> {
    let camera = create_camera();
    let world = create_world(&camera);

    let pixel_colors = create_rays(&camera, SAMPLES_PER_PIXEL)
        .map(|ray_iterator| {
            let mut color = DVec3::ZERO;
            for ray in ray_iterator {
                color += ray_color(&ray, MAX_RAY_BOUNCES, &world)
            }
            color *= RAY_SAMPLE_SCALE_FACTOR;
            return color.clamp(DVec3::ZERO, COLOR_CLAMP_UPPER);
        })
        .progress_count(camera.width as u64 * camera.height as u64);

    let mut pixel_vec = Vec::with_capacity((camera.width * camera.height) as usize);
    pixel_colors.collect_into_vec(&mut pixel_vec);
    let renderer = ppm_renderer::PpmImageRenderer::new("render.ppm").unwrap();

    renderer.render(camera.width, camera.height, pixel_vec.iter());
    Ok(())
}
