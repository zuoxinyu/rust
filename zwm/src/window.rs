use x11rb::protocol::xproto::*;

use crate::types::*;
use crate::container::Container;
use crate::error::RenderError;
use crate::action::Action;

/// The state of a single window that we manage
#[derive(Debug, Clone)]
pub struct ManagedWindow {
    pub window: Window,
    pub frame_window: Window,
    pub x: signed,
    pub y: signed,
    pub width: unsigned,
    pub height: unsigned,
    pub pressing_x: signed,
    pub pressing_y: signed,
    pub pressing: bool,
    pub state: WindowState,
}

impl ManagedWindow {
    pub const TITLEBAR_HEIGHT: unsigned = 20;
    pub fn new(window: Window, frame_window: Window, geom: &GetGeometryReply) -> ManagedWindow {
        ManagedWindow {
            window,
            frame_window,
            x: geom.x as _,
            y: geom.y as _,
            width: geom.width as _,
            height: geom.height as _,
            pressing: false,
            pressing_x: 0,
            pressing_y: 0,
            state: WindowState::Normal,
        }
    }

    pub fn close_x_position(&self) -> signed {
        match self.state {
            WindowState::Normal => std::cmp::max(0, self.width - Self::TITLEBAR_HEIGHT) as _,
            _ => 0,
        }
    }

    pub fn maximum_x_position(&self) -> signed {
        std::cmp::max(0, self.width - Self::TITLEBAR_HEIGHT * 2) as _
    }

    pub fn minimum_x_position(&self) -> signed {
        std::cmp::max(0, self.width - Self::TITLEBAR_HEIGHT * 3) as _
    }

    pub fn on_button(&self, x: signed, y: signed) -> ButtonPos {
        match y < Self::TITLEBAR_HEIGHT as signed{
            // - o x
            true if x > self.close_x_position() && x < self.width as signed => ButtonPos::Close,
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
    fn render(&mut self, _action: Action) -> Result<(), RenderError> {
        todo!()
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
