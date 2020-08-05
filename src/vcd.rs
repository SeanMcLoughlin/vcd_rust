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
    fn load_vcd_with_one_date_command_from_str() {
        let mut contents = r#"$date
            Date text. For example: August 9th, 2020.
        $end"#;
        let mut vcd = VCDLoader::load_from_str(&contents).unwrap();
        assert_eq!(vcd.dates.len(), 1);
        assert_eq!(vcd.dates[0], "Date text. For example: August 9th, 2020.");

        contents = r#"$date
            Some other date text.
        $end"#;
        vcd = VCDLoader::load_from_str(&contents).unwrap();
        assert_eq!(vcd.dates.len(), 1);
        assert_eq!(vcd.dates[0], "Some other date text.");
        
    }

    #[test]
    fn load_vcd_with_multiple_date_commands_from_str() {
        let contents = r#"$date
            Date text 1
        $end
        $date
            Date text 2
        $end"#;
        
        let vcd = VCDLoader::load_from_str(&contents).unwrap();
        assert_eq!(vcd.dates.len(), 2);
        assert_eq!(vcd.dates, vec!["Date text 1", "Date text 2"]);


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
