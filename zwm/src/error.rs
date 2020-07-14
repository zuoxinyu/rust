use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum RenderError {
    BadWindow
}

impl RenderError {}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "config error: {:?}", self)
    }
}

impl Error for RenderError {}