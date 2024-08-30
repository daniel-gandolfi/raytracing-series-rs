use glam::Vec3A;

use crate::{camera::Camera, material::Material, ray::RayHittableEnum, shapes::Sphere};

pub fn create_world(_camera: &Camera) -> Vec<RayHittableEnum> {
    return vec![RayHittableEnum::Sphere(Sphere {
        center: Vec3A::new(0.0, 0.0, -1.0),
        radius: 2.0,
        material: Material::Lambert(crate::texture::TextureEnum::Image(Box::new(
            image::ImageReader::open(
                "/home/daniel/tmp/raytracing-one-weekend-rs/src/assets/earthmap.jpg",
            )
            .expect("failed to open earth image")
            .decode()
            .expect("failed to decode earth image, wrong file format or extension?")
            .to_rgba8(),
        ))),
        velocity: Vec3A::ZERO,
    })];
}

pub fn create_camera(w: u16, ar: f64) -> Camera {
    const LOOKFROM: Vec3A = Vec3A::new(13.0, 2.0, 3.0);
    const LOOKAT: Vec3A = Vec3A::new(0.0, 0.0, 0.0);
    const VUP: Vec3A = Vec3A::new(0.0, 1.0, 0.0);
    const FOV: f32 = 20.0;
    Camera::new(LOOKFROM, LOOKAT, VUP, w, FOV, ar, 0.0, 10.0, 0.0, 1.0)
}
