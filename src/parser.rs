use crate::error::LoadError;
use crate::state_machine::StateMachine;

pub fn parse(
    state_machine: &mut StateMachine,
    line: String,
    line_num: usize,
) -> Result<(), LoadError> {
    let words: Vec<_> = line.split(' ').filter(|c| !c.is_empty()).collect();
    for word in words {
        state_machine.parse_word(word, line_num)?
    }
    Ok(())
}
