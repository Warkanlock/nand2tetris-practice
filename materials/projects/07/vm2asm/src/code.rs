use crate::parser::{Command, CommandType};

#[derive(Debug, PartialEq, Clone)]
pub struct AssemblyInstruction {
    pub instruction: String,
    pub command: Command,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AssemblyGenerator {
    pub instructions: Vec<AssemblyInstruction>,
}

impl AssemblyGenerator {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    fn pick_cache(num: u8) -> String {
        let mut instruction = String::new();

        if num > 15 || num < 13 {
            panic!("cache not available: you can only access R13, R14, R15");
        }

        instruction.push_str(format!("@R{}\n", num).as_str());

        instruction
    }

    fn return_segment(segment: &str) -> &str {
        match segment {
            "local" => "LCL",
            "argument" => "ARG",
            "this" => "THIS",
            "that" => "THAT",
            _ => panic!("segment not found"),
        }
    }

    fn increase_stack() -> String {
        let mut instruction = String::new();

        instruction.push_str("@SP\n");
        instruction.push_str("M=M+1\n");

        instruction
    }

    fn decrease_stack() -> String {
        let mut instruction = String::new();

        instruction.push_str("@SP\n");
        instruction.push_str("M=M-1\n");

        instruction
    }

    fn push_latest_to(address: &str) -> String {
        let mut instruction = String::new();

        instruction.push_str(format!("@{}\n", address).as_str());
        instruction.push_str("A=M\n");
        instruction.push_str("M=D\n");

        instruction
    }

    fn push_latest_to_stack() -> String {
        let mut instruction = String::new();

        instruction.push_str("@SP\n");
        instruction.push_str("A=M\n");
        instruction.push_str("M=D\n");

        instruction
    }

    fn pop_from_stack() -> String {
        let mut instruction = String::new();

        // decrease the stack in n-1
        instruction.push_str(Self::decrease_stack().as_str());

        // get the latest value at RAM[A] where A=SP
        instruction.push_str("D=M\n");

        instruction
    }

    pub fn instructions_to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        for instruction in self.instructions.iter() {
            for byte in instruction.instruction.as_bytes() {
                bytes.push(*byte);
            }
        }

        bytes
    }

    pub fn process_commands(&mut self, commands: &Vec<Command>) {
        let mut instructions: Vec<AssemblyInstruction> = Vec::new();

        for command in commands.iter() {
            let reference_command = command.clone();

            // check the command type
            match reference_command.command_type {
                CommandType::CPush => {
                    let segment = reference_command.arg_1.as_ref().unwrap();
                    let index = reference_command.arg_2.as_ref().unwrap();

                    assert!(
                        index.parse::<u8>().is_ok(),
                        "index must be a number u8 [unsigned integer]"
                    );

                    let mut instruction = String::new();

                    match segment.as_str() {
                        "constant" => {
                            // push the constant to the stack
                            instruction.push_str(format!("@{}\n", index).as_str());
                            instruction.push_str("D=A\n");
                        }
                        "local" | "argument" | "this" | "that" => {
                            // 1. get index value
                            instruction.push_str(format!("@{}\n", index).as_str());
                            instruction.push_str("D=A\n");
                            // 2. get segment at index (segment[base+index])
                            instruction
                                .push_str(format!("@{}\n", Self::return_segment(segment)).as_str());
                            instruction.push_str("A=D+A\n");
                            instruction.push_str("D=M\n"); // RAM[base+segment]
                        }
                        "pointer" => {
                            // 1. check if index 0 or 1
                            match index.as_str() {
                                "0" => {
                                    // should push value of THIS to stack
                                    instruction.push_str(
                                        format!("@{}\n", Self::return_segment("this")).as_str(),
                                    );
                                    instruction.push_str("D=M\n"); // RAM[this]
                                }
                                "1" => {
                                    // should push value of THIS to stack
                                    instruction.push_str(
                                        format!("@{}\n", Self::return_segment("that")).as_str(),
                                    );
                                    instruction.push_str("D=M\n"); // RAM[that]
                                }
                                _ => panic!("invalid pointer instruction"),
                            }
                        }
                        "temp" => {
                            // should access i[0-7] RAM location
                            let temp_base = 5;
                            let temp_index = index.parse::<u8>().unwrap();

                            assert!(temp_index < 8, "temp index must be between 0 and 7");

                            instruction.push_str(format!("@{}\n", temp_base + temp_index).as_str());
                            instruction.push_str("D=M\n");
                        }
                        "static" => {
                            // should access static i[16-255] RAM location
                            let static_base = 16;
                            let static_index = index.parse::<u8>().unwrap();

                            assert!(static_index < 16, "static index must be between 16 and 255");

                            instruction
                                .push_str(format!("@{}\n", static_base + static_index).as_str());
                            instruction.push_str("D=M\n");
                        }
                        _ => {
                            panic!("segment not valid");
                        }
                    }

                    // push latest value available at RAM[A] to stack
                    instruction.push_str(Self::push_latest_to_stack().as_str());

                    // increase the stack
                    instruction.push_str(Self::increase_stack().as_str());

                    // add the instruction to the list
                    instructions.push(AssemblyInstruction {
                        instruction,
                        command: reference_command,
                    });
                }
                CommandType::CPop => {
                    let segment = reference_command.arg_1.as_ref().unwrap();
                    let index = reference_command.arg_2.as_ref().unwrap();
                    let default_cache: u8 = 13;

                    assert!(index.parse::<u8>().is_ok(), "index must be a number u8");

                    let mut instruction = String::new();

                    match segment.as_str() {
                        "local" | "argument" | "this" | "that" => {
                            // get the segment
                            instruction
                                .push_str(format!("@{}\n", Self::return_segment(segment)).as_str());
                            instruction.push_str("D=M\n");

                            // get the index value and add it to the segment
                            instruction.push_str(format!("@{}\n", index).as_str());
                            instruction.push_str("D=D+A\n");

                            // store the value in R13
                            instruction.push_str(Self::pick_cache(default_cache).as_str());
                            instruction.push_str("M=D\n");

                            // decrease the stack
                            instruction.push_str(Self::decrease_stack().as_str());

                            // get the latest value from the stack
                            instruction.push_str("D=M\n");

                            // push latest to R13
                            instruction.push_str(
                                Self::push_latest_to(format!("R{}", default_cache).as_str())
                                    .as_str(),
                            );
                        }
                        "pointer" => {
                            instruction.push_str(Self::pop_from_stack().as_str()); // D=RAM[M] which is the value to be stored

                            // 1. check if index 0 or 1
                            match index.as_str() {
                                "0" => {
                                    instruction.push_str(
                                        format!("@{}\n", Self::return_segment("this")).as_str(),
                                    );
                                    instruction.push_str("M=D\n");
                                }
                                "1" => {
                                    instruction.push_str(
                                        format!("@{}\n", Self::return_segment("that")).as_str(),
                                    );
                                    instruction.push_str("M=D\n");
                                }
                                _ => panic!("invalid pointer instruction"),
                            }
                        }
                        "temp" => {
                            instruction.push_str(Self::pop_from_stack().as_str());

                            // should access i[0-7] RAM location
                            let temp_base = 5;
                            let temp_index = index.parse::<u8>().unwrap();

                            assert!(temp_index < 8, "temp index must be between 0 and 7");

                            instruction.push_str(format!("@{}\n", temp_base + temp_index).as_str());
                            instruction.push_str("M=D\n");
                        }
                        "static" => {
                            instruction.push_str(Self::pop_from_stack().as_str());

                            // should access static i[16-255] RAM location
                            let static_base = 16;
                            let static_index = index.parse::<u8>().unwrap();

                            assert!(static_index < 16, "static index must be between 16 and 255");

                            instruction
                                .push_str(format!("@{}\n", static_base + static_index).as_str());
                            instruction.push_str("M=D\n");
                        }
                        _ => {
                            panic!("segment not valid");
                        }
                    }

                    // add the instruction to the list
                    instructions.push(AssemblyInstruction {
                        instruction,
                        command: reference_command,
                    });
                }
                // if it's arithmetic, we only have one argument and we need to
                // check what's the operation based on that
                CommandType::CArithmetic => {
                    let mut instruction = String::new();
                    let operation = reference_command.arg_1.as_ref().unwrap();

                    match operation.as_str() {
                        "add" => {
                            // 1. pop value from stack
                            // 2. pop value from stack
                            // 3. add the two values
                            instruction.push_str(Self::pop_from_stack().as_str());
                            instruction.push_str(Self::decrease_stack().as_str());
                            instruction.push_str("D=D+M\n");
                            instruction.push_str(Self::push_latest_to_stack().as_str());
                            instruction.push_str(Self::increase_stack().as_str());
                        }
                        "sub" => {
                            // 1. pop value from stack
                            // 2. pop value from stack
                            // 3. subtract the two values
                            instruction.push_str(Self::pop_from_stack().as_str());
                            instruction.push_str(Self::decrease_stack().as_str());
                            instruction.push_str("D=M-D\n");
                            instruction.push_str(Self::push_latest_to_stack().as_str());
                            instruction.push_str(Self::increase_stack().as_str());
                        }
                        "neg" => {
                            // 1. pop value from stack
                            // 2. negate the value
                            instruction.push_str(Self::pop_from_stack().as_str());
                            instruction.push_str("D=-D\n");
                            instruction.push_str(Self::push_latest_to_stack().as_str());
                            instruction.push_str(Self::increase_stack().as_str());
                        }
                        "eq" => {
                            // 1. pop value from stack
                            // 2. pop value from stack
                            // 3. compare the two values
                            // 4. if equal, push -1 to stack, else push 0
                            instruction.push_str(Self::pop_from_stack().as_str());
                            instruction.push_str(Self::decrease_stack().as_str());
                            instruction.push_str("D=M-D\n");
                            instruction.push_str("@EQ_TRUE\n");
                            instruction.push_str("D;JEQ\n");
                            instruction.push_str("D=0\n");
                            instruction.push_str("@EQ_END\n");
                            instruction.push_str("0;JMP\n");
                            instruction.push_str("(EQ_TRUE)\n");
                            instruction.push_str("D=-1\n");
                            instruction.push_str("(EQ_END)\n");
                            instruction.push_str(Self::push_latest_to_stack().as_str());
                            instruction.push_str(Self::increase_stack().as_str());
                        }
                        "gt" => {
                            // 1. pop value from stack
                            // 2. pop value from stack
                            // 3. compare the two values
                            // 4. if greater, push -1 to stack, else push 0
                            instruction.push_str(Self::pop_from_stack().as_str());
                            instruction.push_str(Self::decrease_stack().as_str());
                            instruction.push_str("D=M-D\n");
                            instruction.push_str("@GT_TRUE\n");
                            instruction.push_str("D;JGT\n");
                            instruction.push_str("D=0\n");
                            instruction.push_str("@GT_END\n");
                            instruction.push_str("0;JMP\n");
                            instruction.push_str("(GT_TRUE)\n");
                            instruction.push_str("D=-1\n");
                            instruction.push_str("(GT_END)\n");
                            instruction.push_str(Self::push_latest_to_stack().as_str());
                            instruction.push_str(Self::increase_stack().as_str());
                        }
                        "lt" => {
                            // 1. pop value from stack
                            // 2. pop value from stack
                            // 3. compare the two values
                            // 4. if less, push -1 to stack, else push 0
                            instruction.push_str(Self::pop_from_stack().as_str());
                            instruction.push_str(Self::decrease_stack().as_str());
                            instruction.push_str("D=M-D\n");
                            instruction.push_str("@LT_TRUE\n");
                            instruction.push_str("D;JLT\n");
                            instruction.push_str("D=0\n");
                            instruction.push_str("@LT_END\n");
                            instruction.push_str("0;JMP\n");
                            instruction.push_str("(LT_TRUE)\n");
                            instruction.push_str("D=-1\n");
                            instruction.push_str("(LT_END)\n");
                            instruction.push_str(Self::push_latest_to_stack().as_str());
                            instruction.push_str(Self::increase_stack().as_str());
                        }
                        "and" => {
                            // 1. pop value from stack
                            // 2. pop value from stack
                            // 3. and the two values
                            instruction.push_str(Self::pop_from_stack().as_str());
                            instruction.push_str(Self::decrease_stack().as_str());
                            instruction.push_str("D=D&M\n");
                            instruction.push_str(Self::push_latest_to_stack().as_str());
                            instruction.push_str(Self::increase_stack().as_str());
                        }
                        "or" => {
                            // 1. pop value from stack
                            // 2. pop value from stack
                            // 3. or the two values
                            instruction.push_str(Self::pop_from_stack().as_str());
                            instruction.push_str(Self::decrease_stack().as_str());
                            instruction.push_str("D=D|M\n");
                            instruction.push_str(Self::push_latest_to_stack().as_str());
                            instruction.push_str(Self::increase_stack().as_str());
                        }
                        "not" => {
                            // 1. pop value from stack
                            // 2. not the value
                            instruction.push_str(Self::pop_from_stack().as_str());
                            instruction.push_str("D=!D\n");
                            instruction.push_str(Self::push_latest_to_stack().as_str());
                            instruction.push_str(Self::increase_stack().as_str());
                        }
                        _ => {
                            panic!("{} implemented yet", operation);
                        }
                    }

                    instructions.push(AssemblyInstruction {
                        instruction,
                        command: reference_command,
                    });
                }
            }
        }

        self.instructions = instructions;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_single_command() {
        // initialize a list of valid commands
        // push local 2
        // push local 8
        // add
        let commands = vec![
            Command {
                command_type: CommandType::CPush,
                arg_1: Some("local".to_string()),
                arg_2: Some("2".to_string()),
            },
            Command {
                command_type: CommandType::CPush,
                arg_1: Some("local".to_string()),
                arg_2: Some("8".to_string()),
            },
            Command {
                command_type: CommandType::CArithmetic,
                arg_1: Some("add".to_string()),
                arg_2: None,
            },
        ];

        let mut generator = AssemblyGenerator::new();

        generator.process_commands(&commands);

        assert_eq!(generator.instructions.len(), 3);
    }

    #[test]
    fn process_push_command() {
        // initialize a list of valid commands
        // push local 2
        let commands = vec![Command {
            command_type: CommandType::CPush,
            arg_1: Some("local".to_string()),
            arg_2: Some("2".to_string()),
        }];

        let mut generator = AssemblyGenerator::new();

        generator.process_commands(&commands);

        assert_eq!(generator.instructions.len(), 1);
        assert_eq!(
            generator.instructions[0].instruction,
            "@2\nD=A\n@LCL\nA=D+A\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
    }

    #[test]
    #[should_panic]
    fn should_fail_on_negative_index() {
        // initialize a list of valid commands
        // push local -2
        let commands = vec![Command {
            command_type: CommandType::CPush,
            arg_1: Some("local".to_string()),
            arg_2: Some("-2".to_string()),
        }];

        let mut generator = AssemblyGenerator::new();

        generator.process_commands(&commands);
    }

    #[test]
    fn process_pop_command() {
        // initialize a list of valid commands
        // pop local 2
        let commands = vec![Command {
            command_type: CommandType::CPop,
            arg_1: Some("local".to_string()),
            arg_2: Some("2".to_string()),
        }];

        let mut generator = AssemblyGenerator::new();

        generator.process_commands(&commands);

        assert_eq!(generator.instructions.len(), 1);
        assert_eq!(
            generator.instructions[0].instruction,
            "@LCL\nD=M\n@2\nD=D+A\n@R13\nM=D\n@SP\nM=M-1\nD=M\n@R13\nA=M\nM=D\n"
        );
    }

    #[test]
    fn process_push_static_command() {
        // initialize a list of valid commands
        // push static 2
        let commands = vec![Command {
            command_type: CommandType::CPush,
            arg_1: Some("static".to_string()),
            arg_2: Some("2".to_string()),
        }];

        let mut generator = AssemblyGenerator::new();

        generator.process_commands(&commands);

        assert_eq!(generator.instructions.len(), 1);
        assert_eq!(
            generator.instructions[0].instruction,
            "@18\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
    }

    #[test]
    fn process_push_pointer_command() {
        // initialize a list of valid commands
        // push pointer 0
        let commands = vec![Command {
            command_type: CommandType::CPush,
            arg_1: Some("pointer".to_string()),
            arg_2: Some("0".to_string()),
        }];

        let mut generator = AssemblyGenerator::new();

        generator.process_commands(&commands);

        assert_eq!(generator.instructions.len(), 1);
        assert_eq!(
            generator.instructions[0].instruction,
            "@THIS\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
    }

    #[test]
    fn process_push_temp_command() {
        // initialize a list of valid commands
        // push temp 2
        let commands = vec![Command {
            command_type: CommandType::CPush,
            arg_1: Some("temp".to_string()),
            arg_2: Some("2".to_string()),
        }];

        let mut generator = AssemblyGenerator::new();

        generator.process_commands(&commands);

        assert_eq!(generator.instructions.len(), 1);
        assert_eq!(
            generator.instructions[0].instruction,
            "@7\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
    }

    #[test]
    fn process_pop_pointer_command() {
        // initialize a list of valid commands
        // pop pointer 0
        let commands = vec![Command {
            command_type: CommandType::CPop,
            arg_1: Some("pointer".to_string()),
            arg_2: Some("0".to_string()),
        }];

        let mut generator = AssemblyGenerator::new();

        generator.process_commands(&commands);

        assert_eq!(generator.instructions.len(), 1);
        assert_eq!(
            generator.instructions[0].instruction,
            "@SP\nM=M-1\nD=M\n@THIS\nM=D\n"
        );
    }

    #[test]
    fn process_pop_temp_command() {
        // initialize a list of valid commands
        // pop temp 2
        let commands = vec![Command {
            command_type: CommandType::CPop,
            arg_1: Some("temp".to_string()),
            arg_2: Some("2".to_string()),
        }];

        let mut generator = AssemblyGenerator::new();

        generator.process_commands(&commands);

        assert_eq!(generator.instructions.len(), 1);
        assert_eq!(
            generator.instructions[0].instruction,
            "@SP\nM=M-1\nD=M\n@7\nM=D\n"
        );
    }

    #[test]
    fn process_arithmetic_add_command() {
        let commands = vec![
            Command {
                command_type: CommandType::CPush,
                arg_1: Some("constant".to_string()),
                arg_2: Some("7".to_string()),
            },
            Command {
                command_type: CommandType::CPush,
                arg_1: Some("constant".to_string()),
                arg_2: Some("8".to_string()),
            },
            Command {
                command_type: CommandType::CArithmetic,
                arg_1: Some("add".to_string()),
                arg_2: None,
            },
        ];

        let mut generator = AssemblyGenerator::new();

        generator.process_commands(&commands);

        assert_eq!(generator.instructions.len(), 3);
        assert_eq!(
            generator.instructions[0].instruction,
            "@7\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
        assert_eq!(
            generator.instructions[1].instruction,
            "@8\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
        assert_eq!(
            generator.instructions[2].instruction,
            "@SP\nM=M-1\nD=M\n@SP\nM=M-1\nD=D+M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
    }

    #[test]
    fn process_arithmetic_sub_command() {
        let commands = vec![
            Command {
                command_type: CommandType::CPush,
                arg_1: Some("constant".to_string()),
                arg_2: Some("7".to_string()),
            },
            Command {
                command_type: CommandType::CPush,
                arg_1: Some("constant".to_string()),
                arg_2: Some("8".to_string()),
            },
            Command {
                command_type: CommandType::CArithmetic,
                arg_1: Some("sub".to_string()),
                arg_2: None,
            },
        ];

        let mut generator = AssemblyGenerator::new();

        generator.process_commands(&commands);

        assert_eq!(generator.instructions.len(), 3);
        assert_eq!(
            generator.instructions[0].instruction,
            "@7\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
        assert_eq!(
            generator.instructions[1].instruction,
            "@8\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
        assert_eq!(
            generator.instructions[2].instruction,
            "@SP\nM=M-1\nD=M\n@SP\nM=M-1\nD=M-D\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
    }

    #[test]
    fn process_arithmetic_neg_command() {
        let commands = vec![
            Command {
                command_type: CommandType::CPush,
                arg_1: Some("constant".to_string()),
                arg_2: Some("7".to_string()),
            },
            Command {
                command_type: CommandType::CArithmetic,
                arg_1: Some("neg".to_string()),
                arg_2: None,
            },
        ];

        let mut generator = AssemblyGenerator::new();

        generator.process_commands(&commands);

        assert_eq!(generator.instructions.len(), 2);
        assert_eq!(
            generator.instructions[0].instruction,
            "@7\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
        assert_eq!(
            generator.instructions[1].instruction,
            "@SP\nM=M-1\nD=M\nD=-D\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
    }

    #[test]
    fn process_arithmetic_eq_command() {
        let commands = vec![
            Command {
                command_type: CommandType::CPush,
                arg_1: Some("constant".to_string()),
                arg_2: Some("7".to_string()),
            },
            Command {
                command_type: CommandType::CPush,
                arg_1: Some("constant".to_string()),
                arg_2: Some("8".to_string()),
            },
            Command {
                command_type: CommandType::CArithmetic,
                arg_1: Some("eq".to_string()),
                arg_2: None,
            },
        ];

        let mut generator = AssemblyGenerator::new();

        generator.process_commands(&commands);

        assert_eq!(generator.instructions.len(), 3);
        assert_eq!(
            generator.instructions[0].instruction,
            "@7\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
        assert_eq!(
            generator.instructions[1].instruction,
            "@8\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
        assert_eq!(
            generator.instructions[2].instruction,
            "@SP\nM=M-1\nD=M\n@SP\nM=M-1\nD=M-D\n@EQ_TRUE\nD;JEQ\nD=0\n@EQ_END\n0;JMP\n(EQ_TRUE)\nD=-1\n(EQ_END)\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
    }

    #[test]
    fn process_arithmetic_gt_command() {
        let commands = vec![
            Command {
                command_type: CommandType::CPush,
                arg_1: Some("constant".to_string()),
                arg_2: Some("7".to_string()),
            },
            Command {
                command_type: CommandType::CPush,
                arg_1: Some("constant".to_string()),
                arg_2: Some("8".to_string()),
            },
            Command {
                command_type: CommandType::CArithmetic,
                arg_1: Some("gt".to_string()),
                arg_2: None,
            },
        ];

        let mut generator = AssemblyGenerator::new();

        generator.process_commands(&commands);

        assert_eq!(generator.instructions.len(), 3);
        assert_eq!(
            generator.instructions[0].instruction,
            "@7\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
        assert_eq!(
            generator.instructions[1].instruction,
            "@8\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
        assert_eq!(
            generator.instructions[2].instruction,
            "@SP\nM=M-1\nD=M\n@SP\nM=M-1\nD=M-D\n@GT_TRUE\nD;JGT\nD=0\n@GT_END\n0;JMP\n(GT_TRUE)\nD=-1\n(GT_END)\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
    }

    #[test]
    fn process_arithmetic_lt_command() {
        let commands = vec![
            Command {
                command_type: CommandType::CPush,
                arg_1: Some("constant".to_string()),
                arg_2: Some("7".to_string()),
            },
            Command {
                command_type: CommandType::CPush,
                arg_1: Some("constant".to_string()),
                arg_2: Some("8".to_string()),
            },
            Command {
                command_type: CommandType::CArithmetic,
                arg_1: Some("lt".to_string()),
                arg_2: None,
            },
        ];

        let mut generator = AssemblyGenerator::new();

        generator.process_commands(&commands);

        assert_eq!(generator.instructions.len(), 3);
        assert_eq!(
            generator.instructions[0].instruction,
            "@7\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
        assert_eq!(
            generator.instructions[1].instruction,
            "@8\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
        assert_eq!(
            generator.instructions[2].instruction,
            "@SP\nM=M-1\nD=M\n@SP\nM=M-1\nD=M-D\n@LT_TRUE\nD;JLT\nD=0\n@LT_END\n0;JMP\n(LT_TRUE)\nD=-1\n(LT_END)\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
    }

    #[test]
    fn process_arithmetic_and_command() {
        let commands = vec![
            Command {
                command_type: CommandType::CPush,
                arg_1: Some("constant".to_string()),
                arg_2: Some("7".to_string()),
            },
            Command {
                command_type: CommandType::CPush,
                arg_1: Some("constant".to_string()),
                arg_2: Some("8".to_string()),
            },
            Command {
                command_type: CommandType::CArithmetic,
                arg_1: Some("and".to_string()),
                arg_2: None,
            },
        ];

        let mut generator = AssemblyGenerator::new();

        generator.process_commands(&commands);

        assert_eq!(generator.instructions.len(), 3);
        assert_eq!(
            generator.instructions[0].instruction,
            "@7\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
        assert_eq!(
            generator.instructions[1].instruction,
            "@8\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
        assert_eq!(
            generator.instructions[2].instruction,
            "@SP\nM=M-1\nD=M\n@SP\nM=M-1\nD=D&M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
    }

    #[test]
    fn process_arithmetic_or_command() {
        let commands = vec![
            Command {
                command_type: CommandType::CPush,
                arg_1: Some("constant".to_string()),
                arg_2: Some("7".to_string()),
            },
            Command {
                command_type: CommandType::CPush,
                arg_1: Some("constant".to_string()),
                arg_2: Some("8".to_string()),
            },
            Command {
                command_type: CommandType::CArithmetic,
                arg_1: Some("or".to_string()),
                arg_2: None,
            },
        ];

        let mut generator = AssemblyGenerator::new();

        generator.process_commands(&commands);

        assert_eq!(generator.instructions.len(), 3);
        assert_eq!(
            generator.instructions[0].instruction,
            "@7\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
        assert_eq!(
            generator.instructions[1].instruction,
            "@8\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
        assert_eq!(
            generator.instructions[2].instruction,
            "@SP\nM=M-1\nD=M\n@SP\nM=M-1\nD=D|M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
    }

    #[test]
    fn process_arithmetic_not_command() {
        let commands = vec![
            Command {
                command_type: CommandType::CPush,
                arg_1: Some("constant".to_string()),
                arg_2: Some("7".to_string()),
            },
            Command {
                command_type: CommandType::CArithmetic,
                arg_1: Some("not".to_string()),
                arg_2: None,
            },
        ];

        let mut generator = AssemblyGenerator::new();

        generator.process_commands(&commands);

        assert_eq!(generator.instructions.len(), 2);
        assert_eq!(
            generator.instructions[0].instruction,
            "@7\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
        assert_eq!(
            generator.instructions[1].instruction,
            "@SP\nM=M-1\nD=M\nD=!D\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
    }

}
