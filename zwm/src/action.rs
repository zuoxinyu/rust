#[derive(Copy, Clone)]
pub enum InputAction {
    SetPosition(i32, i32),
    SetSize(u32, u32),
}