use glam::Vec3A;

pub trait Texture {
    fn get_uv_color(&self, u: f32, v: f32, p: &Vec3A) -> Vec3A;
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlainColorTexture(Vec3A);
impl PlainColorTexture {
    pub const fn new(color: Vec3A) -> Self {
        Self(color)
    }
}
impl Texture for &PlainColorTexture {
    fn get_uv_color(&self, u: f32, v: f32, p: &Vec3A) -> Vec3A {
        return self.0.clone();
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct CheckerTexture {
    t0: PlainColorTexture,
    t1: PlainColorTexture,
}
impl CheckerTexture {
    pub const fn new(t0: PlainColorTexture, t1: PlainColorTexture) -> Self {
        Self { t0, t1 }
    }
}

impl Texture for &CheckerTexture {
    fn get_uv_color(&self, u: f32, v: f32, p: &Vec3A) -> Vec3A {
        let sines = p
            .as_ref()
            .map(|x| x * 10.0)
            .map(|x| x.sin())
            .iter()
            .fold(1.0, |mul, el| mul * el);
        let texture = if sines < 0.0 { &self.t0 } else { &self.t1 };
        texture.get_uv_color(u, v, p)
    }
}

#[derive(Debug, Clone)]
pub struct PerlinNoiseTexture {
//    noise: noise::Perlin,
    noise: crate::perlin_noise::PerlinNoise,
}
impl PerlinNoiseTexture {
    pub fn new(size: usize) -> Self {
        Self {
//            noise: noise::Perlin::new(size as u32),
            noise: crate::perlin_noise::PerlinNoise::new(),
        }
    }
}

impl Texture for PerlinNoiseTexture {
    fn get_uv_color(&self, u: f32, v: f32, p: &Vec3A) -> Vec3A {
//        let noise = self.noise.get([p.x as f64,p.y as f64,p.z as f64]);
        let noise = self.noise.get_noise(p) as f32;
//        Vec3A::new(noise*p.x, noise*p.y, noise*p.z)
        Vec3A::splat(noise)
    }
}

#[derive(Debug, Clone)]
pub enum TextureEnum {
    Plain(PlainColorTexture),
    CheckerTexture(CheckerTexture),
    PerlinNoise(PerlinNoiseTexture),
}
