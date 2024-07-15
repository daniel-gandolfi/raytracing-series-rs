use glam::DVec3;
use itertools::Itertools;
use rand::{Rng, SeedableRng};

use crate::{
    camera::Camera, material::Material, ray::RayHittableEnum, rng::get_rng, shapes::Sphere,
};

const MATERIAL_GROUND: Material = Material::Lambert(DVec3 {
    x: 0.5,
    y: 0.5,
    z: 0.5,
});

fn random_color(range: std::ops::Range<f64>) -> DVec3 {
    let mut random = get_rng();
    DVec3::new(
        random.gen_range(range.clone()),
        random.gen_range(range.clone()),
        random.gen_range(range),
    )
}

pub fn create_world(camera: &Camera) -> Vec<RayHittableEnum> {
    const BALL_ITER: u16 = 11;
    const DYN_BALL_COUNT: u16 = BALL_ITER * 2 * BALL_ITER * 2;
    const STATIC_BALL_COUNT: u16 = 4;
    const TOTAL_BALL_COUNT: u16 = STATIC_BALL_COUNT + DYN_BALL_COUNT;

    let rng = get_rng();
    let ball_centers_iter = (-(BALL_ITER as i32)..(BALL_ITER as i32))
        .cartesian_product(-(BALL_ITER as i32)..(BALL_ITER as i32))
        .map(|(a, b)| {
            DVec3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            )
        });
    let mut rng = rand::rngs::SmallRng::from_entropy();
    let main_balls = ball_centers_iter
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
                    velocity: DVec3 {
                        x: rng.gen::<f64>() * rng.gen::<f64>(),
                        y: rng.gen::<f64>() * rng.gen::<f64>(),
                        z: rng.gen::<f64>() * rng.gen::<f64>(),
                    },
                }
            } else if choose_mat < 0.95 {
                //metal
                Sphere {
                    center,
                    radius: 0.2,
                    material: Material::Metal(random_color(0.5..1.0), rng.gen_range(0.0..0.5)),
                    velocity: DVec3 {
                        x: (1.0 + rng.gen::<f64>()) / 2.0,
                        y: (1.0 + rng.gen::<f64>()) / 2.0,
                        z: (1.0 + rng.gen::<f64>()) / 2.0,
                    },
                }
            } else {
                //glass
                Sphere {
                    center,
                    radius: 0.2,
                    material: Material::Dielectric(1.5),
                    velocity: DVec3::ZERO,
                }
            }
        });
    let spheres_iter = vec![Sphere {
        center: DVec3 {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        radius: 1000.0,
        material: MATERIAL_GROUND,
        velocity: DVec3::ZERO,
    }]
    .into_iter()
    .chain(main_balls)
    .chain(vec![
        Sphere {
            center: DVec3::new(0.0, 1.0, 0.0),
            radius: 1.0,
            material: Material::Dielectric(1.5),
            velocity: DVec3::ZERO,
        },
        Sphere {
            center: DVec3::new(-4.0, 1.0, 0.0),
            radius: 1.0,
            material: Material::Lambert(DVec3::new(0.4, 0.2, 0.1)),
            velocity: DVec3::ZERO,
        },
        Sphere {
            center: DVec3::new(4.0, 1.0, 0.0),
            radius: 1.0,
            material: Material::Metal(DVec3::new(0.7, 0.6, 0.5), 0.0),
            velocity: DVec3::ZERO,
        },
    ])
    .map(RayHittableEnum::Sphere);

    let mut world_dyn: Vec<RayHittableEnum> = Vec::with_capacity(TOTAL_BALL_COUNT as usize);

    spheres_iter.collect_into(&mut world_dyn);

    world_dyn
}
