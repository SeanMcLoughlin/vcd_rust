use crate::error::LoadError;
use crate::vcd::VCD;

pub fn parse(s: &str) -> Result<VCD, LoadError> {
    let lines: Vec<&str> = s.lines().collect();
    let vcd = VCD {
        date: get_date_from_lines(&lines).unwrap(),
        version: get_version_from_lines(&lines).unwrap(),
    };
    Ok(vcd)
}

fn get_date_from_lines(lines: &Vec<&str>) -> Result<String, LoadError> {
    get_lines_between_command_and_end(lines, "$date")
}

fn get_version_from_lines(lines: &Vec<&str>) -> Result<String, LoadError> {
    get_lines_between_command_and_end(lines, "$version")
}

fn get_lines_between_command_and_end(lines: &Vec<&str>, command: &str) -> Result<String, LoadError> {
    let mut is_command = false;
    let mut current_command_string = String::new();
    let mut end_line_num = 0;
    for (line_num, line) in lines.iter().enumerate() {
        if line.contains("$end") && current_command_string.len() != 0 {
            is_command = false;
        }
        else if is_command {
            current_command_string = current_command_string + line;
        }
        else if line.contains(command) {
            if current_command_string.len() != 0 {
                return Err(LoadError {line: line_num, info: format!("Multiple {} commands is invalid", command) });
            }
            is_command = true;
        }
        end_line_num = line_num;
    }

    // $command with no $end is invalid
    if is_command {
        Err(LoadError {line: end_line_num, info: format!("{} missing an $end", command) })
    } else {
        Ok(current_command_string.trim().to_string())
    }
}
