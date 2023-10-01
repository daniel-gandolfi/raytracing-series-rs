use glam::DVec3;
use std::fs::File;
use std::io::{BufWriter, Write};

pub trait RayTracingRenderer {
    fn render<T>(&self, width: u16, height: u16, buffer: T) -> std::io::Result<()>
    where
        T: Iterator<Item = DVec3>;
}
pub struct PpmImageRenderer {
    file: File,
}
impl PpmImageRenderer {
    pub fn new(filename: &str) -> std::io::Result<PpmImageRenderer> {
        let file = File::create(filename)?;
        Ok(PpmImageRenderer { file })
    }
}
impl RayTracingRenderer for PpmImageRenderer {
    fn render<T>(&self, width: u16, height: u16, buffer: T) -> std::io::Result<()>
    where
        T: Iterator<Item = DVec3>,
    {
        let mut buf_writer = BufWriter::with_capacity(4096 * 16, &self.file);
        write!(&mut buf_writer, "P3\n{width} {height}\n255\n")?;

        buffer
            .map(|color| {
                write!(
                    &mut buf_writer,
                    "{} {} {}\n",
                    color.x.sqrt() * 255.0,
                    color.y.sqrt() * 255.0,
                    color.z.sqrt() * 255.0
                )
            })
            .find(|res| res.is_err())
            .unwrap_or(Ok(()))
    }
}
