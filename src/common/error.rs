use std::io::Error;

pub type Result<T> = std::result::Result<T, RustubError>;

#[derive(Debug)]
pub enum RustubError {
    IOError(Error),
    MsgError(String),
}
