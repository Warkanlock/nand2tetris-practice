use crate::logs::{log_info, log_success};

pub enum ParserInstructionType {
    AInstruction,
    CInstruction,
    LInstruction,
    Comment,
    Empty,
}

pub struct ParserFields {
    line_number: u32,
    instruction_type: ParserInstructionType,
    instruction_value: String,
}

pub struct Parser {
    pub is_symbolic: bool,
    pub input: String,
    pub fields: Vec<ParserFields>,
}

impl Parser {
    fn _parse_simple(&mut self) {
        log_success("Parsing input file")
    }

    pub fn new(input: &str, is_symbolic: Option<bool>) -> Self {
        Self {
            is_symbolic: is_symbolic.unwrap_or(false),
            input: String::from(input),
            fields: Vec::new(), // always initialize fields to empty vector
        }
    }

    pub fn parse(&mut self) {
        // check if input is defined
        if self.input.is_empty() {
            log_info("Input file is not defined. Please use a valid non-empty file");
            return;
        }

        if self.is_symbolic {
            log_info("Symbolic mode is not supported yet");
        }

        self._parse_simple();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_init_parser() {
        let parser = Parser::new(&String::from("test.asm"), None);

        assert_eq!(parser.is_symbolic, false);
        assert_eq!(parser.input, "test.asm");
        assert_eq!(parser.fields.len(), 0);
    }

    #[test]
    fn fn_parse_simple() {
        let mut parser = Parser::new(&String::from("test.asm"), None);
        parser.parse();
    }
}
