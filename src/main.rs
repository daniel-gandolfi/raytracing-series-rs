use std::io::Write;
use std::fs::File;
use indicatif::ProgressIterator;
use glam::DVec3;

fn save_ppm(
    width: usize,
    buffer: &[u32],
    mut writer: Box< dyn Write>
) -> std::io::Result<()> {
    let height = buffer.len() / width;
    
    writer.write(b"P3\n")?;
    writer.write(format!("{} {}\n", width, height).as_bytes())?;
    writer.write(b"255\n")?;

    std::io::stdout().write(b"creating points\n");
    let points = buffer.into_iter()
        .progress()
    .map(|compound_idx| {
        let i = compound_idx % width as u32;
        let j = compound_idx / width as u32 ;
            (i,j)
    }).map(|point|{
        let r:f32 = (point.0 as f32) / (width -1) as f32;
        let g:f32 = (point.1 as f32) / (height -1) as f32;
        let b: f32 = 0.0;
        DVec3::new(
            (255.999 * r) as u32 as f64, 
            (255.999 * g) as u32 as f64,
            (255.999 * b) as u32 as f64
        )
    });
     points.progress().for_each(|color|{
            writer.write(format!("{} {} {}\n",color.x,color.y,color.z).as_bytes()).unwrap();
     });
    Ok(())
}
fn main() -> std::io::Result<()>{
    println!("Hello, world!");
    let mut file = File::create("render.ppm")?;
    let nums: Vec<u32> = (0..1024*768).collect();
    save_ppm(1024,nums.as_slice(), Box::new(file))?;

    Ok(())
}
