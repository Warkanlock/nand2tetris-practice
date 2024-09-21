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
    Element {
        value: Box<JackToken>,
    },
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

    fn get_current_token(&self) -> Option<&JackToken> {
        if self.current_token >= self.tokens.len() {
            return None;
        }

        Some(&self.tokens[self.current_token])
    }

    fn write_to(&self, input: &str) {
        log_info(input);
    }

    fn parse_expression(&mut self) -> JackNode {
        // expression (op)
        self.advance_token();
        
        let operand_a = self.get_current_token().clone();

        if operand_a.is_none() {
            panic!("Expected first operand before complete expression");
        }

        let operand_a = operand_a.unwrap().clone();

        // validate operator to not be end token (to not be ; and if not return)

        self.advance_token();
        let token = self.get_current_token().clone();

        if token.is_none() {
            panic!("Expected symbol (most likely ;) after first operand to complete expression if no operator is present");
        }

        let operator = token.unwrap();

        // if operator is ; then return the operand_a
        if operator.symbol == Some(";".to_string()) {
            return JackNode::Element { value: Box::new(operand_a) };
        }

        // otherwise continue parsing the expression
        let operator_symbol = operator.symbol.clone().unwrap();

        // define jack operator token
        let operator_token = match operator_symbol.as_str() {
            "+" => JackBinaryOperator::Add,
            "-" => JackBinaryOperator::Substract,
            "*" => JackBinaryOperator::Multiply,
            "/" => JackBinaryOperator::Divide,
            "&" => JackBinaryOperator::And,
            "|" => JackBinaryOperator::Or,
            "<" => JackBinaryOperator::LessThan,
            ">" => JackBinaryOperator::GreaterThan,
            "=" => JackBinaryOperator::Equals,
            _ => panic!("Invalid operator"),
        };

        let operand_b : Option<&JackToken> = self.get_current_token().clone();

        if operand_b.is_none() {
            panic!("Expected operand");
        }

        let operand_b = operand_b.unwrap().clone();

        JackNode::BinaryOp {
            left: Box::new(JackNode::Element {value : Box::new(operand_a) }),
            operator: operator_token,
            right: Box::new(JackNode::Element {value : Box::new(operand_b) }),
        }
    }

    fn parse_let_statement(&mut self) -> JackNode {
        // let identifier '=' expression;
        self.advance_token();

        let identifier = self.get_current_token().clone();

        if identifier.is_none() {
            panic!("Expected identifier after let keyword");
        }

        let identifier = identifier.unwrap().clone();

        self.advance_token();

        // to avoid borrow checker, we just clone the current token
        let operator = self.get_current_token().clone();

        if operator.is_none() {
            panic!("Expected operator after identifier");
        }

        let operator = operator.unwrap().clone();

        // check if operator is '='
        if operator.symbol != Some("=".to_string()) {
            panic!("This let statement it's wrong formatted")
        }

        // parse expression
        let expression = self.parse_expression();

        JackNode::LetStatement {
            identifier: identifier.identifier.clone().unwrap(),
            expression: Box::new(expression),
        }
    }

    fn validate_end_token(&mut self, expected: &str) -> Option<&JackToken> {
        self.advance_token();

        let token = self.get_current_token().clone();

        if token.is_none() {
            return None;
        }

        let token = token.unwrap();

        if token.symbol != Some(expected.to_string()) {
            return None;
        }

        Some(&token)
    }

    pub fn parse(&mut self) -> &Self {
        while self.current_token < self.tokens.len() {
            let token = self.get_current_token();

            if token.is_none() {
                break;
            }

            let token = token.unwrap();

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
    fn test_parser_keyword_let() {
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
        assert_eq!(parser.ast.len(), 1,);

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

    #[should_panic]
    #[test]
    fn test_parser_keyword_let_missing_semicolon() {
        // let a = 1
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
        ];

        let mut parser = JackParser::new(tokens);
        parser.parse();

        assert_eq!(parser.tokens.len(), 4);
        assert_eq!(parser.current_token, 4);
    }

    #[test]
    fn test_parser_expression() {
        // let a = 1 + 1;
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
        parser.parse();

        assert_eq!(parser.tokens.len(), 7);
        assert_eq!(parser.current_token, 7);
        assert_eq!(parser.ast.len(), 1);

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
            parser.get_current_token().unwrap().token_type,
            JackTokenType::INTCONST
        );
        // advance to next token
        parser.advance_token();
        assert_eq!(parser.get_current_token().unwrap().token_type, JackTokenType::SYMBOL);
    }
}
