pub mod marker;
use std::error::Error;
use std::fmt;

#[derive(Clone, PartialEq, Debug, Eq)]
pub struct LoadError {
    pub line: usize,
    pub info: String,
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
        write!(f, "{} at line {}", self.info, self.line)
    }
}
