#[test]
fn create_a_parser_a_use_fields() {
    let parser = asm2hack::parser::Parser::new(&String::from("test.asm"), None);

    // assess the parser fields
    assert_eq!(parser.is_symbolic, false);
    assert_eq!(parser.input, "test.asm");
    assert_eq!(parser.fields.len(), 0);
    
    // use fields and convert those to binary
}
