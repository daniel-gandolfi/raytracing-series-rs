#![allow(clippy::needless_return)]
#![feature(thread_local)]
#![feature(const_trait_impl)]
#![feature(iter_array_chunks)]
#![feature(iter_collect_into)]
#![feature(test)]
#![feature(const_fn_floating_point_arithmetic)]

extern crate test;

use std::sync::mpsc::sync_channel;

use glam::Vec3A;
use indicatif::*;
use ipc::{ClientCommands, MainCommands};
mod bounding_box;
mod camera;
mod ipc;
mod material;
mod pixel_renderer;
mod ppm_renderer;
mod ray;
mod rng;
mod shapes;
mod texture;
mod world_creation;
use crate::camera::Camera;
use crate::world_creation::checkerbox_pattern::{create_camera, create_world};
use ray::{create_rays, Ray, RayHittableEnum};
use rayon::iter::ParallelIterator;
use rayon::prelude::*;

const WIDTH: u16 = if cfg!(debug_assertions) {
    20_u16
} else {
    1200_u16
};
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const HEIGHT: u16 = (WIDTH as f64 / ASPECT_RATIO) as u16;
const PIXEL_COUNT: usize = WIDTH as usize * HEIGHT as usize;

const SAMPLES_PER_PIXEL: usize = if cfg!(debug_assertions) { 4 } else { 48 };
const MAX_RAY_BOUNCES: u8 = if cfg!(debug_assertions) { 4 } else { 8 };
const RAY_SAMPLE_SCALE_FACTOR: f32 = 1.0 / SAMPLES_PER_PIXEL as f32;

fn prepare_world(camera: &Camera) -> Vec<RayHittableEnum> {
    let mut world = create_world(&camera);
    world.sort_by(|a, b| {
        let bb_a = a.bounding_box(camera.open_time, camera.close_time);
        let bb_b = b.bounding_box(camera.open_time, camera.close_time);
        if bb_a.is_none() {
            return std::cmp::Ordering::Greater;
        };
        if bb_b.is_none() {
            return std::cmp::Ordering::Less;
        };
        let a_dist = bb_a
            .unwrap()
            .min
            .distance_squared(camera.position)
            .min(bb_a.unwrap().max.distance_squared(camera.position));
        let b_dist = bb_b
            .unwrap()
            .min
            .distance_squared(camera.position)
            .min(bb_b.unwrap().max.distance_squared(camera.position));
        return a_dist.total_cmp(&b_dist);
    });

    world
}

enum Renderer {
    Ppm,
    PixelWindow { partial_render: bool },
}

impl Renderer {
    const fn has_partial_rendering(&self) -> &bool {
        match self {
            Renderer::Ppm => &false,
            Renderer::PixelWindow { partial_render } => partial_render,
        }
    }
}

#[inline]
fn skybox_color(ray_direction: Vec3A) -> Vec3A {
    let unit = ray_direction.normalize_or_zero();
    let a = 0.5 * (unit.y + 1.0);
    (1.0 - a) * 1.0 + a * Vec3A::new(0.5, 0.7, 1.0)
}

#[inline]
pub fn ray_color<const BVH_CHILDREN_SIZE: usize>(
    ray: &Ray,
    max_bounces: u8,
    world: &bounding_box::BVHNode<BVH_CHILDREN_SIZE>,
) -> Vec3A {
    const RANGE: std::ops::Range<f32> = 0.001..(f32::INFINITY);
    world
        .hit(ray, &RANGE)
        .and_then(|hit| {
            hit.material
                .and_then(|material| material.on_ray_hit(ray, &hit))
                .map(|calc_hit_data| {
                    let attenuation = calc_hit_data.attenuation;
                    let rebounce = calc_hit_data.rebounce;

                    // If we've exceeded the ray bounce limit, no more light is gathered.
                    if max_bounces <= 1 || attenuation == Vec3A::ZERO {
                        return Vec3A::ZERO;
                    }
                    attenuation * ray_color(&rebounce, max_bounces - 1, world)
                })
                .or(Some(Vec3A::ZERO))
        })
        .unwrap_or_else(|| skybox_color(ray.direction.normalize_or_zero()))
}

fn main() -> std::io::Result<()> {
    const RENDERER: Renderer = Renderer::PixelWindow {
        partial_render: true,
    };
    let camera = create_camera(WIDTH, ASPECT_RATIO);
    let world = prepare_world(&camera);
    let (main_tx, main_rx) = sync_channel::<MainCommands>(4);
    let (client_tx, client_rx) = sync_channel::<ClientCommands>(256);
    let camera_width = camera.width;
    let camera_height = camera.height;

    rayon::spawn(move || loop {
        let bvh = bounding_box::BVHNode::<'_, 8>::new(&world, camera.open_time, camera.close_time);
        println!("bvh ready");
        let client_channel = main_rx.recv();
        if client_channel.is_err() {
            println!("main could not receive from client_channel, likely it has been closed");
            return;
        }
        println!("main got command");
        match client_channel.unwrap() {
            MainCommands::Recalc => {
                let pixel_colors = create_rays::<SAMPLES_PER_PIXEL>(&camera)
                    .map(|(pixel, rays_for_pixels)| {
                        let mut color_vec = Vec3A::ZERO;
                        for ray in rays_for_pixels {
                            color_vec += ray_color(&ray, MAX_RAY_BOUNCES, &bvh)
                        }
                        let final_color: u32 = u32::from_be_bytes(
                            color_vec.extend(1.0).as_ref().map(|channel_color| {
                                ((channel_color * RAY_SAMPLE_SCALE_FACTOR)
                                    .sqrt()
                                    .clamp(0.0, 1.0)
                                    * 255.0) as u8
                            }),
                        );

                        return (pixel, final_color);
                    })
                    .progress_count(WIDTH as u64 * HEIGHT as u64);
                if *RENDERER.has_partial_rendering() {
                    pixel_colors.for_each(|data| {
                        client_tx
                            .send(ClientCommands::RedrawPixel(data))
                            .expect("main could not send redraw command");
                    });
                } else {
                    let mut vec: Vec<u32> = Vec::with_capacity(PIXEL_COUNT);
                    pixel_colors
                        .map(|(_, color)| color)
                        .collect_into_vec(&mut vec);
                    client_tx
                        .send(ClientCommands::Redraw(vec))
                        .expect("main could not send redraw command");
                }
            }
        }
    });
    match RENDERER {
        Renderer::Ppm => {
            let renderer = ppm_renderer::PpmImageRenderer::new(camera_width, camera_height);
            renderer.setup_commands(main_tx, client_rx)
        }
        Renderer::PixelWindow { .. } => {
            let renderer = pixel_renderer::PixelRenderer::new(camera_width, camera_height);
            renderer.setup_commands(main_tx, client_rx);
        }
    }
    Ok(())
}
