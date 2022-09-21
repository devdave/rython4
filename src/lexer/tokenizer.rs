use std::io::Read;
use std::mem::discriminant;
use std::path::PathBuf;
use crate::cleaner;
use crate::tokens::{Position, Token, TokError, TType, OPERATOR_RE};
use super::code_line::CodeLine;

use crate::tokens::patterns::{NAME_RE, COMMENT, FLOATING_POINT, POSSIBLE_NAME, POSSIBLE_ONE_CHAR_NAME, SPACE_TAB_FORMFEED_RE, };

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

    pub fn process_file<P>(&mut self, filename: P) -> Result<Vec<Token>, TokError>
        where P: AsRef<std::path::Path>,  {

        let display = filename.as_ref().display();;
        let mut buffer: String = String::new();

        let mut file = std::fs::File::open(&filename).expect("Failed to open file");


        let read_res = file.read_to_string(&mut buffer);
        if let Ok(read_len) = read_res {
            if read_len <= 0 {
                panic!("{:?} is empty or failed to read!", display);
            }
        } else if let Err(read_err) = read_res {
            panic!("Failed to read {:?} - reason {:?}", display, read_err );
        }

        let lines: Vec<String> = cleaner(buffer);

        return self.generate(lines);


    }


    pub fn generate(&mut self, source: Vec<String>) -> Result<Vec<Token>, TokError> {

        let mut product: Vec<Token> = Vec::new();
        let mut state = State::new();

        product.push(Token::quick(TType::Encoding, 0, 0, 0, "utf-8".to_string()));

        for (lineno, line,) in source.into_iter().enumerate() {
            
            match self.process_line(&state, lineno.saturating_add(1), line) {
                Ok(mut tokens) => product.append(&mut tokens),
                Err(issue) => return Err(issue),
            }
        }

        return Ok(product);
    }

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