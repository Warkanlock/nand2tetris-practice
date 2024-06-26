use vm2asm::{code::AssemblyConfiguration, utils};

#[test]
fn create_virtual_machine_and_parse() {
    vm2asm::logs::log_command("create_virtual_machine_and_parse");

    // create a new virtual machine
    let mut parser = vm2asm::parser::Parser::new("add\nsub\nneg\npush local 2", "");

    // parse the virtual machine
    parser.parse();

    // get the commands
    let commands = parser.get_fields();

    let base_name: String = parser.get_base_name();

    assert_eq!(base_name, utils::capitalize_n_letters("root", 1));

    // check the commands
    assert_eq!(commands.len(), 4);

    // check the first command
    assert_eq!(
        commands[0],
        vm2asm::parser::Command {
            command_type: vm2asm::parser::CommandType::CArithmetic,
            arg_1: Some("add".to_string()),
            arg_2: None,
            classname: None,
        }
    );

    // check the second command
    assert_eq!(
        commands[1],
        vm2asm::parser::Command {
            command_type: vm2asm::parser::CommandType::CArithmetic,
            arg_1: Some("sub".to_string()),
            arg_2: None,
            classname: None,
        }
    );

    // check the third command
    assert_eq!(
        commands[2],
        vm2asm::parser::Command {
            command_type: vm2asm::parser::CommandType::CArithmetic,
            arg_1: Some("neg".to_string()),
            arg_2: None,
            classname: None,
        }
    );

    // check the fourth command
    assert_eq!(
        commands[3],
        vm2asm::parser::Command {
            command_type: vm2asm::parser::CommandType::CPush,
            arg_1: Some("local".to_string()),
            arg_2: Some("2".to_string()),
            classname: None,
        }
    );

    // generate a assembly generator based on those commands
    let mut generator =
        vm2asm::code::AssemblyGenerator::new(AssemblyConfiguration { bootstrap: false });

    // process commands
    generator.process_commands(&commands);

    assert_eq!(generator.instructions.len(), 5);
}
