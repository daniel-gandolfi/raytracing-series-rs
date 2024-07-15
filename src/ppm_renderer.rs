use std::io::Write;
use std::sync::mpsc::SyncSender;
use std::thread::{self, JoinHandle};
use std::{fs::File, sync::mpsc::Receiver};

use crate::ipc::{ClientCommands, MainCommands};

pub struct PpmImageRenderer {
    file: File,
    width: u16,
    height: u16,
}
impl PpmImageRenderer {
    pub fn new(width: u16, height: u16) -> PpmImageRenderer {
        let file = File::create("render.ppm").expect("could not create output file");
        PpmImageRenderer {
            file,
            width,
            height,
        }
    }

    pub fn setup_commands(
        self,
        main_tx: SyncSender<MainCommands>,
        client_rx: Receiver<ClientCommands>,
    ) {
        main_tx
            .send(MainCommands::RECALC)
            .expect("could not send recalc command in ppm renderer");
        loop {
            let command_res = client_rx.try_recv();

            if command_res.is_ok() {
                println!("client received command ");
                match command_res.unwrap() {
                    ClientCommands::REDRAW(vec) => {
                        let width = self.width;
                        let height = self.height;
                        let mut buf_writer = std::io::BufWriter::with_capacity(1024, &self.file);
                        write!(&mut buf_writer, "P3\n{width} {height}\n255\n");
                        for color in vec {
                            writeln!(
                                &mut buf_writer,
                                "{} {} {}",
                                (color >> 24 & 255) as u8,
                                (color >> 16 & 255) as u8,
                                (color >> 8 & 255) as u8
                            );
                        }
                        return;
                    }
                }
            }
        }
    }
}
