use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum RenderError {
    BadWindow,
}

impl RenderError {}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "config error: {:?}", self)
    }
}

impl Error for RenderError {}
