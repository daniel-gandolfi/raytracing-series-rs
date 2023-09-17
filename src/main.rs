use core::option::Iter;
use glam::DVec3;
use indicatif::ProgressIterator;
use std::io::{BufWriter, Write};
mod camera;
mod ppm_renderer;
mod ray;
mod shapes;
use crate::camera::Camera;
use crate::ray::create_rays;
use ppm_renderer::PpmImageRenderer;
use ppm_renderer::RayTracingRenderer;
use ray::{ray_color, Ray, RayHittable};
use shapes::Sphere;

fn create_camera() -> Camera {
    const WIDTH: u16 = 1920 as u16;
    Camera::new(DVec3::new(0.0, 0.0, 0.0), WIDTH, 90, 1.0, 16.0 / 9.0)
}

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let camera = create_camera();
    let world: Vec<Box<dyn RayHittable>> = vec![
        Box::new(Sphere {
            center: DVec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            radius: 0.5,
        }),
        Box::new(Sphere {
            center: DVec3 {
                x: 0.0,
                y: -100.5,
                z: -1.0,
            },
            radius: 100.0,
        }),
    ];

    let renderer = ppm_renderer::PpmImageRenderer::new("render.ppm")?;

    std::io::stdout().write(b"creating points\n");
    renderer.render(
        camera.width,
        camera.height,
        create_rays(&camera)
            .map(|ray| ray_color(ray, &world))
            .progress_count(camera.width as u64 * camera.height as u64),
    )?;

    Ok(())
}
