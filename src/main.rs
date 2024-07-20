#![feature(thread_local)]
#![feature(iter_array_chunks)]
#![feature(iter_collect_into)]
#![feature(test)]
#![feature(const_fn_floating_point_arithmetic)]

extern crate test;

use std::sync::mpsc::sync_channel;
use std::thread;

use bounding_box::BVHNode;
use glam::DVec3;
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
mod world_creation;
use crate::camera::Camera;
use crate::world_creation::heavy_rand_world::create_world;
use ray::{create_rays, ray_color, RayHittableEnum};
use rayon::iter::ParallelIterator;
use rayon::prelude::*;

const WIDTH: u16 = if cfg!(debug_assertions) {
    600_u16
} else {
    1200_u16
};
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const HEIGHT: u16 = (WIDTH as f64 / ASPECT_RATIO as f64) as u16;
const PIXEL_COUNT: usize = WIDTH as usize * HEIGHT as usize;
fn create_camera(w: u16, ar: f64) -> Camera {
    const LOOKFROM: DVec3 = DVec3::new(13.0, 2.0, 3.0);
    const LOOKAT: DVec3 = DVec3::new(0.0, 0.0, 0.0);
    const VUP: DVec3 = DVec3::new(0.0, 1.0, 0.0);
    const FOV: f32 = 20.0;
    Camera::new(LOOKFROM, LOOKAT, VUP, w, FOV, ar, 0.0, 10.0, 0.0, 1.0)
}
const SAMPLES_PER_PIXEL: usize = if cfg!(debug_assertions) { 3 } else { 50 };
const MAX_RAY_BOUNCES: u8 = if cfg!(debug_assertions) { 4 } else { 20 };
const RAY_SAMPLE_SCALE_FACTOR: f64 = 1.0 / SAMPLES_PER_PIXEL as f64;

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
fn main() -> std::io::Result<()> {
    const RENDERER: Renderer = Renderer::PixelWindow {
        partial_render: true,
    };
    let camera = create_camera(WIDTH, ASPECT_RATIO);
    let world = prepare_world(&camera);
    let (main_tx, main_rx) = sync_channel::<MainCommands>(4);
    let (client_tx, client_rx) = sync_channel::<ClientCommands>(32);
    let camera_width = camera.width;
    let camera_height = camera.height;
    let handle = thread::Builder::new()
        .name(String::from("raytracer"))
        .spawn(move || {
            let bvh = BVHNode::new(&world, 4, camera.open_time, camera.close_time);
            println!("bvh ready");
            loop {
                let mess = main_rx.recv().expect("main could not receive from channel");
                println!("main got command");
                match mess {
                    MainCommands::Recalc => {
                        let pixel_colors = create_rays(&camera, SAMPLES_PER_PIXEL)
                            .enumerate()
                            .with_min_len(32)
                            .map(|(pixel, ray_iterator)| {
                                let mut color_vec = DVec3::ZERO;
                                for ray in ray_iterator {
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
                            .progress_count(camera.width as u64 * camera.height as u64);
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
            }
        })
        .expect("no issue when creating ratracing thread");
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
    if !handle.is_finished() {
        handle.join().expect("why should join on main fail?");
    }
    Ok(())
}
