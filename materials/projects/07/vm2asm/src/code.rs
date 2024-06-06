use crate::parser::{Command, CommandType};

#[derive(Debug, PartialEq, Clone)]
pub struct AssemblyInstruction {
    pub instruction: String,
    pub command: Command,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AssemblyGenerator {
    pub instructions: Vec<AssemblyInstruction>,
    pub should_bootstrap: bool,
    pub last_function : String,
}

pub struct AssemblyConfiguration {
    pub bootstrap: bool,
}

impl AssemblyGenerator {
    pub fn new(config: AssemblyConfiguration) -> Self {
        Self {
            instructions: Vec::new(),
            should_bootstrap: config.bootstrap,
            last_function: "default".to_string()
        }
    }

    fn add_infinite_loop() -> String {
        let mut instruction = String::new();

        instruction.push_str("(END)\n");
        instruction.push_str("@END\n");
        instruction.push_str("0;JMP\n");

        instruction
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

    fn decrease_stack() -> String {
        let mut instruction = String::new();

        // decrease the stack in n-1
        instruction.push_str("@SP\n");
        instruction.push_str("A=M-1\n");

        instruction
    }

    fn pop_from_stack() -> String {
        let mut instruction = String::new();

        // decrease the stack in n-1
        instruction.push_str("@SP\n");
        instruction.push_str("AM=M-1\n");

        instruction
    }

    fn call(function_name: &str) -> String {
        let mut instruction = String::new();

        instruction.push_str(format!("@{}\n", function_name).as_str());
        instruction.push_str("0;JMP\n");

        instruction
    }

    fn set_stack_pointer_to(value: u16) -> String {
        let mut instruction = String::new();

        instruction.push_str(format!("@{}\n", value).as_str());
        instruction.push_str("D=A\n");
        instruction.push_str("@SP\n");
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

    fn generate_label(&mut self, command: &Command) -> String {
        let label = command.arg_1.as_ref().unwrap();
        let file_name = command.classname.as_ref().unwrap();

        format!("{}.{}${}", file_name, self.last_function, label)
    }

    pub fn process_commands(&mut self, commands: &Vec<Command>) {
        let mut instructions: Vec<AssemblyInstruction> = Vec::new();
        let mut index_label: u8 = 0;

        // if bootstrap, should add the code of:
        // 1. SP=256
        // 2. call Sys.init
        if self.should_bootstrap {
            instructions.push(AssemblyInstruction {
                instruction: Self::set_stack_pointer_to(256),
                command: Command {
                    command_type: CommandType::CPush,
                    arg_1: Some("bootstrap".to_string()),
                    arg_2: None,
                    classname: None,
                },
            });

            instructions.push(AssemblyInstruction {
                instruction: Self::call("Sys.init"),
                command: Command {
                    command_type: CommandType::CCall,
                    arg_1: Some("bootstrap".to_string()),
                    arg_2: None,
                    classname: None,
                },
            });
        }

        for command in commands.iter() {
            let reference_command = command.clone();

            // check the command type
            match reference_command.command_type {
                CommandType::CPush => {
                    let segment = reference_command.arg_1.as_ref().unwrap();
                    let index = reference_command.arg_2.as_ref().unwrap();

                    assert!(
                        index.parse::<u32>().is_ok(),
                        "index must be a number u32 [unsigned integer]"
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

                            // this will pick the underlying file name
                            let classname = reference_command.classname.as_ref().unwrap();

                            instruction.push_str(
                                format!("@{}.{}\n", classname, static_base + static_index).as_str(),
                            );
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

                            // pop value from stack
                            instruction.push_str(Self::pop_from_stack().as_str());

                            // get the latest value from the stack
                            instruction.push_str("D=M\n");

                            // push latest to R13
                            instruction.push_str(
                                Self::push_latest_to(format!("R{}", default_cache).as_str())
                                    .as_str(),
                            );
                        }
                        "pointer" => {
                            // pop value from stack
                            instruction.push_str(Self::pop_from_stack().as_str());

                            // get the latest value from the stack
                            instruction.push_str("D=M\n");

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
                            // pop value from stack
                            instruction.push_str(Self::pop_from_stack().as_str());

                            // get the latest value from the stack
                            instruction.push_str("D=M\n");

                            // should access i[0-7] RAM location
                            let temp_base = 5;
                            let temp_index = index.parse::<u8>().unwrap();

                            assert!(temp_index < 8, "temp index must be between 0 and 7");

                            instruction.push_str(format!("@{}\n", temp_base + temp_index).as_str());
                            instruction.push_str("M=D\n");
                        }
                        "static" => {
                            // pop value from stack
                            instruction.push_str(Self::pop_from_stack().as_str());

                            // get the latest value from the stack
                            instruction.push_str("D=M\n");

                            // should access static i[16-255] RAM location
                            let static_base = 16;
                            let static_index = index.parse::<u8>().unwrap();

                            assert!(static_index < 16, "static index must be between 16 and 255");

                            // this will pick the underlying file name
                            let classname = reference_command.classname.as_ref().unwrap();

                            instruction.push_str(
                                format!("@{}.{}\n", classname, static_base + static_index).as_str(),
                            );
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
                            instruction.push_str("D=M\n");
                            instruction.push_str(Self::pop_from_stack().as_str());
                            instruction.push_str("A=M\n");
                            instruction.push_str("D=A+D\n");
                            instruction.push_str(Self::push_latest_to_stack().as_str());
                            instruction.push_str(Self::increase_stack().as_str());
                        }
                        "sub" => {
                            // 1. pop value from stack
                            // 2. pop value from stack
                            // 3. subtract the two values
                            instruction.push_str(Self::pop_from_stack().as_str());
                            instruction.push_str("D=M\n");
                            instruction.push_str(Self::pop_from_stack().as_str());
                            instruction.push_str("A=M\n");
                            instruction.push_str("D=A-D\n");
                            instruction.push_str(Self::push_latest_to_stack().as_str());
                            instruction.push_str(Self::increase_stack().as_str());
                        }
                        "neg" => {
                            // 1. pop value from stack
                            // 2. negate the value
                            instruction.push_str(Self::decrease_stack().as_str());
                            instruction.push_str("M=-M\n");
                        }
                        "eq" => {
                            index_label += 1;
                            let start_tag = format!("EQ_START_{}", index_label);
                            index_label += 1;
                            let end_tag = format!("EQ_END_{}", index_label);

                            // 1. pop value from stack
                            // 2. pop value from stack
                            // 3. compare the two values
                            // 4. if equal, push -1 to stack, else push 0
                            instruction.push_str(Self::pop_from_stack().as_str());
                            instruction.push_str("D=M\n");
                            instruction.push_str(Self::pop_from_stack().as_str());
                            instruction.push_str("A=M\n");
                            instruction.push_str("D=A-D\n");
                            instruction.push_str(format!("@{}\n", start_tag).as_str());
                            instruction.push_str("D;JEQ\n");
                            instruction.push_str("D=0\n");
                            instruction.push_str(Self::push_latest_to_stack().as_str());
                            instruction.push_str(Self::increase_stack().as_str());
                            instruction.push_str(format!("@{}\n", end_tag).as_str());
                            instruction.push_str("0;JMP\n");
                            instruction.push_str(format!("({})\n", start_tag).as_str());
                            instruction.push_str("D=-1\n");
                            instruction.push_str(Self::push_latest_to_stack().as_str());
                            instruction.push_str(Self::increase_stack().as_str());
                            instruction.push_str(format!("({})\n", end_tag).as_str());
                        }
                        "gt" => {
                            index_label += 1;
                            let start_tag = format!("GT_START_{}", index_label);
                            index_label += 1;
                            let end_tag = format!("GT_END_{}", index_label);

                            // 1. pop value from stack
                            // 2. pop value from stack
                            // 3. compare the two values
                            // 4. if greater, push -1 to stack, else push 0
                            instruction.push_str(Self::pop_from_stack().as_str());
                            instruction.push_str("D=M\n");
                            instruction.push_str(Self::pop_from_stack().as_str());
                            instruction.push_str("A=M\n");
                            instruction.push_str("D=A-D\n");

                            instruction.push_str(format!("@{}\n", start_tag).as_str());
                            instruction.push_str("D;JGT\n");
                            instruction.push_str("D=0\n");
                            instruction.push_str(Self::push_latest_to_stack().as_str());
                            instruction.push_str(Self::increase_stack().as_str());

                            instruction.push_str(format!("@{}\n", end_tag).as_str());
                            instruction.push_str("0;JMP\n");
                            instruction.push_str(format!("({})\n", start_tag).as_str());
                            instruction.push_str("D=-1\n");
                            instruction.push_str(Self::push_latest_to_stack().as_str());
                            instruction.push_str(Self::increase_stack().as_str());
                            instruction.push_str(format!("({})\n", end_tag).as_str());
                        }
                        "lt" => {
                            index_label += 1;
                            let start_tag = format!("LT_START_{}", index_label);
                            index_label += 1;
                            let end_tag = format!("LT_END_{}", index_label);

                            // 1. pop value from stack
                            // 3. compare the two values
                            // 4. if less, push -1 to stack, else push 0
                            instruction.push_str(Self::pop_from_stack().as_str());
                            instruction.push_str("D=M\n");
                            instruction.push_str(Self::pop_from_stack().as_str());
                            instruction.push_str("A=M\n");
                            instruction.push_str("D=A-D\n");

                            instruction.push_str(format!("@{}\n", start_tag).as_str());
                            instruction.push_str("D;JLT\n");
                            instruction.push_str("D=0\n");
                            instruction.push_str(Self::push_latest_to_stack().as_str());
                            instruction.push_str(Self::increase_stack().as_str());

                            instruction.push_str(format!("@{}\n", end_tag).as_str());
                            instruction.push_str("0;JMP\n");
                            instruction.push_str(format!("({})\n", start_tag).as_str());
                            instruction.push_str("D=-1\n");
                            instruction.push_str(Self::push_latest_to_stack().as_str());
                            instruction.push_str(Self::increase_stack().as_str());
                            instruction.push_str(format!("({})\n", end_tag).as_str());
                        }
                        "and" => {
                            // 1. pop value from stack
                            // 2. pop value from stack
                            // 3. and the two values
                            instruction.push_str(Self::pop_from_stack().as_str());
                            instruction.push_str("D=M\n");
                            instruction.push_str(Self::pop_from_stack().as_str());
                            instruction.push_str("A=M\n");
                            instruction.push_str("D=D&A\n");
                            instruction.push_str(Self::push_latest_to_stack().as_str());
                            instruction.push_str(Self::increase_stack().as_str());
                        }
                        "or" => {
                            // 1. pop value from stack
                            // 2. pop value from stack
                            // 3. or the two values
                            instruction.push_str(Self::pop_from_stack().as_str());
                            instruction.push_str("D=M\n");
                            instruction.push_str(Self::pop_from_stack().as_str());
                            instruction.push_str("A=M\n");
                            instruction.push_str("D=D|A\n");
                            instruction.push_str(Self::push_latest_to_stack().as_str());
                            instruction.push_str(Self::increase_stack().as_str());
                        }
                        "not" => {
                            // 1. pop value from stack
                            // 2. not the value
                            instruction.push_str(Self::decrease_stack().as_str());
                            instruction.push_str("M=!M\n");
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
                CommandType::CLabel => {
                    let mut instruction: String = String::new();

                    let final_name = self.generate_label(&reference_command);

                    instruction.push_str(format!("({})\n", final_name).as_str());

                    instructions.push(AssemblyInstruction {
                        instruction,
                        command: reference_command,
                    })
                }
                CommandType::CIf => {
                    let mut instruction: String = String::new();

                    let final_name = self.generate_label(&reference_command);

                    instruction.push_str(Self::pop_from_stack().as_str());
                    instruction.push_str("D=M\n");
                    instruction.push_str(format!("@{}\n", final_name).as_str());
                    instruction.push_str("D;JNE\n");

                    instructions.push(AssemblyInstruction {
                        instruction,
                        command: reference_command,
                    })
                }
                CommandType::CGoto => {
                    let mut instruction: String = String::new();

                    let file_name = reference_command.classname.as_ref().unwrap();
                    let label = reference_command.arg_1.as_ref().unwrap();
                    let final_name = format!("{}.{}${}", file_name, self.last_function, label);

                    instruction.push_str(format!("@{}\n", final_name).as_str());
                    instruction.push_str("0;JMP\n");

                    instructions.push(AssemblyInstruction {
                        instruction,
                        command: reference_command,
                    })
                }
                CommandType::CCall => panic!("not implemented yet"),
                CommandType::CFunction => {
                    let function_name = reference_command.arg_1.as_ref().unwrap();
                    let num_locals = reference_command
                        .arg_2
                        .as_ref()
                        .unwrap()
                        .parse::<u16>()
                        .unwrap();
                    let classname: &String = reference_command.classname.as_ref().unwrap();

                    // copy function name to the last function being called
                    self.last_function = function_name.clone();
                }
                CommandType::CReturn => panic!("not implemented yet"),
            }
        }

        // add infinite loop to the end of the program
        instructions.push(AssemblyInstruction {
            instruction: Self::add_infinite_loop(),
            command: Command {
                command_type: CommandType::CArithmetic,
                arg_1: Some("end".to_string()),
                arg_2: None,
                classname: None,
            },
        });

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
                classname: None,
            },
            Command {
                command_type: CommandType::CPush,
                arg_1: Some("local".to_string()),
                arg_2: Some("8".to_string()),
                classname: None,
            },
            Command {
                command_type: CommandType::CArithmetic,
                arg_1: Some("add".to_string()),
                arg_2: None,
                classname: None,
            },
        ];

        let mut generator = AssemblyGenerator::new(AssemblyConfiguration { bootstrap: false });

        generator.process_commands(&commands);

        assert_eq!(generator.instructions.len(), 4);
    }

    #[test]
    fn process_push_command() {
        // initialize a list of valid commands
        // push local 2
        let commands = vec![Command {
            command_type: CommandType::CPush,
            arg_1: Some("local".to_string()),
            arg_2: Some("2".to_string()),
            classname: None,
        }];

        let mut generator = AssemblyGenerator::new(AssemblyConfiguration { bootstrap: false });

        generator.process_commands(&commands);

        assert_eq!(generator.instructions.len(), 2);
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
            classname: None,
        }];

        let mut generator = AssemblyGenerator::new(AssemblyConfiguration { bootstrap: false });

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
            classname: None,
        }];

        let mut generator = AssemblyGenerator::new(AssemblyConfiguration { bootstrap: false });

        generator.process_commands(&commands);

        assert_eq!(generator.instructions.len(), 2);
        assert_eq!(
            generator.instructions[0].instruction,
            "@LCL\nD=M\n@2\nD=D+A\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n"
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
            classname: Some("test".to_string()),
        }];

        let mut generator = AssemblyGenerator::new(AssemblyConfiguration { bootstrap: false });

        generator.process_commands(&commands);

        assert_eq!(generator.instructions.len(), 2);
        assert_eq!(
            generator.instructions[0].instruction,
            "@test.18\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
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
            classname: None,
        }];

        let mut generator = AssemblyGenerator::new(AssemblyConfiguration { bootstrap: false });

        generator.process_commands(&commands);

        assert_eq!(generator.instructions.len(), 2);
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
            classname: None,
        }];

        let mut generator = AssemblyGenerator::new(AssemblyConfiguration { bootstrap: false });

        generator.process_commands(&commands);

        assert_eq!(generator.instructions.len(), 2);
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
            classname: None,
        }];

        let mut generator = AssemblyGenerator::new(AssemblyConfiguration { bootstrap: false });

        generator.process_commands(&commands);

        assert_eq!(generator.instructions.len(), 2);
        assert_eq!(
            generator.instructions[0].instruction,
            "@SP\nAM=M-1\nD=M\n@THIS\nM=D\n"
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
            classname: None,
        }];

        let mut generator = AssemblyGenerator::new(AssemblyConfiguration { bootstrap: false });

        generator.process_commands(&commands);

        assert_eq!(generator.instructions.len(), 2);
        assert_eq!(
            generator.instructions[0].instruction,
            "@SP\nAM=M-1\nD=M\n@7\nM=D\n"
        );
    }

    #[test]
    fn process_arithmetic_add_command() {
        let commands = vec![
            Command {
                command_type: CommandType::CPush,
                arg_1: Some("constant".to_string()),
                arg_2: Some("7".to_string()),
                classname: None,
            },
            Command {
                command_type: CommandType::CPush,
                arg_1: Some("constant".to_string()),
                arg_2: Some("8".to_string()),
                classname: None,
            },
            Command {
                command_type: CommandType::CArithmetic,
                arg_1: Some("add".to_string()),
                arg_2: None,
                classname: None,
            },
        ];

        let mut generator = AssemblyGenerator::new(AssemblyConfiguration { bootstrap: false });

        generator.process_commands(&commands);

        assert_eq!(generator.instructions.len(), 4);
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
            "@SP\nAM=M-1\nD=M\n@SP\nAM=M-1\nA=M\nD=A+D\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
    }

    #[test]
    fn process_arithmetic_sub_command() {
        let commands = vec![
            Command {
                command_type: CommandType::CPush,
                arg_1: Some("constant".to_string()),
                arg_2: Some("7".to_string()),
                classname: None,
            },
            Command {
                command_type: CommandType::CPush,
                arg_1: Some("constant".to_string()),
                arg_2: Some("8".to_string()),
                classname: None,
            },
            Command {
                command_type: CommandType::CArithmetic,
                arg_1: Some("sub".to_string()),
                arg_2: None,
                classname: None,
            },
        ];

        let mut generator = AssemblyGenerator::new(AssemblyConfiguration { bootstrap: false });

        generator.process_commands(&commands);

        assert_eq!(generator.instructions.len(), 4);
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
            "@SP\nAM=M-1\nD=M\n@SP\nAM=M-1\nA=M\nD=A-D\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
    }

    #[test]
    fn process_arithmetic_neg_command() {
        let commands = vec![
            Command {
                command_type: CommandType::CPush,
                arg_1: Some("constant".to_string()),
                arg_2: Some("7".to_string()),
                classname: None,
            },
            Command {
                command_type: CommandType::CArithmetic,
                arg_1: Some("neg".to_string()),
                arg_2: None,
                classname: None,
            },
        ];

        let mut generator = AssemblyGenerator::new(AssemblyConfiguration { bootstrap: false });

        generator.process_commands(&commands);

        assert_eq!(generator.instructions.len(), 3);
        assert_eq!(
            generator.instructions[0].instruction,
            "@7\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
        assert_eq!(generator.instructions[1].instruction, "@SP\nA=M-1\nM=-M\n");
    }

    #[test]
    fn process_arithmetic_eq_command() {
        let commands = vec![
            Command {
                command_type: CommandType::CPush,
                arg_1: Some("constant".to_string()),
                arg_2: Some("7".to_string()),
                classname: None,
            },
            Command {
                command_type: CommandType::CPush,
                arg_1: Some("constant".to_string()),
                arg_2: Some("8".to_string()),
                classname: None,
            },
            Command {
                command_type: CommandType::CArithmetic,
                arg_1: Some("eq".to_string()),
                arg_2: None,
                classname: None,
            },
        ];

        let mut generator = AssemblyGenerator::new(AssemblyConfiguration { bootstrap: false });

        generator.process_commands(&commands);

        assert_eq!(generator.instructions.len(), 4);
        assert_eq!(
            generator.instructions[0].instruction,
            "@7\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
        assert_eq!(
            generator.instructions[1].instruction,
            "@8\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
    }

    #[test]
    fn process_arithmetic_gt_command_with_bootstrap() {
        let commands = vec![
            Command {
                command_type: CommandType::CPush,
                arg_1: Some("constant".to_string()),
                arg_2: Some("7".to_string()),
                classname: None,
            },
            Command {
                command_type: CommandType::CPush,
                arg_1: Some("constant".to_string()),
                arg_2: Some("8".to_string()),
                classname: None,
            },
            Command {
                command_type: CommandType::CArithmetic,
                arg_1: Some("gt".to_string()),
                arg_2: None,
                classname: None,
            },
        ];

        let mut generator = AssemblyGenerator::new(AssemblyConfiguration { bootstrap: true });

        generator.process_commands(&commands);

        assert_eq!(generator.instructions.len(), 6);

        assert_eq!(
            generator.instructions[0].instruction,
            "@256\nD=A\n@SP\nM=D\n"
        );
        assert_eq!(generator.instructions[1].instruction, "@Sys.init\n0;JMP\n");
        assert_eq!(
            generator.instructions[2].instruction,
            "@7\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
        assert_eq!(
            generator.instructions[3].instruction,
            "@8\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
    }

    #[test]
    fn process_arithmetic_gt_command() {
        let commands = vec![
            Command {
                command_type: CommandType::CPush,
                arg_1: Some("constant".to_string()),
                arg_2: Some("7".to_string()),
                classname: None,
            },
            Command {
                command_type: CommandType::CPush,
                arg_1: Some("constant".to_string()),
                arg_2: Some("8".to_string()),
                classname: None,
            },
            Command {
                command_type: CommandType::CArithmetic,
                arg_1: Some("gt".to_string()),
                arg_2: None,
                classname: None,
            },
        ];

        let mut generator = AssemblyGenerator::new(AssemblyConfiguration { bootstrap: false });

        generator.process_commands(&commands);

        assert_eq!(generator.instructions.len(), 4);
        assert_eq!(
            generator.instructions[0].instruction,
            "@7\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
        assert_eq!(
            generator.instructions[1].instruction,
            "@8\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
    }

    #[test]
    fn process_arithmetic_lt_command() {
        let commands = vec![
            Command {
                command_type: CommandType::CPush,
                arg_1: Some("constant".to_string()),
                arg_2: Some("7".to_string()),
                classname: None,
            },
            Command {
                command_type: CommandType::CPush,
                arg_1: Some("constant".to_string()),
                arg_2: Some("8".to_string()),
                classname: None,
            },
            Command {
                command_type: CommandType::CArithmetic,
                arg_1: Some("lt".to_string()),
                arg_2: None,
                classname: None,
            },
        ];

        let mut generator = AssemblyGenerator::new(AssemblyConfiguration { bootstrap: false });

        generator.process_commands(&commands);

        assert_eq!(generator.instructions.len(), 4);
        assert_eq!(
            generator.instructions[0].instruction,
            "@7\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
        assert_eq!(
            generator.instructions[1].instruction,
            "@8\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
    }

    #[test]
    fn process_arithmetic_and_command() {
        let commands = vec![
            Command {
                command_type: CommandType::CPush,
                arg_1: Some("constant".to_string()),
                arg_2: Some("7".to_string()),
                classname: None,
            },
            Command {
                command_type: CommandType::CPush,
                arg_1: Some("constant".to_string()),
                arg_2: Some("8".to_string()),
                classname: None,
            },
            Command {
                command_type: CommandType::CArithmetic,
                arg_1: Some("and".to_string()),
                arg_2: None,
                classname: None,
            },
        ];

        let mut generator = AssemblyGenerator::new(AssemblyConfiguration { bootstrap: false });

        generator.process_commands(&commands);

        assert_eq!(generator.instructions.len(), 4);
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
            "@SP\nAM=M-1\nD=M\n@SP\nAM=M-1\nA=M\nD=D&A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
    }

    #[test]
    fn process_arithmetic_or_command() {
        let commands = vec![
            Command {
                command_type: CommandType::CPush,
                arg_1: Some("constant".to_string()),
                arg_2: Some("7".to_string()),
                classname: None,
            },
            Command {
                command_type: CommandType::CPush,
                arg_1: Some("constant".to_string()),
                arg_2: Some("8".to_string()),
                classname: None,
            },
            Command {
                command_type: CommandType::CArithmetic,
                arg_1: Some("or".to_string()),
                arg_2: None,
                classname: None,
            },
        ];

        let mut generator = AssemblyGenerator::new(AssemblyConfiguration { bootstrap: false });

        generator.process_commands(&commands);

        assert_eq!(generator.instructions.len(), 4);
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
            "@SP\nAM=M-1\nD=M\n@SP\nAM=M-1\nA=M\nD=D|A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
    }

    #[test]
    fn process_arithmetic_not_command() {
        let commands = vec![
            Command {
                command_type: CommandType::CPush,
                arg_1: Some("constant".to_string()),
                arg_2: Some("7".to_string()),
                classname: None,
            },
            Command {
                command_type: CommandType::CArithmetic,
                arg_1: Some("not".to_string()),
                arg_2: None,
                classname: None,
            },
        ];

        let mut generator = AssemblyGenerator::new(AssemblyConfiguration { bootstrap: false });

        generator.process_commands(&commands);

        assert_eq!(generator.instructions.len(), 3);
        assert_eq!(
            generator.instructions[0].instruction,
            "@7\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
        assert_eq!(generator.instructions[1].instruction, "@SP\nA=M-1\nM=!M\n");
    }

    #[test]
    fn process_infinite_loop() {
        let commands = vec![];

        let mut generator = AssemblyGenerator::new(AssemblyConfiguration { bootstrap: false });

        generator.process_commands(&commands);

        assert_eq!(generator.instructions.len(), 1);
        assert_eq!(
            generator.instructions[0].instruction,
            "(END)\n@END\n0;JMP\n"
        );
    }

    #[test]
    fn process_label_command() {
        let commands = vec![Command {
            command_type: CommandType::CLabel,
            arg_1: Some("loopLabel".to_string()),
            arg_2: None,
            classname: Some("SimpleFunction".to_string()),
        }];

        let mut generator = AssemblyGenerator::new(AssemblyConfiguration { bootstrap: false });

        generator.process_commands(&commands);

        assert_eq!(generator.instructions.len(), 2);
        assert_eq!(
            generator.instructions[0].instruction,
            "(SimpleFunction.default$loopLabel)\n"
        );
    }

    #[test]
    fn process_goto_command() {
        let commands = vec![Command {
            command_type: CommandType::CGoto,
            arg_1: Some("loopLabel".to_string()),
            arg_2: None,
            classname: Some("SimpleFunction".to_string()),
        }];

        let mut generator = AssemblyGenerator::new(AssemblyConfiguration { bootstrap: false });

        generator.process_commands(&commands);

        assert_eq!(generator.instructions.len(), 2);
        assert_eq!(
            generator.instructions[0].instruction,
            "@SimpleFunction.default$loopLabel\n0;JMP\n"
        );
    }
}
