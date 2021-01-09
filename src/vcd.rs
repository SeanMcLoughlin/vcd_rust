use crate::types::{timescale::TimeScale, variable::Variable};
use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct VCD {
    pub date: String,
    pub version: String,
    pub timescale: TimeScale,
    pub comments: Vec<String>,
    pub variables: HashMap<String, Variable>,
}
