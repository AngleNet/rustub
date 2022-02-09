use std::fmt::{write, Display, Formatter};
use std::io::Error;

pub type Result<T> = std::result::Result<T, RustubError>;

#[derive(Debug)]
pub enum RustubError {
    UntypedError(&'static str),
    IOError(Error, &'static str),
    AstNodeVisitError(&'static str),
    UnimplementedError(&'static str),
}

impl Display for RustubError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RustubError::IOError(e, m) => {
                write!(f, "IO Error :: {} :: {}", e, m)
            }
            RustubError::UntypedError(m) => {
                write!(f, "{}", m)
            }
            RustubError::UnimplementedError(m) => {
                write!(f, "Not Implemented :: {}", m)
            }
            RustubError::AstNodeVisitError(m) => {
                write!(f, "AST Visit Error :: {}", m)
            }
        }
    }
}
