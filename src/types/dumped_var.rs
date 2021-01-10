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
        let value = DumpedVar::get_value(input, line_num)?;
        let identifier = DumpedVar::get_identifier(input, line_num)?;
        Ok(DumpedVar { value, identifier })
    }

    pub fn append(&mut self, line_num: usize, input: &'a str) -> Result<(), LoadError> {
        if DumpedVar::is_vector_append(input) {
            if DumpedVar::is_vector_start(input) {
                self.value = DumpedVar::get_value(input, line_num)?;
            } else {
                self.identifier = DumpedVar::get_vector_identifier_str_for_append(input, line_num)?;
            }
        } else {
            self.value = DumpedVar::get_value(input, line_num)?;
            self.identifier = DumpedVar::get_identifier(input, line_num)?;
        }
        Ok(())
    }

    fn get_value(input: &str, line_num: usize) -> Result<LogicalValue, LoadError> {
        if DumpedVar::is_vector(input) {
            DumpedVar::get_vector_value(input, line_num)
        } else {
            DumpedVar::get_scalar_value(input, line_num)
        }
    }

    fn get_identifier(input: &str, line_num: usize) -> Result<&str, LoadError> {
        if DumpedVar::is_vector(input) {
            DumpedVar::get_vector_identifier_str(input, line_num)
        } else {
            DumpedVar::get_scalar_identifier_str(input, line_num)
        }
    }

    fn get_vector_value(input: &str, line_num: usize) -> Result<LogicalValue, LoadError> {
        let possible_error = LoadError::InvalidVarDump { line: line_num };

        let vector_str = DumpedVar::get_vector_value_str(input, line_num)?;

        if let Some(input_value) = vector_str.strip_prefix('b') {
            match usize::from_str_radix(input_value, 2) {
                Ok(output) => Ok(LogicalValue::Value(output)),
                Err(_) => Err(possible_error),
            }
        } else if let Some(input_value) = vector_str.strip_prefix('r') {
            match usize::from_str_radix(input_value, 10) {
                Ok(output) => Ok(LogicalValue::Value(output)),
                Err(_) => Err(possible_error),
            }
        } else {
            Err(possible_error)
        }
    }

    fn get_scalar_value(input: &str, line_num: usize) -> Result<LogicalValue, LoadError> {
        let value_char = input.chars().collect::<Vec<char>>()[0];
        let value_string = value_char.to_string();
        match value_string.parse::<usize>() {
            Ok(output) => Ok(LogicalValue::Value(output)),
            Err(_) => match LogicalValue::from_str(&value_string) {
                Ok(output) => Ok(output),
                Err(_) => Err(LoadError::InvalidVarDump { line: line_num }),
            },
        }
    }

    fn get_scalar_identifier_str(input: &str, line_num: usize) -> Result<&str, LoadError> {
        match input.len() {
            2 => Ok(&input[1..2]),
            _ => Err(LoadError::InvalidVarDump { line: line_num }),
        }
    }

    fn get_vector_value_str(input: &str, line_num: usize) -> Result<&str, LoadError> {
        if let Some(identifier) = DumpedVar::split_vector_input(input).get(0) {
            Ok(identifier)
        } else {
            Err(LoadError::InvalidVarDump { line: line_num })
        }
    }

    fn get_vector_identifier_str(input: &str, line_num: usize) -> Result<&str, LoadError> {
        if let Some(identifier) = DumpedVar::split_vector_input(input).get(1) {
            Ok(identifier)
        } else {
            Err(LoadError::InvalidVarDump { line: line_num })
        }
    }

    fn get_vector_identifier_str_for_append(
        input: &str,
        line_num: usize,
    ) -> Result<&str, LoadError> {
        if let Some(identifier) = DumpedVar::split_vector_input(input).get(0) {
            Ok(identifier)
        } else {
            Err(LoadError::InvalidVarDump { line: line_num })
        }
    }

    fn split_vector_input(input: &str) -> Vec<&str> {
        input.split_whitespace().collect::<Vec<&str>>()
    }

    fn is_vector(input: &str) -> bool {
        input.starts_with('b') || input.starts_with('r')
    }

    fn is_vector_append(input: &str) -> bool {
        DumpedVar::is_vector(input) || !input.chars().next().map(char::is_numeric).unwrap_or(false)
    }

    fn is_vector_start(input: &str) -> bool {
        DumpedVar::is_vector(input)
    }
}

#[cfg(test)]
mod functional_tests {
    use super::*;

    #[test]
    fn new_dumped_var_from_scalar() {
        assert_eq!(
            DumpedVar {
                value: LogicalValue::Value(0),
                identifier: "%"
            },
            DumpedVar::new(0, "0%").unwrap()
        );
        assert_eq!(
            DumpedVar {
                value: LogicalValue::Value(1),
                identifier: "{"
            },
            DumpedVar::new(0, "1{").unwrap()
        );
        assert_eq!(
            DumpedVar {
                value: LogicalValue::Value(0),
                identifier: "*"
            },
            DumpedVar::new(0, "0*").unwrap()
        );
    }

    #[test]
    fn new_dumped_var_from_vector() {
        assert_eq!(
            DumpedVar {
                value: LogicalValue::Value(10),
                identifier: "&"
            },
            DumpedVar::new(0, "b1010 &").unwrap()
        );
        assert_eq!(
            DumpedVar {
                value: LogicalValue::Value(13),
                identifier: "^"
            },
            DumpedVar::new(0, "b1101 ^").unwrap()
        );
        assert_eq!(
            DumpedVar {
                value: LogicalValue::Value(42),
                identifier: "("
            },
            DumpedVar::new(0, "r42 (").unwrap()
        );
    }

    #[test]
    fn test_append_scalar() {
        let mut dumped_var = DumpedVar::default();
        dumped_var.append(0, "0*").unwrap();
        assert_eq!(
            DumpedVar {
                value: LogicalValue::Value(0),
                identifier: "*"
            },
            dumped_var
        );
    }

    #[test]
    fn test_append_vector() {
        let mut dumped_var = DumpedVar::default();
        dumped_var.append(0, "b1010").unwrap();
        dumped_var.append(0, "!").unwrap();
        assert_eq!(
            DumpedVar {
                value: LogicalValue::Value(10),
                identifier: "!"
            },
            dumped_var
        );
    }
}

#[cfg(test)]
mod error_tests {
    use super::*;

    #[test]
    fn test_invalid_scalar_throws_error() {
        assert_eq!(
            DumpedVar::new(1, "garbage").err(),
            Some(LoadError::InvalidVarDump { line: 1 }),
        );

        assert_eq!(
            DumpedVar::new(2, "0also garbage").err(),
            Some(LoadError::InvalidVarDump { line: 2 }),
        );
    }

    #[test]
    fn test_invalid_vector_throws_error() {
        assert_eq!(
            DumpedVar::new(1, "bgarbage &").err(),
            Some(LoadError::InvalidVarDump { line: 1 }),
        );

        assert_eq!(
            DumpedVar::new(2, "y1010 ^").err(),
            Some(LoadError::InvalidVarDump { line: 2 }),
        );

        assert_eq!(
            DumpedVar::new(3, "1 @").err(),
            Some(LoadError::InvalidVarDump { line: 3 }),
        );

        assert_eq!(
            DumpedVar::new(4, "ralso_garbage @").err(),
            Some(LoadError::InvalidVarDump { line: 4 }),
        );

        assert_eq!(
            DumpedVar::new(5, "b1010").err(),
            Some(LoadError::InvalidVarDump { line: 5 }),
        );
    }
}
