use glam::Vec3A;

#[derive(Default, Debug)]
pub struct Camera {
    pub width: u16,
    pub height: u16,
    pub position: Vec3A,

    pub open_time: f32,
    pub close_time: f32,

    viewport_height: f32,
    viewport_width: f32,
    pixel_delta_u: Vec3A,
    pixel_delta_v: Vec3A,
    viewport_upper_left: Vec3A,
    pixel_00_loc: Vec3A,
    defocus_angle: f32,    // Defocus disk horizontal radius
    defocus_disk_u: Vec3A, // Defocus disk horizontal radius
    defocus_disk_v: Vec3A, // Defocus disk vertical radius
}

impl Camera {
    pub fn new(
        look_from: Vec3A,
        look_at: Vec3A,
        vup: Vec3A,
        image_width: u16,
        fov: f32,
        aspect_ratio: f64,
        aperture: f32,
        focus_dist: f32,
        shutter_open_time: f32,
        shutter_close_time: f32,
    ) -> Camera {
        let height: u16 = (image_width as f64 / aspect_ratio) as u16;
        let theta = fov.to_radians();
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f32 / height as f32);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (look_from - look_at).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u; // Vector across viewport horizontal edge
        let viewport_v = viewport_height * -v; // Vector down viewport vertical edge

        let height = if height < 1 { 1 } else { height };
        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / Vec3A::splat(image_width as f32);
        let pixel_delta_v = viewport_v / Vec3A::splat(height as f32);

        let position = look_from;
        let viewport_upper_left = position - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel_00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = focus_dist * (aperture / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            open_time: shutter_open_time,
            close_time: shutter_close_time,
            width: image_width,
            height,
            position: look_from,
            viewport_height,
            viewport_width,
            pixel_delta_u,
            pixel_delta_v,
            viewport_upper_left,
            pixel_00_loc,
            defocus_angle: aperture,
            defocus_disk_u,
            defocus_disk_v,
        }
    }
    pub const fn aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }
    pub const fn viewport_height(&self) -> f32 {
        self.viewport_height
    }
    pub const fn viewport_width(&self) -> f32 {
        self.viewport_width
    }
    pub const fn delta_pixel_u(&self) -> Vec3A {
        self.pixel_delta_u
    }
    pub const fn delta_pixel_v(&self) -> Vec3A {
        self.pixel_delta_v
    }
    pub const fn viewport_upper_left(&self) -> Vec3A {
        self.viewport_upper_left
    }
    pub const fn pixel_00_loc(&self) -> Vec3A {
        self.pixel_00_loc
    }
    pub const fn defocus_angle(&self) -> f32 {
        self.defocus_angle
    }
    pub const fn defocus_disk_u(&self) -> Vec3A {
        self.defocus_disk_u
    }
    pub const fn defocus_disk_v(&self) -> Vec3A {
        // Defocus disk vertical radius
        self.defocus_disk_v
    }
}
