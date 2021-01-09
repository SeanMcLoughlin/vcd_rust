#[derive(Debug, Clone, Eq, PartialEq, EnumString)]
pub enum LogicalValue {
    #[strum(serialize = "1")]
    One,
    #[strum(serialize = "0")]
    Zero,
    #[strum(serialize = "x")]
    X,
    #[strum(serialize = "z")]
    Z,
}
