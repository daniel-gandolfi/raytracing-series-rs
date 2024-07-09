use glam::DVec3;

#[derive(Default, Debug)]
pub struct Camera {
    pub width: u16,
    pub height: u16,
    pub position: DVec3,

    fov: f32,
    viewport_height: f64,
    viewport_width: f64,
    pixel_delta_u: DVec3,
    pixel_delta_v: DVec3,
    viewport_upper_left: DVec3,
    pixel_00_loc: DVec3,
    defocus_angle: f64,    // Defocus disk horizontal radius
    defocus_disk_u: DVec3, // Defocus disk horizontal radius
    defocus_disk_v: DVec3, // Defocus disk vertical radius
}

impl Camera {
    pub fn new(
        look_from: DVec3,
        look_at: DVec3,
        vup: DVec3,
        image_width: u16,
        fov: f32,
        aspect_ratio: f64,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Camera {
        let height: u16 = (image_width as f64 / aspect_ratio) as u16;
        let theta = (fov as f64).to_radians();
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (look_from - look_at).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u; // Vector across viewport horizontal edge
        let viewport_v = viewport_height * -v; // Vector down viewport vertical edge

        let height = if height < 1 { 1 } else { height };
        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / DVec3::splat(image_width as f64);
        let pixel_delta_v = viewport_v / DVec3::splat(height as f64);

        let position = look_from;
        let viewport_upper_left = position - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel_00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = focus_dist * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            width: image_width,
            height,
            fov,
            position: look_from,
            viewport_height,
            viewport_width,
            pixel_delta_u,
            pixel_delta_v,
            viewport_upper_left,
            pixel_00_loc,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }
    pub const fn aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }
    pub const fn viewport_height(&self) -> f64 {
        self.viewport_height
    }
    pub const fn viewport_width(&self) -> f64 {
        self.viewport_width
    }
    pub const fn delta_pixel_u(&self) -> DVec3 {
        self.pixel_delta_u
    }
    pub const fn delta_pixel_v(&self) -> DVec3 {
        self.pixel_delta_v
    }
    pub const fn viewport_upper_left(&self) -> DVec3 {
        self.viewport_upper_left
    }
    pub const fn pixel_00_loc(&self) -> DVec3 {
        self.pixel_00_loc
    }
    pub const fn defocus_angle(&self) -> f64 {
        self.defocus_angle
    }
    pub const fn defocus_disk_u(&self) -> DVec3 {
        self.defocus_disk_u
    }
    pub const fn defocus_disk_v(&self) -> DVec3 {
        // Defocus disk vertical radius
        self.defocus_disk_v
    }
}
