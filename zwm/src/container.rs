use crate::action::Action;
use crate::error::RenderError;

/// Container is the basic abstraction for window, namespace, layout
///
/// Example:
///
/// ```rust
///
/// use zwm::error::RenderError;
/// use zwm::container::Container;
///
/// struct Rect {
///     width: u32,
///     height: u32,
///     x: i32,
///     y: i32,
///     children: Vec<Box<dyn Container>>,
/// }
///
/// impl Container for Rect {
///     fn render(&mut self, action: InputAction) -> Result<(), RenderError> {
///         // do something for self
///         println!("{}{}{}{}", self.x, self.y, self.width, self.height);
///
///         // propagate to children
///         return self.children().iter_mut().map(|child|
///             match action {
///                 InputAction::SetPosition(x, y) => child.render(InputAction::SetPosition(x, y)),
///                 InputAction::SetSize(w, h) => child.render(InputAction::SetSize(w, h)),
///             }
///         ).fold(Ok(()), |acc, x| acc.and(x));
///     }
///
///     fn parent(&self) -> Box<dyn Container> {
///         Box::new(Rect { width: 0, height: 0, x: 0, y: 0, children: vec![] })
///     }
///
///     fn children(&mut self) -> &mut Vec<Box<dyn Container>> {
///         &mut self.children
///     }
///
///     fn insert_child(&mut self, child: Box<dyn Container>) -> Result<(), RenderError> {
///         self.children.push(child);
///         Ok(())
///     }
///
///     fn remove_child(&mut self, _child: &dyn Container) -> Result<(), RenderError> {
///         self.children.retain(|_x| {
///             false
///         });
///         Ok(())
///     }
/// }
/// ```
pub trait Container {
    fn render(&mut self, action: Action) -> Result<(), RenderError>;
    fn parent(&self) -> Box<dyn Container>;
    fn children(&mut self) -> &mut Vec<Box<dyn Container>>;
    fn insert_child(&mut self, _: Box<dyn Container>) -> Result<(), RenderError>;
    fn remove_child(&mut self, _: &dyn Container) -> Result<(), RenderError>;
}
