use crate::error::LoadError;
use crate::vcd::VCD;

pub fn parse(s: &str) -> Result<VCD, LoadError> {
    let lines: Vec<&str> = s.lines().collect();
    let vcd = VCD {
        dates: vec![lines[1].trim().to_string()],
    };
    Ok(vcd)
}
