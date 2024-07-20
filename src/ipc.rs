pub enum MainCommands {
    Recalc,
}
pub enum ClientCommands {
    Redraw(Vec<u32>),
    RedrawPixel((usize, u32)),
}
