use core::hash;

use crate::{
    logs::log_info,
    tokenizer::{JackKeyword, JackToken, JackTokenType},
};

#[derive(Debug)]
pub enum JackBinaryOperator {
    Add,
    Substract,
    Multiply,
    Divide,
    And,         // &
    Or,          // |
    LessThan,    // <
    GreaterThan, // >
    Equals,      // =
}

#[derive(Debug)]
pub enum JackUnaryOperator {
    Negate, // -
    Not,    // ~
}

#[derive(Debug)]
pub enum JackNode {
    BinaryOp {
        left: Box<JackNode>,
        operator: JackBinaryOperator,
        right: Box<JackNode>,
    },
    UnaryOp {
        operator: JackUnaryOperator,
        term: Box<JackNode>,
    },
}

#[derive(Debug)]
pub struct JackParser {
    pub ast: Option<JackNode>,
    current_token: usize, // current track of the token being read
    tokens: Vec<JackToken>,
}

impl JackParser {
    pub fn new(tokens: Vec<JackToken>) -> JackParser {
        JackParser {
            tokens,
            ast: None,
            current_token: 0,
        }
    }

    fn advance_token(&mut self) {
        if self.current_token <= self.tokens.len() {
            self.current_token += 1;
        } else {
            panic!("End of tokens reached. Cannot advance further.");
        }
    }

    fn get_current_token(&self) -> &JackToken {
        &self.tokens[self.current_token]
    }

    pub fn parse(&mut self) -> &Self {
        while self.current_token < self.tokens.len() {
            let token = self.get_current_token();

            match token.token_type {
                JackTokenType::KEYWORD => {
                    log_info(&format!("Keyword: {:?}", token.keyword));
                }
                JackTokenType::SYMBOL => {
                    log_info(&format!("Symbol: {:?}", token.symbol));
                }
                JackTokenType::IDENTIFIER => {
                    log_info(&format!("Identifier: {:?}", token.identifier));
                }
                JackTokenType::INTCONST => {
                    log_info(&format!("IntConst: {:?}", token.int_val));
                }
                JackTokenType::STRINGCONST => {
                    log_info(&format!("StringConst: {:?}", token.string_val));
                }
            }

            self.advance_token();
        }
        self
    }
}

mod tests {
    use crate::{parser::*, tokenizer::*};

    #[test]
    fn test_parser_keyword() {
        // let a = 1;
        let tokens = vec![
            JackToken {
                token_type: JackTokenType::KEYWORD,
                keyword: Some(JackKeyword::LET),
                symbol: None,
                identifier: None,
                int_val: None,
                string_val: None,
            },
            JackToken {
                token_type: JackTokenType::IDENTIFIER,
                keyword: None,
                symbol: None,
                identifier: Some("a".to_string()),
                int_val: None,
                string_val: None,
            },
            JackToken {
                token_type: JackTokenType::SYMBOL,
                keyword: None,
                symbol: Some("=".to_string()),
                identifier: None,
                int_val: None,
                string_val: None,
            },
            JackToken {
                token_type: JackTokenType::INTCONST,
                keyword: None,
                symbol: None,
                identifier: None,
                int_val: Some(1),
                string_val: None,
            },
            JackToken {
                token_type: JackTokenType::SYMBOL,
                keyword: None,
                symbol: Some(";".to_string()),
                identifier: None,
                int_val: None,
                string_val: None,
            },
        ];

        let mut parser = JackParser::new(tokens);
        parser.parse();
    }

    #[test]
    fn test_parser_advance_token() {
        // let a = 1;
        let tokens = vec![
            JackToken {
                token_type: JackTokenType::KEYWORD,
                keyword: Some(JackKeyword::LET),
                symbol: None,
                identifier: None,
                int_val: None,
                string_val: None,
            },
            JackToken {
                token_type: JackTokenType::IDENTIFIER,
                keyword: None,
                symbol: None,
                identifier: Some("a".to_string()),
                int_val: None,
                string_val: None,
            },
            JackToken {
                token_type: JackTokenType::SYMBOL,
                keyword: None,
                symbol: Some("=".to_string()),
                identifier: None,
                int_val: None,
                string_val: None,
            },
            JackToken {
                token_type: JackTokenType::INTCONST,
                keyword: None,
                symbol: None,
                identifier: None,
                int_val: Some(1),
                string_val: None,
            },
            JackToken {
                token_type: JackTokenType::SYMBOL,
                keyword: None,
                symbol: Some(";".to_string()),
                identifier: None,
                int_val: None,
                string_val: None,
            },
        ];

        let mut parser = JackParser::new(tokens);
        parser.advance_token();
        assert_eq!(parser.current_token, 1);
    }

    #[should_panic(expected = "End of tokens reached. Cannot advance further.")]
    #[test]
    fn test_parser_advance_token_until_overflow() {
        // a++;
        let tokens = vec![
            JackToken {
                token_type: JackTokenType::IDENTIFIER,
                keyword: None,
                symbol: None,
                identifier: Some("a".to_string()),
                int_val: None,
                string_val: None,
            },
            JackToken {
                token_type: JackTokenType::SYMBOL,
                keyword: None,
                symbol: Some("+".to_string()),
                identifier: None,
                int_val: None,
                string_val: None,
            },
            JackToken {
                token_type: JackTokenType::SYMBOL,
                keyword: None,
                symbol: Some("+".to_string()),
                identifier: None,
                int_val: None,
                string_val: None,
            },
        ];

        let mut parser = JackParser::new(tokens);
        parser.advance_token(); // a
        parser.advance_token(); // +
        parser.advance_token(); // +
        parser.advance_token(); // ;
        parser.advance_token(); // should panic
    }

    #[test]
    fn test_parser_get_current_token() {
        // 1 + 1;
        let tokens = vec![
            JackToken {
                token_type: JackTokenType::INTCONST,
                keyword: None,
                symbol: None,
                identifier: None,
                int_val: Some(1),
                string_val: None,
            },
            JackToken {
                token_type: JackTokenType::SYMBOL,
                keyword: None,
                symbol: Some("+".to_string()),
                identifier: None,
                int_val: None,
                string_val: None,
            },
            JackToken {
                token_type: JackTokenType::INTCONST,
                keyword: None,
                symbol: None,
                identifier: None,
                int_val: Some(1),
                string_val: None,
            },
            JackToken {
                token_type: JackTokenType::SYMBOL,
                keyword: None,
                symbol: Some(";".to_string()),
                identifier: None,
                int_val: None,
                string_val: None,
            },
        ];

        let mut parser = JackParser::new(tokens);
        assert_eq!(
            parser.get_current_token().token_type,
            JackTokenType::INTCONST
        );
        // advance to next token
        parser.advance_token();
        assert_eq!(parser.get_current_token().token_type, JackTokenType::SYMBOL);
    }
}
