use core::hash;

use crate::{
    logs::{log_info, log_warn},
    tokenizer::{JackKeyword, JackToken, JackTokenType},
};

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum JackUnaryOperator {
    Negate, // -
    Not,    // ~
}

#[derive(Debug, PartialEq)]
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
    LetStatement {
        identifier: String,
        expression: Box<JackNode>,
    },
    Expression {},
}

#[derive(Debug)]
pub struct JackParser {
    pub ast: Vec<JackNode>,
    current_token: usize, // current track of the token being read
    tokens: Vec<JackToken>,
}

impl JackParser {
    pub fn new(tokens: Vec<JackToken>) -> JackParser {
        JackParser {
            tokens,
            ast: vec![],
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

    fn write_to(&self, input: &str) {
        log_info(input);
    }

    fn parse_expression(&mut self) -> JackNode {
              JackNode::Expression {}
    }

    fn parse_let_statement(&mut self) -> JackNode {
        // let identifier '=' expression;
        self.advance_token();

        let identifier = self.get_current_token().clone();
        self.advance_token();

        // to avoid borrow checker, we just clone the current token
        let operator = self.get_current_token().clone();
        self.advance_token();

        // check if operator is '='
        if operator.symbol != Some("=".to_string()) {
            panic!("This let statement it's wrong formatted")
        }


        // parse expression
        let expression = self.parse_expression();
        self.advance_token();

        let end_token = self.get_current_token().clone();

        if end_token.symbol != Some(";".to_string()) {
            panic!("Expected ';' at the end of let statement");
        }

        JackNode::LetStatement {
            identifier: identifier.identifier.clone().unwrap(),
            expression: Box::new(expression),
        }
    }

    pub fn parse(&mut self) -> &Self {
        while self.current_token < self.tokens.len() {
            let token = self.get_current_token();

            match token.token_type {
                JackTokenType::KEYWORD => {
                    log_warn(&format!("Keyword: {:?}", token.keyword));
                    match token.keyword {
                        Some(JackKeyword::LET) => {
                            let let_statement = self.parse_let_statement();
                            self.ast.push(let_statement);
                        }
                        _ => {}
                    }
                }
                JackTokenType::SYMBOL => {
                    log_warn(&format!("Symbol: {:?}", token.symbol));
                }
                JackTokenType::IDENTIFIER => {
                    log_warn(&format!("Identifier: {:?}", token.identifier));
                }
                JackTokenType::INTCONST => {
                    log_warn(&format!("IntConst: {:?}", token.int_val));
                }
                JackTokenType::STRINGCONST => {
                    log_warn(&format!("StringConst: {:?}", token.string_val));
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

        assert_eq!(parser.tokens.len(), 5);
        assert_eq!(parser.current_token, 5);
        assert_eq!(
            parser.ast.len(),
            1,
        );

        let let_statement = &parser.ast[0];

        match let_statement {
            JackNode::LetStatement {
                identifier,
                expression,
            } => {
                assert_eq!(identifier, "a");
            }
            _ => panic!("Expected LetStatement"),
        }
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
