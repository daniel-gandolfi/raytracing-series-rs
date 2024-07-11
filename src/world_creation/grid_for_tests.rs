use glam::DVec3;
use itertools::Itertools;

use crate::{
    camera::Camera,
    material::Material,
    ray::RayHittableEnum,
    shapes::Sphere,
};

const MATERIAL_GROUND: Material = Material::Lambert(DVec3 {
    x: 0.5,
    y: 0.5,
    z: 0.5,
});

pub fn create_world(camera: &Camera) -> Vec<RayHittableEnum> {
    const BALL_ITER: u16 = 14;
    const DYN_BALL_COUNT: u16 = BALL_ITER * 2 * BALL_ITER * 2;
    const STATIC_BALL_COUNT: u16 = 4;
    const TOTAL_BALL_COUNT: u16 = STATIC_BALL_COUNT + DYN_BALL_COUNT;

    let ball_centers_iter = (-(BALL_ITER as i32)..(BALL_ITER as i32))
        .cartesian_product(-(BALL_ITER as i32)..(BALL_ITER as i32))
        .map(|(a, b)| DVec3::new(a as f64, 0.2, b as f64));
    let spheres_iter = ball_centers_iter
        .map(|center| {
            let choose_mat = center.length_squared() % 6.0;
            if choose_mat < 3.0 {
                //diffuse
                let albedo = center / (BALL_ITER as f64) * center / (BALL_ITER as f64);

                Sphere {
                    center,
                    radius: 0.2,
                    material: Material::Lambert(albedo),
                    velocity: DVec3::ZERO
                }
            } else if choose_mat < 5.0 {
                //metal
                Sphere {
                    center,
                    radius: 0.2,
                    material: Material::Metal(
                        center / (BALL_ITER as f64),
                        center.length_squared().recip(),
                    ),
                    velocity: DVec3::ZERO
                }
            } else {
                //glass
                Sphere {
                    center,
                    radius: 0.2,
                    material: Material::Dielectric(1.5),
                    velocity: DVec3::ZERO
                }
            }
        })
        .chain(vec![
            Sphere {
                center: DVec3 {
                    x: 0.0,
                    y: -1000.0,
                    z: 0.0,
                },
                radius: 1000.0,
                material: MATERIAL_GROUND,
                velocity: DVec3::ZERO
            },
            Sphere {
                center: DVec3::new(0.0, 1.0, 0.0),
                radius: 1.0,
                material: Material::Dielectric(1.5),
            velocity: DVec3::ZERO
            },
            Sphere {
                center: DVec3::new(-4.0, 1.0, 0.0),
                radius: 1.0,
                material: Material::Lambert(DVec3::new(0.4, 0.2, 0.1)),
            velocity: DVec3::ZERO
            },
            Sphere {
                center: DVec3::new(4.0, 1.0, 0.0),
                radius: 1.0,
                material: Material::Metal(DVec3::new(0.7, 0.6, 0.5), 0.0),
            velocity: DVec3::ZERO
            },
        ])
        .map(RayHittableEnum::Sphere)
        .sorted_by(|a, b| match (a, b) {
            (RayHittableEnum::Sphere(a), RayHittableEnum::Sphere(b)) => b
                .center
                .distance_squared(camera.position)
                .total_cmp(&a.center.distance_squared(camera.position)),
        });
    let mut world_dyn = Vec::with_capacity(TOTAL_BALL_COUNT as usize);

    spheres_iter.collect_into(&mut world_dyn);

    world_dyn
}
