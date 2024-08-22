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
    fn get_uv_color(&self, _u: f32, _v: f32, _p: &Vec3A) -> Vec3A {
        return self.0;
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
    noise: Box<crate::perlin_noise::PerlinNoise>,
    scale: f32,
}
impl PerlinNoiseTexture {
    pub fn new(scale: f32) -> Self {
        Self {
            //            noise: noise::Perlin::new(size as u32),
            noise: Box::new(crate::perlin_noise::PerlinNoise::new()),
            scale,
        }
    }
}

impl Texture for PerlinNoiseTexture {
    fn get_uv_color(&self, _u: f32, _v: f32, p: &Vec3A) -> Vec3A {
        //        let noise = self.noise.get([p.x as f64,p.y as f64,p.z as f64]);
        //        Vec3A::new(noise*p.x, noise*p.y, noise*p.z)

        //        Vec3A::splat(self.noise.get_noise(&Vec3A::new(
        //    p.x * self.scale,
        //    p.y * self.scale,
        //    p.z * self.scale,
        //)).remap(-1.0, 1.0, 0.0, 1.0) )

        Vec3A::splat(
            0.5 * (1.0 + (self.scale * p.z + 10.0 * self.noise.turbulence(p, Some(7))).sin()),
        )
    }
}

#[derive(Debug, Clone)]
pub enum TextureEnum {
    Plain(PlainColorTexture),
    CheckerTexture(CheckerTexture),
    PerlinNoise(PerlinNoiseTexture),
}
