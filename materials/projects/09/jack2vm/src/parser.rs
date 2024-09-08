use core::hash;

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

    fn advance_token(&mut self) {
        if self.current_token < self.tokens.len() {
            self.current_token += 1;
        }
    }

    fn get_current_token(&self) -> &JackToken {
        &self.tokens[self.current_token]
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

    #[test]
    fn test_parser_advance_token() {
        let tokens = vec![
            JackToken { token_type: JackTokenType::KEYWORD, keyword: Some(JackKeyword::LET), symbol: None, identifier: None, int_val: None, string_val: None },
            JackToken { token_type: JackTokenType::IDENTIFIER, keyword: None, symbol: None, identifier: Some("a".to_string()), int_val: None, string_val: None },
            JackToken { token_type: JackTokenType::SYMBOL, keyword: None, symbol: Some("=".to_string()), identifier: None, int_val: None, string_val: None },
            JackToken { token_type: JackTokenType::INTCONST, keyword: None, symbol: None, identifier: None, int_val: Some(1), string_val: None },
            JackToken { token_type: JackTokenType::SYMBOL, keyword: None, symbol: Some(";".to_string()), identifier: None, int_val: None, string_val: None }
        ];

        let mut parser = JackParser::new(tokens);
        parser.advance_token();
        assert_eq!(parser.current_token, 1);
    }

    #[test]
    fn test_parser_get_current_token() {
        let tokens = vec![
            JackToken { token_type: JackTokenType::KEYWORD, keyword: Some(JackKeyword::LET), symbol: None, identifier: None, int_val: None, string_val: None },
            JackToken { token_type: JackTokenType::IDENTIFIER, keyword: None, symbol: None, identifier: Some("a".to_string()), int_val: None, string_val: None },
            JackToken { token_type: JackTokenType::SYMBOL, keyword: None, symbol: Some("=".to_string()), identifier: None, int_val: None, string_val: None },
            JackToken { token_type: JackTokenType::INTCONST, keyword: None, symbol: None, identifier: None, int_val: Some(1), string_val: None },
            JackToken { token_type: JackTokenType::SYMBOL, keyword: None, symbol: Some(";".to_string()), identifier: None, int_val: None, string_val: None }
        ];

        let parser = JackParser::new(tokens);
        let current_token = parser.get_current_token();
        assert_eq!(current_token.token_type, JackTokenType::KEYWORD);
    }
}
