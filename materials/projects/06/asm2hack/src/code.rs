use crate::parser::{ParserFields, ParserInstructionType};

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryInstruction {
    pub instruction: ParserFields,
    pub binary: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryInstructionStrategy {
    Dest,
    Comp,
    Jump,
}

fn get_binary_form(number : u16) -> String {
    (0..16) 
        .rev()
        .map(|i| (number >> i) & 1) // this will get the i bit
        .map(|b| b.to_string()) // convert to string
        .collect::<Vec<String>>()
        .join("")
}

fn get_dest_form(dest: &str) -> String {
    match dest {
        "M" => String::from("001"),
        "D" => String::from("010"),
        "MD" => String::from("011"),
        "A" => String::from("100"),
        "AM" => String::from("101"),
        "AD" => String::from("110"),
        "AMD" => String::from("111"),
        _ => String::from("000"),
    }
}

fn get_comb_form(comb: &str) -> String {
    match comb {
        "0" => String::from("0101010"),
        "1" => String::from("0111111"),
        "-1" => String::from("0111010"),
        "D" => String::from("0001100"),
        "A" => String::from("0110000"),
        "!D" => String::from("0001101"),
        "!A" => String::from("0110001"),
        "-D" => String::from("0001111"),
        "-A" => String::from("0110011"),
        "D+1" => String::from("0011111"),
        "A+1" => String::from("0110111"),
        "D-1" => String::from("0001110"),
        "A-1" => String::from("0110010"),
        "D+A" => String::from("0000010"),
        "D-A" => String::from("0010011"),
        "A-D" => String::from("0000111"),
        "D&A" => String::from("0000000"),
        "D|A" => String::from("0010101"),
        "M" => String::from("1110000"),
        "!M" => String::from("1110001"),
        "-M" => String::from("1110011"),
        "M+1" => String::from("1110111"),
        "M-1" => String::from("1110010"),
        "D+M" => String::from("1000010"),
        "D-M" => String::from("1010011"),
        "M-D" => String::from("1000111"),
        "D&M" => String::from("1000000"),
        "D|M" => String::from("1010101"),
        _ => String::from("0000000"),
    }
}

fn get_jump_form(jump: &str) -> String {
    match jump {
        "JGT" => String::from("001"),
        "JEQ" => String::from("010"),
        "JGE" => String::from("011"),
        "JLT" => String::from("100"),
        "JNE" => String::from("101"),
        "JLE" => String::from("110"),
        "JMP" => String::from("111"),
        _ => String::from("000"),
    }
}

fn apply_strategy(strategy: BinaryInstructionStrategy, source: &str, destination: &mut String) {
    match strategy {
        BinaryInstructionStrategy::Dest => {
            destination.push_str(&get_dest_form(source));
        },
        BinaryInstructionStrategy::Comp => {
            destination.push_str(&get_comb_form(source));
        },
        BinaryInstructionStrategy::Jump => {
            destination.push_str(&get_jump_form(source));
        },
    }
}

pub fn process_fields(fields: &Vec<ParserFields>) -> Vec<BinaryInstruction> { 
    let mut binary_instructions: Vec<BinaryInstruction> = Vec::new();
    
    // iterate over each field and generate a binary_instruction
    for field in fields.iter() {
        // copy field into instruction
        let reference_field = field.clone();

        // apply technique depending on the isntruction type
        
        let binary_form : Option<String> = match field.instruction_type {
            ParserInstructionType::AInstruction => {
                // convert field.instruction_value to binary
                let u16_value = field.instruction_value.unwrap();

                // get binary form of the instruction
                let binary_value = get_binary_form(u16_value);

                Some(binary_value)
            },
            ParserInstructionType::CInstruction => {
                // knowing it's a C instruction we need to get the binary form 
                // of each part of it as: dest=comp;jump

                // all the c-instructions start with a fixed-set of bits
                let fixed_binary_form = String::from("111");

                // get the final binary form
                let mut final_binary = String::from(fixed_binary_form);

                // check comparison
                if field.instruction_comp.is_some() {
                    // get comp binary form
                    let comp_value = field.instruction_comp.clone().unwrap();
                    apply_strategy(BinaryInstructionStrategy::Comp, comp_value.as_str(), &mut final_binary);
                } else {
                    // 7-bytes comp unused since it does not have the instruction
                    let empty_binary_form = String::from("0000000");
                    final_binary.push_str(&empty_binary_form);
                }

                // check destination
                if field.instruction_dest.is_some() {
                    // get dest binary form
                    let dest_value = field.instruction_dest.clone().unwrap();
                    apply_strategy(BinaryInstructionStrategy::Dest, dest_value.as_str(), &mut final_binary);
                } else {
                    // 3-bytes dest unused since it does not have the instruction
                    let empty_binary_form = String::from("000");
                    final_binary.push_str(&empty_binary_form);
                }

                // check jump if exists
                if field.instruction_jump.is_some() {
                    let jump_value = field.instruction_jump.clone().unwrap();
                    apply_strategy(BinaryInstructionStrategy::Jump, jump_value.as_str(), &mut final_binary);
                } else {
                    // 3-bytes jump unused since it does not nhave the instruction
                    let empty_binary_form = String::from("000");
                    final_binary.push_str(&empty_binary_form);
                }


                Some(final_binary)
            },
            ParserInstructionType::Comment |
            ParserInstructionType::LInstruction => {
                None
            },
        };

        let binary_instruction = BinaryInstruction {
            instruction: reference_field, 
            binary: binary_form.unwrap_or(String::from("<not_implemented_yet>")),
        };

        binary_instructions.push(binary_instruction);
    }

    binary_instructions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_array_of_empty_fields() {
        let fields = vec![];

        let binary_instructions = process_fields(&fields);

        assert_eq!(binary_instructions.len(), 0);
    }

    #[test]
    fn process_array_of_one_field_binary() {
        let unique_field : ParserFields = ParserFields {
                line_number : 0,
                instruction_type: ParserInstructionType::AInstruction,
                instruction_symbol: None,
                instruction_value: Some(10),
                instruction_dest: None,
                instruction_comp: None,
                instruction_jump: None,
        };

        let fields = vec![unique_field];

        let binary_instructions = process_fields(&fields);

        assert_eq!(binary_instructions.len(), 1);
        assert_eq!(binary_instructions[0].instruction.line_number, 0);
        assert_eq!(binary_instructions[0].instruction.instruction_type, ParserInstructionType::AInstruction);
        assert_eq!(binary_instructions[0].instruction.instruction_value, Some(10));
        assert_eq!(binary_instructions[0].binary, String::from("0000000000001010"));
    }

    #[test]
    fn process_field_non_implemented_yet() {
        let unique_field : ParserFields = ParserFields {
                line_number : 0,
                instruction_type: ParserInstructionType::CInstruction,
                instruction_symbol: None,
                instruction_value: None,
                instruction_dest: None,
                instruction_comp: None,
                instruction_jump: None,
        };

        let fields = vec![unique_field];

        let binary_instructions = process_fields(&fields);

        assert_eq!(binary_instructions.len(), 1);
        assert_eq!(binary_instructions[0].instruction.line_number, 0);
        assert_eq!(binary_instructions[0].instruction.instruction_type, ParserInstructionType::CInstruction);
        assert_eq!(binary_instructions[0].instruction.instruction_value, None);
        assert_eq!(binary_instructions[0].binary, String::from("1110000000000000"));
    }

    #[test]
    fn process_field_c_instruction() {
        let unique_field : ParserFields = ParserFields {
                line_number : 0,
                instruction_type: ParserInstructionType::CInstruction,
                instruction_symbol: None,
                instruction_value: None,
                instruction_dest: Some(String::from("M")),
                instruction_comp: Some(String::from("1")),
                instruction_jump: None
        };

        let fields = vec![unique_field];

        let binary_instructions = process_fields(&fields);

        assert_eq!(binary_instructions.len(), 1);
        assert_eq!(binary_instructions[0].instruction.line_number, 0);
        assert_eq!(binary_instructions[0].instruction.instruction_type, ParserInstructionType::CInstruction);
        assert_eq!(binary_instructions[0].instruction.instruction_value, None);
        // first three bits are always fixed to 111
        assert_eq!(binary_instructions[0].binary, String::from("1110111111001000"));
    }

    #[test]
    fn process_field_c_instruction_with_dest_and_jump() {
        let unique_field : ParserFields = ParserFields {
                line_number : 0,
                instruction_type: ParserInstructionType::CInstruction,
                instruction_symbol: None,
                instruction_value: None,
                instruction_dest: Some(String::from("M")),
                instruction_comp: Some(String::from("1")),
                instruction_jump: Some(String::from("JGT"))
        };

        let fields = vec![unique_field];

        let binary_instructions = process_fields(&fields);

        assert_eq!(binary_instructions.len(), 1);
        assert_eq!(binary_instructions[0].instruction.line_number, 0);
        assert_eq!(binary_instructions[0].instruction.instruction_type, ParserInstructionType::CInstruction);
        assert_eq!(binary_instructions[0].instruction.instruction_value, None);
        // first three bits are always fixed to 111
        assert_eq!(binary_instructions[0].binary, String::from("1110111111001001"));
    }

    #[test]
    fn process_field_c_instruction_with_dest_and_jump_and_comp() {
        let unique_field : ParserFields = ParserFields {
                line_number : 0,
                instruction_type: ParserInstructionType::CInstruction,
                instruction_symbol: None,
                instruction_value: None,
                instruction_dest: Some(String::from("M")),
                instruction_comp: Some(String::from("D+M")),
                instruction_jump: Some(String::from("JGT"))
        };

        let fields = vec![unique_field];

        let binary_instructions = process_fields(&fields);

        assert_eq!(binary_instructions.len(), 1);
        assert_eq!(binary_instructions[0].instruction.line_number, 0);
        assert_eq!(binary_instructions[0].instruction.instruction_type, ParserInstructionType::CInstruction);
        assert_eq!(binary_instructions[0].instruction.instruction_value, None);
        // first three bits are always fixed to 111
        assert_eq!(binary_instructions[0].binary, String::from("1111000010001001"));
    }
}
