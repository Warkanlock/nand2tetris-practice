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

pub const JACK_SYMBOLS: [&str; 19] = [
    "{", "}", "(", ")", "[", "]", ".", ",", ";", "+", "-", "*", "/", "&", "|", "<", ">", "=", "~",
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
                    symbols.push(acc_word.to_string());
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
        for line in content_as_lines.iter() {
            // if the line is a comment, discard it as a whole and continue
            if JackTokenizer::has_comment(line) {
                continue;
            }

            // split content without whitespaces
            let content: Vec<&str> =
                line.split_whitespace().collect::<Vec<&str>>();

            for element in content.iter() {
                if JackTokenizer::has_symbol(element) {
                    self.extract_symbols(element, &mut symbols);
                } else {
                    symbols.push(element.to_string());
                }
            }
        }

        // iterate across internal tokens of the content file
        for (index, input_string) in symbols.iter().enumerate() {
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
}
