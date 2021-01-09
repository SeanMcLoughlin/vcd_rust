#[macro_use]
extern crate derive_builder;
extern crate strum;
#[macro_use]
extern crate strum_macros;

use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::error::LoadError;
use crate::error::LoadError::{FileOpenError, FileReadError};
use crate::parser::parse;
use crate::state_machine::StateMachine;
use crate::vcd::VCD;

pub mod error;
pub mod parser;
pub mod state_machine;
pub mod string_helpers;
pub mod types;
pub mod vcd;

pub fn load_from_str(s: &str) -> Result<VCD, LoadError> {
    let mut state_machine = StateMachine::default();
    let mut line_num = 1;
    for line in s.lines() {
        parse(&mut state_machine, line.to_string(), line_num)?;
        line_num += 1;
    }
    line_num -= 1;
    state_machine.cleanup(line_num)?;
    Ok(state_machine.vcd)
}

pub fn load_from_file(filename: String) -> Result<VCD, LoadError> {
    match File::open(filename.as_str()) {
        Ok(file) => {
            let mut state_machine = StateMachine::default();
            let mut line_num = 1;
            for line in BufReader::new(file).lines() {
                match line {
                    Ok(line) => parse(&mut state_machine, line, line_num)?,
                    Err(_) => return Err(FileReadError { line: line_num }),
                };
                line_num += 1;
            }
            line_num -= 1;
            state_machine.cleanup(line_num)?;
            Ok(state_machine.vcd)
        }
        Err(e) => Err(FileOpenError {
            filename,
            error: e.to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::types::logical_value::LogicalValue;
    use crate::types::{
        scope::{Scope, ScopeType},
        timescale::{TimeScale, TimeUnit},
        variable::{VarType, Variable, VariableBuilder},
    };

    use super::*;

    fn get_scope_vec(scopes: Vec<(ScopeType, &str)>) -> Vec<Scope> {
        let mut scope_vec: Vec<Scope> = vec![];
        for (scope_type, id) in scopes.iter() {
            scope_vec.push(Scope::init(scope_type.clone(), id.to_string()));
        }
        scope_vec
    }

    fn get_var_hash_map(variables: Vec<Variable>) -> HashMap<String, Variable> {
        let mut var_hash_map = HashMap::<String, Variable>::new();
        for var in variables {
            var_hash_map.insert(var.ascii_identifier.clone(), var.clone());
        }
        var_hash_map
    }

    #[test]
    fn end_without_matching_command_throws_error() {
        let lines = r#"$end"#;
        let exp_err = LoadError::DanglingEnd { line: 1 };
        assert_eq!(load_from_str(lines).err(), Some(exp_err));
    }

    #[test]
    fn date_command() {
        let contents = "$date Date text $end";
        let vcd = load_from_str(contents).unwrap();
        assert_eq!(vcd.date, "Date text".to_string());
    }

    #[test]
    fn date_command_newline() {
        let contents = r#"$date
    Date text
$end"#;
        let vcd = load_from_str(contents).unwrap();
        assert_eq!(vcd.date, "Date text".to_string());
    }

    #[test]
    fn date_command_with_no_end_throws_load_error() {
        let contents = r#"$date
Date text"#;
        let err = load_from_str(contents).err();
        let exp_err = LoadError::MissingEnd {
            line: 2,
            command: "date".to_string(),
        };
        assert_eq!(err, Some(exp_err));
    }

    #[test]
    fn date_command_with_no_end_and_new_command_begins_throws_load_error() {
        let contents = r#"$date
    Date text
$version
    The version is 1.0
$end"#;
        let err = load_from_str(contents).err();
        let exp_err = LoadError::MissingEnd {
            line: 3,
            command: "date".to_string(),
        };
        assert_eq!(err, Some(exp_err));
    }

    #[test]
    fn version_command_multiple_newlines() {
        let contents = r#"$version

The version number is 1.1

$end"#;
        let vcd = load_from_str(contents).unwrap();
        assert_eq!(vcd.version, "The version number is 1.1");
    }

    #[test]
    fn version_command() {
        let contents = r#"$version This version number is 2.0 $end"#;
        let vcd = load_from_str(contents).unwrap();
        assert_eq!(vcd.version, "This version number is 2.0");
    }

    #[test]
    fn version_command_with_no_end_throws_load_error() {
        let contents = r#"$version
            This version has no end"#;
        let err = load_from_str(contents).err();
        let exp_err = LoadError::MissingEnd {
            line: 2,
            command: "version".to_string(),
        };
        assert_eq!(err, Some(exp_err));
    }

    #[test]
    fn vcd_file_with_multiple_versions_throws_error() {
        let contents = r#"$version
    Version 1.0
$end
$version
    Version 2.0. Which version is the right version?
$end"#;
        let err = load_from_str(contents).err();
        let exp_err = LoadError::InvalidMultipleCommand {
            line: 4,
            command: "version".to_string(),
        };
        assert_eq!(err, Some(exp_err));
    }

    #[test]
    fn vcd_file_with_multiple_dates_throws_error() {
        let contents = r#"$date
    May 31st, 2020
$end
$date
    August 9th, 2020. Which is the correct date?
$end"#;
        let err = load_from_str(contents).err();
        let exp_err = LoadError::InvalidMultipleCommand {
            line: 4,
            command: "date".to_string(),
        };
        assert_eq!(err, Some(exp_err));
    }

    #[test]
    fn timescale_command() {
        let contents = "$timescale 1 ps $end";
        let vcd = load_from_str(contents).unwrap();
        assert_eq!(vcd.timescale, TimeScale::new(1, TimeUnit::PS));
    }

    #[test]
    fn comment_command_with_one_comment() {
        let contents = "$comment this is a comment $end";
        let vcd = load_from_str(contents).unwrap();
        assert_eq!(vcd.comments, vec!["this is a comment"]);
    }

    #[test]
    fn comment_command_with_multiple_comments() {
        let contents = r#"$comment
    This is comment 1
$end
$comment
    This is comment 2
$end"#;
        let vcd = load_from_str(contents).unwrap();
        assert_eq!(vcd.comments, vec!["This is comment 1", "This is comment 2"]);
    }

    #[test]
    fn comment_command_with_no_end_throws_load_error() {
        let contents = "$comment This comment is missing an end";
        let err = load_from_str(contents).err();
        let exp_err = LoadError::MissingEnd {
            line: 1,
            command: "comment".to_string(),
        };
        assert_eq!(err, Some(exp_err));
    }

    #[test]
    fn parse_one_lvl1_scope_with_one_var() {
        let lines = r#"$scope module lvl_1 $end
$var wire 8 # data $end"#;
        let exp_var: Variable = VariableBuilder::default()
            .scope(get_scope_vec(vec![(ScopeType::Module, "lvl_1")]))
            .var_type(VarType::Wire)
            .bit_width(8)
            .ascii_identifier("#".to_string())
            .reference("data".to_string())
            .build()
            .unwrap();
        let exp_vars = get_var_hash_map(vec![exp_var]);
        let act_vars = load_from_str(lines).unwrap().variables;
        assert_eq!(exp_vars, act_vars);
    }

    #[test]
    fn parse_two_lvl1_scopes_each_with_one_var() {
        let lines = r#"$scope module lvl_1_one $end
$var wire 8 # data $end
$upscope $end
$scope module lvl_1_two $end
$var integer 2 & num $end"#;
        let exp_vars = get_var_hash_map(vec![
            VariableBuilder::default()
                .scope(get_scope_vec(vec![(ScopeType::Module, "lvl_1_one")]))
                .var_type(VarType::Wire)
                .bit_width(8)
                .ascii_identifier("#".to_string())
                .reference("data".to_string())
                .build()
                .unwrap(),
            VariableBuilder::default()
                .scope(get_scope_vec(vec![(ScopeType::Module, "lvl_1_two")]))
                .var_type(VarType::Integer)
                .bit_width(2)
                .ascii_identifier("&".to_string())
                .reference("num".to_string())
                .build()
                .unwrap(),
        ]);
        let act_vars = load_from_str(lines).unwrap().variables;
        assert_eq!(exp_vars, act_vars);
    }

    #[test]
    fn parse_one_lvl2_scope_with_one_var() {
        let lines = r#"$scope module lvl_1 $end
$scope task lvl_2 $end
$var reg 3 ' my_name $end"#;
        let scope_vec = get_scope_vec(vec![
            (ScopeType::Module, "lvl_1"),
            (ScopeType::Task, "lvl_2"),
        ]);
        let exp_var: Variable = VariableBuilder::default()
            .scope(scope_vec)
            .var_type(VarType::Reg)
            .bit_width(3)
            .ascii_identifier("'".to_string())
            .reference("my_name".to_string())
            .build()
            .unwrap();
        let exp_vars = get_var_hash_map(vec![exp_var]);
        let act_vars = load_from_str(lines).unwrap().variables;
        assert_eq!(exp_vars, act_vars);
    }

    #[test]
    fn parse_one_lvl2_scope_with_two_vars() {
        let lines = r#"$scope fork lvl_1 $end
$scope begin lvl_2 $end
$var event 2 { my_event $end
$var tri 1 } my_tri $end"#;
        let scope_vec = get_scope_vec(vec![
            (ScopeType::Fork, "lvl_1"),
            (ScopeType::Begin, "lvl_2"),
        ]);
        let exp_vars = get_var_hash_map(vec![
            VariableBuilder::default()
                .scope(scope_vec.clone())
                .var_type(VarType::Event)
                .bit_width(2)
                .ascii_identifier("{".to_string())
                .reference("my_event".to_string())
                .build()
                .unwrap(),
            VariableBuilder::default()
                .scope(scope_vec)
                .var_type(VarType::Tri)
                .bit_width(1)
                .ascii_identifier("}".to_string())
                .reference("my_tri".to_string())
                .build()
                .unwrap(),
        ]);
        let act_vars = load_from_str(lines).unwrap().variables;
        assert_eq!(exp_vars, act_vars);
    }

    #[test]
    fn parse_one_lvl1_scope_with_one_var_with_var_parameters_on_newlines() {
        let lines = r#"$scope task lvl_1 $end
$var
event
2
p
my_ref
$end"#;
        let exp_var: Variable = VariableBuilder::default()
            .scope(get_scope_vec(vec![(ScopeType::Task, "lvl_1")]))
            .var_type(VarType::Event)
            .bit_width(2)
            .ascii_identifier("p".to_string())
            .reference("my_ref".to_string())
            .build()
            .unwrap();
        let exp_vars = get_var_hash_map(vec![exp_var]);
        let act_vars = load_from_str(lines).unwrap().variables;
        assert_eq!(exp_vars, act_vars);
    }

    #[test]
    fn parse_one_lvl1_scope_with_scope_parameters_on_newlines() {
        let lines = r#"$scope
module
name
$end
$var wire 8 # data $end"#;
        let exp_var: Variable = VariableBuilder::default()
            .scope(get_scope_vec(vec![(ScopeType::Module, "name")]))
            .var_type(VarType::Wire)
            .bit_width(8)
            .ascii_identifier("#".to_string())
            .reference("data".to_string())
            .build()
            .unwrap();
        let exp_vars = get_var_hash_map(vec![exp_var]);
        let act_vars = load_from_str(lines).unwrap().variables;
        assert_eq!(exp_vars, act_vars);
    }

    #[test]
    fn single_pre_simulation_dumped_var_can_be_parsed() {
        let lines = r#"$timescale 1 ps $end
        $scope module top_mod $end
        $var wire 1 * my_bit $end
        $enddefinitions $end
        $dumpvars
        0*
        $end"#;
        let mut exp_var: Variable = VariableBuilder::default()
            .scope(get_scope_vec(vec![(ScopeType::Module, "top_mod")]))
            .var_type(VarType::Wire)
            .bit_width(1)
            .ascii_identifier("*".to_string())
            .reference("my_bit".to_string())
            .build()
            .unwrap();
        exp_var.transitions.insert(-1, LogicalValue::Zero);
        let exp_vars = get_var_hash_map(vec![exp_var]);
        let act_vars = load_from_str(lines).unwrap().variables;
        assert_eq!(exp_vars, act_vars);
    }

    #[test]
    fn var_missing_end_same_line_throws_error() {
        let lines = r#"$scope module name $end
$var event 2 e my_var"#;

        let exp_err = LoadError::MissingEnd {
            command: "var".to_string(),
            line: 2,
        };

        assert_eq!(load_from_str(lines).err(), Some(exp_err));
    }

    #[test]
    fn var_missing_end_different_line_throws_error() {
        let lines = r#"$scope module name $end
$var
event
2
e
my_var"#;

        let exp_err = LoadError::MissingEnd {
            command: "var".to_string(),
            line: 6,
        };
        assert_eq!(load_from_str(lines).err(), Some(exp_err));
    }

    #[test]
    fn var_missing_end_middle_of_file_throws_error() {
        let lines = r#"$scope module name $end
$var event 2 e my_var
$upscope $end"#;

        let exp_err = LoadError::MissingEnd {
            command: "var".to_string(),
            line: 3,
        };

        assert_eq!(load_from_str(lines).err(), Some(exp_err));
    }

    #[test]
    fn scope_missing_end_same_line_throws_error() {
        let lines = r#"$scope module name"#;
        let exp_err = LoadError::MissingEnd {
            command: "scope".to_string(),
            line: 1,
        };
        assert_eq!(load_from_str(lines).err(), Some(exp_err));
    }

    #[test]
    fn scope_missing_end_different_line_throws_error() {
        let lines = r#"$scope
module
name"#;
        let exp_err = LoadError::MissingEnd {
            command: "scope".to_string(),
            line: 3,
        };
        assert_eq!(load_from_str(lines).err(), Some(exp_err));
    }

    #[test]
    fn scope_missing_end_middle_of_file_throws_error() {
        let lines = r#"$scope
module
name
$var integer 8 a my_var $end"#;
        let exp_err = LoadError::MissingEnd {
            command: "scope".to_string(),
            line: 4,
        };
        assert_eq!(load_from_str(lines).err(), Some(exp_err));
    }

    #[test]
    fn upscope_missing_end_same_line_throws_error() {
        let lines = r#"$upscope"#;
        let exp_err = LoadError::MissingEnd {
            command: "upscope".to_string(),
            line: 1,
        };
        assert_eq!(load_from_str(lines).err(), Some(exp_err));
    }

    #[test]
    fn upscope_missing_end_middle_of_file_throws_error() {
        let lines = r#"$scope module name $end
$upscope
$scope module other_name $end"#;
        let exp_err = LoadError::MissingEnd {
            command: "upscope".to_string(),
            line: 3,
        };
        assert_eq!(load_from_str(lines).err(), Some(exp_err));
    }

    #[test]
    fn var_with_too_few_params_throws_error() {
        let lines = r#"$scope module lvl_1 $end
$var wire 8 # $end"#;
        let exp_err = LoadError::TooFewParameters {
            command: "var".to_string(),
            line: 2,
        };
        assert_eq!(load_from_str(lines).err(), Some(exp_err));
    }

    #[test]
    fn var_declared_with_empty_hierarchy_throws_error() {
        let lines = r#"$var wire 8 # data $end"#;
        let exp_err = LoadError::ScopeStackEmpty {
            command: "var".to_string(),
            line: 1,
        };
        assert_eq!(load_from_str(lines).err(), Some(exp_err));
    }

    #[test]
    fn var_with_too_many_parameters_throws_error() {
        let lines = r#"$scope module lvl_1 $end
$var wire 8 # data BAD_PARAM $end"#;
        let exp_err = LoadError::TooManyParameters {
            command: "var".to_string(),
            line: 2,
        };
        assert_eq!(load_from_str(lines).err(), Some(exp_err));
    }

    #[test]
    fn upscope_with_empty_hierarchy_throws_error() {
        let lines = r#"$upscope $end"#;
        let exp_err = LoadError::ScopeStackEmpty {
            line: 1,
            command: "upscope".to_string(),
        };
        assert_eq!(load_from_str(lines).err(), Some(exp_err));
    }

    #[test]
    fn upscope_with_too_many_parameters_throws_error() {
        let lines = r#"$upscope parameter $end"#;
        let exp_err = LoadError::InvalidParameterForCommand {
            parameter: "parameter".to_string(),
            command: "upscope".to_string(),
            line: 1,
        };
        assert_eq!(load_from_str(lines).err(), Some(exp_err));
    }

    #[test]
    fn dump_commands_without_enddefinitions_throws_error() {
        let mut lines: &str;
        let mut exp_err: LoadError;

        lines = r#"$dumpvars $end"#;
        exp_err = LoadError::DumpWithoutEnddefinitions { line: 1 };
        assert_eq!(load_from_str(lines).err(), Some(exp_err));

        lines = r#"$dumpall $end"#;
        exp_err = LoadError::DumpWithoutEnddefinitions { line: 1 };
        assert_eq!(load_from_str(lines).err(), Some(exp_err));

        lines = r#"$dumpon $end"#;
        exp_err = LoadError::DumpWithoutEnddefinitions { line: 1 };
        assert_eq!(load_from_str(lines).err(), Some(exp_err));
    }
}
