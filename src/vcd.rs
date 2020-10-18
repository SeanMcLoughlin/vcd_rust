use crate::error::LoadError;
use crate::parser;
use crate::types::TimeScale;
use std::fs::File;
use std::io::Read;

pub struct VCD {
    pub date: String,
    pub version: String,
    pub timescale: TimeScale,
    pub comments: Vec<String>,
}

impl VCD {
    pub fn new() -> VCD {
        VCD {
            date: String::new(),
            version: String::new(),
            timescale: TimeScale::new(),
            comments: Vec::new(),
        }
    }
}

pub struct VCDLoader;

impl VCDLoader {
    pub fn load_from_str(s: String) -> Result<VCD, LoadError> {
        let vcd = parser::parse(s)?;
        Ok(vcd)
    }

    #[allow(unused_variables)]
    pub fn load_from_file(filename: String) -> Result<VCD, LoadError> {
        let mut content = String::new();
        match File::open(&filename[..]) {
            Ok(mut file) => {
                file.read_to_string(&mut content).unwrap();
            }
            Err(error) => {
                println!("Error opening file {}: {}", filename, error);
            }
        }
        Ok(parser::parse(content))?
    }
}
