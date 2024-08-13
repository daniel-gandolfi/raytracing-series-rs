use glam::Vec3A;
use itertools::Itertools;

use crate::{
    camera::Camera,
    material::Material,
    ray::RayHittableEnum,
    shapes::Sphere,
    texture::{PlainColorTexture, TextureEnum},
};

const MATERIAL_GROUND: Material = Material::Lambert(TextureEnum::CheckerTexture(
    crate::texture::CheckerTexture::new(
        PlainColorTexture::new(Vec3A::new(0.2, 0.3, 0.1)),
        PlainColorTexture::new(Vec3A::new(0.9, 0.9, 0.9)),
    ),
));

pub fn create_world(_camera: &Camera) -> Vec<RayHittableEnum> {
    const BALL_ITER: u16 = 14;
    const DYN_BALL_COUNT: u16 = BALL_ITER * 2 * BALL_ITER * 2;
    const STATIC_BALL_COUNT: u16 = 4;
    const TOTAL_BALL_COUNT: u16 = STATIC_BALL_COUNT + DYN_BALL_COUNT;

    let ball_centers_iter = (-(BALL_ITER as i32)..(BALL_ITER as i32))
        .cartesian_product(-(BALL_ITER as i32)..(BALL_ITER as i32))
        .map(|(a, b)| Vec3A::new(a as f32, 0.2, b as f32));
    let spheres_iter = vec![Sphere {
        center: Vec3A::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: MATERIAL_GROUND,
        velocity: Vec3A::ZERO,
    }]
    .into_iter()
    .chain(ball_centers_iter.map(|center| {
        let choose_mat = center.length_squared() % 6.0;
        if choose_mat < 3.0 {
            //diffuse
            let albedo = center / (BALL_ITER as f32) * center / (BALL_ITER as f32);

            Sphere {
                center,
                radius: 0.2,
                material: Material::Lambert(TextureEnum::Plain(PlainColorTexture::new(albedo))),
                velocity: Vec3A::ZERO,
            }
        } else if choose_mat < 5.0 {
            //metal
            Sphere {
                center,
                radius: 0.2,
                material: Material::Metal(
                    center / (BALL_ITER as f32),
                    center.length_squared().recip(),
                ),
                velocity: Vec3A::ZERO,
            }
        } else {
            //glass
            Sphere {
                center,
                radius: 0.2,
                material: Material::Dielectric(1.5),
                velocity: Vec3A::ZERO,
            }
        }
    }))
    .chain([
        Sphere {
            center: Vec3A::new(0.0, 1.0, 0.0),
            radius: 1.0,
            material: Material::Dielectric(1.5),
            velocity: Vec3A::ZERO,
        },
        Sphere {
            center: Vec3A::new(-4.0, 1.0, 0.0),
            radius: 1.0,
            material: Material::Lambert(TextureEnum::Plain(PlainColorTexture::new(Vec3A::new(
                0.4, 0.2, 0.1,
            )))),
            velocity: Vec3A::ZERO,
        },
        Sphere {
            center: Vec3A::new(4.0, 1.0, 0.0),
            radius: 1.0,
            material: Material::Metal(Vec3A::new(0.7, 0.6, 0.5), 0.0),
            velocity: Vec3A::ZERO,
        },
    ])
    .map(RayHittableEnum::Sphere);
    let mut world_dyn = Vec::with_capacity(TOTAL_BALL_COUNT as usize);

    spheres_iter.collect_into(&mut world_dyn);

    world_dyn
}

pub fn create_camera(w: u16, ar: f64) -> Camera {
    const LOOKFROM: Vec3A = Vec3A::new(13.0, 2.0, 3.0);
    const LOOKAT: Vec3A = Vec3A::new(0.0, 0.0, 0.0);
    const VUP: Vec3A = Vec3A::new(0.0, 1.0, 0.0);
    const FOV: f32 = 20.0;
    Camera::new(LOOKFROM, LOOKAT, VUP, w, FOV, ar, 0.0, 10.0, 0.0, 1.0)
}
