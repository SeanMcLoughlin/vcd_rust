use crate::error::LoadError;
use crate::types::scope::Scope;
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, Clone, Eq, PartialEq, EnumString)]
pub enum VarType {
    #[strum(serialize = "event")]
    Event,
    #[strum(serialize = "integer")]
    Integer,
    #[strum(serialize = "parameter")]
    Parameter,
    #[strum(serialize = "real")]
    Real,
    #[strum(serialize = "reg")]
    Reg,
    #[strum(serialize = "supply0")]
    Supply0,
    #[strum(serialize = "supply1")]
    Supply1,
    #[strum(serialize = "time")]
    Time,
    #[strum(serialize = "tri")]
    Tri,
    #[strum(serialize = "triand")]
    TriAnd,
    #[strum(serialize = "trior")]
    TriOr,
    #[strum(serialize = "trireg")]
    TriReg,
    #[strum(serialize = "tri0")]
    Tri0,
    #[strum(serialize = "tri1")]
    Tri1,
    #[strum(serialize = "wand")]
    WAnd,
    #[strum(serialize = "wire")]
    Wire,
    #[strum(serialize = "wor")]
    WOr,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum BuildState {
    VarType,
    Size,
    Identifier,
    Reference,
    Done,
}

impl BuildState {
    fn next(&self, line_num: usize) -> Result<Self, LoadError> {
        use BuildState::*;
        match *self {
            VarType => Ok(Size),
            Size => Ok(Identifier),
            Identifier => Ok(Reference),
            Reference => Ok(Done),
            Done => Err(LoadError::TooManyParameters {
                line: line_num,
                command: "var".to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone, Builder)]
pub struct Variable {
    pub scope: Vec<Scope>,
    pub var_type: VarType,
    pub bit_width: usize,
    pub ascii_identifier: String,
    pub reference: String,

    #[builder(default = "BuildState::VarType", setter(skip))]
    state: BuildState,
}

impl Variable {
    pub fn new() -> Variable {
        Variable {
            scope: vec![],
            var_type: VarType::Event,
            bit_width: 0,
            ascii_identifier: "".to_string(),
            reference: "".to_string(),
            state: BuildState::VarType,
        }
    }

    pub fn append(&mut self, word: &str, line_num: usize) -> Result<(), LoadError> {
        match self.state {
            BuildState::VarType => self.write_var_type(word, line_num)?,
            BuildState::Size => self.write_bit_width(word, line_num)?,
            BuildState::Identifier => self.ascii_identifier = word.to_string(),
            BuildState::Reference => self.reference = word.to_string(),
            _ => {}
        }
        self.state = self.state.next(line_num)?;
        Ok(())
    }

    pub fn is_done(&self) -> bool {
        self.state == BuildState::Done
    }

    fn write_var_type(&mut self, word: &str, line_num: usize) -> Result<(), LoadError> {
        self.var_type = match VarType::from_str(word) {
            Ok(var_type) => var_type,
            Err(_) => {
                return Err(LoadError::InvalidParameterForCommand {
                    line: line_num,
                    command: "$var".to_string(),
                    parameter: word.to_string(),
                })
            }
        };
        Ok(())
    }

    fn write_bit_width(&mut self, word: &str, line_num: usize) -> Result<(), LoadError> {
        self.bit_width = match word.parse::<usize>() {
            Ok(bit_width) => bit_width,
            Err(_) => {
                return Err(LoadError::InvalidParameterForCommand {
                    line: line_num,
                    command: "$var".to_string(),
                    parameter: word.to_string(),
                })
            }
        };
        Ok(())
    }
}

impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        return self.scope == other.scope
            && self.var_type == other.var_type
            && self.bit_width == other.bit_width
            && self.ascii_identifier == other.ascii_identifier
            && self.reference == other.reference;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_variable_1() {
        let exp_var = VariableBuilder::default()
            .scope(vec![])
            .var_type(VarType::Wire)
            .bit_width(8)
            .ascii_identifier("#".to_string())
            .reference("data".to_string())
            .build()
            .unwrap();
        let mut act_var = Variable::new();
        for word in vec!["wire", "8", "#", "data"] {
            act_var.append(word, 0).unwrap();
        }
        assert_eq!(exp_var, act_var);
        assert!(act_var.is_done());
    }

    #[test]
    fn build_variable_2() {
        let exp_var = VariableBuilder::default()
            .scope(vec![])
            .var_type(VarType::TriReg)
            .bit_width(4)
            .ascii_identifier("e".to_string())
            .reference("my_reference".to_string())
            .build()
            .unwrap();

        let mut act_var = Variable::new();
        for word in vec!["trireg", "4", "e", "my_reference"] {
            act_var.append(word, 0).unwrap();
        }
        assert_eq!(exp_var, act_var);
        assert!(act_var.is_done());
    }

    #[test]
    fn invalid_var_type_throws_error() {
        let mut act_var = Variable::new();
        let err = act_var.append("NotAVarType", 0).err();
        let exp_err = LoadError::InvalidParameterForCommand {
            line: 0,
            command: "$var".to_string(),
            parameter: "NotAVarType".to_string(),
        };
        assert_eq!(err, Some(exp_err))
    }

    #[test]
    fn non_digit_bit_width_throws_error() {
        let mut act_var = Variable::new();
        act_var.append("wire", 0).unwrap();
        let err = act_var.append("NotADigit", 0).err();
        let exp_err = LoadError::InvalidParameterForCommand {
            line: 0,
            command: "$var".to_string(),
            parameter: "NotADigit".to_string(),
        };
        assert_eq!(err, Some(exp_err));
    }

    #[test]
    fn extra_params_in_var_throws_error() {
        let mut act_var = Variable::new();
        for word in vec!["wire", "8", "e", "my_reference"] {
            act_var.append(word, 0).unwrap();
        }
        let err = act_var.append("ExtraParam", 0).err();
        let exp_err = LoadError::TooManyParameters {
            line: 0,
            command: "var".to_string(),
        };
        assert_eq!(err, Some(exp_err));
    }
}
