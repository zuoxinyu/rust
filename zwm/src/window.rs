use x11rb::protocol::xproto::*;
use crate::container::Container;
use crate::error::RenderError;
use crate::action::InputAction;

/// The state of a single window that we manage
#[derive(Debug, Clone)]
pub struct ManagedWindow {
    pub window: Window,
    pub frame_window: Window,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub pressing_x: i16,
    pub pressing_y: i16,
    pub pressing: bool,
    pub state: WindowState,
}

impl ManagedWindow {
    pub const TITLEBAR_HEIGHT: u16 = 20u16;
    pub fn new(window: Window, frame_window: Window, geom: &GetGeometryReply) -> ManagedWindow {
        ManagedWindow {
            window,
            frame_window,
            x: geom.x,
            y: geom.y,
            width: geom.width,
            height: geom.height,
            pressing: false,
            pressing_x: 0,
            pressing_y: 0,
            state: WindowState::Normal,
        }
    }

    pub fn close_x_position(&self) -> i16 {
        match self.state {
            WindowState::Normal => std::cmp::max(0, self.width - Self::TITLEBAR_HEIGHT) as _,
            _ => 0,
        }
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

impl PartialEq for ManagedWindow {
    fn eq(&self, other: &Self) -> bool {
        self.frame_window == other.frame_window
    }
}

impl Container for ManagedWindow {
    fn render(&mut self, _action: InputAction) -> Result<(), RenderError> {
        unimplemented!()
    }

    fn parent(&self) -> Box<dyn Container> {
        unimplemented!()
    }

    fn children(&mut self) -> &mut Vec<Box<dyn Container>> {
        unimplemented!()
    }

    fn insert_child(&mut self, _: Box<dyn Container>) -> Result<(), RenderError> {
        unimplemented!()
    }

    fn remove_child(&mut self, _: &dyn Container) -> Result<(), RenderError> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub enum ButtonPos {
    Close,
    Maximum,
    Minimum,
    None,
}

#[derive(Debug, Copy, Clone)]
pub enum WindowState {
    Normal,
    Closed,
    Maximum,
    Minimum,
}