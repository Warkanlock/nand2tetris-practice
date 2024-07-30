pub struct JackCommand {
 // we should diagram a jack token
}

pub struct JackTokenizer {
    pub instructions: Vec<JackCommand>,
}

impl JackTokenizer {
    pub fn new() -> Self {
        JackTokenizer {
            instructions: Vec::new(),
        }
    }
    
}

#[cfg(test)]
mod tests {
    use crate::parser::JackTokenizer;

    #[test]
    fn test_jack_tokenizer() {
        let tokenizer = JackTokenizer::new();
        assert_eq!(tokenizer.instructions.len(), 0);
    }
}
