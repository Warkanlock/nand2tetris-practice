#[derive(Debug, PartialEq, Copy, Clone)]
pub struct JackCommand {
    // we should diagram a jack token
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct JackToken {}

pub struct JackTokenizer {
    pub content: String, // file content from input
    pub instructions: Vec<JackCommand>,
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
    pub fn new(content: &String, verbose : bool) -> Self {
        JackTokenizer {
            content: content.to_string(), // copy content, not really handy if big files
            instructions: Vec::new(),
            tokens: Vec::new(),
            verbose,
        }
    }

    pub fn parse(&self) -> &Self {
        // invalidate parse if content is not valid
        if self.content.len() == 0 {
            panic!("Cannot use parse without a valid content")
        }

        /*
         * Our approach here will be to go top-down (instead of bottom-up)
         */
        let internal_tokens : Vec<&str> = self.content.split(' ').collect();

        // iterate across internal tokens of the content file
        for internal_token in internal_tokens {
            if self.verbose {
                println!("token > {:?}", internal_token);
            }
        }

        // TODO: parse content into Jack Commands and then into JackTokens
        &self
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
    #[should_panic(expected = "Cannot use parse without a valid content")]
    fn test_empty_initialization() {
        let tokenizer : JackTokenizer = JackTokenizer::new(&String::from(""), false);
        tokenizer.parse();
    }
}
