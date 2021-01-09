use crate::error::LoadError;
use crate::types::logical_value::LogicalValue;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DumpedVar<'a> {
    pub value: LogicalValue,
    pub identifier: &'a str,
}

impl<'a> Default for DumpedVar<'a> {
    fn default() -> Self {
        DumpedVar {
            value: LogicalValue::X,
            identifier: "",
        }
    }
}

impl<'a> DumpedVar<'a> {
    pub fn new(line_num: usize, input: &'a str) -> Result<Self, LoadError> {
        let value_char = input.chars().collect::<Vec<char>>()[0];
        let value_string = value_char.to_string();
        let value_str = value_string.as_str();
        let value = match LogicalValue::from_str(value_str) {
            Ok(value) => value,
            Err(_) => return Err(LoadError::InvalidVarDump { line: line_num }),
        };
        let identifier = match input.len() {
            2 => &input[1..2],
            _ => return Err(LoadError::InvalidVarDump { line: line_num }),
        };
        Ok(DumpedVar { value, identifier })
    }
}

//
// pub fn convert_vector_value_to_integer(word: &str, line_num: usize) -> Result<usize, LoadError> {
//     if word.len() <= 1 {
//         return Err(LoadError::InvalidVarDump { line: line_num });
//     }
//
//     let value = &word[1..];
//     let radix;
//     match word.chars().next() {
//         Some('b') => radix = 2,
//         Some('r') => radix = 10,
//         _ => return Err(LoadError::InvalidVarDump { line: line_num }),
//     }
//
//     match usize::from_str_radix(value, radix) {
//         Ok(value) => Ok(value),
//         Err(_) => Err(LoadError::InvalidVarDump { line: line_num }),
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_dumped_var_from_scalar() {
        assert_eq!(
            DumpedVar {
                value: LogicalValue::Zero,
                identifier: "%"
            },
            DumpedVar::new(0, "0%").unwrap()
        );
        assert_eq!(
            DumpedVar {
                value: LogicalValue::One,
                identifier: "{"
            },
            DumpedVar::new(0, "1{").unwrap()
        );
        assert_eq!(
            DumpedVar {
                value: LogicalValue::Zero,
                identifier: "*"
            },
            DumpedVar::new(0, "0*").unwrap()
        );
    }

    #[test]
    fn test_invalid_dumped_var_throws_error() {
        assert_eq!(
            DumpedVar::new(1, "garbage").err(),
            Some(LoadError::InvalidVarDump { line: 1 }),
        );

        assert_eq!(
            DumpedVar::new(2, "0also garbage").err(),
            Some(LoadError::InvalidVarDump { line: 2 }),
        );
    }
}
