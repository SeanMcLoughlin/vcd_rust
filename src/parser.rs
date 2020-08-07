use crate::error::LoadError;
use crate::vcd::VCD;
use crate::types::{TimeScale};

pub fn parse(s: &str) -> Result<VCD, LoadError> {
    let vcd = VCD {
        date: get_date_from_lines(s).unwrap(),
        version: get_version_from_lines(s).unwrap(),
        timescale: get_timescale_from_lines(s).unwrap(),
    };
    Ok(vcd)
}

fn get_date_from_lines(lines: &str) -> Result<String, LoadError> {
    get_lines_between_command_and_end(lines, "$date")
}

fn get_version_from_lines(lines: &str) -> Result<String, LoadError> {
    get_lines_between_command_and_end(lines, "$version")
}

fn get_timescale_from_lines(lines: &str) -> Result<TimeScale, LoadError> {
    let timescale_str = get_lines_between_command_and_end(lines, "$timescale")?;
    Ok(TimeScale::load_from_str(timescale_str))
}

fn get_lines_between_command_and_end(lines: &str, command: &str) -> Result<String, LoadError> {
    let mut currently_parsing_command = false;
    let mut current_command_string = String::new();
    let mut line_num = 1;
    let words: Vec<_> = lines.split(" ").filter(|c| !c.is_empty()).collect();
    for word in words {

        let word_wo_newlines = word.replace("\n", "");

        if is_different_command(&word_wo_newlines, command) && currently_parsing_command {
            return Err(LoadError {line: line_num, info: format!("{} missing an $end", command) });
        }

        if is_end(&word_wo_newlines) && current_command_string.len() != 0 {
            currently_parsing_command = false;
        }
        else if currently_parsing_command {
            current_command_string = current_command_string + " " + &word_wo_newlines[..];
        }
        else if is_command(&word_wo_newlines, command) {
            if current_command_string.len() != 0 {
                return Err(LoadError {line: line_num, info: format!("Multiple {} commands is invalid", command) });
            }
            currently_parsing_command = true;
        }

        if is_end_of_line(word) {
            line_num += 1;
        }
    }

    if currently_parsing_command {
        Err(LoadError {line: line_num, info: format!("{} missing an $end", command) })
    } else {
        Ok(current_command_string.trim().to_string())
    }
}

fn is_different_command(word: &String, command: &str) -> bool {
    word.starts_with("$") && word != command && word != "$end"
}

fn is_end(word: &String) -> bool {
    word == "$end"
}

fn is_command(word: &String, command: &str) -> bool {
    word == command
}

fn is_end_of_line(word: &str) -> bool {
    word.contains("\n")
}
