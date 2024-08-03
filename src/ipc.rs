pub enum MainCommands {
    Recalc,
}

pub enum ClientCommands {
    Redraw(Vec<u32>),
    RedrawPixel((u32, u32)),
}
