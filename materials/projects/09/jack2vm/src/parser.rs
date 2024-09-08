use crate::logs::log_info;

#[derive(Debug, PartialEq, Clone)]
pub struct JackInstruction {
    pub line: usize,
    pub value: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct JackToken {
    pub token_type: JackTokenType,

    // depending on the token_type, the following fields will be filled
    // or not
    //
    // note: a better way to represent this is through `enum Value(T)`
    pub keyword: Option<JackKeyword>,
    pub symbol: Option<String>,
    pub identifier: Option<String>,
    pub int_val: Option<i32>,
    pub string_val: Option<String>,
}

pub const JACK_SYMBOLS: [&str; 19] = [
    "{", "}", "(", ")", "[", "]", ".", ",", ";", "+", "-", "*", "/", "&", "|", "<", ">", "=", "~",
];

pub const JACK_KEYWORDS: [&str; 21] = [
    "class",
    "constructor",
    "function",
    "method",
    "field",
    "static",
    "var",
    "int",
    "char",
    "boolean",
    "void",
    "true",
    "false",
    "null",
    "this",
    "let",
    "do",
    "if",
    "else",
    "while",
    "return",
];

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum JackTokenType {
    KEYWORD,
    SYMBOL,
    IDENTIFIER,
    INTCONST,
    STRINGCONST,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum JackKeyword {
    CLASS,
    CONSTRUCTOR,
    FUNCTION,
    METHOD,
    FIELD,
    STATIC,
    VAR,
    INT,
    CHAR,
    BOOLEAN,
    VOID,
    TRUE,
    FALSE,
    NULL,
    THIS,
    LET,
    DO,
    IF,
    ELSE,
    WHILE,
    RETURN,
}

#[derive(Debug, PartialEq, Clone)]
pub struct JackNodeElement {
    pub element_type: JackTokenType,
    pub value: String,
    pub children: Vec<JackNodeElement>,
}

pub struct JackTokenizer {
    pub content: String, // file content from input
    pub instructions: Vec<JackInstruction>,
    pub tokens: Vec<JackToken>,
    // a list of elements that will be used to generate the AST
    pub ast: Vec<JackNodeElement>,
    verbose: bool,
}

/*
 * Process of syntax analysis:
 *
 * 1. Lexical anaylsis (tokenizer): break the input into tokens
 * 2. Syntax analysis (parser): build a parse tree
 * 3. Semantic analysis: check the parse tree for semantic errors
 * 4. Code generation: generate code from the parse tree
 */
impl JackTokenizer {
    pub fn new(content: &String, verbose: bool) -> Self {
        JackTokenizer {
            content: content.to_string(), // copy content, not really handy if big files
            instructions: Vec::new(),
            tokens: Vec::new(),
            ast: Vec::new(),
            verbose,
        }
    }

    fn is_symbol(s: char) -> bool {
        JACK_SYMBOLS
            .iter()
            .any(|&symbol| s == symbol.chars().next().unwrap())
    }

    fn has_comment(s: &str) -> bool {
        s.starts_with("/")
            || s.ends_with("/")
            || s.starts_with("/*")
            || s.ends_with("*/")
            || s.contains("//")
            || s.contains("/*")
            || s.contains("*/")
    }

    fn has_symbol(s: &str) -> bool {
        JACK_SYMBOLS.iter().any(|&symbol| s.contains(symbol))
    }

    fn is_string(s: &str) -> bool {
        s.starts_with("\"") || s.contains("\"")
    }

    fn get_next_token(&mut self) -> Option<JackToken> {
        if self.tokens.len() == 0 {
            return None;
        }

        let token = self.tokens.remove(0);

        return Some(token);
    }

    pub fn extract_symbols(&self, element: &str, symbols: &mut Vec<String>) -> () {
        let mut word = element;

        // partial word store internally
        let mut acc_word = String::new();

        while word.len() > 0 {
            let next_char = word.chars().next().unwrap();

            if JackTokenizer::is_symbol(next_char) {
                // if detect a symbol, we should add the previous word
                // to the array and the symbol, and start again
                let end = word.find(next_char).unwrap();

                let word = &word[..end];

                // insert the word as it is now
                if !word.is_empty() {
                    symbols.push(word.to_string());
                }

                // insert the next symbol
                if !next_char.is_whitespace() {
                    symbols.push(next_char.to_string());
                }
            } else {
                // check if the next char is a symbol, if so, insert the
                // word now
                //
                // e.g: 1; } -> 1, ;, }
                //
                // otherwise we will end up adding the word to the end
                // of the array
                let next_next_char = word.chars().nth(1);

                if next_next_char.is_some() && JackTokenizer::is_symbol(next_next_char.unwrap()) {
                    acc_word.push(next_char);
                    symbols.push(acc_word.trim().to_string());
                    acc_word = String::new();
                } else {
                    // if it's not a symbol, just add the char to the word
                    acc_word.push(next_char);
                }
            }

            word = &word[1..];
        }

        // if there's any remaining word, add it to the array
        if !acc_word.is_empty() {
            symbols.push(acc_word.to_string());
        }

        // remove empty symbols
        symbols.retain(|x| !x.is_empty());
    }

    fn get_keyword_text_from_index(index: usize) -> &'static str {
        return match index {
            0 => "class",
            1 => "constructor",
            2 => "function",
            3 => "method",
            4 => "field",
            5 => "static",
            6 => "var",
            7 => "int",
            8 => "char",
            9 => "boolean",
            10 => "void",
            11 => "true",
            12 => "false",
            13 => "null",
            14 => "this",
            15 => "let",
            16 => "do",
            17 => "if",
            18 => "else",
            19 => "while",
            20 => "return",
            _ => panic!("Keyword not found"),
        };
    }

    fn get_keyword_from_index(index: usize) -> JackKeyword {
        return match index {
            0 => JackKeyword::CLASS,
            1 => JackKeyword::CONSTRUCTOR,
            2 => JackKeyword::FUNCTION,
            3 => JackKeyword::METHOD,
            4 => JackKeyword::FIELD,
            5 => JackKeyword::STATIC,
            6 => JackKeyword::VAR,
            7 => JackKeyword::INT,
            8 => JackKeyword::CHAR,
            9 => JackKeyword::BOOLEAN,
            10 => JackKeyword::VOID,
            11 => JackKeyword::TRUE,
            12 => JackKeyword::FALSE,
            13 => JackKeyword::NULL,
            14 => JackKeyword::THIS,
            15 => JackKeyword::LET,
            16 => JackKeyword::DO,
            17 => JackKeyword::IF,
            18 => JackKeyword::ELSE,
            19 => JackKeyword::WHILE,
            20 => JackKeyword::RETURN,
            _ => panic!("Keyword not found"),
        };
    }

    fn compile_class(&mut self) {}

    fn compile_class_var_dec(&mut self) {}

    fn compile_subroutine_declaration(&mut self) {}

    fn compile_parameter_list(&mut self) {}

    fn compile_subroutine_body(&mut self) {}

    fn compile_var_dec(&mut self) {}

    fn compile_statements(&mut self, parent_node: &mut JackNodeElement) {
        let token = self.get_next_token().unwrap();

        match token.token_type {
            JackTokenType::KEYWORD => match token.keyword.unwrap() {
                JackKeyword::LET => {
                    let let_tree = self.compile_let(&token);
                    parent_node.children.push(let_tree);
                }
                JackKeyword::IF => {
                    let if_tree = self.compile_if();
                    parent_node.children.push(if_tree);
                }
                _ => log_info("[KEYWORD] implemented yet"),
            },
            _ => log_info("[TOKEN] implemented yet"),
        }
    }

    fn compile_let(&mut self, token: &JackToken) -> JackNodeElement {
        let mut let_node = JackNodeElement {
            element_type: JackTokenType::KEYWORD,
            value: JackTokenizer::get_keyword_text_from_index(token.keyword.unwrap() as usize)
                .to_string(),
            children: vec![],
        };

        // varName
        let identifier = self.get_next_token().unwrap();
        let identifier_token = JackNodeElement {
            element_type: JackTokenType::IDENTIFIER,
            value: identifier.identifier.unwrap(),
            children: vec![],
        };

        let_node.children.push(identifier_token);

        // check if it's an array of variables
        let symbol_or_equal = self.get_next_token().unwrap();
        let symbol_or_equal_value = symbol_or_equal.symbol.unwrap();
        if symbol_or_equal_value == "[" {
            let bracket_node = JackNodeElement {
                element_type: JackTokenType::SYMBOL,
                value: "[".to_string(),
                children: vec![],
            };

            // expression
            let expression_node = self.compile_expression();

            // closing bracket
            let closing_bracket = self.get_next_token().unwrap();
            let closing_bracket_node = JackNodeElement {
                element_type: JackTokenType::SYMBOL,
                value: "]".to_string(),
                children: vec![],
            };

            // add the nodes to the let_node
            let_node.children.push(bracket_node);
            let_node.children.push(expression_node);
            let_node.children.push(closing_bracket_node);
        } else {
            let equal_token = JackNodeElement {
                element_type: JackTokenType::SYMBOL,
                value: symbol_or_equal_value,
                children: vec![],
            };

            let_node.children.push(equal_token);
        }

        // expression
        let expression_node = self.compile_expression();
        let_node.children.push(expression_node);

        // semicolon
        let semicolon = self.get_next_token();

        if semicolon.is_none() {
            return let_node;
        }

        let semicolon_token = JackNodeElement {
            element_type: JackTokenType::SYMBOL,
            value: semicolon.unwrap().symbol.unwrap(),
            children: vec![],
        };

        let_node.children.push(semicolon_token);

        let_node
    }

    fn compile_if(&mut self) -> JackNodeElement {
        let root = self.get_next_token().unwrap();

        let mut if_node = JackNodeElement {
            element_type: JackTokenType::KEYWORD,
            value: JackTokenizer::get_keyword_text_from_index(root.keyword.unwrap() as usize)
                .to_string(),
            children: vec![],
        };

        let opening_parenthesis = self.get_next_token().unwrap();
        let opening_parenthesis_node = JackNodeElement {
            element_type: JackTokenType::SYMBOL,
            value: opening_parenthesis.symbol.unwrap(),
            children: vec![],
        };

        let expression = self.compile_expression();

        let closing_parenthesis = self.get_next_token().unwrap();
        let closing_parenthesis_node = JackNodeElement {
            element_type: JackTokenType::SYMBOL,
            value: closing_parenthesis.symbol.unwrap(),
            children: vec![],
        };

        let opening_bracket = self.get_next_token().unwrap();
        let opening_bracket_node = JackNodeElement {
            element_type: JackTokenType::SYMBOL,
            value: opening_bracket.symbol.unwrap(),
            children: vec![],
        };

        if_node.children.push(opening_parenthesis_node);
        if_node.children.push(expression);
        if_node.children.push(closing_parenthesis_node);
        if_node.children.push(opening_bracket_node);

        self.compile_statements(&mut if_node); // compile statements

        let closing_bracket = self.get_next_token().unwrap();
        let closing_bracket_node = JackNodeElement {
            element_type: JackTokenType::SYMBOL,
            value: closing_bracket.symbol.unwrap(),
            children: vec![],
        };

        if_node.children.push(closing_bracket_node);

        // TODO: check if there's an else statement
        if_node
    }

    fn compile_while(&mut self) {}

    fn compile_do(&mut self) {}

    fn compile_return(&mut self) {}

    fn compile_expression(&mut self) -> JackNodeElement {
        // term
        let mut term = self.compile_term().unwrap();

        // op
        let op_term = self.get_next_token().unwrap();
        if op_term.token_type != JackTokenType::SYMBOL {
            return term;
        }

        let mut op_node = JackNodeElement {
            element_type: JackTokenType::SYMBOL,
            value: op_term.symbol.unwrap(),
            children: vec![],
        };

        // term
        let operator_term_node = self.compile_term();

        if operator_term_node.is_none() {
            return term;
        }

        op_node.children.push(operator_term_node.unwrap());
        term.children.push(op_node.clone());

        op_node
    }

    fn compile_term(&mut self) -> Option<JackNodeElement> {
        // integerConstant
        let token = self.get_next_token();

        if !token.is_none() {
            let current_token = token.unwrap();
            if current_token.token_type == JackTokenType::INTCONST {
                return Some(JackNodeElement {
                    element_type: JackTokenType::INTCONST,
                    value: current_token.int_val.unwrap().to_string(),
                    children: vec![],
                });
            }
        }

        None
    }

    fn compile_expression_list(&mut self) {}

    pub fn parse_tree(&mut self) -> &Self {
        while self.tokens.len() > 0 {
            let token = self.get_next_token().unwrap();

            match token.token_type {
                JackTokenType::KEYWORD => match token.keyword.unwrap() {
                    JackKeyword::LET => {
                        let let_tree = self.compile_let(&token);
                        self.ast.push(let_tree);
                    }
                    JackKeyword::IF => {
                        let if_tree = self.compile_if();
                        self.ast.push(if_tree);
                    }
                    _ => log_info("[KEYWORD] implemented yet"),
                },
                _ => log_info("[TOKEN] implemented yet"),
            }
        }

        self
    }

    pub fn prepare_tree(&mut self) -> &Self {
        while self.tokens.len() > 0 {
            let token = self.get_next_token().unwrap();

            match token.token_type {
                JackTokenType::KEYWORD => {
                    let token_id = token.keyword.unwrap() as u8;
                    println!(
                        "<keyword>\t{}\t</keyword>",
                        JackTokenizer::get_keyword_text_from_index(token_id as usize)
                    );
                }
                JackTokenType::SYMBOL => {
                    if let Some(ref symbol) = token.symbol {
                        println!("<symbol>\t{}\t</symbol>", symbol);
                    }
                }
                JackTokenType::IDENTIFIER => {
                    if let Some(ref identifier) = token.identifier {
                        println!("<identifier>\t{}\t</identifier>", identifier);
                    }
                }
                JackTokenType::INTCONST => {
                    if let Some(int_const) = token.int_val {
                        println!("<integerConstant>\t{}\t</integerConstant>", int_const);
                    }
                }
                JackTokenType::STRINGCONST => {
                    if let Some(ref string_const) = token.string_val {
                        println!("<stringConstant>\t{}\t</stringConstant>", string_const);
                    }
                }
            }
        }

        self
    }

    pub fn tokenize(&mut self) -> &Self {
        // invalidate tokenizer if content is not valid
        if self.content.len() == 0 {
            panic!("Cannot use tokenize without a valid content")
        }

        /*
         * Our approach here will be to go top-down (instead of bottom-up)
         */

        // remove breaklines and split by whitespace
        let content_as_lines: Vec<&str> = self.content.lines().collect::<Vec<&str>>();

        // split elements of the content by symbols if any
        let mut symbols: Vec<String> = Vec::new();

        // divide the content into lines of code
        for (i, line) in content_as_lines.iter().enumerate() {
            // if the line is a comment, discard it as a whole and continue
            if JackTokenizer::has_comment(line) {
                continue;
            }

            // this will help us to use strings with spaces
            //
            // e.g: "hello world" -> ["hello world"]
            // e.g: "hello\nworld" -> ["hello world"] // still hello world
            //
            // this code allows us to use strings inside the code
            //
            if JackTokenizer::is_string(line) {
                let start = line.find("\"").unwrap();
                let end = line.rfind("\"").unwrap();

                // Extract the string including quotes
                let word = &line[start..=end];

                // Handle symbols within the string
                if JackTokenizer::has_symbol(&line) {
                    // get symbol is present in the string
                    let symbol = JACK_SYMBOLS.iter().find(|&s| line.contains(s));

                    // if the symbol is present, we should split the string
                    if symbol.is_some() {
                        // split by symbol and insert the symbol as well
                        let remaining = line.split(symbol.unwrap()).collect::<Vec<&str>>().join("");

                        // insert the string symbols
                        let remaining_without_word = remaining.replace(&word, "");
                        let remaining_trimmed: &str = &remaining_without_word.trim();
                        let content: Vec<&str> =
                            remaining_trimmed.split_whitespace().collect::<Vec<&str>>();

                        for element in content.iter() {
                            self.extract_symbols(element, &mut symbols);
                        }

                        // insert the word
                        symbols.push(word.to_string());

                        // insert the symbol
                        symbols.push(symbol.unwrap().to_string());
                    }
                } else {
                    symbols.push(word.to_string());
                }

                continue;
            }

            // split content without whitespaces
            let content: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();

            for element in content.iter() {
                if JackTokenizer::has_symbol(element) {
                    self.extract_symbols(element, &mut symbols);
                } else {
                    symbols.push(element.to_string());
                }
            }
        }

        // iterate across internal symbols and store instructions
        //
        // this step is not required not even mandatory but it's to have an
        // in memory representation before the tokenization
        for (index, input_string) in symbols.iter().enumerate() {
            if self.verbose {
                log_info(format!("input > {:?}", input_string).as_str());
            }

            // push the command into the instructions vector
            self.instructions.push(JackInstruction {
                line: index + 1,
                value: input_string.to_string(),
            });
        }

        // iteratae across instructions and specify the token type of each
        for element in self.instructions.to_owned().clone() {
            // let's check the token type based on the information present on the element.value
            match element.value.to_string() {
                e if JACK_SYMBOLS.contains(&e.as_str()) => self.tokens.push(JackToken {
                    token_type: JackTokenType::SYMBOL,
                    keyword: None,
                    int_val: None,
                    string_val: None,
                    symbol: Some(element.value),
                    identifier: None,
                }),
                e if JACK_KEYWORDS.contains(&e.as_str()) => {
                    // find index of the keyword in the array
                    let index = JACK_KEYWORDS.iter().position(|&r| r == e).unwrap();

                    // push the token into the tokens array
                    self.tokens.push(JackToken {
                        token_type: JackTokenType::KEYWORD,
                        keyword: Some(JackTokenizer::get_keyword_from_index(index)),
                        int_val: None,
                        string_val: None,
                        symbol: None,
                        identifier: None,
                    })
                }
                e if e.starts_with("\"") => self.tokens.push(JackToken {
                    token_type: JackTokenType::STRINGCONST,
                    keyword: None,
                    int_val: None,
                    string_val: Some(element.value),
                    symbol: None,
                    identifier: None,
                }),
                e if e.parse::<i32>().is_ok() => self.tokens.push(JackToken {
                    token_type: JackTokenType::INTCONST,
                    keyword: None,
                    int_val: Some(e.parse::<i32>().unwrap()),
                    string_val: None,
                    symbol: None,
                    identifier: None,
                }),
                e if e.chars().all(char::is_alphanumeric) => self.tokens.push(JackToken {
                    token_type: JackTokenType::IDENTIFIER,
                    keyword: None,
                    int_val: None,
                    string_val: None,
                    symbol: None,
                    identifier: Some(element.value),
                }),
                _ => log_info(format!("No token type found for element {:?}", element).as_str()),
            }
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{JackKeyword, JackTokenType, JackTokenizer};

    #[test]
    fn test_initialization() {
        let tokenizer = JackTokenizer::new(&String::from("// empty content"), false);
        assert_eq!(tokenizer.instructions.len(), 0);
    }

    #[test]
    fn test_parse_simple_command_with_spaces() {
        let mut tokenizer: JackTokenizer = JackTokenizer::new(&String::from("{let a = 1;}"), false);
        tokenizer.tokenize();
        assert_eq!(tokenizer.instructions.len(), 7);

        // copy first instruction
        let first = tokenizer.instructions[0].clone();
        let last = tokenizer.instructions[tokenizer.instructions.len() - 1].clone();

        // assert
        assert_eq!(first.value, "{");
        assert_eq!(last.value, "}");
    }

    #[test]
    fn test_parse_simple_command_with_comments() {
        let mut tokenizer: JackTokenizer =
            JackTokenizer::new(&String::from("// this is a test\n{ let a = 1; }"), false);
        tokenizer.tokenize();
        assert_eq!(tokenizer.instructions.len(), 7);

        // copy first instruction
        let first = tokenizer.instructions[0].clone();
        let last = tokenizer.instructions[tokenizer.instructions.len() - 1].clone();

        // assert
        assert_eq!(first.value, "{");
        assert_eq!(last.value, "}");
    }

    #[test]
    fn test_parse_simple_command_as_string() {
        let mut tokenizer: JackTokenizer = JackTokenizer::new(&String::from("\"hello\""), false);
        tokenizer.tokenize();
        assert_eq!(tokenizer.instructions.len(), 1);

        // copy first instruction
        let first = tokenizer.instructions[0].clone();
        let last = tokenizer.instructions[tokenizer.instructions.len() - 1].clone();

        // assert
        assert_eq!(first.value, "\"hello\"");
    }

    #[test]
    fn test_parse_simple_command() {
        let mut tokenizer: JackTokenizer =
            JackTokenizer::new(&String::from("{ let a = 1; }"), false);
        tokenizer.tokenize();
        assert_eq!(tokenizer.instructions.len(), 7);

        // copy first instruction
        let first = tokenizer.instructions[0].clone();
        let last = tokenizer.instructions[tokenizer.instructions.len() - 1].clone();

        // assert
        assert_eq!(first.value, "{");
        assert_eq!(last.value, "}");
    }

    #[test]
    fn test_parse_simple_command_with_symbol() {
        let mut tokenizer: JackTokenizer = JackTokenizer::new(&String::from("{}"), false);
        tokenizer.tokenize();
        assert_eq!(tokenizer.instructions.len(), 2);

        // copy first instruction
        let first = tokenizer.instructions[0].clone();
        let last = tokenizer.instructions[tokenizer.instructions.len() - 1].clone();

        // assert
        assert_eq!(first.value, "{");
        assert_eq!(last.value, "}");

        // check if the tokenizer has the right amount of tokens
        assert_eq!(tokenizer.tokens.len(), 2);

        // check tokens type as well
        assert_eq!(tokenizer.tokens[0].token_type, JackTokenType::SYMBOL);
    }

    #[test]
    fn test_parse_simple_command_with_keyword() {
        let mut tokenizer: JackTokenizer = JackTokenizer::new(&String::from("class"), false);
        tokenizer.tokenize();
        assert_eq!(tokenizer.instructions.len(), 1);

        // copy first instruction
        let first = tokenizer.instructions[0].clone();

        // assert
        assert_eq!(first.value, "class");

        // check tokens
        assert_eq!(tokenizer.tokens.len(), 1);

        // check tokens type as well
        assert_eq!(tokenizer.tokens[0].token_type, JackTokenType::KEYWORD);
        assert_eq!(tokenizer.tokens[0].keyword.unwrap(), JackKeyword::CLASS);
    }

    #[test]
    fn test_parse_simple_command_with_string() {
        let mut tokenizer: JackTokenizer = JackTokenizer::new(&String::from("\"hello\""), false);
        tokenizer.tokenize();
        assert_eq!(tokenizer.instructions.len(), 1);

        // copy first instruction
        let first = tokenizer.instructions[0].clone();

        // assert
        assert_eq!(first.value, "\"hello\"");

        // check tokens
        assert_eq!(tokenizer.tokens.len(), 1);

        // check tokens type as well
        assert_eq!(tokenizer.tokens[0].token_type, JackTokenType::STRINGCONST);
    }

    #[test]
    fn test_parse_simple_command_with_string_weird() {
        let mut tokenizer: JackTokenizer =
            JackTokenizer::new(&String::from("\"hello 934_93\";"), false);
        tokenizer.tokenize();
        assert_eq!(tokenizer.instructions.len(), 2);

        // copy first instruction
        let first = tokenizer.instructions[0].clone();
        let last = tokenizer.instructions[tokenizer.instructions.len() - 1].clone();

        // assert
        assert_eq!(first.value, "\"hello 934_93\"");
        assert_eq!(last.value, ";");

        // check tokens
        assert_eq!(tokenizer.tokens.len(), 2);

        // check tokens type as well
        assert_eq!(tokenizer.tokens[0].token_type, JackTokenType::STRINGCONST);
        assert_eq!(tokenizer.tokens[1].token_type, JackTokenType::SYMBOL);
    }

    #[test]
    fn test_parse_simple_command_with_string_simple_with_variety() {
        let mut tokenizer: JackTokenizer =
            JackTokenizer::new(&String::from("\"hello world\";"), false);
        tokenizer.tokenize();
        assert_eq!(tokenizer.instructions.len(), 2);

        // copy first instruction
        let first = tokenizer.instructions[0].clone();
        let last = tokenizer.instructions[tokenizer.instructions.len() - 1].clone();

        // assert
        assert_eq!(first.value, "\"hello world\"");
        assert_eq!(last.value, ";");

        // check tokens
        assert_eq!(tokenizer.tokens.len(), 2);

        // check tokens type as well
        assert_eq!(tokenizer.tokens[0].token_type, JackTokenType::STRINGCONST);
        assert_eq!(tokenizer.tokens[1].token_type, JackTokenType::SYMBOL);
    }

    #[test]
    fn test_parse_simple_command_with_string_simple() {
        let mut tokenizer: JackTokenizer =
            JackTokenizer::new(&String::from("\"hello world\""), false);
        tokenizer.tokenize();
        assert_eq!(tokenizer.instructions.len(), 1);

        // copy first instruction
        let first = tokenizer.instructions[0].clone();

        // assert
        assert_eq!(first.value, "\"hello world\"");

        // check tokens
        assert_eq!(tokenizer.tokens.len(), 1);

        // check tokens type as well
        assert_eq!(tokenizer.tokens[0].token_type, JackTokenType::STRINGCONST);
    }

    #[test]
    fn test_parse_complex_operation() {
        let mut tokenizer: JackTokenizer = JackTokenizer::new(
            &String::from("let int a = 1\nlet String b = \"this is an string\";"),
            false,
        );
        tokenizer.tokenize();

        // check instructions
        assert_eq!(tokenizer.instructions.len(), 11);
    }

    #[test]
    fn test_parse_simple_command_with_int() {
        let mut tokenizer: JackTokenizer = JackTokenizer::new(&String::from("1"), false);
        tokenizer.tokenize();
        assert_eq!(tokenizer.instructions.len(), 1);

        // copy first instruction
        let first = tokenizer.instructions[0].clone();

        // assert
        assert_eq!(first.value, "1");

        // check tokens
        assert_eq!(tokenizer.tokens.len(), 1);

        // check tokens type as well
        assert_eq!(tokenizer.tokens[0].token_type, JackTokenType::INTCONST);
    }

    #[test]
    fn test_parse_simple_command_with_identifier() {
        let mut tokenizer: JackTokenizer = JackTokenizer::new(&String::from("a"), false);
        tokenizer.tokenize();
        assert_eq!(tokenizer.instructions.len(), 1);

        // copy first instruction
        let first = tokenizer.instructions[0].clone();

        // assert
        assert_eq!(first.value, "a");

        // check tokens
        assert_eq!(tokenizer.tokens.len(), 1);

        // check tokens type as well
        assert_eq!(tokenizer.tokens[0].token_type, JackTokenType::IDENTIFIER);
    }

    #[test]
    #[should_panic(expected = "Cannot use tokenize without a valid content")]
    fn test_empty_initialization() {
        let mut tokenizer: JackTokenizer = JackTokenizer::new(&String::from(""), false);
        tokenizer.tokenize();
    }

    #[test]
    fn check_if_has_symbol() {
        let symbol: &str = "a;{}";
        assert_eq!(JackTokenizer::has_symbol(symbol), true)
    }

    #[test]
    fn check_if_is_symbol() {
        let symbol: char = '{';
        assert_eq!(JackTokenizer::is_symbol(symbol), true)
    }

    #[test]
    fn check_if_is_symbol_false() {
        let symbol: char = 'a';
        assert_eq!(JackTokenizer::is_symbol(symbol), false)
    }

    #[test]
    fn test_parse_tree_complex() {
        let mut tokenizer: JackTokenizer = JackTokenizer::new(&String::from("let a = 1;\n"), false);
        tokenizer.tokenize();

        // check instructions
        assert_eq!(tokenizer.instructions.len(), 5);

        tokenizer.parse_tree();

        println!("{:?}", tokenizer.ast);

        assert_eq!(tokenizer.tokens.len(), 0);
        assert_eq!(tokenizer.ast.len(), 1); // one let statement
    }
}
