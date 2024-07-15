use glam::DVec3;

use crate::{camera::Camera, material::Material, ray::RayHittableEnum, shapes::Sphere};

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
const MATERIAL_BUBBLE: Material = Material::Dielectric(1.0 / 1.5);
const MATERIAL_RIGHT: Material = Material::Metal(DVec3::new(0.8, 0.6, 0.2), 1.0);

pub fn create_world(camera: &Camera) -> Vec<RayHittableEnum> {
    vec![
        RayHittableEnum::Sphere(Sphere {
            center: DVec3::new(0.0, -100.5, -1.0),
            radius: 100.0,
            material: MATERIAL_GROUND,
            velocity: DVec3::ZERO,
        }),
        RayHittableEnum::Sphere(Sphere {
            center: DVec3::new(1.0, 0.0, -1.0),
            radius: 0.5,
            material: MATERIAL_RIGHT,
            velocity: DVec3::ZERO,
        }),
        RayHittableEnum::Sphere(Sphere {
            center: DVec3::new(0.0, 0.0, -1.2),
            radius: 0.5,
            material: MATERIAL_CENTER,
            velocity: DVec3::ZERO,
        }),
        RayHittableEnum::Sphere(Sphere {
            center: DVec3::new(-1.0, 0.0, -1.0),
            radius: 0.5,
            material: MATERIAL_LEFT,
            velocity: DVec3::ZERO,
        }),
        RayHittableEnum::Sphere(Sphere {
            center: DVec3::new(-1.0, 0.0, -1.0),
            radius: 0.4,
            material: MATERIAL_BUBBLE,
            velocity: DVec3::ZERO,
        }),
    ]
}
