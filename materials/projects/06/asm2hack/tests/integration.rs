use asm2hack::utils;

#[test]
fn create_a_parser_a_use_fields() {
    // read from file
    let input = "./tests/files/input.asm";

    let input_content = utils::read_file(input);

    // create a new parser based on input.asm
    let parser = asm2hack::parser::Parser::new(&input_content, false);

    // assess the parser fields
    assert_eq!(parser.is_symbolic, false);
    assert_eq!(parser.input, input_content);

    // use fields and convert those to binary
    let fields = parser.get_fields();
    assert_eq!(fields.len(), 0);
}
