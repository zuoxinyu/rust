use crate::action::Action;
use crate::container::Container;
use x11rb::protocol::xproto;

impl Container for xproto::Window {
    fn render(&mut self, _action: Action) -> Result<(), crate::error::RenderError> {
        Ok(())
    }
    fn parent(&self) -> Box<dyn Container> {
        todo!()
    }
    fn children(&mut self) -> &mut Vec<Box<dyn Container>> {
        todo!()
    }
    fn insert_child(&mut self, _: Box<dyn Container>) -> Result<(), crate::error::RenderError> {
        todo!()
    }
    fn remove_child(&mut self, _: &dyn Container) -> Result<(), crate::error::RenderError> {
        todo!()
    }
}
