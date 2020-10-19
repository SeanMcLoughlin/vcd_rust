use crate::error::LoadError;
use crate::parser::vcd_parser::VCDParser;
use crate::types::vcd_variable::{VCDScope, VCDVariable};

pub struct DefinitionsParser {
    lines: String,
}

impl VCDParser for DefinitionsParser {
    fn parse(&self) -> Result<Vec<VCDVariable>, LoadError> {
        let mut current_var = VCDVariable::new();
        let mut current_scope = VCDScope::new();
        let mut scope_stack = Vec::<VCDScope>::new();

        let mut defining_var = false;
        let mut downscope = false;
        let mut upscope = false;

        let mut variable_list = Vec::<VCDVariable>::new();

        for line in self.lines.lines() {
            for word in DefinitionsParser::split_line_into_words(line) {
                let word_wo_newlines = DefinitionsParser::remove_newlines(word);
                match word_wo_newlines.as_str() {
                    "$scope" => {
                        downscope = true;
                        continue;
                    }
                    "$upscope" => {
                        upscope = true;
                        continue;
                    }
                    "$var" => {
                        defining_var = true;
                        continue;
                    }
                    "$end" => {
                        if downscope {
                            scope_stack.push(current_scope);
                            current_scope = VCDScope::new();
                        } else if upscope {
                            scope_stack.pop();
                        } else if defining_var {
                            current_var.scope = scope_stack.clone();
                            variable_list.push(current_var);
                            current_var = VCDVariable::new();
                        }

                        downscope = false;
                        upscope = false;
                        defining_var = false;
                        continue;
                    }
                    _ => {}
                }
                if downscope {
                    current_scope.append_value(word);
                } else if defining_var {
                    current_var.append_value(word);
                }
            }
        }
        Ok(variable_list)
    }
}

impl DefinitionsParser {
    pub fn new() -> DefinitionsParser {
        DefinitionsParser {
            lines: "".to_string(),
        }
    }

    pub fn lines(&mut self, lines: &String) -> &mut DefinitionsParser {
        self.lines = lines.clone(); // NICE TO HAVE: Remove clone()?
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::vcd_variable::{
        VCDDataType, VCDScopeBuilder, VCDScopeType, VCDVariableBuilder,
    };

    fn get_scope_vec(scopes: Vec<(VCDScopeType, &str)>) -> Vec<VCDScope> {
        let mut scope_vec: Vec<VCDScope> = vec![];
        for (scope_type, id) in scopes.iter() {
            scope_vec.push(
                VCDScopeBuilder::default()
                    .scope_type(scope_type.clone())
                    .id(id.to_string())
                    .build()
                    .unwrap(),
            );
        }
        return scope_vec;
    }

    #[test]
    fn parse_one_lvl1_scope_with_one_var() {
        let lines = String::from(
            r#"$scope module lvl_1 $end
$var wire 8 # data $end"#,
        );
        let exp_var: VCDVariable = VCDVariableBuilder::default()
            .scope(get_scope_vec(vec![(VCDScopeType::Module, "lvl_1")]))
            .data_type(VCDDataType::Wire)
            .bit_width(8)
            .ascii_identifier("#".to_string())
            .name("data".to_string())
            .build()
            .unwrap();
        let exp_vars = vec![exp_var];
        let act_vars = DefinitionsParser::new().lines(&lines).parse().unwrap();
        assert_eq!(exp_vars, act_vars);
    }

    #[test]
    fn parse_two_lvl1_scopes_each_with_one_var() {
        let lines = String::from(
            r#"$scope module lvl_1_one $end
$var wire 8 # data $end
$upscope $end
$scope module lvl_1_two $end
$var integer 2 & num $end"#,
        );
        let exp_vars = vec![
            VCDVariableBuilder::default()
                .scope(get_scope_vec(vec![(VCDScopeType::Module, "lvl_1_one")]))
                .data_type(VCDDataType::Wire)
                .bit_width(8)
                .ascii_identifier("#".to_string())
                .name("data".to_string())
                .build()
                .unwrap(),
            VCDVariableBuilder::default()
                .scope(get_scope_vec(vec![(VCDScopeType::Module, "lvl_1_two")]))
                .data_type(VCDDataType::Integer)
                .bit_width(2)
                .ascii_identifier("&".to_string())
                .name("num".to_string())
                .build()
                .unwrap(),
        ];
        let act_vars = DefinitionsParser::new().lines(&lines).parse().unwrap();
        assert_eq!(exp_vars, act_vars);
    }

    #[test]
    fn parse_one_lvl2_scope_with_one_var() {
        let lines = String::from(
            r#"$scope module lvl_1 $end
$scope task lvl_2 $end
$var reg 3 ' my_name $end"#,
        );
        let scope_vec = get_scope_vec(vec![
            (VCDScopeType::Module, "lvl_1"),
            (VCDScopeType::Task, "lvl_2"),
        ]);
        let exp_var: VCDVariable = VCDVariableBuilder::default()
            .scope(scope_vec)
            .data_type(VCDDataType::Reg)
            .bit_width(3)
            .ascii_identifier("'".to_string())
            .name("my_name".to_string())
            .build()
            .unwrap();
        let exp_vars = vec![exp_var];
        let act_vars = DefinitionsParser::new().lines(&lines).parse().unwrap();
        assert_eq!(exp_vars, act_vars);
    }

    #[test]
    fn parse_one_lvl2_scope_with_two_vars() {
        let lines = String::from(
            r#"$scope fork lvl_1 $end
$scope begin lvl_2 $end
$var event 2 { my_event $end
$var tri 1 } my_tri $end"#,
        );
        let scope_vec = get_scope_vec(vec![
            (VCDScopeType::Fork, "lvl_1"),
            (VCDScopeType::Begin, "lvl_2"),
        ]);
        let exp_vars = vec![
            VCDVariableBuilder::default()
                .scope(scope_vec.clone())
                .data_type(VCDDataType::Event)
                .bit_width(2)
                .ascii_identifier("{".to_string())
                .name("my_event".to_string())
                .build()
                .unwrap(),
            VCDVariableBuilder::default()
                .scope(scope_vec.clone())
                .data_type(VCDDataType::Tri)
                .bit_width(1)
                .ascii_identifier("}".to_string())
                .name("my_tri".to_string())
                .build()
                .unwrap(),
        ];
        let act_vars = DefinitionsParser::new().lines(&lines).parse().unwrap();
        assert_eq!(exp_vars, act_vars);
    }
}
