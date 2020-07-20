use crate::error::RenderError;
use crate::window::ManagedWindow;
use crate::types::*;

#[derive(Copy, Clone)]
pub enum Action {
    Destroy,
    Focus,
    Show,
    Hide,
    Maximum,
    Minimum,
    Fullscreen,
    MouseRelease(signed, signed),
    Move(signed, signed),
    SetPosition(signed, signed),
    SetSize(unsigned, unsigned),
    SetPressing(signed, signed),
}

pub trait WindowOperator {
    fn render(&self, window: &mut ManagedWindow, action: Action) -> Result<(), RenderError>;
}

pub mod xcb {
    use x11rb::connection::Connection;
    use x11rb::protocol::xproto::*;
    use x11rb::CURRENT_TIME;

    use super::Action;
    use crate::error::RenderError;
    use crate::window::ManagedWindow;
    use crate::container::Container;

    impl<T> super::WindowOperator for T
    where
        T: Connection + ConnectionExt,
    {
        fn render(
            &self,
            window: &mut ManagedWindow,
            action: Action,
        ) -> Result<(), RenderError> {
            match action {
                Action::SetSize(w, h) => {
                    let aux = ConfigureWindowAux::default().width(w as u32).height(h as u32 + ManagedWindow::TITLEBAR_HEIGHT as u32);
                    self.configure_window(window.frame_window, &aux)?;
                    let aux = ConfigureWindowAux::default().width(w as u32).height(h as u32 + ManagedWindow::TITLEBAR_HEIGHT as u32);
                    self.configure_window(window.frame_window, &aux)?;
                    window.width = w;
                    window.height = h;
                }
                Action::SetPosition(x, y) => {
                    let aux = ConfigureWindowAux::default().x(x as i32).y(y as i32);
                    self.configure_window(window.frame_window, &aux)?;
                    window.x = x;
                    window.y = y;
                }
                Action::Show => {
                    self.map_window(window.frame_window)?;
                }
                Action::Hide => {
                    self.unmap_window(window.frame_window)?;
                }
                Action::Destroy => {
                    self.destroy_window(window.frame_window)?;
                }
                Action::Focus => {
                    self.set_input_focus(InputFocus::Parent, window.frame_window, CURRENT_TIME)?;
                    let aux = ConfigureWindowAux::default().stack_mode(StackMode::Above);
                    self.configure_window(window.frame_window, &aux)?;
                }
                Action::SetPressing(x, y) => {
                    window.pressing = true;
                    window.pressing_x = x;
                    window.pressing_y = y;
                }
                Action::Move(x, y) => {
                    if window.pressing {
                        window.pressing_x = x;
                        window.pressing_y = y;
                        window.render(Action::SetPosition(x, y))?;
                    }
                }
                _ => {}
            };

            self.flush()?;
            Ok(())
        }
    }

    impl std::convert::From<x11rb::errors::ConnectionError> for RenderError {
        fn from(_: x11rb::errors::ConnectionError) -> Self {
            todo!()
        }
    }
}
