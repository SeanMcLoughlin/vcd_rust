#[derive(Default, Eq, PartialEq, Clone)]
pub struct DumpedVar {
    pub value: usize,
    pub identifier: String,
}

impl DumpedVar {
    pub fn new() -> Self {
        DumpedVar {
            value: 0,
            identifier: "".to_string(),
        }
    }
}
