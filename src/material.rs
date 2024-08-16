use crate::{
    ray::{random_unit_vector, HitRecord, Ray},
    rng::get_rng,
    texture::Texture,
};
use glam::Vec3A;
use rand::Rng;

#[derive(Debug, Clone)]
pub enum Material {
    Lambert(crate::texture::TextureEnum),
    Metal(Vec3A, f32),
    Dielectric(f32),
}

pub struct MaterialCalc {
    pub attenuation: Vec3A,
    pub rebounce: Ray,
}

fn is_scatter_near_zero(direction: &Vec3A) -> bool {
    const LIMIT: f32 = 1e-8;
    const LIMIT_DEVC3: Vec3A = Vec3A::splat(LIMIT);

    direction.cmplt(LIMIT_DEVC3).all()
}

fn reflect(direction: Vec3A, normal: Vec3A) -> Vec3A {
    direction - 2.0 * direction.dot(normal) * normal
}

fn refract(uv: Vec3A, normal: Vec3A, etai_over_etat: f32) -> Vec3A {
    let cos_theta = 1.0_f32.min(-uv.dot(normal));
    let ray_out_perp = etai_over_etat * (uv + cos_theta * normal);
    let ray_out_parallel = -(1.0_f32 - ray_out_perp.length_squared()).abs().sqrt() * normal;

    ray_out_perp + ray_out_parallel
}

fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
    // Use Schlick's approximation for reflectance.
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl Material {
    pub fn on_ray_hit(&self, ray: &Ray, hit: &HitRecord) -> Option<MaterialCalc> {
        match self {
            Material::Lambert(texture) => {
                let scatter_direction = hit.normal + random_unit_vector();

                let scattered = Ray {
                    origin: hit.point,
                    direction: if is_scatter_near_zero(&scatter_direction) {
                        hit.normal
                    } else {
                        scatter_direction
                    },
                    time: ray.time,
                };

                Some(MaterialCalc {
                    attenuation: match texture {
                        crate::texture::TextureEnum::Plain(texture) => {
                            texture.get_uv_color(0.0, 0.0, &hit.point)
                        }
                        crate::texture::TextureEnum::CheckerTexture(texture) => {
                            texture.get_uv_color(0.0, 0.0, &hit.point)
                        }
                        crate::texture::TextureEnum::PerlinNoise(texture) => {
                            texture.get_uv_color(0.0, 0.0, &hit.point)
                        }
                    },
                    rebounce: scattered,
                })
            }
            Material::Metal(albedo, fuzziness) => {
                let reflected = reflect(ray.direction.normalize(), hit.normal);

                let scatter_direction: Vec3A =
                    random_unit_vector().mul_add(Vec3A::splat(*fuzziness), reflected);

                if scatter_direction.dot(hit.normal) >= -0.000005 {
                    let scattered = Ray {
                        origin: hit.point,
                        direction: scatter_direction,
                        time: ray.time,
                    };

                    return Some(MaterialCalc {
                        attenuation: *albedo,
                        rebounce: scattered,
                    });
                }

                None
            }
            Material::Dielectric(index_of_refraction) => {
                let attenuation = Vec3A::ONE;
                let refraction_ratio: f32 = if hit.front_face {
                    1.0 / index_of_refraction
                } else {
                    *index_of_refraction
                };

                let unit_direction = ray.direction.normalize();
                let cos_theta = 1.0_f32.min(-unit_direction.dot(hit.normal));
                let sin_theta = (1.0_f32 - cos_theta * cos_theta).sqrt();

                let cannot_refract = refraction_ratio * sin_theta > 1.0;

                let rebounce_direction = if cannot_refract
                    || reflectance(cos_theta, refraction_ratio) > get_rng().gen()
                {
                    reflect(unit_direction, hit.normal)
                } else {
                    refract(unit_direction, hit.normal, refraction_ratio)
                };

                let scattered = Ray {
                    origin: hit.point,
                    direction: rebounce_direction,
                    time: ray.time,
                };
                Some(MaterialCalc {
                    attenuation,
                    rebounce: scattered,
                })
            }
        }
    }
}
