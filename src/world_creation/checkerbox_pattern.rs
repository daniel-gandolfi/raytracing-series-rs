use glam::Vec3A;

use crate::{
    camera::Camera,
    material::Material,
    ray::RayHittableEnum,
    shapes::Sphere,
    texture::{CheckerTexture, PlainColorTexture, TextureEnum},
};

pub fn create_world(_camera: &Camera) -> Vec<RayHittableEnum> {
    let checker_texture = CheckerTexture::new(
        PlainColorTexture::new(Vec3A::new(0.2, 0.3, 0.1)),
        PlainColorTexture::new(Vec3A::new(0.9, 0.9, 0.9)),
    );
    let material = Material::Lambert(TextureEnum::CheckerTexture(checker_texture));
    return vec![
        RayHittableEnum::Sphere(Sphere::new(
            material.clone(),
            Vec3A::new(0.0, -10.0, 0.0),
            Vec3A::ZERO,
            10.0,
        )),
        RayHittableEnum::Sphere(Sphere::new(
            material,
            Vec3A::new(0.0, 10.0, 0.0),
            Vec3A::ZERO,
            10.0,
        )),
    ];
}

pub fn create_camera(w: u16, ar: f64) -> Camera {
    const LOOKFROM: Vec3A = Vec3A::new(13.0, 2.0, 3.0);
    const LOOKAT: Vec3A = Vec3A::new(0.0, 0.0, 0.0);
    const VUP: Vec3A = Vec3A::new(0.0, 1.0, 0.0);
    const FOV: f32 = 20.0;
    Camera::new(LOOKFROM, LOOKAT, VUP, w, FOV, ar, 0.0, 10.0, 0.0, 1.0)
}
