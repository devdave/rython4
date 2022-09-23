use std::io::Read;


use crate::cleaner;
use crate::tokens::{Position, Token, TokError, TType, OPERATOR_RE};
use super::code_line::CodeLine;

use crate::tokens::patterns::{NAME_RE, COMMENT, FLOATING_POINT, POSSIBLE_NAME, POSSIBLE_ONE_CHAR_NAME, SPACE_TAB_FORMFEED_RE, NUMBER };

//TODO put these somewhere better
const MAXINDENT: usize = 999;
const TABSIZE: usize = 8;



enum StringType {
    SingleApos,
    TripleApos,
    SingleQuote,
    TripleQuote,
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

        let display = filename.as_ref().display();
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

    pub fn process_single_line(&mut self, raw_line: String) -> Result<Vec<Token>, TokError> {

        let lines: Vec<String> = cleaner(raw_line);

        return self.generate(lines);
    }

    pub fn process_interactive(&mut self, input: String, state: &State) {

    }


    pub fn generate(&mut self, source: Vec<String>) -> Result<Vec<Token>, TokError> {

        let mut product: Vec<Token> = Vec::new();
        let mut state = State::new();

        if self.config.skip_encoding == false {
            product.push(Token::quick(TType::Encoding, 0, 0, 0, "utf-8".to_string()));
        }


        for (lineno, line,) in source.clone().into_iter().enumerate() {

            match self.process_line(&mut state, lineno.saturating_add(1), line) {
                Ok(mut tokens) => product.append(&mut tokens),
                Err(issue) => return Err(issue),
            }
        }

        if self.config.skip_endmarker == false {
            product.push(Token::quick(TType::EndMarker, source.len()+1, 0, 0, "".to_string()));
        }

        return Ok(product);
    }

    fn process_line(&self, state: &mut State, lineno: usize, line: String) -> Result<Vec<Token>, TokError> {
        let mut product: Vec<Token> = Vec::new();
        println!("Parsing {}-`{:?}`", lineno, line);

        let mut code = CodeLine::new(line);
        let mut is_statement = false;

        //Deal with blank lines
        if code.is_empty() == true {
            //Blow away the indent stack!
            if state.indent_stack.len() > 0 {
                state.indent_stack.pop();
                product.push(Token::quick(TType::Dedent, lineno, 0, 0, "".to_string()));
            }
            product.push(Token::quick(TType::NL, lineno, 0, 0, "\n".to_string()));
            return Ok(product);
        }

        //Handle indent/dedet here




        while code.remaining() > 0 {
            let col_pos = code.position();
            if let Some((new_pos, found)) = code.return_match(POSSIBLE_NAME.to_owned()) {
                product.push(Token::quick(TType::Name, lineno, col_pos, new_pos, found));
                is_statement = true;

            } else if let Some((new_pos, found)) = code.return_match(POSSIBLE_ONE_CHAR_NAME.to_owned()) {
                product.push(Token::quick(TType::Name, lineno, col_pos, new_pos, found));
                is_statement = true;

            }
            else if let Some((new_pos, found)) = code.return_match(FLOATING_POINT.to_owned()) {
                product.push(Token::quick(TType::Number, lineno, col_pos, new_pos, found));
            }
            //The "SUPER" Number regex
            else if let Some((new_pos, found)) = code.return_match(NUMBER.to_owned()) {
                product.push(Token::quick(TType::Number, lineno, col_pos, new_pos, found));
            }
            else if let Some((new_pos, found)) = code.return_match(OPERATOR_RE.to_owned()) {
                product.push(Token::quick(TType::Op, lineno, col_pos, new_pos, found));
                is_statement = true;
            }
            //Look for WS
            else if let Some((new_pos, found)) = code.return_match(SPACE_TAB_FORMFEED_RE.to_owned()) {
                //and ignore it

            }
            else {
                println!("No pattern matched!");
                if let Some(sym) = code.get() {
                    if sym == " " {
                        //skipping white space
                    } else if sym == "\n" {
                        if is_statement == false {
                            product.push(Token::quick(TType::NL, lineno, col_pos, code.position(), "\n".to_string()));
                        } else {
                            product.push(Token::quick(TType::Newline, lineno, col_pos, code.position(), "\n".to_string()));
                        }
                        break;
                    } else {
                        return Err(TokError::BadCharacter(sym.chars().nth(0).expect("char")));
                    }
                }

            }

        }



        return Ok(product);

    }

}