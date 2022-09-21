
use crate::tokens::{Position, Token, TokError};

enum StringType {
    SIMGLE_APOS,
    TRIPLE_APOS,
    SINGLE_QUOTE,
    TRIPLE_QUOTE,
}

#[derive(Default)]
pub struct TConfig {
    pub skip_encoding: bool,
    pub skip_endmarker: bool,
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



pub struct Tokenizer {
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
    fn process_line(&self, state: &State, lineno: usize, line: String) -> Result<Vec<Token>, TokError> {
        let mut product: Vec<Token> = Vec::new();
        println!("Parsing {}-`{:?}`", lineno, line);

        let mut code = CodeLine::new(line);


        while code.remaining() > 0 {
            let col_pos = code.position();
            if let Some((new_pos, found)) = code.return_match(POSSIBLE_NAME.to_owned()) {
                product.push(Token::quick(TType::Name, lineno, col_pos, new_pos, found));

            } else if let Some((new_pos, found)) = code.return_match(POSSIBLE_ONE_CHAR_NAME.to_owned()) {
                product.push(Token::quick(TType::Name, lineno, col_pos, new_pos, found));

            }
            else if let Some((new_pos, found)) = code.return_match(FLOATING_POINT.to_owned()) {
                product.push(Token::quick(TType::Number, lineno, col_pos, new_pos, found));
            }
            else if let Some((new_pos, found)) = code.return_match(OPERATOR_RE.to_owned()) {
                product.push(Token::quick(TType::Op, lineno, col_pos, new_pos, found));
            }
            //Look for WS
            else if let Some((new_pos, found)) = code.return_match(SPACE_TAB_FORMFEED_RE.to_owned()) {
                //and ignore it
            }
            else {
                if let Some(sym) = code.get() {
                    if sym == " " {
                        //skipping white space
                    } else if sym == "\n" {
                        product.push(Token::quick(TType::NL, lineno, col_pos, code.position(), "\n".to_string()));
                        break;
                    }
                }
                break;
            }

        }

        return Ok(product);
    }

}