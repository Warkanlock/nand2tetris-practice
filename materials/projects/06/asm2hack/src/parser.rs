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
        log_success("Parsing input file without symbolic links")
    }

    fn _parse_complex(&mut self) {
        log_success("Parsing input file with symbolic links")
    }

    pub fn new(input: &str, is_symbolic: bool) -> Self {
        Self {
            is_symbolic,
            input: String::from(input),
            fields: Vec::new(), // always initialize fields to empty vector
        }
    }

    pub fn parse(&mut self) {
        // check if input is defined
        if self.input.is_empty() {
            panic!("Input file is empty");
        }

        if self.is_symbolic {
            self._parse_complex();
        }

        self._parse_simple();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_init_parser() {
        let input = "random string";
        let parser = Parser::new(input, false);

        assert_eq!(parser.is_symbolic, false);
        assert_eq!(parser.input, input);
        assert_eq!(parser.fields.len(), 0);
    }
    
    #[test]
    fn fn_parse_complex_non_valid_string() {
        let input = "random string";
        let mut parser = Parser::new(input, true);

        assert_eq!(parser.is_symbolic, true);
        assert_eq!(parser.input, input);
        assert_eq!(parser.fields.len(), 0);

        // check if throw error
        parser.parse();
    }

    #[test]
    fn fn_parse_simple_non_valid_string() {
        let input = "random string";
        let parser = Parser::new(input, false);

        assert_eq!(parser.is_symbolic, false);
        assert_eq!(parser.input, input);
        assert_eq!(parser.fields.len(), 0);
    }

    #[test]
    #[should_panic(expected = "Input file is empty")]
    fn fn_parse_simple_throw_when_empty_input() {
        let input = "";
        let mut parser = Parser::new(input, false);

        assert_eq!(parser.is_symbolic, false);
        assert_eq!(parser.input, input);
        assert_eq!(parser.fields.len(), 0);

        // check if throw error
        parser.parse();
    }
}
