use crate::tokenizer::JackToken;

pub struct JackParser {
    pub tokens : Vec<JackToken>
}

impl JackParser {
    pub fn new() -> JackParser {
        JackParser {
            tokens : Vec::new()
        }
    }

    pub fn parse(&mut self, tokens : Vec<JackToken>) -> &Self {
        self
    }
}
