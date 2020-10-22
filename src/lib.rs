#[macro_use]
extern crate derive_builder;
extern crate strum;
#[macro_use]
extern crate strum_macros;
pub mod error;
pub mod parser;
pub mod state_machine;
pub mod string_helpers;
pub mod types;
pub mod vcd;

use crate::error::LoadErrorEnum;
use crate::parser::Parser;
use crate::vcd::VCD;
use std::fs::File;

pub fn load_from_str(s: &str) -> Result<VCD, LoadErrorEnum> {
    let mut parser = Parser::new();
    let vcd = parser.parse_from_string(s)?;
    Ok(vcd)
}

pub fn load_from_file(filename: String) -> Result<VCD, LoadErrorEnum> {
    let mut parser = Parser::new();
    return match File::open(&filename[..]) {
        Ok(file) => Ok(parser.parse_from_file(file))?,
        Err(error) => Err(LoadErrorEnum::FileOpenError {
            filename,
            error: error.to_string(),
        }),
    };
}
