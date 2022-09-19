
use crate::tokens::{Position, Token, TokError};

enum StringType {
    SIMGLE_APOS,
    TRIPLE_APOS,
    SINGLE_QUOTE,
    TRIPLE_QUOTE,
}

#[derive(Default)]
struct TConfig {
    pub skip_encoding: bool,
}

struct State {
    // Parenthesis symbol ([{ and starting position
    paren_def: Vec<(char, Position)>,
    //For now accept/allow only spaces
    indent_stack: Vec<usize>,
    //string handling logic
    string_continues: bool,
    string_type: Option<StringType>,
    string_start: Option<Position>,
    string_buffer: String,
}

impl State {
    fn new() -> Self {
        Self {
            paren_def: Vec::new(),
            indent_stack: Vec::new(),
            string_continues: false,
            string_type: None,
            string_start: None,
            string_buffer: "".into(),
        }
    }
}



struct Tokenizer {
    config: TConfig,

}

impl Tokenizer {

    pub fn new(config: TConfig) -> Self {
        Self {
            config,
        }
    }




    pub fn generate(&mut self, source: Vec<String>) -> Result<Vec<Token>, TokError> {

        let product: Vec<Token> = Vec::new();
        let state = State::new();

        for (lineno, line,) in source.into_iter().enumerate() {
            println!("Parsing {}-`{:?}`", lineno, line);
        }

        return Ok(product);
    }

}