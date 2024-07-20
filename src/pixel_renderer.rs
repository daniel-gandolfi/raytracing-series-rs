use std::{
    process::exit,
    sync::mpsc::{Receiver, SyncSender},
    time::Duration,
};

use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize, event_loop::EventLoop, keyboard::KeyCode,
    platform::pump_events::EventLoopExtPumpEvents, window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use crate::ipc::{ClientCommands, MainCommands};

pub struct PixelRenderer {
    width: u16,
    height: u16,
}
impl PixelRenderer {
    pub const fn new(width: u16, height: u16) -> PixelRenderer {
        let pixel_renderer = PixelRenderer { width, height };
        pixel_renderer
    }

    pub fn setup_commands(
        self,
        main_tx: SyncSender<MainCommands>,
        client_rx: Receiver<ClientCommands>,
    ) {
        let mut event_loop = EventLoop::new().expect("could not instantiate event loop");
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
        loop {
            let mut commands = Vec::with_capacity(32);

            while let Ok(command_res) = client_rx.try_recv() {
                commands.push(command_res);
            }
            let has_commands_to_run = commands.len() != 0;
            while let Some(command) = commands.pop() {
                match command {
                    ClientCommands::Redraw(vec) => {
                        for (frame, color) in pixels.frame_mut().chunks_exact_mut(4).zip(vec.iter())
                        {
                            frame.copy_from_slice(&[
                                (color >> 24 & 255) as u8,
                                (color >> 16 & 255) as u8,
                                (color >> 8 & 255) as u8,
                                255,
                            ])
                        }
                    }
                    ClientCommands::RedrawPixel((pixel, color)) => {
                        pixels.frame_mut()[pixel * 4..pixel * 4 + 4].copy_from_slice(&[
                            (color >> 24 & 255) as u8,
                            (color >> 16 & 255) as u8,
                            (color >> 8 & 255) as u8,
                            255,
                        ]);
                    }
                }
            }
            if has_commands_to_run {
                pixels.render().expect("could not render pixels");
            }
            event_loop.pump_events(Some(Duration::new(0, 500)), |event, _| {
                // Handle input events
                if input.update(&event) {
                    // Close events
                    if input.key_pressed(KeyCode::Escape) || input.close_requested() {
                        exit(0);
                    }

                    if input.key_pressed(KeyCode::F5) {
                        let res = main_tx.send(MainCommands::Recalc);
                        if res.is_ok() {
                            println!(" REDRAW command sent SUCCESSFULLY ");
                        } else {
                            println!(" REDRAW command sent FAILED ");
                        }
                    }
                }
            });
        }
    }
}
