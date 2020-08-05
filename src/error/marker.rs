#[derive(Clone, Copy, PartialEq, Debug, Eq)]
pub struct Marker {
    pub index: usize,
    pub line: usize,
    pub col: usize,
}

impl Marker {
    #[allow(dead_code)]
    fn new(index: usize, line: usize, col: usize) -> Marker {
        Marker { index, line, col }
    }
}
