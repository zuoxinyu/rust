use x11rb::protocol::xproto::*;

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
    pub const TITLEBAR_HEIGHT: u16 = 20u16;
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
        std::cmp::max(0, self.width - Self::TITLEBAR_HEIGHT) as _
    }

    pub fn maximum_x_position(&self) -> i16 {
        std::cmp::max(0, self.width - Self::TITLEBAR_HEIGHT * 2) as _
    }

    pub fn minimum_x_position(&self) -> i16 {
        std::cmp::max(0, self.width - Self::TITLEBAR_HEIGHT * 3) as _
    }

    pub fn on_button(&self, x: i16, y: i16) -> ButtonPos {
        match y < Self::TITLEBAR_HEIGHT as i16 {
            // - o x
            true if x > self.close_x_position() && x < self.width as i16 => ButtonPos::Close,
            true if x > self.maximum_x_position() && x < self.close_x_position() => ButtonPos::Maximum,
            true if x > self.minimum_x_position() && x < self.maximum_x_position() => ButtonPos::Minimum,
            _ => ButtonPos::None,
        }

    }
}

pub enum ButtonPos {
    Close,
    Maximum,
    Minimum,
    None,
}
