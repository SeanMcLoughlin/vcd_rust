use crate::error::LoadError;
use crate::parser;

pub struct VCD {
    pub dates: Vec<String>,
}

impl VCD {
    pub fn new() -> VCD {
        VCD { dates: Vec::new() }
    }
}

pub struct VCDLoader;

impl VCDLoader {
    pub fn load_from_str(s: &str) -> Result<VCD, LoadError> {
        let vcd = parser::parse(s)?;
        Ok(vcd)
    }

    #[allow(unused_variables)]
    pub fn load_from_file(f: &str) -> Result<VCD, LoadError> {
        Ok(VCD::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn load_vcd_with_file_with_date_command() {
        let contents = r#"$date
            Date text. For example: August 9th, 2020.
        $end"#;
        let vcd = VCDLoader::load_from_str(&contents).unwrap();
        assert_ne!(vcd.dates.len(), 0);
        assert_eq!(vcd.dates[0], "Date text. For example: August 9th, 2020.");
    }

    #[test]
    #[allow(dead_code)]
    fn load_vcd_file_from_string_with_invalid_contents_throws_load_error() {
        // TODO
    }

    #[test]
    #[allow(dead_code)]
    fn load_vcd_file_from_file_with_invalid_contents_throws_load_error() {
        // TODO
    }
}
