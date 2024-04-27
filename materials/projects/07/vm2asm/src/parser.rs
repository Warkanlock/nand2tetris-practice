use crate::logs::log_success;

#[derive(Debug, PartialEq, Clone)]
pub enum CommandType {
    CArithmetic,
    CPush,
    CPop,
    // CIf,
    // CFunction,
    // CReturn,
    // CCall,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Command {
    pub arg_1: Option<String>,
    pub arg_2: Option<String>, // only for push, pop, function, call
    pub command_type: CommandType,
}

pub struct Parser {
    pub commands: Vec<Command>,
    pub input: String,
}

impl Parser {
    pub fn get_fields(&self) -> &Vec<Command> {
        &self.commands
    }

    pub fn new(input: &str) -> Self {
        Self {
            commands: Vec::new(),
            input: input.to_string(),
        }
    }

    fn _parse_simple(&mut self) {
        log_success("Parsing input file");

        // should parse input and get commands into self.commands vector
        for line in self.input.lines() {
            // discard empty lines
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") {
                continue;
            }

            // we should split spaces in line
            let line_parts: Vec<&str> = line.split_whitespace().collect();

            // get the command type and arguments ( part[1] and part[2] )
            match line_parts.get(0) {
                Some(&"push") => self.commands.push(Command {
                    command_type: CommandType::CPush,
                    arg_1: Some(line_parts[1].to_string()),
                    arg_2: Some(line_parts[2].to_string()),
                }),
                Some(&"pop") => self.commands.push(Command {
                    command_type: CommandType::CPop,
                    arg_1: Some(line_parts[1].to_string()),
                    arg_2: Some(line_parts[2].to_string()),
                }),
                Some(&"add") => self.commands.push(Command {
                    command_type: CommandType::CArithmetic,
                    arg_1: Some("add".to_string()),
                    arg_2: None,
                }),
                Some(&"sub") => self.commands.push(Command {
                    command_type: CommandType::CArithmetic,
                    arg_1: Some("sub".to_string()),
                    arg_2: None,
                }),
                Some(&"eq") => self.commands.push(Command {
                    command_type: CommandType::CArithmetic,
                    arg_1: Some("eq".to_string()),
                    arg_2: None,
                }),
                Some(&"lt") => self.commands.push(Command {
                    command_type: CommandType::CArithmetic,
                    arg_1: Some("lt".to_string()),
                    arg_2: None,
                }),
                Some(&"gt") => self.commands.push(Command {
                    command_type: CommandType::CArithmetic,
                    arg_1: Some("gt".to_string()),
                    arg_2: None,
                }),
                Some(&"neg") => self.commands.push(Command {
                    command_type: CommandType::CArithmetic,
                    arg_1: Some("neg".to_string()),
                    arg_2: None,
                }),
                Some(&"and") => self.commands.push(Command {
                    command_type: CommandType::CArithmetic,
                    arg_1: Some("and".to_string()),
                    arg_2: None,
                }),
                Some(&"or") => self.commands.push(Command {
                    command_type: CommandType::CArithmetic,
                    arg_1: Some("or".to_string()),
                    arg_2: None,
                }),
                Some(&"not") => self.commands.push(Command {
                    command_type: CommandType::CArithmetic,
                    arg_1: Some("not".to_string()),
                    arg_2: None,
                }),
                _ => continue,
            };
        }

        log_success("Parse completed successfully");
    }

    pub fn parse(&mut self) {
        if self.input.is_empty() {
            panic!("Input should be defined before trying to parse");
        }

        // call the parse method
        self._parse_simple();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_parser() {
        let input = "push constant 7";
        let parser = Parser::new(input);

        assert_eq!(parser.input, input);
        assert_eq!(parser.commands.len(), 0);
    }

    #[test]
    fn get_parser_commands() {
        let parser = Parser::new("");

        assert_eq!(parser.get_fields(), &Vec::new());
    }

    #[test]
    fn should_discard_empty_lines() {
        let mut parser = Parser::new("\n\n\n");
        parser.parse();

        assert_eq!(parser.commands.len(), 0);
    }

    #[test]
    fn parse_push_command() {
        let mut parser = Parser::new("push constant 7");
        parser.parse();

        assert_eq!(parser.commands.len(), 1);
        assert_eq!(
            parser.commands[0],
            Command {
                command_type: CommandType::CPush,
                arg_1: Some("constant".to_string()),
                arg_2: Some("7".to_string())
            }
        );
    }

    #[test]
    fn parse_pop_command() {
        let mut parser = Parser::new("pop local 0");
        parser.parse();

        assert_eq!(parser.commands.len(), 1);
        assert_eq!(
            parser.commands[0],
            Command {
                command_type: CommandType::CPop,
                arg_1: Some("local".to_string()),
                arg_2: Some("0".to_string())
            }
        );
    }

    #[test]
    fn parse_mutliple_commands() {
        let mut parser = Parser::new("push constant 7\npop local 0");
        parser.parse();

        assert_eq!(parser.commands.len(), 2);
        assert_eq!(
            parser.commands[0],
            Command {
                command_type: CommandType::CPush,
                arg_1: Some("constant".to_string()),
                arg_2: Some("7".to_string())
            }
        );
        assert_eq!(
            parser.commands[1],
            Command {
                command_type: CommandType::CPop,
                arg_1: Some("local".to_string()),
                arg_2: Some("0".to_string())
            }
        );
    }

    #[test]
    fn parse_multiple_arithmetic_commands() {
        let mut parser = Parser::new("add\nsub\n");
        parser.parse();

        assert_eq!(parser.commands.len(), 2);
        assert_eq!(
            parser.commands[0],
            Command {
                command_type: CommandType::CArithmetic,
                arg_1: Some("add".to_string()),
                arg_2: None
            }
        );
        assert_eq!(
            parser.commands[1],
            Command {
                command_type: CommandType::CArithmetic,
                arg_1: Some("sub".to_string()),
                arg_2: None
            }
        );
    }

    #[test]
    fn parse_arithmetic_command() {
        let mut parser = Parser::new("add");
        parser.parse();

        assert_eq!(parser.commands.len(), 1);
        assert_eq!(
            parser.commands[0],
            Command {
                command_type: CommandType::CArithmetic,
                arg_1: Some("add".to_string()),
                arg_2: None
            }
        );
    }

    #[test]
    #[should_panic(expected = "Input should be defined before trying to parse")]
    fn fail_at_parsing_empty_input() {
        let mut parser = Parser::new("");
        parser.parse();
    }
}
