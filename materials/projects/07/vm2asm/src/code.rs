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

                    assert!(index.parse::<u8>().is_ok(), "index must be a number u8");

                    let mut instruction = String::new();

                    match segment.as_str() {
                        "constant" => {
                            // push the constant to the stack
                            instruction.push_str(format!("@{}\n", index).as_str());
                            instruction.push_str("D=A\n");
                        }
                        "local" | "argument" | "this" | "that" | "pointer" => {
                            // get the segment
                            instruction
                                .push_str(format!("@{}\n", Self::return_segment(segment)).as_str());
                            instruction.push_str("D=M\n");

                            // get the index value and add it to the segment
                            instruction.push_str(format!("@{}\n", index).as_str());
                            instruction.push_str("D=D+A\n");

                            // fetch value from memory
                            instruction.push_str("A=D\n");
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
                        "local" | "argument" | "this" | "that" | "pointer" => {
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
                    let operation = reference_command.arg_1.as_ref().unwrap();

                    instructions.push(AssemblyInstruction {
                        instruction: format!("{} implemented yet", operation).to_string(),
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
    fn process_single_command_with_push() {
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
            "@LCL\nD=M\n@2\nD=D+A\nA=D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
    }

    #[test]
    fn process_single_command_with_pop() {
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
}
