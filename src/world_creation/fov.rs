use glam::Vec3A;

use crate::{camera::Camera, material::Material, ray::RayHittableEnum, shapes::Sphere};

const MATERIAL_GROUND: Material = Material::Lambert(Vec3A::new(0.8, 0.8, 0.0));
const MATERIAL_CENTER: Material = Material::Lambert(Vec3A::new(0.1, 0.2, 0.5));
const MATERIAL_LEFT: Material = Material::Dielectric(1.5);
const MATERIAL_BUBBLE: Material = Material::Dielectric(1.0 / 1.5);
const MATERIAL_RIGHT: Material = Material::Metal(Vec3A::new(0.8, 0.6, 0.2), 1.0);

pub fn create_world(_camera: &Camera) -> Vec<RayHittableEnum> {
    vec![
        RayHittableEnum::Sphere(Sphere {
            center: Vec3A::new(0.0, -100.5, -1.0),
            radius: 100.0,
            material: MATERIAL_GROUND,
            velocity: Vec3A::ZERO,
        }),
        RayHittableEnum::Sphere(Sphere {
            center: Vec3A::new(1.0, 0.0, -1.0),
            radius: 0.5,
            material: MATERIAL_RIGHT,
            velocity: Vec3A::ZERO,
        }),
        RayHittableEnum::Sphere(Sphere {
            center: Vec3A::new(0.0, 0.0, -1.2),
            radius: 0.5,
            material: MATERIAL_CENTER,
            velocity: Vec3A::ZERO,
        }),
        RayHittableEnum::Sphere(Sphere {
            center: Vec3A::new(-1.0, 0.0, -1.0),
            radius: 0.5,
            material: MATERIAL_LEFT,
            velocity: Vec3A::ZERO,
        }),
        RayHittableEnum::Sphere(Sphere {
            center: Vec3A::new(-1.0, 0.0, -1.0),
            radius: 0.4,
            material: MATERIAL_BUBBLE,
            velocity: Vec3A::ZERO,
        }),
    ]
}
