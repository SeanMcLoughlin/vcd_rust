pub mod error;
pub mod parser;
pub mod vcd;

pub use crate::vcd::VCDLoader;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
