#![feature(iter_array_chunks)]

use glam::DVec3;
use indicatif::ProgressIterator;
use rand::{rngs::ThreadRng, Rng};
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
    const WIDTH: u16 = 1200_u16;

    let lookfrom = DVec3::new(13.0, 2.0, 3.0);
    let lookat = DVec3::new(0.0, 0.0, 0.0);
    let vup = DVec3::new(0.0, 1.0, 0.0);
    let fov = 20.0;
    Camera::new(lookfrom, lookat, vup, WIDTH, fov, 16.0 / 9.0, 0.6, 10.0)
}
const SAMPLES_PER_PIXEL: usize = 500;
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
const MATERIAL_RIGHT: Material = Material::Metal(DVec3::new(0.8, 0.6, 0.2), 0.0);
fn random_color(range: std::ops::Range<f64>) -> DVec3 {
    let mut random = ThreadRng::default();
    DVec3::new(
        random.gen_range(range.clone()),
        random.gen_range(range.clone()),
        random.gen_range(range),
    )
}
fn create_world() -> Vec<Box<dyn RayHittable>> {
    let mut world: Vec<Box<dyn RayHittable>> = Vec::new();
    world.push(Box::new(Sphere {
        center: DVec3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
        material: MATERIAL_GROUND,
    }));

    let ball_centers_iter = (-11..11)
        .into_iter()
        .flat_map(|a| (-11..11).into_iter().map(move |b| (a, b)))
        .map(|(a, b)| {
            DVec3::new(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            )
        });
    let spheres_iter = ball_centers_iter
        .filter(|center| (*center - DVec3::new(4.0, 0.2, 0.0)).length() > 0.9)
        .map(|center| {
            let choose_mat = rand::random::<f32>();
            if choose_mat < 0.8 {
                //diffuse
                let albedo = random_color(0.0..1.0) * random_color(0.0..1.0);

                Sphere {
                    center,
                    radius: 0.2,
                    material: Material::Lambert(albedo),
                }
            } else if choose_mat < 0.95 {
                //metal
                Sphere {
                    center,
                    radius: 0.2,
                    material: Material::Metal(
                        random_color(0.5..1.0),
                        rand::thread_rng().gen_range(0.0..0.5),
                    ),
                }
            } else {
                //glass
                Sphere {
                    center,
                    radius: 0.2,
                    material: Material::Dielectric(1.5),
                }
            }
        });
    spheres_iter.for_each(|sphere| {
        world.push(Box::new(sphere));
    });

    world.push(Box::new(Sphere {
        center: DVec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Dielectric(1.5),
    }));
    world.push(Box::new(Sphere {
        center: DVec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Lambert(DVec3::new(0.4, 0.2, 0.1)),
    }));

    world.push(Box::new(Sphere {
        center: DVec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Metal(DVec3::new(0.7, 0.6, 0.5), 0.0),
    }));

    world
}
fn main() -> std::io::Result<()> {
    let camera = create_camera();
    let world: Vec<Box<dyn RayHittable>> = create_world();

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
