use crate::error::LoadError;

pub struct CommandParser {
    lines: String,
    command: String,
    enforce_only_one_of_command: bool,
}

impl CommandParser {
    pub fn new() -> CommandParser {
        CommandParser {
            lines: "".to_string(),
            command: "".to_string(),
            enforce_only_one_of_command: false,
        }
    }

    pub fn lines(&mut self, lines: &String) -> &mut CommandParser {
        self.lines = lines.clone(); // NICE TO HAVE: Remove clone()?
        self
    }

    pub fn command(&mut self, command_in: &String) -> &mut CommandParser {
        self.command = command_in.clone(); // NICE TO HAVE: Remove clone()?
        self
    }

    pub fn enforce_only_one_of_command(&mut self, enforcement: bool) -> &mut CommandParser {
        self.enforce_only_one_of_command = enforcement;
        self
    }

    pub fn parse(&self) -> Result<Vec<String>, LoadError> {
        let mut currently_parsing_command = false;
        let mut current_command_string = String::new();
        let mut command_vec = Vec::new();
        let mut line_num = 1;
        for line in self.lines.lines() {
            let words: Vec<_> = line.split(" ").filter(|c| !c.is_empty()).collect();
            for word in words {
                let word_wo_newlines = word.replace("\n", ""); // TODO: Replace with trait method

                if self.is_different_command(&word_wo_newlines, &self.command)
                    && currently_parsing_command
                {
                    return Err(LoadError {
                        line: line_num,
                        info: format!("{} missing an $end", self.command),
                    });
                }

                if self.is_end(&word_wo_newlines) && current_command_string.len() != 0 {
                    currently_parsing_command = false;
                    command_vec.push(current_command_string.trim().to_string());
                    current_command_string = String::new();
                } else if currently_parsing_command {
                    current_command_string = current_command_string + " " + &word_wo_newlines[..];
                } else if self.is_command(&word_wo_newlines, &self.command) {
                    if command_vec.len() != 0 && self.enforce_only_one_of_command {
                        return Err(LoadError {
                            line: line_num,
                            info: format!("Multiple {} commands is invalid", self.command),
                        });
                    }
                    currently_parsing_command = true;
                }

                if self.is_end_of_line(word) {
                    line_num += 1;
                }
            }
        }

        // Not finding any command in string is invalid
        if command_vec.len() == 0 {
            command_vec.push(String::new());
        }

        match currently_parsing_command {
            true => Err(LoadError {
                line: line_num,
                info: format!("{} missing an $end", self.command),
            }),
            false => Ok(command_vec),
        }
    }

    fn is_different_command(&self, word: &String, command: &str) -> bool {
        word.starts_with("$") && word != command && word != "$end"
    }

    fn is_end(&self, word: &String) -> bool {
        word == "$end"
    }

    fn is_command(&self, word: &String, command: &String) -> bool {
        word == command
    }

    fn is_end_of_line(&self, word: &str) -> bool {
        word.contains("\n")
    }
}
