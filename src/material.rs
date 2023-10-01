use crate::ray::{random_unit_vector, HitRecord, Ray};
use glam::DVec3;

pub enum Material {
    Lambert(DVec3),
    Metal(DVec3, f64),
    Dielectric(f64)
}

pub struct MaterialCalc {
    pub attenuation: DVec3,
    pub rebounce: Ray,
}

fn is_scatter_near_zero(direction: &DVec3) -> bool {
    const LIMIT: f64 = 1e-8;

    direction
        .cmplt(DVec3 {
            x: LIMIT,
            y: LIMIT,
            z: LIMIT,
        })
        .all()
}

fn reflect(direction: DVec3, normal: DVec3) -> DVec3 {
    direction - 2.0 * direction.dot(normal) * normal
}

fn refract(uv: DVec3, normal : DVec3, etai_over_etat: f64) -> DVec3 {
    let cos_theta = 1.0_f64.min(-uv.dot(normal));
    let ray_out_perp = etai_over_etat * (uv + cos_theta * normal);
    let ray_out_parallel = -(1.0_f64 - ray_out_perp.length_squared()).abs().sqrt() * normal;

    ray_out_perp + ray_out_parallel
}

impl Material {
    pub fn on_ray_hit(&self, ray: &Ray, hit: &HitRecord) -> Option<MaterialCalc> {
        match self {
            Material::Lambert(albedo) => {
                let scatter_direction = hit.normal + random_unit_vector();

                let scattered = Ray {
                    origin: hit.point,
                    direction: if is_scatter_near_zero(&scatter_direction) {
                        hit.normal
                    } else {
                        scatter_direction
                    },
                };

                Some(MaterialCalc {
                    attenuation: *albedo,
                    rebounce: scattered,
                })
            }
            Material::Metal(albedo, fuzziness) => {
                let reflected = reflect(ray.direction.normalize(), hit.normal);

                let scatter_direction = reflected + *fuzziness * random_unit_vector();

                if scatter_direction.dot(hit.normal) >= -0.000005 {
                    let scattered = Ray {
                        origin: hit.point,
                        direction: scatter_direction,
                    };

                    return Some(MaterialCalc {
                        attenuation: *albedo,
                        rebounce: scattered,
                    });
                }

                None
            }
            Material::Dielectric(index_of_refraction) => {
                let attenuation = DVec3::ONE;
                let refraction_ratio =  if hit.front_face { 
                    1.0 / index_of_refraction 
                } else { 
                    *index_of_refraction 
                };

                let unit_direction = ray.direction.normalize();
                let cos_theta = 1.0_f64.min(-unit_direction.dot(hit.normal));
                let sin_theta = (1.0_f64 - cos_theta*cos_theta).sqrt();

                let cannot_refract = refraction_ratio * sin_theta > 1.0;

                let rebounce_direction = if cannot_refract { reflect(unit_direction, hit.normal) } else { refract(unit_direction, hit.normal, refraction_ratio) };

                let scattered = Ray {
                    origin: hit.point,
                    direction: rebounce_direction
                };
                Some(MaterialCalc { attenuation, rebounce: scattered })
            }
        }
    }
}
