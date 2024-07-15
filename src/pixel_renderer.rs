use std::{
    process::exit,
    sync::mpsc::{Receiver, SyncSender},
};

use pixels::{Pixels, SurfaceTexture};
use winit::{dpi::LogicalSize, event_loop::EventLoop, keyboard::KeyCode, window::WindowBuilder};
use winit_input_helper::WinitInputHelper;

use crate::ipc::{ClientCommands, MainCommands};

pub struct PixelRenderer {
    width: u16,
    height: u16,
}
impl PixelRenderer {
    pub fn new(width: u16, height: u16) -> PixelRenderer {
        let pixel_renderer = PixelRenderer { width, height };
        pixel_renderer
    }

    pub fn setup_commands(
        self,
        main_tx: SyncSender<MainCommands>,
        client_rx: Receiver<ClientCommands>,
    ) {
        let event_loop = EventLoop::new().expect("could not instantiate event loop");
        let window = {
            let size = LogicalSize::new(self.width, self.height);
            WindowBuilder::new()
                .with_title("raytracing series")
                .with_inner_size(size)
                .with_min_inner_size(size)
                .build(&event_loop)
                .unwrap()
        };

        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let mut pixels = Pixels::new(self.width as u32, self.height as u32, surface_texture)
            .expect("could not instantiate Pixels class");
        let mut input = WinitInputHelper::new();
        event_loop
            .run(move |event, _| {
                let command_res = client_rx.try_recv();

                if command_res.is_ok() {
                    println!("client received command ");
                    match command_res.unwrap() {
                        ClientCommands::REDRAW(vec) => {
                            for (i, pixel) in pixels.frame_mut().chunks_exact_mut(4).enumerate() {
                                let color = vec[i];
                                pixel.copy_from_slice(&[
                                    (color >> 24 & 255) as u8,
                                    (color >> 16 & 255) as u8,
                                    (color >> 8 & 255) as u8,
                                    (color & 255) as u8,
                                ]);
                            }
                            pixels.render();
                        }
                    }
                }

                // Handle input events
                if input.update(&event) {
                    // Close events
                    if input.key_pressed(KeyCode::Escape) || input.close_requested() {
                        exit(0);
                    }

                    if input.key_pressed(KeyCode::F5) {
                        let res = main_tx.send(MainCommands::RECALC);
                        if res.is_ok() {
                            println!(" REDRAW command sent SUCCESSFULLY ");
                        } else {
                            println!(" REDRAW command sent FAILED ");
                        }
                    }
                }
            })
            .expect("could not run event loop");
    }
}
