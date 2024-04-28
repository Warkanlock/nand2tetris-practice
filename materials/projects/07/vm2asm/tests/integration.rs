#[test]
fn create_virtual_machine_and_parse() {
    vm2asm::logs::log_command("create_virtual_machine_and_parse");

    // create a new virtual machine
    let mut parser = vm2asm::parser::Parser::new("add\nsub\nneg\npush local 2");

    // parse the virtual machine
    parser.parse();

    // get the commands
    let commands = parser.get_fields();

    // check the commands
    assert_eq!(commands.len(), 4);

    // check the first command
    assert_eq!(
        commands[0],
        vm2asm::parser::Command {
            command_type: vm2asm::parser::CommandType::CArithmetic,
            arg_1: Some("add".to_string()),
            arg_2: None
        }
    );

    // check the second command
    assert_eq!(
        commands[1],
        vm2asm::parser::Command {
            command_type: vm2asm::parser::CommandType::CArithmetic,
            arg_1: Some("sub".to_string()),
            arg_2: None
        }
    );

    // check the third command
    assert_eq!(
        commands[2],
        vm2asm::parser::Command {
            command_type: vm2asm::parser::CommandType::CArithmetic,
            arg_1: Some("neg".to_string()),
            arg_2: None
        }
    );

    // check the fourth command
    assert_eq!(
        commands[3],
        vm2asm::parser::Command {
            command_type: vm2asm::parser::CommandType::CPush,
            arg_1: Some("local".to_string()),
            arg_2: Some("2".to_string())
        }
    );

    // generate a assembly generator based on those commands
    let mut generator = vm2asm::code::AssemblyGenerator::new();

    // process commands
    generator.process_commands(&commands);

    assert_eq!(generator.instructions.len(), 4)
}
