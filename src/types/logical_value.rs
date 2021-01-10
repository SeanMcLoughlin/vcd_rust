#[derive(Debug, Clone, Eq, PartialEq, EnumString)]
pub enum LogicalValue {
    #[strum(disabled)]
    Value(usize),
    #[strum(serialize = "x")]
    X,
    #[strum(serialize = "z")]
    Z,
}
