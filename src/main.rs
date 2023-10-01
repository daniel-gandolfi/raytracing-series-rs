#![feature(iter_array_chunks)]

use glam::DVec3;
use indicatif::ProgressIterator;
use std::io::Write;
mod camera;
mod material;
mod ppm_renderer;
mod ray;
mod shapes;
use crate::camera::Camera;
use crate::material::Material;
use ppm_renderer::RayTracingRenderer;
use ray::{create_rays, ray_color, RayHittable};
use shapes::Sphere;

fn create_camera() -> Camera {
    const WIDTH: u16 = 600_u16;
    Camera::new(DVec3::new(0.0, 0.0, 0.0), WIDTH, 90, 1.0, 16.0 / 9.0)
}
const SAMPLES_PER_PIXEL: usize = 80;
const MAX_RAY_BOUNCES: u8 = 50;

const MATERIAL_GROUND: Material = Material::Lambert(DVec3 {
    x: 0.8,
    y: 0.8,
    z: 0.0,
});
const MATERIAL_CENTER: Material = Material::Lambert(DVec3 {
    x: 0.1,
    y: 0.2,
    z: 0.5,
});
const MATERIAL_LEFT: Material = Material::Dielectric(1.5);
const MATERIAL_RIGHT: Material = Material::Metal(
    DVec3 {
        x: 0.8,
        y: 0.6,
        z: 0.2,
    },
    0.01,
);

fn main() -> std::io::Result<()> {
    let camera = create_camera();
    let world: Vec<Box<dyn RayHittable>> = vec![
        Box::new(Sphere {
            center: DVec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            radius: 0.5,
            material: MATERIAL_CENTER,
        }),
        Box::new(Sphere {
            center: DVec3 {
                x: 0.0,
                y: 1.0,
                z: -1.0,
            },
            radius: 0.5,
            material: MATERIAL_CENTER,
        }),
        Box::new(Sphere {
            center: DVec3 {
                x: -1.0,
                y: 0.0,
                z: -1.0,
            },
            radius: -0.4,
            material: MATERIAL_LEFT,
        }),
        Box::new(Sphere {
            center: DVec3 {
                x: 1.0,
                y: 0.0,
                z: -1.0,
            },
            radius: 0.5,
            material: MATERIAL_RIGHT,
        }),
        Box::new(Sphere {
            center: DVec3 {
                x: 0.0,
                y: -100.5,
                z: -1.0,
            },
            radius: 100.0,
            material: MATERIAL_GROUND,
        }),
    ];

    let renderer = ppm_renderer::PpmImageRenderer::new("render.ppm")?;

    std::io::stdout().write(b"creating points\n");
    let ray_sample_scale_factor = DVec3::splat(1.0 / SAMPLES_PER_PIXEL as f64);
    let color_clamp_upper = DVec3::splat(0.99999999999);
    renderer.render(
        camera.width,
        camera.height,
        create_rays(&camera, SAMPLES_PER_PIXEL)
            .array_chunks::<SAMPLES_PER_PIXEL>()
            .map(|ray_window| {
                let color = (ray_window
                    .iter()
                    .map(|ray| ray_color(&ray, MAX_RAY_BOUNCES, &world))
                    .sum::<DVec3>()
                    * ray_sample_scale_factor)
                    .clamp(DVec3::ZERO, color_clamp_upper);
                color
            })
            .progress_count(camera.width as u64 * camera.height as u64),
    )?;

    Ok(())
}
