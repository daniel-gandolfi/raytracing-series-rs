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
#[derive(Debug, Clone, PartialEq)]
pub enum TextureEnum {
    Plain(PlainColorTexture),
    CheckerTexture(CheckerTexture),
}
