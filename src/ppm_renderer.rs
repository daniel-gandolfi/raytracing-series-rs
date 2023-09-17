use glam::DVec3;
use std::fs::File;
use std::io::{BufWriter, Write};

pub trait RayTracingRenderer {
    fn render<T>(self: &Self, width: u16, height: u16, buffer: T) -> std::io::Result<()>
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
    fn render<T>(self: &Self, width: u16, height: u16, buffer: T) -> std::io::Result<()>
    where
        T: Iterator<Item = DVec3>,
    {
        let mut buf_writer = BufWriter::with_capacity(4096, &self.file);
        buf_writer.write(b"P3\n")?;
        buf_writer.write(format!("{} {}\n", width, height).as_bytes())?;
        buf_writer.write(b"255\n")?;

        buffer.for_each(|color| {
            buf_writer
                .write(
                    format!(
                        "{} {} {}\n",
                        color.x.sqrt() * 255.0,
                        color.y.sqrt() * 255.0,
                        color.z.sqrt() * 255.0
                    )
                    .as_bytes(),
                )
                .unwrap();
        });
        Ok(())
    }
}
