use crate::error::LoadError;
use crate::vcd::VCD;

pub fn parse(s: &str) -> Result<VCD, LoadError> {
    let lines: Vec<&str> = s.lines().collect();
    let vcd = VCD {
        dates: get_dates_from_lines(lines),
    };
    Ok(vcd)
}

fn get_dates_from_lines(lines: Vec<&str>) -> Vec<String> {
    let mut is_date = false;
    let mut dates: Vec<String> = Vec::new();
    let mut current_date: String = String::new();
    for line in lines {
        if line.contains("$end") {
            if current_date.len() != 0 {
                dates.push(current_date.trim().to_string());
            }
            current_date = String::new();
            is_date = false;
        }
        else if is_date {
            current_date = current_date + line;
        }
        else if line.contains("$date") {
            is_date = true;
        }
    }
    dates
}
