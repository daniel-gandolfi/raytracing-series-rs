use crate::ray::{random_unit_vector, HitRecord, Ray};
use glam::DVec3;

pub enum Material {
    Lambert(DVec3),
    Metal(DVec3, f64),
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

fn metal_reflect(direction: DVec3, normal: DVec3) -> DVec3 {
    direction - 2.0 * direction.dot(normal) * normal
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
                let reflected = metal_reflect(ray.direction.normalize(), hit.normal);

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
        }
    }
}
