use crate::logs::log_success;

#[derive(Debug, PartialEq)]
pub enum ParserInstructionType {
    AInstruction, // address-instruction
    CInstruction, // compute-instruction
    LInstruction, // label-instruction
    Comment,
}

#[derive(Debug, PartialEq)]
pub struct ParserFields {
    /// internal line number to track the line in the file
    line_number: usize,
    /// used for all instructions
    pub instruction_type: ParserInstructionType,
    /// used for A-instructions and L-instructions
    /// we store the value of the instruction (or the symbol)
    pub instruction_value: Option<u16>,
    /// used for C-instructions
    /// we store the instruction complete computation
    pub instruction_symbol: Option<String>,
    /// instructions used for C-instructions
    /// we store the instruction destination
    /// we store the instruction jump
    /// we store the instruction computation
    pub instruction_dest: Option<String>,
    pub instruction_jump: Option<String>,
    pub instruction_comp: Option<String>,
}

pub struct Parser {
    pub is_symbolic: bool,
    pub input: String,
    pub fields: Vec<ParserFields>,
}

impl Parser {
    pub fn get_fields(&self) -> &Vec<ParserFields> {
        &self.fields
    }

    fn _parse_simple(&mut self) {
        log_success("Parsing input file without symbolic links");

        // 1. break lines on input
        let lines = self.input.lines();

        // 2. calculate the field type and store it into the fields array
        for (index, mut line) in lines.enumerate() {
            // remove spaces from line on both sides
            line = line.trim();
            let line_number: usize = index + 1;

            // revisit the first character of the line
            match line.chars().nth(0) {
                Some('/') if line.starts_with("//") => {
                    // check if line is a comment
                    self.fields.push(ParserFields {
                        line_number,
                        instruction_type: ParserInstructionType::Comment,
                        instruction_symbol: None,
                        instruction_value: None,
                        instruction_dest: None,
                        instruction_jump: None,
                        instruction_comp: None,
                    });
                }
                Some('(') => {
                    // check if line is a label
                    self.fields.push(ParserFields {
                        line_number,
                        instruction_type: ParserInstructionType::LInstruction,
                        instruction_symbol: Some(line.replace(")", "").chars().skip(1).collect()),
                        instruction_value: None,
                        instruction_dest: None,
                        instruction_jump: None,
                        instruction_comp: None,
                    });
                }
                Some('@') => {
                    // extract from line the value or the symbolic reference
                    let is_value = line.chars().skip(1).next().unwrap().is_numeric();

                    if is_value {
                        // check if line is an A instruction
                        self.fields.push(ParserFields {
                            line_number,
                            instruction_type: ParserInstructionType::AInstruction,
                            instruction_symbol: None,
                            // set to 0 in case it fails to parse
                            instruction_value: Some(
                                line.replace("@", "").parse::<u16>().unwrap_or(0),
                            ),
                            instruction_dest: None,
                            instruction_jump: None,
                            instruction_comp: None,
                        });
                    } else {
                        // check if line is an A instruction
                        self.fields.push(ParserFields {
                            line_number,
                            instruction_type: ParserInstructionType::AInstruction,
                            instruction_symbol: Some(line.replace("@", "")),
                            instruction_value: None,
                            instruction_dest: None,
                            instruction_jump: None,
                            instruction_comp: None,
                        });
                    }
                }
                Some(_) => {
                    // split and collect the parts
                    let parts: Vec<&str> = line.split('=').collect();

                    // check if parts is higher than 2 at least
                    if parts.len() == 1 {
                        panic!("Invalid instruction schema: {}.", line)
                    }

                    // get dest, comp and jump
                    let dest = parts.get(0).map(|s| s.to_string());
                    let (comp, jump) = parts.get(1).map_or((None, None), |s| {
                        // divide second part based on ;
                        let mut split = s.split(';').map(|s| s.to_string());

                        // return tuple
                        (split.next(), split.next())
                    });

                    // validate if the isntruction is valid
                    if comp.as_deref() == Some("") || dest.as_deref() == Some("") {
                        panic!("Invalid instruction format: {}", line)
                    }

                    // check if line is a C instruction
                    self.fields.push(ParserFields {
                        line_number,
                        instruction_type: ParserInstructionType::CInstruction,
                        instruction_symbol: Some(line.to_string()),
                        instruction_value: None,
                        instruction_dest: dest,
                        instruction_comp: comp,
                        instruction_jump: jump,
                    });
                }
                None => {
                    continue;
                }
            };
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
            return self._parse_complex();
        }

        return self._parse_simple();
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
    // only this test should run
    fn fn_parse_simple_valid_asm() {
        let input_asm = "@20";

        let mut parser = Parser::new(input_asm, false);

        assert_eq!(parser.is_symbolic, false);
        assert_eq!(parser.input, input_asm);
        assert_eq!(parser.fields.len(), 0);

        parser.parse();

        assert_eq!(parser.fields.len(), 1);
        assert_eq!(parser.fields[0].line_number, 1);
        assert_eq!(
            parser.fields[0].instruction_type,
            ParserInstructionType::AInstruction
        );
        assert_eq!(parser.fields[0].instruction_value, Some(20));
    }

    #[test]
    fn fn_parse_simple_valid_asm_ainnstruction() {
        let input_asm = "@reference";

        let mut parser = Parser::new(input_asm, false);

        assert_eq!(parser.is_symbolic, false);
        assert_eq!(parser.input, input_asm);
        assert_eq!(parser.fields.len(), 0);

        parser.parse();

        assert_eq!(parser.fields.len(), 1);
        assert_eq!(parser.fields[0].line_number, 1);
        assert_eq!(
            parser.fields[0].instruction_type,
            ParserInstructionType::AInstruction
        );
        assert_eq!(parser.fields[0].instruction_value, None);
        assert_eq!(
            parser.fields[0].instruction_symbol,
            Some("reference".to_string())
        );
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
        assert_eq!(parser.fields[0].line_number, 1);
        assert_eq!(
            parser.fields[0].instruction_type,
            ParserInstructionType::AInstruction
        );
        assert_eq!(parser.fields[0].instruction_value, Some(20));

        // the comment should not have a value and should be ignored
        assert_eq!(parser.fields[1].line_number, 2);
        assert_eq!(
            parser.fields[1].instruction_type,
            ParserInstructionType::Comment
        );
        assert_eq!(parser.fields[1].instruction_value, None);
    }

    #[test]
    fn fn_parse_simple_valid_asm_with_cinstruction() {
        let input_asm = "@20\nD=D+A";

        let mut parser = Parser::new(input_asm, false);

        assert_eq!(parser.is_symbolic, false);
        assert_eq!(parser.input, input_asm);
        assert_eq!(parser.fields.len(), 0);

        parser.parse();

        // validate that we are counting the comment as field as well
        assert_eq!(parser.fields.len(), 2);

        let c_instrument = parser.fields.iter().find(|&x| x.line_number == 2).unwrap();

        assert_eq!(c_instrument.line_number, 2);
        assert_eq!(
            c_instrument.instruction_type,
            ParserInstructionType::CInstruction
        );
        assert_eq!(c_instrument.instruction_symbol, Some("D=D+A".to_string()));
        assert_eq!(c_instrument.instruction_value, None);
    }

    #[test]
    fn fn_get_fields_after_parse() {
        let input_asm = "@20\nD=D+A";

        let mut parser = Parser::new(input_asm, false);

        assert_eq!(parser.is_symbolic, false);
        assert_eq!(parser.input, input_asm);
        assert_eq!(parser.fields.len(), 0);

        parser.parse();

        // validate that we are counting the comment as field as well
        assert_eq!(parser.get_fields().len(), 2);
    }

    #[test]
    fn fn_get_dest_comp_instruction() {
        let input_asm = "D=D+A";

        let mut parser = Parser::new(input_asm, false);

        assert_eq!(parser.is_symbolic, false);
        assert_eq!(parser.input, input_asm);
        assert_eq!(parser.fields.len(), 0);

        parser.parse();

        // validate that we are counting the comment as field as well
        assert_eq!(parser.get_fields().len(), 1);

        let c_instrument: &ParserFields =
            parser.fields.iter().find(|&x| x.line_number == 1).unwrap();

        assert_eq!(c_instrument.line_number, 1);
        assert_eq!(
            c_instrument.instruction_type,
            ParserInstructionType::CInstruction
        );
        assert_eq!(c_instrument.instruction_symbol, Some("D=D+A".to_string()));
        assert_eq!(c_instrument.instruction_value, None);
        assert_eq!(c_instrument.instruction_dest, Some("D".to_string()));
        assert_eq!(c_instrument.instruction_comp, Some("D+A".to_string()));
        assert_eq!(c_instrument.instruction_jump, None);
    }

    #[test]
    fn fn_get_dest_comp_jump_instruction() {
        let input_asm = "D=D+A;JMP"; // unconditional jump

        let mut parser = Parser::new(input_asm, false);

        assert_eq!(parser.is_symbolic, false);
        assert_eq!(parser.input, input_asm);
        assert_eq!(parser.fields.len(), 0);

        parser.parse();

        // validate that we are counting the comment as field as well
        assert_eq!(parser.get_fields().len(), 1);

        let c_instrument: &ParserFields =
            parser.fields.iter().find(|&x| x.line_number == 1).unwrap();

        assert_eq!(c_instrument.line_number, 1);
        assert_eq!(
            c_instrument.instruction_type,
            ParserInstructionType::CInstruction
        );
        assert_eq!(
            c_instrument.instruction_symbol,
            Some("D=D+A;JMP".to_string())
        );
        assert_eq!(c_instrument.instruction_value, None);
        assert_eq!(c_instrument.instruction_dest, Some("D".to_string()));
        assert_eq!(c_instrument.instruction_comp, Some("D+A".to_string()));
        assert_eq!(c_instrument.instruction_jump, Some("JMP".to_string()));
    }

    #[test]
    #[should_panic(expected = "Invalid instruction format: D=")]
    fn fn_get_incorrect_dest_comp_instruction() {
        let input_asm = "D="; // unconditional jump

        let mut parser = Parser::new(input_asm, false);

        assert_eq!(parser.is_symbolic, false);
        assert_eq!(parser.input, input_asm);
        assert_eq!(parser.fields.len(), 0);

        parser.parse();
    }

    #[test]
    #[should_panic(expected = "Invalid instruction format: =D+A")]
    fn fn_get_incorrect_comp_instruction() {
        let input_asm = "=D+A"; // unconditional jump

        let mut parser = Parser::new(input_asm, false);

        assert_eq!(parser.is_symbolic, false);
        assert_eq!(parser.input, input_asm);
        assert_eq!(parser.fields.len(), 0);

        parser.parse();
    }

    #[test]
    #[should_panic(expected = "Invalid instruction schema: ;JMP")]
    fn fn_get_incorrect_jmp_only_instruction() {
        let input_asm = ";JMP"; // unconditional jump

        let mut parser = Parser::new(input_asm, false);

        assert_eq!(parser.is_symbolic, false);
        assert_eq!(parser.input, input_asm);
        assert_eq!(parser.fields.len(), 0);

        parser.parse();
    }

    #[test]
    #[should_panic(expected = "Invalid instruction schema: D")]
    fn fn_get_empty_instruction() {
        let input_asm = "D"; // unconditional jump

        let mut parser = Parser::new(input_asm, false);

        assert_eq!(parser.is_symbolic, false);
        assert_eq!(parser.input, input_asm);
        assert_eq!(parser.fields.len(), 0);

        parser.parse();
    }
}
