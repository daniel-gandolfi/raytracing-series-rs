use glam::Vec3A;
use rand::Rng;
const POINT_COUNT: usize = 256;

#[derive(Debug, Clone)]
pub struct PerlinNoise {
    randfloat: [f64; POINT_COUNT],
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

    pub fn get_noise(&self, p: &Vec3A) -> f64 {
        let i = (4.0 * p.x).floor().abs() as usize & 255;
        let j = (4.0 * p.y).floor().abs() as usize & 255;
        let k = (4.0 * p.z).floor().abs() as usize & 255;

        self.randfloat[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }

}