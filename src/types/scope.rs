use crate::error::LoadError;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq, EnumString)]
pub enum ScopeType {
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

#[derive(Debug, Clone, Eq, PartialEq)]
enum BuildState {
    ScopeType,
    Identifier,
    Done,
}

impl BuildState {
    fn next(&self, line_num: usize) -> Result<Self, LoadError> {
        use BuildState::*;
        match *self {
            ScopeType => Ok(Identifier),
            Identifier => Ok(Done),
            Done => Err(LoadError::TooManyParameters {
                line: line_num,
                command: "$scope".to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Scope {
    pub scope_type: ScopeType,
    pub identifier: String,
    state: BuildState,
}

impl PartialEq for Scope {
    fn eq(&self, other: &Self) -> bool {
        self.scope_type == other.scope_type && self.identifier == other.identifier
    }
}

impl Default for Scope {
    fn default() -> Self {
        Scope::new()
    }
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            scope_type: ScopeType::Begin,
            identifier: "".to_string(),
            state: BuildState::ScopeType,
        }
    }

    pub fn init(scope_type: ScopeType, identifier: String) -> Self {
        Scope {
            scope_type,
            identifier,
            state: BuildState::ScopeType,
        }
    }

    pub fn append(&mut self, word: &str, line_num: usize) -> Result<(), LoadError> {
        match self.state {
            BuildState::ScopeType => self.write_scope_type(word, line_num)?,
            BuildState::Identifier => self.write_identifier(word.to_string())?,
            _ => {}
        }
        self.state = self.state.next(line_num)?;
        Ok(())
    }

    fn write_scope_type(&mut self, word: &str, line_num: usize) -> Result<(), LoadError> {
        self.scope_type = match ScopeType::from_str(word) {
            Ok(scope_type) => scope_type,
            Err(_) => {
                return Err(LoadError::InvalidParameterForCommand {
                    line: line_num,
                    command: "$scope".to_string(),
                    parameter: word.to_string(),
                })
            }
        };
        Ok(())
    }

    fn write_identifier(&mut self, word: String) -> Result<(), LoadError> {
        self.identifier = word;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_scope_1() {
        let mut scope = Scope::new();
        scope.append("module", 0).unwrap();
        scope.append("top", 0).unwrap();
        assert_eq!(scope.scope_type, ScopeType::Module);
        assert_eq!(scope.identifier, "top");
    }

    #[test]
    fn build_scope_2() {
        let mut scope = Scope::new();
        scope.append("task", 0).unwrap();
        scope.append("my_task", 0).unwrap();
        assert_eq!(scope.scope_type, ScopeType::Task);
        assert_eq!(scope.identifier, "my_task");
    }

    #[test]
    fn invalid_scope_type_throws_error() {
        let mut scope = Scope::new();
        let err = scope.append("NotAScopeType", 0).err();
        let exp_err = LoadError::InvalidParameterForCommand {
            line: 0,
            command: "$scope".to_string(),
            parameter: "NotAScopeType".to_string(),
        };
        assert_eq!(err, Some(exp_err));
    }

    #[test]
    fn extra_params_in_scope_throws_error() {
        let mut scope = Scope::new();
        scope.append("task", 0).unwrap();
        scope.append("my_task", 0).unwrap();
        let err = scope.append("my_task", 0).err();
        let exp_err = LoadError::TooManyParameters {
            line: 0,
            command: "$scope".to_string(),
        };
        assert_eq!(err, Some(exp_err));
    }
}
