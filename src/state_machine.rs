use crate::error::LoadError;
use crate::string_helpers::append_word;
use crate::types::{dumped_var::DumpedVar, scope::Scope, variable::Variable};
use crate::vcd::VCD;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, EnumString, ToString)]
#[strum(serialize_all = "snake_case")]
enum ParserState {
    End,
    Comment,
    Date,
    Version,
    Timescale,
    Scope,
    Upscope,
    Var,
    Dumpall,
    Dumpoff,
    Dumpon,
    Dumpvars,
    Enddefinitions,
    #[strum(disabled)]
    ReadingDumpedVars,
}

pub struct StateMachine {
    pub vcd: VCD,
    scope: Scope,
    var: Variable,
    comment: String,
    scope_stack: Vec<Scope>,
    state: ParserState,
    singular_commands_seen: HashMap<ParserState, bool>,
    parsing_header: bool,
    time: isize,
}

impl Default for StateMachine {
    fn default() -> Self {
        StateMachine {
            state: ParserState::End,
            scope: Scope::new(),
            var: Variable::default(),
            comment: String::new(),
            scope_stack: vec![],
            vcd: VCD::default(),
            singular_commands_seen: StateMachine::get_singular_commands_seen(),
            parsing_header: true,
            time: -1,
        }
    }
}

impl StateMachine {
    fn get_singular_commands_seen() -> HashMap<ParserState, bool> {
        use ParserState::*;
        let mut map: HashMap<ParserState, bool> = HashMap::new();
        for state in &[Version, Date, Timescale] {
            map.insert(*state, false);
        }
        map
    }

    pub fn parse_word(&mut self, word: &str, line_num: usize) -> Result<(), LoadError> {
        if StateMachine::is_cmd(&word) {
            self.try_transition(word, line_num)?;
        } else {
            self.do_work(word, line_num)?;
        }
        Ok(())
    }

    fn try_transition(&mut self, cmd: &str, line_num: usize) -> Result<(), LoadError> {
        use ParserState::*;
        let next_state = if let Some(cmd_wo_dollar) = cmd.strip_prefix('$') {
            ParserState::from_str(cmd_wo_dollar).unwrap()
        } else {
            ParserState::ReadingDumpedVars
        };
        self.state = match self.state {
            End => {
                self.check_if_end_followed_by_end(line_num, next_state)?;
                self.check_if_invalid_multiple_command(line_num, next_state)?;
                if next_state == ParserState::Var {
                    self.update_variable_scope(line_num, next_state)?;
                }
                next_state
            }
            _ => {
                self.check_if_missing_end(line_num, next_state)?;

                match self.state {
                    Var => {
                        self.check_if_var_is_done(line_num)?;
                        self.append_variable();
                    }
                    Comment => self.append_comment(),
                    Scope => self.push_to_scope_stack(),
                    Upscope => {
                        self.check_if_scope_stack_is_empty(line_num, self.state)?;
                        self.scope_stack.pop();
                    }
                    Enddefinitions => self.parsing_header = false,
                    Dumpall | Dumpoff | Dumpon | Dumpvars | ReadingDumpedVars => {
                        if self.parsing_header {
                            return Err(LoadError::DumpWithoutEnddefinitions { line: line_num });
                        }
                    }
                    _ => {}
                }

                ParserState::End
            }
        };
        Ok(())
    }

    fn check_if_end_followed_by_end(
        &mut self,
        line_num: usize,
        state: ParserState,
    ) -> Result<(), LoadError> {
        match state {
            ParserState::End => Err(LoadError::DanglingEnd { line: line_num }),
            _ => Ok(()),
        }
    }

    fn check_if_missing_end(
        &mut self,
        line_num: usize,
        state: ParserState,
    ) -> Result<(), LoadError> {
        match state {
            ParserState::End => Ok(()),
            _ => Err(LoadError::MissingEnd {
                line: line_num,
                command: self.state.to_string(),
            }),
        }
    }

    fn check_if_invalid_multiple_command(
        &mut self,
        line_num: usize,
        state: ParserState,
    ) -> Result<(), LoadError> {
        if let Some(seen) = self.singular_commands_seen.get(&state) {
            match seen {
                true => {
                    return Err(LoadError::InvalidMultipleCommand {
                        line: line_num,
                        command: state.to_string(),
                    })
                }
                false => *self.singular_commands_seen.get_mut(&state).unwrap() = true,
            }
        }
        Ok(())
    }

    pub fn cleanup(&self, line_num: usize) -> Result<(), LoadError> {
        match self.state {
            ParserState::End | ParserState::Dumpvars => {}
            _ => {
                return Err(LoadError::MissingEnd {
                    line: line_num,
                    command: self.state.to_string(),
                })
            }
        }
        Ok(())
    }

    fn append_variable(&mut self) {
        self.vcd
            .variables
            .insert(self.var.ascii_identifier.clone(), self.var.clone());
        self.var = Variable::default();
    }

    fn check_if_var_is_done(&mut self, line_num: usize) -> Result<(), LoadError> {
        match self.var.is_done() {
            true => Ok(()),
            false => Err(LoadError::TooFewParameters {
                line: line_num,
                command: "var".to_string(),
            }),
        }
    }

    fn update_variable_scope(
        &mut self,
        line_num: usize,
        state: ParserState,
    ) -> Result<(), LoadError> {
        self.check_if_scope_stack_is_empty(line_num, state)?;
        self.var.scope = self.scope_stack.clone();
        Ok(())
    }

    fn append_comment(&mut self) {
        self.vcd.comments.push(self.comment.clone());
        self.comment = String::new();
    }

    fn push_to_scope_stack(&mut self) {
        self.scope_stack.push(self.scope.clone());
        self.scope = Scope::new();
    }

    fn check_if_scope_stack_is_empty(
        &mut self,
        line_num: usize,
        state: ParserState,
    ) -> Result<(), LoadError> {
        if self.scope_stack.is_empty() {
            Err(LoadError::ScopeStackEmpty {
                line: line_num,
                command: state.to_string(),
            })
        } else {
            Ok(())
        }
    }

    fn do_work(&mut self, word: &str, line_num: usize) -> Result<(), LoadError> {
        use ParserState::*;
        match self.state {
            Comment => append_word(&mut self.comment, word),
            Date => append_word(&mut self.vcd.date, word),
            Version => append_word(&mut self.vcd.version, word),
            Timescale => self.vcd.timescale.append(word, line_num)?,
            Scope => self.scope.append(word, line_num)?,
            Var => self.var.append(word, line_num)?,
            Dumpall | Dumpoff | Dumpon | Dumpvars | ReadingDumpedVars => {
                self.read_dump(word, line_num)?
            }
            Enddefinitions | Upscope => {
                StateMachine::raise_invalid_param(self.state.to_string(), line_num, word)?
            }
            _ => {}
        }
        Ok(())
    }

    fn read_dump(&mut self, word: &str, line_num: usize) -> Result<(), LoadError> {
        let dumped_var = DumpedVar::new(line_num, word)?;
        if let Some(variable) = self.vcd.variables.get_mut(dumped_var.identifier) {
            variable.add_transition(self.time, dumped_var.value)
        }
        Ok(())
    }

    fn raise_invalid_param(
        command: String,
        line_num: usize,
        parameter: &str,
    ) -> Result<(), LoadError> {
        Err(LoadError::InvalidParameterForCommand {
            line: line_num,
            command,
            parameter: parameter.to_string(),
        })
    }

    fn is_cmd(word: &str) -> bool {
        word.starts_with('$')
    }
}
