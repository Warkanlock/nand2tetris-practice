use crate::parser::{Command, CommandType};

#[derive(Debug, PartialEq, Clone)]
pub struct AssemblyInstruction {
    pub instruction: String,
    pub command: Command,
}

fn process_commands(commands: &Vec<Command>) -> Vec<AssemblyInstruction> {
    let mut instructions: Vec<AssemblyInstruction> = Vec::new();

    for command in commands.iter() {
        let reference_command = command.clone();

        // check the command type
        match reference_command.command_type {
            CommandType::CPush => {
                let segment = reference_command.arg_1.as_ref().unwrap();
                let index = reference_command.arg_2.as_ref().unwrap();
                println!("push to segment {} with {}", segment, index);

                instructions.push(AssemblyInstruction {
                    instruction: format!("push {} {}", segment, index),
                    command: reference_command,
                });
            }
            CommandType::CPop => {
                let segment = reference_command.arg_1.as_ref().unwrap();
                let index = reference_command.arg_2.as_ref().unwrap();
                println!("pop from segment {} with {}", segment, index);

                instructions.push(AssemblyInstruction {
                    instruction: format!("pop {} {}", segment, index),
                    command: reference_command,
                });
            }
            // if it's arithmetic, we only have one argument and we need to
            // check what's the operation based on that
            CommandType::CArithmetic => {
                let operation = reference_command.arg_1.as_ref().unwrap();

                println!("apply operation > {}", operation);

                instructions.push(AssemblyInstruction {
                    instruction: format!("apply operation > {}", operation),
                    command: reference_command,
                });
            }
        }
    }

    instructions
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

        let assembly = process_commands(&commands);

        assert_eq!(assembly.len(), 3);
    }
}
