use thiserror::Error;

#[derive(Clone, PartialEq, Debug, Eq, Error)]
pub enum LoadError {
    #[error("Error opening file {}: {}", filename, error)]
    FileOpenError { filename: String, error: String },

    #[error("line {}: Error reading file at this point", line)]
    FileReadError { line: usize },

    #[error("line {}: {} missing an $end", line, command)]
    MissingEnd { command: String, line: usize },

    #[error("line {}: More than one {} command is invalid", line, command)]
    InvalidMultipleCommand { command: String, line: usize },

    #[error("line {}: Dangling $end", line)]
    DanglingEnd { line: usize },

    #[error(
        "line {}: Invalid parameter {} for command {}",
        line,
        parameter,
        command
    )]
    InvalidParameterForCommand {
        line: usize,
        parameter: String,
        command: String,
    },

    #[error("line {}: {} has too few parameters", line, command)]
    TooFewParameters { line: usize, command: String },

    #[error("line {}: {} has too many parameters", line, command)]
    TooManyParameters { line: usize, command: String },

    #[error("line {}: {} declared with empty scope", line, command)]
    ScopeStackEmpty { line: usize, command: String },

    #[error("line {}: Found time value {}, expected integer", line, value)]
    InvalidTimeValue { line: usize, value: String },

    #[error(
        "line {}: Found timescale {}, expected one of: [ ms us ns ps ]",
        line,
        time_scale
    )]
    InvalidTimeScale { line: usize, time_scale: String },

    #[error("line {}: Variable dump formatted improperly", line)]
    InvalidVarDump { line: usize },

    #[error("line {}: Tried to dump variables before $enddefinitions", line)]
    DumpWithoutEnddefinitions { line: usize },
}
