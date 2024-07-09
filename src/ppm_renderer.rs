use std::fs::File;
use std::io::Write;

pub trait RayTracingRenderer {
    fn render<Pixel, IteratorType>(
        &self,
        width: u16,
        height: u16,
        buffer: IteratorType,
    ) -> std::io::Result<()>
    where
        Pixel: AsRef<[f64; 3]>,
        IteratorType: Iterator<Item = Pixel>;
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
    fn render<Pixel, IteratorType>(
        &self,
        width: u16,
        height: u16,
        buffer: IteratorType,
    ) -> std::io::Result<()>
    where
        Pixel: AsRef<[f64; 3]>,
        IteratorType: Iterator<Item = Pixel>,
    {
        let mut buf_writer = std::io::BufWriter::with_capacity(1024, &self.file);
        write!(&mut buf_writer, "P3\n{width} {height}\n255\n")?;

        buffer
            .map(move |color| {
                let [r, g, b] = color.as_ref();
                writeln!(
                    &mut buf_writer,
                    "{} {} {}",
                    r.sqrt() * 255.0,
                    g.sqrt() * 255.0,
                    b.sqrt() * 255.0
                )
            })
            .find(|err| err.is_err())
            .unwrap_or(Ok(()))
    }
}
