use crate::logs::{log_info, log_success};

#[derive(Debug, PartialEq)]
pub enum ParserInstructionType {
    AInstruction,
    CInstruction,
    LInstruction,
    Comment,
}

#[derive(Debug, PartialEq)]
pub struct ParserFields {
    line_number: u32,
    instruction_type: ParserInstructionType,
    instruction_value: Option<String>,
}

pub struct Parser {
    pub is_symbolic: bool,
    pub input: String,
    pub fields: Vec<ParserFields>,
}

impl Parser {
    fn _parse_simple(&mut self) {
        log_success("Parsing input file without symbolic links");

        // 1. break lines on input
        let lines = self.input.lines();

        // 2. calculate the field type and store it into the fields array
        for mut line in lines {
            // remove spaces from line on both sides
            line = line.trim();

            println!("line: {}", line);

            // revisit the first character of the line
            match line.chars().nth(0) {
                Some('/') if line.starts_with("//") => {
                    // check if line is a comment
                    self.fields.push(ParserFields {
                        line_number: 0,
                        instruction_type: ParserInstructionType::Comment,
                        instruction_value: None,
                    });
                }
                Some('(') => {
                    // check if line is a label
                    self.fields.push(ParserFields {
                        line_number: 0,
                        instruction_type: ParserInstructionType::LInstruction,
                        instruction_value: None,
                    });
                }
                Some('@') => {
                    // check if line is an A instruction
                    self.fields.push(ParserFields {
                        line_number: 0,
                        instruction_type: ParserInstructionType::AInstruction,
                        // get the value from the A-instruction
                        instruction_value: Some(line.chars().skip(1).collect()),
                    });
                }
                Some(_) => {
                    // check if line is a C instruction
                    self.fields.push(ParserFields {
                        line_number: 0,
                        instruction_type: ParserInstructionType::CInstruction,
                        // get the instruction from the C-instruction
                        instruction_value: Some(line.to_string())
                    });
                }
                None => {
                    continue;
                }
            };

            log_info(line);
        }
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

    #[test]
    fn fn_parse_simple_valid_asm() {
        let input_asm = "@20";

        let mut parser = Parser::new(input_asm, false);

        assert_eq!(parser.is_symbolic, false);
        assert_eq!(parser.input, input_asm);
        assert_eq!(parser.fields.len(), 0);

        parser.parse();

        assert_eq!(parser.fields.len(), 1);
        assert_eq!(parser.fields[0].line_number, 0);
        assert_eq!(
            parser.fields[0].instruction_type,
            ParserInstructionType::AInstruction
        );
        assert_eq!(parser.fields[0].instruction_value, Some("20".to_string()));
    }

    #[test]
    fn fn_parse_simple_valid_asm_with_comment() {
        let input_asm = "@20\n// this is a comment";

        let mut parser = Parser::new(input_asm, false);

        assert_eq!(parser.is_symbolic, false);
        assert_eq!(parser.input, input_asm);
        assert_eq!(parser.fields.len(), 0);

        parser.parse();

        // validate that we are counting the comment as field as well
        assert_eq!(parser.fields.len(), 2);

        // the comment should not have a value and should be ignored
        assert_eq!(parser.fields[0].line_number, 0);
        assert_eq!(
            parser.fields[0].instruction_type,
            ParserInstructionType::AInstruction
        );
        assert_eq!(parser.fields[0].instruction_value, Some("20".to_string()));

        // the comment should not have a value and should be ignored
        assert_eq!(parser.fields[1].line_number, 0);
        assert_eq!(
            parser.fields[1].instruction_type,
            ParserInstructionType::Comment
        );
        assert_eq!(parser.fields[1].instruction_value, None);
    }
}
