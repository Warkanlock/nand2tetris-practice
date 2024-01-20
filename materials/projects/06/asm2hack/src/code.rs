use crate::parser::{ParserFields, ParserInstructionType};

pub struct BinaryInstruction {
    pub instruction: ParserFields,
    pub binary: String,
}

fn get_binary_form(number : u16) -> String {
    (0..16) 
        .rev()
        .map(|i| (number >> i) & 1) // this will get the i bit
        .map(|b| b.to_string()) // convert to string
        .collect::<Vec<String>>()
        .join("")
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
            ParserInstructionType::CInstruction |
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
        assert_eq!(binary_instructions[0].binary, String::from("<not_implemented_yet>"));
    }
}
