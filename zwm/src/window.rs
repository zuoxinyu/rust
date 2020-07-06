use x11rb::protocol::xproto::*;

pub const TITLEBAR_HEIGHT: u16 = 20;
/// The state of a single window that we manage
#[derive(Debug, Clone)]
pub struct WindowState {
    pub window: Window,
    pub frame_window: Window,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub pressing: bool,
    pub pressing_x: i16,
    pub pressing_y: i16,
}

impl WindowState {
    pub fn new(window: Window, frame_window: Window, geom: &GetGeometryReply) -> WindowState {
        WindowState {
            window,
            frame_window,
            x: geom.x,
            y: geom.y,
            width: geom.width,
            height: geom.height,
            pressing: false,
            pressing_x: 0,
            pressing_y: 0,
        }
    }

    pub fn close_x_position(&self) -> i16 {
        std::cmp::max(0, self.width - TITLEBAR_HEIGHT) as _
    }
}
