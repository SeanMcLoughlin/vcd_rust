use crate::error::LoadError;
use crate::state_machine::StateMachine;
use crate::vcd::VCD;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub struct Parser {
    state_machine: StateMachine,
}

impl Default for Parser {
    fn default() -> Self {
        Parser::new()
    }
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            state_machine: StateMachine::new(),
        }
    }

    pub fn parse_from_string(&mut self, s: &str) -> Result<VCD, LoadError> {
        let mut line_num = 1;
        for line in s.lines() {
            self.parse(line.to_string(), line_num)?;
            line_num += 1;
        }
        line_num -= 1;
        self.state_machine.cleanup(line_num)?;
        Ok(self.state_machine.vcd.clone()) // TODO: Refactor to use take() to prevent clone
    }

    pub fn parse_from_file(&mut self, file: File) -> Result<VCD, LoadError> {
        let mut line_num = 1;
        for line in BufReader::new(file).lines() {
            match line {
                Ok(line) => self.parse(line, line_num)?,
                Err(_) => panic!("Failed reading file"), // FIXME cleanup
            };
            line_num += 1;
        }
        line_num -= 1;
        self.state_machine.cleanup(line_num)?;
        Ok(self.state_machine.vcd.clone()) // TODO: Refactor to use take() to prevent clone
    }

    fn parse(&mut self, line: String, line_num: usize) -> Result<(), LoadError> {
        let words: Vec<_> = line.split(' ').filter(|c| !c.is_empty()).collect();
        for word in words {
            self.state_machine.parse_word(word, line_num)?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        scope::{Scope, ScopeType},
        timescale::{TimeScale, TimeUnit},
        variable::{VarType, Variable, VariableBuilder},
    };
    use std::collections::HashMap;

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
        assert_eq!(Parser::new().parse_from_string(lines).err(), Some(exp_err));
    }

    #[test]
    fn date_command() {
        let contents = "$date Date text $end";
        let vcd = Parser::new().parse_from_string(contents).unwrap();
        assert_eq!(vcd.date, "Date text".to_string());
    }

    #[test]
    fn date_command_newline() {
        let contents = r#"$date
    Date text
$end"#;
        let vcd = Parser::new().parse_from_string(contents).unwrap();
        assert_eq!(vcd.date, "Date text".to_string());
    }

    #[test]
    fn date_command_with_no_end_throws_load_error() {
        let contents = r#"$date
Date text"#;
        let err = Parser::new().parse_from_string(contents).err();
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
        let err = Parser::new().parse_from_string(contents).err();
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
        let vcd = Parser::new().parse_from_string(contents).unwrap();
        assert_eq!(vcd.version, "The version number is 1.1");
    }

    #[test]
    fn version_command() {
        let contents = r#"$version This version number is 2.0 $end"#;
        let vcd = Parser::new().parse_from_string(contents).unwrap();
        assert_eq!(vcd.version, "This version number is 2.0");
    }

    #[test]
    fn version_command_with_no_end_throws_load_error() {
        let contents = r#"$version
            This version has no end"#;
        let err = Parser::new().parse_from_string(contents).err();
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
        let err = Parser::new().parse_from_string(contents).err();
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
        let err = Parser::new().parse_from_string(contents).err();
        let exp_err = LoadError::InvalidMultipleCommand {
            line: 4,
            command: "date".to_string(),
        };
        assert_eq!(err, Some(exp_err));
    }

    #[test]
    fn timescale_command() {
        let contents = "$timescale 1 ps $end";
        let vcd = Parser::new().parse_from_string(contents).unwrap();
        assert_eq!(vcd.timescale, TimeScale::init(1, TimeUnit::PS));
    }

    #[test]
    fn comment_command_with_one_comment() {
        let contents = "$comment this is a comment $end";
        let vcd = Parser::new().parse_from_string(contents).unwrap();
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
        let vcd = Parser::new().parse_from_string(contents).unwrap();
        assert_eq!(vcd.comments, vec!["This is comment 1", "This is comment 2"]);
    }

    #[test]
    fn comment_command_with_no_end_throws_load_error() {
        let contents = "$comment This comment is missing an end";
        let err = Parser::new().parse_from_string(contents).err();
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
        let act_vars = Parser::new().parse_from_string(lines).unwrap().variables;
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
        let act_vars = Parser::new().parse_from_string(lines).unwrap().variables;
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
        let act_vars = Parser::new().parse_from_string(lines).unwrap().variables;
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
        let act_vars = Parser::new().parse_from_string(lines).unwrap().variables;
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
        let act_vars = Parser::new().parse_from_string(lines).unwrap().variables;
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
        let act_vars = Parser::new().parse_from_string(lines).unwrap().variables;
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

        assert_eq!(Parser::new().parse_from_string(lines).err(), Some(exp_err));
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
        assert_eq!(Parser::new().parse_from_string(lines).err(), Some(exp_err));
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

        assert_eq!(Parser::new().parse_from_string(lines).err(), Some(exp_err));
    }

    #[test]
    fn scope_missing_end_same_line_throws_error() {
        let lines = r#"$scope module name"#;
        let exp_err = LoadError::MissingEnd {
            command: "scope".to_string(),
            line: 1,
        };
        assert_eq!(Parser::new().parse_from_string(lines).err(), Some(exp_err));
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
        assert_eq!(Parser::new().parse_from_string(lines).err(), Some(exp_err));
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
        assert_eq!(Parser::new().parse_from_string(lines).err(), Some(exp_err));
    }

    #[test]
    fn upscope_missing_end_same_line_throws_error() {
        let lines = r#"$upscope"#;
        let exp_err = LoadError::MissingEnd {
            command: "upscope".to_string(),
            line: 1,
        };
        assert_eq!(Parser::new().parse_from_string(lines).err(), Some(exp_err));
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
        assert_eq!(Parser::new().parse_from_string(lines).err(), Some(exp_err));
    }

    #[test]
    fn var_with_too_few_params_throws_error() {
        let lines = r#"$scope module lvl_1 $end
$var wire 8 # $end"#;
        let exp_err = LoadError::TooFewParameters {
            command: "var".to_string(),
            line: 2,
        };
        assert_eq!(Parser::new().parse_from_string(lines).err(), Some(exp_err));
    }

    #[test]
    fn var_declared_with_empty_hierarchy_throws_error() {
        let lines = r#"$var wire 8 # data $end"#;
        let exp_err = LoadError::ScopeStackEmpty {
            command: "var".to_string(),
            line: 1,
        };
        assert_eq!(Parser::new().parse_from_string(lines).err(), Some(exp_err));
    }

    #[test]
    fn var_with_too_many_parameters_throws_error() {
        let lines = r#"$scope module lvl_1 $end
$var wire 8 # data BAD_PARAM $end"#;
        let exp_err = LoadError::TooManyParameters {
            command: "var".to_string(),
            line: 2,
        };
        assert_eq!(Parser::new().parse_from_string(lines).err(), Some(exp_err));
    }

    #[test]
    fn upscope_with_empty_hierarchy_throws_error() {
        let lines = r#"$upscope $end"#;
        let exp_err = LoadError::ScopeStackEmpty {
            line: 1,
            command: "upscope".to_string(),
        };
        assert_eq!(Parser::new().parse_from_string(lines).err(), Some(exp_err));
    }

    #[test]
    fn upscope_with_too_many_parameters_throws_error() {
        let lines = r#"$upscope parameter $end"#;
        let exp_err = LoadError::InvalidParameterForCommand {
            parameter: "parameter".to_string(),
            command: "upscope".to_string(),
            line: 1,
        };
        assert_eq!(Parser::new().parse_from_string(lines).err(), Some(exp_err));
    }

    #[test]
    fn enddefinitions_throws_no_error() {
        let lines = r#"$enddefinitions $end"#;
        assert!(Parser::new().parse_from_string(lines).is_ok());
    }

    #[test]
    fn enddefinitions_with_newlines_throws_no_error() {
        let lines = r#"$enddefinitions 
$end"#;
        assert!(Parser::new().parse_from_string(lines).is_ok());
    }

    #[test]
    fn enddefinitions_missing_end_throws_error() {
        let lines = r#"$comment my_comment $end
$enddefinitions"#;
        let exp_err = LoadError::MissingEnd {
            command: "enddefinitions".to_string(),
            line: 2,
        };
        assert_eq!(Parser::new().parse_from_string(lines).err(), Some(exp_err));
    }

    #[test]
    fn enddefinitions_with_too_many_parameters_throws_error() {
        let lines = r#"$comment Go to line 2 $end
$comment Go to line 3 $end
$enddefinitions INVALID_PARAMETER $end"#;
        let exp_err = LoadError::InvalidParameterForCommand {
            parameter: "INVALID_PARAMETER".to_string(),
            command: "enddefinitions".to_string(),
            line: 3,
        };
        assert_eq!(Parser::new().parse_from_string(lines).err(), Some(exp_err));
    }

    #[test]
    fn dumpvars_scalar() {
        let lines = r#"$scope module top $end
$var wire 1 { data1 $end
$var wire 1 } data2 $end
$upscope $end
$enddefinitions $end
$dumpvars
0{
1}
$end"#;
        let vars = Parser::new().parse_from_string(lines).unwrap().variables;
        assert_eq!(vars["{"].events, vec![(0, 0)].into_iter().collect());
        assert_eq!(vars["}"].events, vec![(0, 1)].into_iter().collect());
    }

    #[test]
    #[ignore]
    fn dumpvars_invalid_identifier_throws_error() {}

    #[test]
    #[ignore]
    fn dumpvars_bit_width_too_large_throws_error() {}

    #[test]
    #[ignore]
    fn dumpvars_missing_end_throws_error() {}

    #[test]
    #[ignore]
    fn dumpall_1() {}

    #[test]
    #[ignore]
    fn dumpall_missing_end_throws_error() {}

    #[test]
    #[ignore]
    fn dumpall_not_including_all_var_declarations_throws_error() {}

    #[test]
    #[ignore]
    fn dumpoff_1() {}

    #[test]
    #[ignore]
    fn dumpoff_missing_end_throws_error() {}

    #[test]
    #[ignore]
    fn dumpoff_not_including_all_var_declarations_throws_error() {}

    #[test]
    #[ignore]
    fn dumpoff_not_setting_vars_to_x_throws_error() {}

    #[test]
    #[ignore]
    fn dumpon_1() {}

    #[test]
    #[ignore]
    fn dumpon_missing_end_throws_error() {}

    #[test]
    #[ignore]
    fn dumpon_not_including_all_var_declarations_throws_error() {}
}
