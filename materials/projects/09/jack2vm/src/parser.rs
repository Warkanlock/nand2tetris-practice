#[derive(Debug, PartialEq, Clone)]
pub struct JackInstruction {
    pub line: usize,
    pub value: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct JackToken {
    pub token_type: JackTokenType,
    pub keyword: JackKeyword,

    // depending on the token_type, the following fields will be filled
    // or not
    pub symbol: Option<char>,
    pub identifier: Option<String>,
    pub int_val: Option<i32>,
    pub string_val: Option<String>,
}

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
    METHOD,
    FUNCTION,
    CONSTRUCTOR,
    INT,
    BOOLEAN,
    CHAR,
    VOID,
    VAR,
    STATIC,
    FIELD,
    LET,
    DO,
    IF,
    ELSE,
    WHILE,
    RETURN,
    TRUE,
    FALSE,
    NULL,
    THIS,
}

pub struct JackTokenizer {
    pub content: String, // file content from input
    pub instructions: Vec<JackInstruction>,
    pub tokens: Vec<JackToken>,
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
            verbose,
        }
    }

    pub fn tokenize(&mut self) -> &Self {
        // invalidate tokenizer if content is not valid
        if self.content.len() == 0 {
            panic!("Cannot use tokenize without a valid content")
        }

        /*
         * Our approach here will be to go top-down (instead of bottom-up)
         */
        let content_without_breaklines = self.content.lines().collect::<Vec<&str>>().join(" ");
        let content_without_whitespace: Vec<&str> = content_without_breaklines.split_whitespace().collect::<Vec<&str>>();

        // iterate across internal tokens of the content file
        for (index, input_string) in content_without_whitespace.iter().enumerate() {
            if self.verbose {
                println!("input string > {:?}", input_string);
            }

            // push the command into the instructions vector
            self.instructions.push(JackInstruction {
                line: index + 1,
                value: input_string.to_string(),
            });
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::JackTokenizer;

    #[test]
    fn test_initialization() {
        let tokenizer = JackTokenizer::new(&String::from("// empty content"), false);
        assert_eq!(tokenizer.instructions.len(), 0);
    }

    #[test]
    fn test_parse_simple_command() {
        let mut tokenizer: JackTokenizer = JackTokenizer::new(&String::from("let a = 1;"), false); 
        tokenizer.tokenize();
        assert_eq!(tokenizer.instructions.len(), 4);

        // copy first instruction
        let first = tokenizer.instructions[0].clone();

        // assert
        assert_eq!(first.value, "let");
    }

    #[test]
    #[should_panic(expected = "Cannot use tokenize without a valid content")]
    fn test_empty_initialization() {
        let mut tokenizer: JackTokenizer = JackTokenizer::new(&String::from(""), false);
        tokenizer.tokenize();
    }
}
