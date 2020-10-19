pub mod vcd_parser;

use crate::error::LoadError;
use crate::types::{timescale::TimeScale, vcd_variable::VCDVariable};
use crate::vcd::VCD;
use vcd_parser::command_parser::CommandParser;

pub fn parse(s: String) -> Result<VCD, LoadError> {
    let vcd = VCD {
        date: get_date_from_lines(&s).unwrap(),
        version: get_version_from_lines(&s).unwrap(),
        timescale: get_timescale_from_lines(&s).unwrap(),
        comments: get_comments_from_lines(&s).unwrap(),
        variables: get_variables_from_lines(&s).unwrap(),
    };
    Ok(vcd)
}

fn get_date_from_lines(lines: &String) -> Result<String, LoadError> {
    let mut parsed_date = CommandParser::new()
        .lines(lines)
        .command(&String::from("$date"))
        .enforce_only_one_of_command(true)
        .parse()
        .unwrap();
    Ok(parsed_date.remove(0))
}

fn get_version_from_lines(lines: &String) -> Result<String, LoadError> {
    let mut parsed_version = CommandParser::new()
        .lines(lines)
        .command(&String::from("$version"))
        .enforce_only_one_of_command(true)
        .parse()
        .unwrap();
    Ok(parsed_version.remove(0))
}

fn get_timescale_from_lines(lines: &String) -> Result<TimeScale, LoadError> {
    let timescale_str = CommandParser::new()
        .lines(lines)
        .command(&String::from("$timescale"))
        .enforce_only_one_of_command(true)
        .parse()
        .unwrap()
        .remove(0);
    Ok(TimeScale::load_from_str(timescale_str))
}

fn get_comments_from_lines(lines: &String) -> Result<Vec<String>, LoadError> {
    CommandParser::new()
        .lines(lines)
        .command(&String::from("$comment"))
        .enforce_only_one_of_command(false)
        .parse()
}

fn get_variables_from_lines(lines: &String) -> Result<Vec<VCDVariable>, LoadError> {
    Ok(vec![])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::timescale::TimeUnit;

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

    // TODO: Add tests for scope and variable parsing
}
