use std::io::{Write, BufWriter};
use core::option::Iter;
use std::fs::File;
use indicatif::ProgressIterator;
use glam::DVec3;

#[derive(Default, Debug)]
struct Camera {
    width: u16,
    height: u16,
    position: DVec3,
    fov: u8,
    focal_length: f64
}
impl Camera {
    fn aspect_ratio(self: &Self) -> f32  {
        self.width as f32  / self.height as f32 
    }
    fn viewport_height(self: &Self)  -> f64{
        2.0
    }
    fn viewport_width(self: &Self ) -> f64{
        self.viewport_height() * (self.width / self.height) as f64
    }
    fn viewport_u(self: &Self) -> DVec3{
        DVec3::new(self.viewport_width() as f64, 0.0,0.0)
    }
    fn viewport_v(self: &Self) -> DVec3{
        DVec3::new(0.0, -(self.viewport_height() as f64),0.0)
    }
    fn delta_pixel_u(self: &Self) -> DVec3 {
        self.viewport_u() / (self.width as f64)
    }
    fn delta_pixel_v(self: &Self) -> DVec3 {
        self.viewport_v() / (self.height as f64)
    }
    fn viewport_upper_left(self: &Self) -> DVec3 {
       self.position - DVec3::new(0.0,0.0, self.focal_length as f64) - self.viewport_u() / 2.0 - self.viewport_v() / 2.0
    }
    fn pixel_00_loc(self: &Self) -> DVec3{
        self.viewport_upper_left() + 0.5 * (self.delta_pixel_u()+ self.delta_pixel_v())
    }
}
#[derive(Default, Debug)]
struct Ray {
    origin: DVec3,
    direction: DVec3
}
trait RayHittable {
    fn hit(self: &Self, ray: &Ray) -> bool;
}

struct Sphere {
    center: DVec3,
    radius: f64
}
impl RayHittable for Sphere {
    fn hit(self: &Self, ray:&Ray) -> bool {
        let center = self.center;
        let oc = ray.origin - center;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.dot(oc)  - self.radius * self.radius;
        let discriminant = b*b - 4.0*a*c;
//        std::io::stdout().write((discriminant.to_string()).as_bytes());
        discriminant >= 0.0
    }
}


fn create_camera() -> Camera {
    const aspect_ratio:f64 = 16.0 / 9.0;
    const width:u16 = 1280 as u16;
    const height:u16 = (width as f64 / aspect_ratio) as u16;
    Camera {
        width,
        height: if height < 1 { 1 } else { height } ,
        fov: 90,
        position: DVec3::new(0.0,0.0,0.0),
        focal_length: 1.0
    }
}

const SPHERE :Sphere =  Sphere{
    center: DVec3 {x:0.0,y:0.0,z:-1.0}, 
    radius: 0.5
};
fn ray_color(ray:Ray) -> DVec3 {
    let hit_sphere = SPHERE.hit(&ray) ;
    if hit_sphere {
        return DVec3{x:1.0,y:0.0,z:0.0};
    }
    let unit = ray.direction.normalize_or_zero();
    let a = 0.5*(unit.y + 1.0);
    return (1.0-a)*DVec3::new(1.0, 1.0, 1.0) + a*DVec3::new(0.5, 0.7, 1.0);
}

fn save_ppm<T>(
    width: u16,
    height: u16,
    buffer: T,
    mut writer: Box< dyn Write>
) -> std::io::Result<()> 
where T: Iterator<Item=DVec3>{
    let mut buf_writer = BufWriter::with_capacity(2048,writer); 
    buf_writer.write(b"P3\n")?;
    buf_writer.write(format!("{} {}\n", width, height).as_bytes())?;
    buf_writer.write(b"255\n")?;

    std::io::stdout().write(b"creating points\n");
    buffer
        .progress_count(width as u64 *height as u64)
        .for_each(|color|{
            buf_writer.write(format!("{} {} {}\n",color.x * 255.0,color.y*255.0,color.z*255.0 ).as_bytes()).unwrap();
     });
    Ok(())
}

fn main() -> std::io::Result<()>{
    println!("Hello, world!");
    let camera = create_camera();

    let mut file = File::create("render.ppm")?;
    let color_iter = (0..camera.height).flat_map(|j|{
        let camera_ref = &camera;
        let pixel00_loc = camera.pixel_00_loc();
        let pixel_delta_u = camera.delta_pixel_u();
        let pixel_delta_v = camera.delta_pixel_v();
        (0..camera.width).map(move |i|{
            let camera = camera_ref; 
            let pixel_center = pixel00_loc + (i as f64 * pixel_delta_u)  + (j as f64 * pixel_delta_v);
            let camera_position = camera.position; 
            let direction = pixel_center - camera_position;
            Ray {
                origin: camera_position.clone(),
                direction
            }
        })
    }).map(ray_color);
    save_ppm(camera.width ,camera.height, color_iter , Box::new(file))?;

    Ok(())
}
