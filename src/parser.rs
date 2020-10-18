use crate::error::LoadError;
use crate::types::TimeScale;
use crate::vcd::VCD;

pub fn parse(s: String) -> Result<VCD, LoadError> {
    let vcd = VCD {
        date: get_date_from_lines(&s).unwrap(),
        version: get_version_from_lines(&s).unwrap(),
        timescale: get_timescale_from_lines(&s).unwrap(),
        comments: get_comments_from_lines(&s).unwrap(),
    };
    Ok(vcd)
}

fn get_date_from_lines(lines: &String) -> Result<String, LoadError> {
    let mut parsed_date = CommandParser::new()
        .lines(lines)
        .command(&String::from("$date"))
        .enforce_only_one_of_command(true)
        .parse_command()
        .unwrap();
    Ok(parsed_date.remove(0))
}

fn get_version_from_lines(lines: &String) -> Result<String, LoadError> {
    let mut parsed_version = CommandParser::new()
        .lines(lines)
        .command(&String::from("$version"))
        .enforce_only_one_of_command(true)
        .parse_command()
        .unwrap();
    Ok(parsed_version.remove(0))
}

fn get_timescale_from_lines(lines: &String) -> Result<TimeScale, LoadError> {
    let timescale_str = CommandParser::new()
        .lines(lines)
        .command(&String::from("$timescale"))
        .enforce_only_one_of_command(true)
        .parse_command()
        .unwrap()
        .remove(0);
    Ok(TimeScale::load_from_str(timescale_str))
}

fn get_comments_from_lines(lines: &String) -> Result<Vec<String>, LoadError> {
    CommandParser::new()
        .lines(lines)
        .command(&String::from("$comment"))
        .enforce_only_one_of_command(false)
        .parse_command()
}

struct CommandParser {
    lines: String,
    command: String,
    enforce_only_one_of_command: bool,
}

impl CommandParser {
    pub fn new() -> CommandParser {
        CommandParser {
            lines: String::from(""),
            command: String::from(""),
            enforce_only_one_of_command: false,
        }
    }

    pub fn lines(&mut self, lines: &String) -> &mut CommandParser {
        self.lines = lines.clone(); // FIXME: Remove clone?
        self
    }

    pub fn command(&mut self, command_in: &String) -> &mut CommandParser {
        self.command = command_in.clone(); // FIXME: Remove clone?
        self
    }

    pub fn enforce_only_one_of_command(&mut self, enforcement: bool) -> &mut CommandParser {
        self.enforce_only_one_of_command = enforcement;
        self
    }

    fn parse_command(&self) -> Result<Vec<String>, LoadError> {
        let mut currently_parsing_command = false;
        let mut current_command_string = String::new();
        let mut command_vec = Vec::new();
        let mut line_num = 1;
        for line in self.lines.lines() {
            let words: Vec<_> = line.split(" ").filter(|c| !c.is_empty()).collect();
            for word in words {
                let word_wo_newlines = word.replace("\n", "");

                if self.is_different_command(&word_wo_newlines, &self.command)
                    && currently_parsing_command
                {
                    return Err(LoadError {
                        line: line_num,
                        info: format!("{} missing an $end", self.command),
                    });
                }

                if self.is_end(&word_wo_newlines) && current_command_string.len() != 0 {
                    currently_parsing_command = false;
                    command_vec.push(current_command_string.trim().to_string());
                    current_command_string = String::new();
                } else if currently_parsing_command {
                    current_command_string = current_command_string + " " + &word_wo_newlines[..];
                } else if self.is_command(&word_wo_newlines, &self.command) {
                    if command_vec.len() != 0 && self.enforce_only_one_of_command {
                        return Err(LoadError {
                            line: line_num,
                            info: format!("Multiple {} commands is invalid", self.command),
                        });
                    }
                    currently_parsing_command = true;
                }

                if self.is_end_of_line(word) {
                    line_num += 1;
                }
            }
        }

        // Not finding any command in string is invalid
        if command_vec.len() == 0 {
            command_vec.push(String::new());
        }

        match currently_parsing_command {
            true => Err(LoadError {
                line: line_num,
                info: format!("{} missing an $end", self.command),
            }),
            false => Ok(command_vec),
        }
    }

    fn is_different_command(&self, word: &String, command: &str) -> bool {
        word.starts_with("$") && word != command && word != "$end"
    }

    fn is_end(&self, word: &String) -> bool {
        word == "$end"
    }

    fn is_command(&self, word: &String, command: &String) -> bool {
        word == command
    }

    fn is_end_of_line(&self, word: &str) -> bool {
        word.contains("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::TimeUnit;

    #[test]
    fn date_command() {
        let contents = String::from("$date Date text $end");
        let vcd = parse(contents).unwrap();
        assert_eq!(vcd.date, "Date text".to_string());
    }

    #[test]
    fn date_command_newline() {
        let contents = String::from(
            r#"$date
    Date text
$end"#,
        );
        let vcd = parse(contents).unwrap();
        assert_eq!(vcd.date, "Date text".to_string());
    }

    #[test]
    #[should_panic(expected = "$date missing an $end")]
    fn date_command_with_no_end_throws_load_error() {
        let contents = String::from(
            r#"$date
Date text"#,
        );
        parse(contents).unwrap();
    }

    #[test]
    #[should_panic(expected = "$date missing an $end")]
    fn date_command_with_no_end_and_new_command_begins_throws_load_error() {
        let contents = String::from(
            r#"$date
    Date text
$version
    The version is 1.0
$end"#,
        );
        parse(contents).unwrap();
    }

    #[test]
    fn version_command_multiple_newlines() {
        let contents = String::from(
            r#"$version

The version number is 1.1

$end"#,
        );
        let vcd = parse(contents).unwrap();
        assert_eq!(vcd.version, "The version number is 1.1");
    }

    #[test]
    fn version_command() {
        let contents = String::from(r#"$version This version number is 2.0 $end"#);
        let vcd = parse(contents).unwrap();
        assert_eq!(vcd.version, "This version number is 2.0");
    }

    #[test]
    #[should_panic(expected = "$version missing an $end")]
    fn version_command_with_no_end_throws_load_error() {
        let contents = String::from(
            r#"$version
            This version has no end"#,
        );
        parse(contents).unwrap();
    }

    #[test]
    #[should_panic(expected = "Multiple $version commands is invalid")]
    fn vcd_file_with_multiple_versions_throws_error() {
        let contents = String::from(
            r#"$version
    Version 1.0
$end
$version
    Version 2.0. Which version is the right version?
$end"#,
        );
        parse(contents).unwrap();
    }

    #[test]
    #[should_panic(expected = "Multiple $date commands is invalid")]
    fn vcd_file_with_multiple_dates_throws_error() {
        let contents = String::from(
            r#"$date
    May 31st, 2020
$end
$date
    August 9th, 2020. Which is the correct date?
$end"#,
        );
        parse(contents).unwrap();
    }

    #[test]
    fn timescale_command() {
        let contents = String::from("$timescale 1ps $end");
        let vcd = parse(contents).unwrap();
        assert_eq!(
            vcd.timescale,
            TimeScale {
                value: 1,
                unit: TimeUnit::PS
            }
        );
    }

    #[test]
    fn comment_command_with_one_comment() {
        let contents = String::from("$comment this is a comment $end");
        let vcd = parse(contents).unwrap();
        assert_eq!(vcd.comments, vec!["this is a comment"]);
    }

    #[test]
    fn comment_command_with_multiple_comments() {
        let contents = String::from(
            r#"$comment
    This is comment 1
$end
$comment
    This is comment 2
$end"#,
        );
        let vcd = parse(contents).unwrap();
        assert_eq!(vcd.comments, vec!["This is comment 1", "This is comment 2"]);
    }

    #[test]
    #[should_panic(expected = "$comment missing an $end")]
    fn comment_command_with_no_end_throws_load_error() {
        let contents = String::from("$comment This comment is missing an end");
        parse(contents).unwrap();
    }
}
