use crate::tokenizer::JackToken;

pub enum JackOperator {
    Add,
    Substract,
    Multiply,
    Divide,
    And,
    Or,
    LessThan,
    GreaterThan,
    Equals,
    Negate,
    Not
}

pub enum JackVarTypes {
    Int,
    Char,
    Bool,
    Class(String)
}

pub enum JackNode {
    BinaryOp { left : Box<JackNode>, operator: JackOperator, right: Box<JackNode> },
    UnaryOp { operator: JackOperator, term: Box<JackNode>}
}

pub struct JackParser {
    pub ast : Option<JackNode>,
    current_token : usize, // current track of the token being read
    tokens: Vec<JackToken>
}

impl JackParser {
    pub fn new(tokens: Vec<JackToken>) -> JackParser {
        JackParser {
            tokens,
            ast: None,
            current_token: 0
        }
    }

    pub fn parse(&mut self) -> &Self {
        self
    }
}

mod tests {
    use crate::{parser::*, tokenizer::*};

    #[test]
    fn test_parser() {
        // let a = 1;
        let tokens = vec![
            JackToken { token_type: JackTokenType::KEYWORD, keyword: Some(JackKeyword::LET), symbol: None, identifier: None, int_val: None, string_val: None },
            JackToken { token_type: JackTokenType::IDENTIFIER, keyword: None, symbol: None, identifier: Some("a".to_string()), int_val: None, string_val: None },
            JackToken { token_type: JackTokenType::SYMBOL, keyword: None, symbol: Some("=".to_string()), identifier: None, int_val: None, string_val: None },
            JackToken { token_type: JackTokenType::INTCONST, keyword: None, symbol: None, identifier: None, int_val: Some(1), string_val: None },
            JackToken { token_type: JackTokenType::SYMBOL, keyword: None, symbol: Some(";".to_string()), identifier: None, int_val: None, string_val: None }
        ];

        let mut parser = JackParser::new(tokens);
        parser.parse();
    }
}
