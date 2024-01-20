use asm2hack::utils;

#[test]
fn create_parser_use_fields() {
    // read from file
    let input = "./tests/files/input.asm";

    let input_content = utils::read_file(input);

    // create a new parser based on input.asm
    let mut parser = asm2hack::parser::Parser::new(&input_content, false);

    // assess the parser fields
    assert_eq!(parser.is_symbolic, false);
    assert_eq!(parser.input, input_content);

    // use fields and convert those to binary
    let fields = parser.get_fields();
    assert_eq!(fields.len(), 0);

    // check fields are correct from the input.asm file
    parser.parse();

    let fields = parser.get_fields();

    // total lines from input_content without empty spaces
    let total_lines = input_content
        .lines()
        .filter(|line| !line.is_empty())
        .count();

    // check total fields analyzed
    assert_eq!(fields.len(), total_lines);

    // check A-instruction line
    let line = fields.get(1).unwrap();

    assert_eq!(line.instruction_symbol, None);
    assert_eq!(line.instruction_dest, None);
    assert_eq!(line.instruction_comp, None);
    assert_eq!(line.instruction_jump, None);
    assert_eq!(line.instruction_value, Some(2));
    assert_eq!(
        line.instruction_type,
        asm2hack::parser::ParserInstructionType::AInstruction
    );

    // check C-instruction line
    let line = fields.get(4).unwrap();
    assert_eq!(line.instruction_symbol, Some("D=D+A".to_string()));
    assert_eq!(line.instruction_dest, Some("D".to_string()));
    assert_eq!(line.instruction_comp, Some("D+A".to_string()));
    assert_eq!(line.instruction_jump, None);
    assert_eq!(
        line.instruction_type,
        asm2hack::parser::ParserInstructionType::CInstruction
    );

    // process those fields
    let binary_instructions = asm2hack::code::process_fields(&fields);

    // check total binary instructions
    assert_eq!(binary_instructions.len(), total_lines);
}
