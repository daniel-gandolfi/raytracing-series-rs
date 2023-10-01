use glam::DVec3;

#[derive(Default, Debug)]
pub struct Camera {
    pub width: u16,
    pub height: u16,
    pub position: DVec3,
    fov: u8,
    focal_length: f64,
}

impl Camera {
    pub fn new(
        position: DVec3,
        width: u16,
        fov: u8,
        focal_length: f64,
        aspect_ratio: f64,
    ) -> Camera {
        let height: u16 = (width as f64 / aspect_ratio) as u16;
        Camera {
            width,
            height: if height < 1 { 1 } else { height },
            fov,
            position,
            focal_length,
        }
    }
    pub fn aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }
    pub fn viewport_height(&self) -> f64 {
        2.0
    }
    pub fn viewport_width(&self) -> f64 {
        self.viewport_height() * (self.width as f64 / self.height as f64)
    }
    fn viewport_u(&self) -> DVec3 {
        DVec3::new(self.viewport_width(), 0.0, 0.0)
    }
    fn viewport_v(&self) -> DVec3 {
        DVec3::new(0.0, -(self.viewport_height()), 0.0)
    }
    pub fn delta_pixel_u(&self) -> DVec3 {
        self.viewport_u() / (self.width as f64)
    }
    pub fn delta_pixel_v(&self) -> DVec3 {
        self.viewport_v() / (self.height as f64)
    }
    pub fn viewport_upper_left(&self) -> DVec3 {
        self.position
            - DVec3::new(0.0, 0.0, self.focal_length)
            - self.viewport_u() / 2.0
            - self.viewport_v() / 2.0
    }
    pub fn pixel_00_loc(&self) -> DVec3 {
        self.viewport_upper_left() + 0.5 * (self.delta_pixel_u() + self.delta_pixel_v())
    }
}
