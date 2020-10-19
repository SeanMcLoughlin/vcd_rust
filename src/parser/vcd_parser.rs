pub mod command_parser;
pub mod definitions_parser;
pub mod vardump_parser;

use crate::error::LoadError;
use crate::types::vcd_variable::VCDVariable;

pub trait VCDParser {
    fn parse(&self) -> Result<Vec<VCDVariable>, LoadError>;

    fn split_line_into_words(line: &str) -> Vec<&str> {
        line.split(" ").filter(|c| !c.is_empty()).collect()
    }

    fn is_end(word: &String) -> bool {
        word == "$end"
    }

    fn is_command(word: &String, command: &String) -> bool {
        word == command
    }

    fn is_end_of_line(word: &String) -> bool {
        word.contains("\n")
    }

    fn remove_newlines(word: &str) -> String {
        word.replace("\n", "")
    }
}
