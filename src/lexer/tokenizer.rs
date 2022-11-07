use std::cmp::Ordering;

use std::io::Read;

use regex::Regex;


use crate::cleaner;
use crate::lexer::tokenizer::StringType::TripleQuote;
use crate::tokens::{Position, Token, TokError, TType, OPERATOR_RE};
use super::code_line::CodeLine;

use crate::tokens::patterns::{
                            // NAME_RE,
                            COMMENT,
                            BL_COMMENT,
                            FLOATING_POINT,
                            POSSIBLE_NAME,
                            POSSIBLE_ONE_CHAR_NAME,
                            SPACE_TAB_FORMFEED_RE,
                          NUMBER,
                          CAPTURE_QUOTE_STRING,
                          CAPTURE_APOS_STRING,
                          TRIPLE_QUOTE_START,
                          TRIPLE_QUOTE_CLOSE,
                          TRIPLE_SINGLE_START,
                          TRIPLE_SINGLE_CLOSE,
                          CAPTURE_TRIPLE_STRING,
                          ANY_NAME
};





//TODO put these somewhere better
const MAXINDENT: usize = 999;
const TABSIZE: usize = 8;


#[derive(Debug)]
enum StringType {
    SingleApos,
    TripleApos,
    SingleQuote,
    TripleSingleQuote,
    TripleQuote,
}

#[derive(Default)]
pub struct TConfig {
    pub skip_encoding: bool,
    pub skip_endmarker: bool,
}

#[derive(Debug)]
struct State {
    // Parenthesis symbol ([{ and starting position
    paren_depth: Vec<(char, Position)>,
    //For now accept/allow only spaces
    indent_stack: Vec<usize>,
    //string handling logic
    string_continues: bool,
    string_type: Option<StringType>,
    string_start: Option<Position>,
    string_buffer: String,
    //Indentation logic
    indent: usize,
    tabsize: usize,
    altindentstack: Vec<usize>,
    line_continues: bool,
}

impl State {
    fn new() -> Self {
        Self {
            paren_depth: Vec::new(),
            indent_stack: Vec::new(),
            string_continues: false,
            string_type: None,
            string_start: None,
            string_buffer: "".into(),
            indent: 0,
            tabsize: TABSIZE, //... apparently determined by a fair dice roll.
            altindentstack: Vec::new(),
            line_continues: false,

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

    pub fn tokenize_file<P>(filename:P, config: TConfig) -> Result<Vec<Token>, TokError>
        where P: AsRef<std::path::Path>, {
        let mut tokenizer = Tokenizer::new(config);
        return tokenizer.process_file(filename);
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

    pub fn process_interactive(&mut self, _input: String, _state: &State) {

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
                Err(issue) => {
                    println!("tokenizer failure: {:?}", product);
                    return Err(issue)
                },
            }

        }

        //Check for indents and push matching dedents
        if state.indent_stack.len() > 0 {
            while state.indent_stack.len() > 0 {
                let _last_size = state.indent_stack.pop().unwrap();
                product.push(Token::quick(TType::Dedent, source.len()+1, 0, 0, "".to_string()));
            }
        }

        if state.paren_depth.len() > 0 {
            println!("State: {:#?}", state);
            let (last_paren, pos) = state.paren_depth.pop().expect("paren");
            println!("tokenizer failure: {:#?}", product);

            return Err(TokError::UnmatchedClosingParen(last_paren));
        }

        if self.config.skip_endmarker == false {
            // product.push(Token::quick(TType::NL, source.len()+1, 0, 1, "\n".to_string()));
            product.push(Token::quick(TType::EndMarker, source.len()+1, 0, 0, "".to_string()));
        }

        return Ok(product);
    }

    fn process_line(&self, state: &mut State, lineno: usize, line: String) -> Result<Vec<Token>, TokError> {
        let mut product: Vec<Token> = Vec::new();


        let mut is_statement = false;

        //Deal with blank lines
        if line.len() == 1 && line == "\n" {

            return Ok(product);
        }




        if state.string_continues == false {


            //Handle blank lines
            if line.trim().len() == 0 {
                //We are done!
                return Ok(product);
            }

            //only do indent and dedent if we're not inside brackets
            if state.paren_depth.len() == 0 && state.line_continues == false {

                //Ignore blank lines with comments
                if let Some(test) = BL_COMMENT.find(&line) {

                }
                //Handle indent/dedent here if there is a statement

                else if let Some(ws_match) = SPACE_TAB_FORMFEED_RE.find(&line) {

                    //TODO make sure there is no mixing of tabs, spaces, and form feed.
                    //TODO drop support for form feed?
                    let current_size: usize = ws_match.end() - ws_match.start();
                    let last_size = state.indent_stack.last().unwrap_or(&0);

                    match current_size.cmp(last_size) {
                        Ordering::Greater => {
                            //push on a new indent
                            if state.indent_stack.len() + 1 > MAXINDENT {
                                return Err(TokError::TooDeep);
                            }
                            state.indent_stack.push(current_size);
                            product.push(Token::quick(TType::Indent, lineno, 0, 0, "".to_string()));
                            state.indent = current_size;
                        },
                        Ordering::Less => {

                            //Pop that indent!
                            //TODO this is flawed and needs to pop only to the correct/new indentation

                            while state.indent_stack.len() > 0 {
                                let last_size = state.indent_stack.pop().unwrap();
                                if last_size != current_size {
                                    product.push(Token::quick(TType::Dedent, lineno, 0, 0, "".to_string()));
                                    state.indent = current_size;
                                } else {
                                    state.indent_stack.push(last_size);
                                    break;
                                }
                            }
                        },
                        Ordering::Equal => {
                            //Do nothing
                        }
                    }
                } else if state.indent_stack.len() > 0 && line.trim().chars().nth(0).unwrap_or('N') != '#' {
                    //Pop all indents

                    while state.indent_stack.len() > 0 {
                        let last_size = state.indent_stack.pop().unwrap();

                        product.push(Token::quick(TType::Dedent, lineno, 0, 0, "".to_string()));
                    }

                    state.indent = 0;
                }
            }
        }

        //reset this flag as its served its purpose
        state.line_continues = false;



        let mut code = CodeLine::new(line);

        while code.remaining() > 0 {
            let col_pos = code.position();

            if state.string_continues == true {
                //TODO check for string continuation type/state.type to use the correct regex
                if let Some((new_pos, found)) = code.return_match(TRIPLE_QUOTE_CLOSE.to_owned()) {
                    state.string_buffer = format!("{}{}", state.string_buffer, found);
                    let start = state.string_start.as_ref().unwrap().clone();

                    product.push(Token::Make(
                        TType::String,
                        start,
                        Position::m(new_pos, lineno),
                        state.string_buffer.clone())
                    );
                    state.string_start = None;
                    state.string_continues = false;
                    state.string_type = None;

                }
                else if let Some((new_pos, found)) = code.return_match(TRIPLE_SINGLE_CLOSE.to_owned()) {
                    state.string_buffer = format!("{}{}", state.string_buffer, found);
                    let start = state.string_start.as_ref().unwrap().clone();

                    product.push(
                        Token::Make(
                            TType::String,
                            start,
                            Position::m(new_pos, lineno),
                            state.string_buffer.clone(),
                        )
                    );
                    state.string_start = None;
                    state.string_continues = false;
                    state.string_type = None;
                }
                else {
                    //Consume the whole line
                    if let Some((_new_pos, found )) = code.return_match(Regex::new(r#"\A((\n|.)*)"#).expect("regex")) {
                        state.string_buffer = format!("{}{}", state.string_buffer, found);
                    }

                }


            }

            //Capture single line triple quoted string
            else if let Some((new_pos, found )) = code.return_match(CAPTURE_TRIPLE_STRING.to_owned()) {
                product.push(Token::quick(TType::String, lineno, col_pos, new_pos, found));
            }
            //Capture multi-line string start here
            else if let Some((_new_pos, found)) = code.return_match(TRIPLE_QUOTE_START.to_owned()) {
                //Assume this consumed the entire line!
                state.string_continues = true;
                state.string_start = Some(Position::m(col_pos, lineno));
                state.string_buffer = found;
                state.string_type = Some(StringType::TripleQuote);
            }
            //Look for "string"
            else if let Some((new_pos, found)) = code.return_match(CAPTURE_QUOTE_STRING.to_owned()) {

                product.push(Token::quick(TType::String, lineno, col_pos, new_pos, found));
            }
            else if let Some((_, found)) = code.return_match(TRIPLE_SINGLE_START.to_owned()) {
                state.string_continues = true;
                state.string_start = Some(Position::m(col_pos, lineno));
                state.string_buffer = found;
                state.string_type = Some(StringType::TripleSingleQuote);
            }
            //Look for 'string'
            else if let Some((new_pos, found)) = code.return_match(CAPTURE_APOS_STRING.to_owned()) {
                product.push(Token::quick(TType::String, lineno, col_pos, new_pos, found));
            }


            else if let Some((new_pos, found)) = code.return_match(POSSIBLE_NAME.to_owned()) {



                //Check for async and await operators
                if found == "async" {
                    product.push(Token::quick(TType::Async, lineno, col_pos, new_pos, found));
                }
                else if found == "await" {
                    product.push(Token::quick(TType::Await, lineno, col_pos, new_pos, found));
                }
                else {
                    product.push(Token::quick(TType::Name, lineno, col_pos, new_pos, found));
                }
                is_statement = true;

            } else if let Some((new_pos, found)) = code.return_match(POSSIBLE_ONE_CHAR_NAME.to_owned()) {
                product.push(Token::quick(TType::Name, lineno, col_pos, new_pos, found));
                is_statement = true;

            }
            //Attempt to capture floats - TODO test if still needed
            else if let Some((new_pos, found)) = code.return_match(FLOATING_POINT.to_owned()) {
                product.push(Token::quick(TType::Number, lineno, col_pos, new_pos, found));
                is_statement = true;
            }
            //The "SUPER" Number regex
            else if let Some((new_pos, found)) = code.return_match(NUMBER.to_owned()) {
                product.push(Token::quick(TType::Number, lineno, col_pos, new_pos, found));
                is_statement = true;
            }
            else if let Some((new_pos, found)) = code.return_match(OPERATOR_RE.to_owned()) {



                if found == "(" || found == "[" || found == "{" {
                    state.paren_depth.push(
                        (found.chars().nth(0).expect("expected char"),
                            Position::t2((lineno, col_pos))
                        )
                    );
                } else if found == ")" || found == "]" || found == "}" {
                    let current = found.chars().nth(0).expect("char");

                    if state.paren_depth.len() == 0 {
                        return Err(TokError::UnmatchedClosingParen(current));
                    }
                    if let Some((last_paren, start_pos)) = state.paren_depth.pop() {
                        if (
                            (last_paren == '(' && current != ')')
                            || (last_paren == '[' && current != ']')
                            || (last_paren == '{' && current != '}')
                        ) {
                            return Err(TokError::MismatchedClosingParenOnLine(current, last_paren, lineno));
                        }
                    } else {
                        panic!("Expected element in paren stack but got nothing: {:#?}", state);
                    }
                }

                product.push(Token::quick(TType::Op, lineno, col_pos, new_pos, found));
                is_statement = true;
            }
            //Look for WS
            else if let Some((_, found)) = code.return_match(SPACE_TAB_FORMFEED_RE.to_owned()) {
                //and ignore it
                if state.string_continues == true {
                    state.string_buffer = format!("{}{}", state.string_buffer, found);
                }

            }
            //Look for comments
            else if let Some((_, _)) = code.return_match(COMMENT.to_owned()) {
                //Don't add comments into product
                //Consume the newline
                if code.peek().unwrap() == "\n" {
                    code.get();
                    if product.len() > 0 && is_statement == true && state.paren_depth.len() == 0 {
                        product.push(Token::quick(TType::NL, lineno, col_pos, code.position(), "".to_string()));
                    }
                }
                //product.push(Token::quick(TType::Comment, lineno, col_pos, new_pos, found));
            }
            //Last ditch look for Name's
            else if let Some((_, found)) = code.return_match(ANY_NAME.to_owned()) {
                //TODO remove once I've caught the bugs that lead to this
                println!("Captured any name: {:?} @ {}:{}", found, lineno, col_pos);

            }
            else {
                if let Some(sym) = code.get() {
                    if sym == " " {
                        //skipping white space
                        //Except if we're inside a triple quoted/multiline string!
                        if state.string_continues == true {
                            state.string_buffer = format!("{}{}", state.string_buffer, sym);
                        }
                    }
                    else if sym == "\\" {
                        //Don't do anything, TODO how to signal a line continuation?
                        state.line_continues = true;
                        //abort processing for now, nothing matters after a \
                        return Ok(product);
                    }
                     else if sym == "\n" {

                        if state.paren_depth.len() > 0 {
                            continue
                        }
                        else if state.string_continues == true {
                            // TODO is this really the fastest/"best" way to append to a String?
                            state.string_buffer = format!("{}{}", state.string_buffer, sym);
                        }
                        //TODO FIX the parsing grammar mixup between Newline and NL - this is a hack until then
                        else if true {
                            product.push(Token::quick(TType::NL, lineno, col_pos, code.position().saturating_sub(1), "".to_string()));
                        } else {
                            product.push(Token::quick(TType::Newline, lineno, col_pos, code.position(), "".to_string()));
                        }
                        break;
                    } else {
                        println!("Bad character @ {}:{}", lineno, col_pos);
                        println!("State; {:#?}", state);
                        return Err(TokError::BadCharacter(sym.chars().nth(0).expect("char")));
                    }
                }

            }

        }



        return Ok(product);

    }

}