mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use std::error;
use std::fmt;

pub type Error = Box<dyn error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct StrError<'a>(&'a str);
impl<'a> error::Error for StrError<'a> {}

impl<'a> fmt::Display for StrError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Delegate to the Display impl for `&str`:
        self.0.fmt(f)
    }
}

fn main() -> Result<()> {
    todo!()
}
