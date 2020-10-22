use crate::error::LoadError;
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, Clone, Eq, PartialEq, EnumString)]
pub enum VCDDataType {
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

#[derive(Debug, Clone, Eq, PartialEq, EnumString)]
pub enum VCDScopeType {
    #[strum(serialize = "begin")]
    Begin,
    #[strum(serialize = "fork")]
    Fork,
    #[strum(serialize = "function")]
    Function,
    #[strum(serialize = "module")]
    Module,
    #[strum(serialize = "task")]
    Task,
}

#[derive(Debug, Clone, Eq, Builder)]
pub struct VCDScope {
    pub scope_type: VCDScopeType,
    pub id: String,

    #[builder(setter(skip = "true"))]
    num_vars_seen: usize,
}

impl VCDScope {
    pub fn new() -> VCDScope {
        VCDScope {
            scope_type: VCDScopeType::Begin,
            id: "".to_string(),
            num_vars_seen: 0,
        }
    }

    pub fn append_value(&mut self, word: &str) {
        match self.num_vars_seen {
            0 => {
                self.scope_type = VCDScopeType::from_str(word).unwrap();
            }
            1 => {
                self.id = word.to_string();
            }
            _ => {} // TODO: Throw error
        }
        self.num_vars_seen += 1;
    }
}

impl PartialEq for VCDScope {
    fn eq(&self, other: &Self) -> bool {
        return self.scope_type == other.scope_type && self.id == other.id;
    }
}

#[derive(Debug, Clone, Builder)]
pub struct VCDVariable {
    pub scope: Vec<VCDScope>,
    pub data_type: VCDDataType,
    pub bit_width: usize,
    pub ascii_identifier: String,
    pub name: String,

    #[builder(setter(skip = "true"))]
    num_vars_seen: usize,
}

impl PartialEq for VCDVariable {
    fn eq(&self, other: &Self) -> bool {
        return self.scope == other.scope
            && self.data_type == other.data_type
            && self.bit_width == other.bit_width
            && self.ascii_identifier == other.ascii_identifier
            && self.name == other.name;
    }
}

impl VCDVariable {
    pub fn new() -> VCDVariable {
        VCDVariable {
            scope: vec![],
            data_type: VCDDataType::Event,
            bit_width: 0,
            ascii_identifier: "".to_string(),
            name: "".to_string(),
            num_vars_seen: 0,
        }
    }

    pub fn append_value(&mut self, word: &str) -> Result<(), LoadError> {
        match self.num_vars_seen {
            0 => {
                self.data_type = VCDDataType::from_str(word).unwrap();
            }
            1 => {
                self.bit_width = word.parse::<usize>().unwrap();
            }
            2 => {
                self.ascii_identifier = word.to_string();
            }
            3 => {
                self.name = word.to_string();
            }
            _ => {
                return Err(LoadError {
                    line: 0, // TODO
                    info: "$var has too many parameters".to_string(),
                });
            }
        }
        self.num_vars_seen += 1;
        Ok(())
    }

    pub fn is_done(&self) -> bool {
        let exp_num_vars_seen = 4;
        self.num_vars_seen >= exp_num_vars_seen
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn append_value_to_vcd_variable() {
        let exp_var = VCDVariableBuilder::default()
            .scope(vec![])
            .data_type(VCDDataType::Wire)
            .bit_width(8)
            .ascii_identifier("#".to_string())
            .name("data".to_string())
            .build()
            .unwrap();

        let mut act_var = VCDVariable::new();
        act_var.append_value("wire").unwrap();
        act_var.append_value("8").unwrap();
        act_var.append_value("#").unwrap();
        act_var.append_value("data").unwrap();

        assert_eq!(exp_var.data_type, act_var.data_type);
        assert_eq!(exp_var.bit_width, act_var.bit_width);
        assert_eq!(exp_var.ascii_identifier, act_var.ascii_identifier);
        assert_eq!(exp_var.name, act_var.name);
    }
}
