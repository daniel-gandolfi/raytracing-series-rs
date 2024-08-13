use glam::Vec3A;
use itertools::Itertools;
use rand::Rng;

use crate::{
    camera::Camera,
    material::Material,
    ray::RayHittableEnum,
    rng::get_rng,
    shapes::Sphere,
    texture::{PlainColorTexture, TextureEnum},
};

const MATERIAL_GROUND: Material = Material::Lambert(TextureEnum::CheckerTexture(
    crate::texture::CheckerTexture::new(
        PlainColorTexture::new(Vec3A::new(0.2, 0.3, 0.1)),
        PlainColorTexture::new(Vec3A::new(0.9, 0.9, 0.9)),
    ),
));

fn random_color(range: std::ops::Range<f32>) -> Vec3A {
    let random = get_rng();
    Vec3A::new(
        random.gen_range(range.clone()),
        random.gen_range(range.clone()),
        random.gen_range(range),
    )
}

pub fn create_world(_camera: &Camera) -> Vec<RayHittableEnum> {
    const BALL_ITER: u16 = 8;
    const DYN_BALL_COUNT: u16 = BALL_ITER * 2 * BALL_ITER * 2;
    const STATIC_BALL_COUNT: u16 = 4;
    const TOTAL_BALL_COUNT: u16 = STATIC_BALL_COUNT + DYN_BALL_COUNT;

    let rng = get_rng();
    let ball_centers_iter = (-(BALL_ITER as i32)..(BALL_ITER as i32))
        .cartesian_product(-(BALL_ITER as i32)..(BALL_ITER as i32))
        .map(|(a, b)| {
            Vec3A::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            )
        });
    let rng = get_rng();
    let main_balls = ball_centers_iter
        .filter(|center| (*center - Vec3A::new(4.0, 0.2, 0.0)).length() > 0.9)
        .map(|center| {
            let choose_mat = rng.gen::<f32>();
            if choose_mat < 0.8 {
                //diffuse
                let albedo = random_color(0.0..1.0) * random_color(0.0..1.0);

                Sphere {
                    center,
                    radius: 0.2,
                    material: Material::Lambert(TextureEnum::Plain(PlainColorTexture::new(albedo))),
                    velocity: Vec3A::new(
                        rng.gen::<f32>() * rng.gen::<f32>(),
                        rng.gen::<f32>() * rng.gen::<f32>(),
                        rng.gen::<f32>() * rng.gen::<f32>(),
                    ),
                }
            } else if choose_mat < 0.95 {
                //metal
                Sphere {
                    center,
                    radius: 0.2,
                    material: Material::Metal(random_color(0.5..1.0), rng.gen_range(0.0..0.5)),
                    velocity: Vec3A::new(
                        (1.0 + rng.gen::<f32>()) / 2.0,
                        (1.0 + rng.gen::<f32>()) / 2.0,
                        (1.0 + rng.gen::<f32>()) / 2.0,
                    ),
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
        });
    let spheres_iter = vec![Sphere {
        center: Vec3A::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: MATERIAL_GROUND,
        velocity: Vec3A::ZERO,
    }]
    .into_iter()
    .chain(main_balls)
    .chain(vec![
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

    let mut world_dyn: Vec<RayHittableEnum> = Vec::with_capacity(TOTAL_BALL_COUNT as usize);

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
