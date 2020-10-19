#[macro_use]
extern crate derive_builder;
extern crate strum;
#[allow(unused_imports)]
#[macro_use]
extern crate strum_macros;

pub mod error;
pub mod parser;
pub mod types;
pub mod vcd;

pub use crate::vcd::VCDLoader;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
