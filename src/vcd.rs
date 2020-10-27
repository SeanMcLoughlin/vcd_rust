use crate::types::{timescale::TimeScale, variable::Variable};
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct VCD {
    pub date: String,
    pub version: String,
    pub timescale: TimeScale,
    pub comments: Vec<String>,
    pub variables: HashMap<String, Variable>,
}

impl VCD {
    pub fn new() -> VCD {
        VCD {
            date: String::new(),
            version: String::new(),
            timescale: TimeScale::new(),
            comments: Vec::new(),
            variables: HashMap::new(),
        }
    }
}
