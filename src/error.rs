pub mod marker;
use marker::Marker;
use std::error::Error;
use std::fmt;

#[derive(Clone, PartialEq, Debug, Eq)]
pub struct LoadError {
    mark: Marker,
    info: String,
}

impl Error for LoadError {
    fn description(&self) -> &str {
        self.info.as_ref()
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} at line {} col {}", self.info, self.mark.line, self.mark.col)
    }
}