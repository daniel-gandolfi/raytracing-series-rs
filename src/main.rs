use std::{io::{Write, BufWriter}, ops::Mul, ops::Range};
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
        self.viewport_height() * (self.width as f64 / self.height as f64)
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
impl Ray {
    fn at(self: &Self, t: f64) -> DVec3 {
        self.origin + t * self.direction
    }
}
trait RayHittable {
    fn hit(self: &Self, ray: &Ray, range: Range<f64>) -> Option<(
        f64, // t
        DVec3, // point
        DVec3, // normal,
        bool // front_face
    )>;
}

struct Sphere {
    center: DVec3,
    radius: f64
}
impl RayHittable for Sphere {
    fn hit(self: &Self, ray:&Ray, range: Range<f64>) -> Option<(f64, DVec3,DVec3, bool)>{
        let center = self.center;
        let oc = ray.origin - center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared()  - self.radius * self.radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 { 
             Option::None
        } else {
            let sqrtd = discriminant.sqrt();
            let mut root = (-half_b - sqrtd) / a;
            if !range.contains(&root) && &range.start != &root {
                root = (-half_b + sqrtd) / a;
                if !range.contains(&root) && &range.start != &root {
                    return Option::None;
                }            
            }
    
            let hit_t = root;
            let hit_point = ray.at(hit_t) ;
            let hit_normal = (hit_point - center) / self.radius;               let front_face = ray.direction.dot(hit_normal) < 0.0;
            Option::Some((
                hit_t,
                hit_point,
                if front_face {hit_normal }else {-hit_normal},
                front_face
            ))
        } 
    }
}


fn create_camera() -> Camera {
    const aspect_ratio:f64 = 16.0 / 9.0;
    const width:u16 = 1920 as u16;
    const height:u16 = (width as f64 / aspect_ratio) as u16;
    Camera {
        width,
        height: if height < 1 { 1 } else { height } ,
        fov: 90,
        position: DVec3::new(0.0,0.0,0.0),
        focal_length: 1.0
    }
}



fn ray_color(ray:Ray, world: &Vec<Box<dyn  RayHittable>>) -> DVec3 {
    let normal_opt: Option<(f64, DVec3,DVec3, bool)> = world.iter().find_map(|obj| {
        let range = 0.0..(f64::INFINITY);
        obj.hit(&ray, range)
    });
    
        
    if let Some(hit_record) = normal_opt {
        return (hit_record.2 + DVec3::ONE).mul(0.5); 
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
    let mut buf_writer = BufWriter::with_capacity(4096,writer); 
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
    let world : Vec<Box<dyn RayHittable>> = vec![
        Box::new(Sphere{
            center: DVec3 {x:0.0,y:0.0,z:-1.0},
            radius: 0.5
        }),
        Box::new(Sphere{
            center: DVec3 {x:0.0,y:-100.5,z:-1.0},
            radius: 100.0
        })
    ];

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
    }).map(|ray| {
        ray_color(ray, &world)
    });
    save_ppm(camera.width ,camera.height, color_iter , Box::new(file))?;

    Ok(())
}
