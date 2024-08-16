use glam::{FloatExt, Vec3A};
use rand::Rng;
const POINT_COUNT: usize = 256;

#[derive(Debug, Clone)]
pub struct PerlinNoise {
    randfloat: [f32; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

fn permute(p: &mut [usize; POINT_COUNT], n: usize) {
    let mut rng = rand::thread_rng();
    for i in (1..n).rev() {
        let target = rng.gen_range(0..=i);
        p.swap(i, target);
    }
}
fn generate_perm(p: &mut [usize; POINT_COUNT]) {
    for i in 0..POINT_COUNT {
        p[i] = i;
    }

    permute(p, POINT_COUNT);
}
fn trilinear_interpolate(kernel: &[[[f32; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let mut acc = 0.0;
    
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 { 

                let u_lerp = (i as f32*u + (1-i) as f32 *(1.0-u));
                let v_lerp = (j as f32*v + (1-j) as f32*(1.0-v));
                let w_lerp = (k as f32*w + (1-k) as f32*(1.0-w));
                acc += 
                u_lerp * 
                v_lerp * 
                w_lerp * 
                kernel[i][j][k];
            }
        }
    }
    return acc;
}

impl PerlinNoise {

    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut randfloat = [0.0; POINT_COUNT];
        for i in 0..POINT_COUNT {
            randfloat[i] = rng.gen();
        }

        let mut perm_x = [0; POINT_COUNT];
        let mut perm_y = [0; POINT_COUNT];
        let mut perm_z = [0; POINT_COUNT];

        generate_perm(&mut perm_x);
        generate_perm(&mut perm_y);
        generate_perm(&mut perm_z);

        PerlinNoise {
            randfloat,
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
        let u = u*u*(3.0-2.0*u);
        let v = v*v*(3.0-2.0*v);
        let w = w*w*(3.0-2.0*w);
        let i = p.x.trunc().abs() as usize;
        let j = p.y.trunc().abs() as usize;
        let k = p.z.trunc().abs() as usize;

        let mut kernel = [[[0.0 as f32; 2]; 2]; 2];
        

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    kernel[di][dj][dk] = 
                    self.randfloat[
                        self.perm_x[(i+di)&255] ^ 
                        self.perm_y[(j+dj)&255] ^ 
                        self.perm_z[(k+dk)&255]
                    ];
                }
            }
        }
        return trilinear_interpolate(&kernel, u, v, w);
    }

}