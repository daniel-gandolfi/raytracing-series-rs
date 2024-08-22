use glam::Vec3A;
use rand::Rng;
const POINT_COUNT: usize = 256;

#[derive(Debug, Clone)]
pub struct PerlinNoise {
    randvec: [Vec3A; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

fn permute(p: &mut [usize; POINT_COUNT]) {
    let mut rng = rand::thread_rng();
    for i in (1..POINT_COUNT).rev() {
        let target = rng.gen_range(0..=i);
        p.swap(i, target);
    }
}

const DIMENSIONS: usize = 2;
fn trilinear_interpolate(kernel: &[[[Vec3A; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let mut acc = 0.0;
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    for (i, j_dim) in kernel.iter().enumerate().take(2) {
        for (j, k_dim) in j_dim.iter().enumerate().take(2) {
            for (k, element) in k_dim.iter().enumerate().take(2) {
                let weight_v = Vec3A::new(u - i as f32, v - j as f32, w - k as f32);

                let u_lerp = i as f32 * uu + (1 - i) as f32 * (1.0 - uu);
                let v_lerp = j as f32 * vv + (1 - j) as f32 * (1.0 - vv);
                let w_lerp = k as f32 * ww + (1 - k) as f32 * (1.0 - ww);
                acc += u_lerp * v_lerp * w_lerp * weight_v.dot(*element);
            }
        }
    }
    return acc;
}

impl PerlinNoise {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut randfloat = [Vec3A::Z; POINT_COUNT];

        let mut perm_x = [0; POINT_COUNT];
        let mut perm_y = [0; POINT_COUNT];
        let mut perm_z = [0; POINT_COUNT];
        for i in 0..POINT_COUNT {
            randfloat[i] = Vec3A::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            )
            .normalize();
            perm_x[i] = i;
            perm_y[i] = i;
            perm_z[i] = i;
        }

        permute(&mut perm_x);
        permute(&mut perm_y);
        permute(&mut perm_z);

        PerlinNoise {
            randvec: randfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn get_noise(&self, p: &Vec3A) -> f32 {
        // for some reason the original (p.x - p.x.floor()) does not map the tiles to the correct decimal values resulting in repeating tiles
        // This seems to be caused by negative numbers so i just use positive ones
        let u = p.x.fract().abs();
        let v = p.y.fract().abs();
        let w = p.z.fract().abs();

        let i = p.x.trunc().abs() as usize;
        let j = p.y.trunc().abs() as usize;
        let k = p.z.trunc().abs() as usize;

        let mut kernel = [[[Vec3A::ZERO; 2]; 2]; 2];

        for (di, j_dim) in kernel.iter_mut().enumerate().take(2) {
            for (dj, k_dim) in j_dim.iter_mut().enumerate().take(2) {
                for (dk, element) in k_dim.iter_mut().enumerate().take(2) {
                    *element = self.randvec[self.perm_x[(i + di) & 255]
                        ^ self.perm_y[(j + dj) & 255]
                        ^ self.perm_z[(k + dk) & 255]];
                }
            }
        }
        return trilinear_interpolate(&kernel, u, v, w);
    }

    pub fn turbulence(&self, p: &Vec3A, depth: Option<i32>) -> f32 {
        let depth = depth.unwrap_or(7);
        let mut acc = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;
        for _i in 0..depth {
            acc += self.get_noise(&temp_p) * weight;
            weight *= 0.5;
            temp_p *= 2.0;
        }
        return acc.abs();
    }
}
