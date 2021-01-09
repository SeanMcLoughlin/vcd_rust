use crate::error::LoadError;

pub fn get_value_from_scalar(word: &str, line_num: usize) -> Result<usize, LoadError> {
    const RADIX: u32 = 2;
    match word.chars().collect::<Vec<char>>()[0].to_digit(RADIX) {
        Some(value) => Ok(value as usize),
        None => Err(LoadError::InvalidVarDump { line: line_num }),
    }
}

pub fn get_identifier_from_scalar(word: &str, line_num: usize) -> Result<String, LoadError> {
    let scalar_chars = word.chars().collect::<Vec<char>>();
    match scalar_chars.len() {
        2 => Ok(scalar_chars[1].to_string()),
        _ => Err(LoadError::InvalidVarDump { line: line_num }),
    }
}

pub fn convert_vector_value_to_integer(word: &str, line_num: usize) -> Result<usize, LoadError> {
    if word.len() <= 1 {
        return Err(LoadError::InvalidVarDump { line: line_num });
    }

    let value = &word[1..];
    let radix;
    match word.chars().next() {
        Some('b') => radix = 2,
        Some('r') => radix = 10,
        _ => return Err(LoadError::InvalidVarDump { line: line_num }),
    }

    match usize::from_str_radix(value, radix) {
        Ok(value) => Ok(value),
        Err(_) => Err(LoadError::InvalidVarDump { line: line_num }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_value_from_scalar() {
        assert_eq!(0, get_value_from_scalar("0%", 0).unwrap());
        assert_eq!(1, get_value_from_scalar("1{", 0).unwrap());
        assert_eq!(0, get_value_from_scalar("0*", 0).unwrap());
    }

    #[test]
    fn test_invalid_var_dump_in_get_value_from_scalar_throws_error() {
        assert_eq!(
            get_value_from_scalar("garbage", 1).err(),
            Some(LoadError::InvalidVarDump { line: 1 }),
        );

        assert_eq!(
            get_value_from_scalar("also garbage", 2).err(),
            Some(LoadError::InvalidVarDump { line: 2 }),
        );
    }

    #[test]
    fn test_get_identifier_from_scalar() {
        assert_eq!(
            "%".to_string(),
            get_identifier_from_scalar("0%", 0).unwrap()
        );
        assert_eq!(
            "{".to_string(),
            get_identifier_from_scalar("1{", 0).unwrap()
        );
        assert_eq!(
            "*".to_string(),
            get_identifier_from_scalar("0*", 0).unwrap()
        );
    }

    #[test]
    fn test_invalid_identifier_from_scalar_throws_error() {
        assert_eq!(
            get_identifier_from_scalar("garbage", 1).err(),
            Some(LoadError::InvalidVarDump { line: 1 }),
        );

        assert_eq!(
            get_identifier_from_scalar("also garbage", 2).err(),
            Some(LoadError::InvalidVarDump { line: 2 }),
        );
    }
}
