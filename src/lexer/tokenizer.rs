
use std::cmp::Ordering;

use std::io::Read;


use regex::Regex;


use crate::cleaner;
// use crate::lexer::tokenizer::StringType::TripleQuote;
use crate::tokens::{Position, Token, TokError, TType, OPERATOR_RE};
use super::code_line::CodeLine;

use crate::tokens::patterns::{
                            // NAME_RE,
                            COMMENT,
                            BL_COMMENT,
                            // FLOATING_POINT,
                            // POSSIBLE_NAME,
                            // POSSIBLE_ONE_CHAR_NAME,
                            SPACE_TAB_FORMFEED_RE,
                          // NUMBER,
                          // CAPTURE_QUOTE_STRING,
                          // CAPTURE_APOS_STRING,
                          // TRIPLE_QUOTE_START,
                          TRIPLE_QUOTE_CLOSE,
                          // TRIPLE_SINGLE_START,
                          TRIPLE_SINGLE_CLOSE,
                          // CAPTURE_TRIPLE_STRING,
                          // ANY_NAME,
                        SINGLE_APOS_CLOSE,
                        SINGLE_QUOTE_CLOSE,
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

pub struct TConfig {
    pub skip_encoding: bool,
    pub skip_endmarker: bool,
}

impl TConfig {

    pub fn default() -> Self {
        Self {
            skip_encoding: true,
            skip_endmarker: false,
        }
    }

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

    pub fn tokenize_file<P>(filename: P, config: TConfig) -> Result<Vec<Token>, TokError>
        where P: AsRef<std::path::Path>, {
        let mut tokenizer = Tokenizer::new(config);
        return tokenizer.process_file(filename);
    }

    pub fn process_file<P>(&mut self, filename: P) -> Result<Vec<Token>, TokError>
        where P: AsRef<std::path::Path>, {
        let display = filename.as_ref().display();
        let mut buffer: String = String::new();

        let mut file = std::fs::File::open(&filename).expect("Failed to open file");

        println!("File opened, reading to string");

        let read_res = file.read_to_string(&mut buffer);
        if let Ok(read_len) = read_res {
            if read_len <= 0 {
                panic!("{:?} is empty or failed to read!", display);
            }
        } else if let Err(read_err) = read_res {
            panic!("Failed to read {:?} - reason {:?}", display, read_err);
        }

        println!("File loaded, cleaning now");
        let lines: Vec<String> = cleaner(buffer);

        println!("Tokenizing now");
        return self.generate(&lines);
    }

    pub fn process_single_line(&mut self, raw_line: String) -> Result<Vec<Token>, TokError> {
        let lines: Vec<String> = cleaner(raw_line);

        return self.generate(&lines);
    }

    pub fn process_interactive(&mut self, _input: String, _state: &State) {}


    pub fn generate(&mut self, source: &Vec<String>) -> Result<Vec<Token>, TokError> {
        //TODO break this down into subfunctions as it has gotten too long
        let mut product: Vec<Token> = Vec::new();
        let mut state = State::new();

        if self.config.skip_encoding == false {
            product.push(Token::quick(TType::Encoding, 0, 0, 0, "utf-8".to_string()));
        }


        for (lineno, line, ) in source.clone().into_iter().enumerate() {
            match self.process_line(&mut state, lineno.saturating_add(1), line) {
                Ok(mut tokens) => {
                    println!("Processed {:?}", source.get(lineno));
                    println!("Got {:#?}", tokens);
                    product.append(&mut tokens)
                },
                Err(issue) => {
                    println!("tokenizer failure: {:#?}", product);
                    return Err(issue)
                },
            }
        }

        //Check for indents and push matching dedents
        if state.indent_stack.len() > 0 {
            while state.indent_stack.len() > 0 {
                let _last_size = state.indent_stack.pop().unwrap();
                product.push(Token::quick(TType::Dedent, source.len(), 0, 0, "".to_string()));
            }
        }

        if state.paren_depth.len() > 0 {
            println!("State: {:#?}", state);
            let (last_paren, _pos) = state.paren_depth.pop().expect("paren");


            return Err(TokError::UnmatchedClosingParen(last_paren));
        }

        if self.config.skip_endmarker == false {
            // product.push(Token::quick(TType::NL, source.len()+1, 0, 1, "\n".to_string()));
            product.push(Token::quick(TType::EndMarker, source.len() + 1, 0, 0, "".to_string()));
        }

        return Ok(product);
    }

    fn fetch_hexidecimal(&mut self, code: &mut CodeLine, _state: &State) -> Result<Option<String>, TokError>
    {
        let mut found: String = String::from("0x");


        while code.remaining() > 0 {
            match code.peek_char() {
                Some('0'..='9') => {
                    let sym = code.get_char().expect("symbol");
                    found.push(sym);
                },
                Some('a'..='f') | Some('A'..='F') => {
                    let sym = code.get_char().expect("sym");
                    found.push(sym);
                },
                Some('_') => {
                    //Do nothing/ignore
                    code.get_char();
                },
                Some(' ') | Some('\n') | Some('#') | Some('\\') => {
                    //Finished the hexidecimal
                    return Ok(Some(found));
                },
                _ => {
                    return Ok(Some(found));
                }
            }
        }
        return Ok(Some(found));
    }

    fn fetch_binary(&mut self, code: &mut CodeLine, _state: &State) -> Result<Option<String>, TokError>
    {
        let mut found: String = String::from("0b");


        while code.remaining() > 0 {
            match code.peek_char() {
                Some('0') | Some('1') => {
                    let sym = code.get_char().expect("symbol");
                    found.push(sym);
                },
                Some(' ') | Some('\n') | Some('#') | Some('\\') => {
                    //Finished the hexidecimal
                    return Ok(Some(found));
                },
                Some('_') => {
                    //Do nothing/ignore
                    code.get_char();
                },
                _ => {
                    return Ok(Some(found));
                    //Err how did we get here?
                    // panic!("How did we get here? {:?} @ {:#?}", code.line, state);
                }
            }
        }
        return Ok(Some(found));
    }

    fn fetch_octal(&mut self, code: &mut CodeLine, _state: &State) -> Result<Option<String>, TokError>
    {
        let mut found: String = String::from("0o");


        while code.remaining() > 0 {
            match code.peek_char() {
                Some('0'..='7') => {
                    let sym = code.get_char().expect("symbol");
                    found.push(sym);
                },
                Some(' ') | Some('\n') | Some('#') | Some('\\') => {
                    //Finished the hexidecimal
                    return Ok(Some(found));
                },
                Some('_') => {
                    //Do nothing/ignore
                    code.get_char();
                },
                _ => {
                    //Err how did we get here?
                    // panic!("How did we get here? {:?} @ {:#?}", code.line, state);
                    return Ok(Some(found));
                }
            }
        }
        if found.len() > 0 {
            return Ok(Some(found));
        }

        return Ok(None);
    }



    fn attempt_string(quote: char, found: String, code: &mut CodeLine, state: &mut State) -> Result<Option<(TType, String)>, TokError>
    {
        //Assume we have STR_PREFIX + ( ' or " ) in found
        let mut quote_size = 1;
        let mut end_quote_size = 0;
        let mut body = found.clone();

        let test = code.get_char();

        //Check if this is a triple quoted string
        if test != None && test.unwrap() == quote {

            body.push(test.unwrap());

            let next_test = code.get_char();
            if next_test != None && next_test.unwrap() == quote {
                body.push(next_test.unwrap());
                quote_size = 3
            } else {
                code.rewind();
                end_quote_size = 1;
            }
        } else {
            code.rewind();
        }




        while (end_quote_size != quote_size) && code.remaining() > 0 {
            let next = code.get_char();
            if next == None || (next.unwrap() == '\n' && quote_size == 1)  {
                return Err(TokError::UnterminatedString);
            }

            let sym = next.unwrap();
            let mut escaped = false;

            if sym == quote {
                end_quote_size += 1;
            } else {
                end_quote_size = 0;
                if sym == '\\' {
                    escaped = true;
                    //Is this a line continuation?
                    body.push(sym);


                    if code.remaining() > 0 {
                        let escaped = code.get_char().unwrap();
                        body.push(escaped);
                    } else if quote_size != 3 {
                        return Err(TokError::UnterminatedString);
                    }
                }
            }
            if escaped != true {
                body.push(sym);
            }

        }

        if end_quote_size != quote_size {
            state.string_continues = true;
            state.string_buffer = body;
            if quote_size == 3 {
                if quote ==  '"' {
                    state.string_type = Some(StringType::TripleQuote);
                } else {
                    state.string_type = Some(StringType::TripleApos);
                }
            } else {
                if quote == '"' {
                    state.string_type = Some(StringType::SingleQuote);
                } else {
                    state.string_type = Some(StringType::SingleApos);
                }
            }
            return Ok(None);

        } else {
            return Ok(Some((TType::String, body)));
        }


    }

    fn is_potential_identifier_start(test: Option<char>) -> bool {

        let char_code: usize = test.unwrap_or_default() as usize;
        if char_code > 128 {
            return true;
        }

        match test {
            Some('a'..='z') | Some('A'..='Z') | Some('_') => {
                true
            },
            _ => {
                //This is not a valid start to a name
                false
            }
        }
    }

    fn is_potential_identifier_char(test: Option<char>) -> bool {

        let char_code: usize = test.unwrap_or_default() as usize;
        if char_code > 128 {
            return true;
        }

        match test {
            Some('a'..='z') | Some('A'..='Z') | Some('_') => {
                true
            },
            Some('0'..='9') => {
                true
            },
            _ => {
                //This is not a valid start to a name
                false
            }
        }

    }

    fn attempt_identifiers(&mut self, code: &mut CodeLine, state: &mut State) -> Result<Option<(TType, String)>, TokError>
    {
        let mut is_name = false;
        let mut found: String = String::new();

        let mut saw_b = false;
        let mut saw_r = false;
        let mut saw_u = false;
        let mut saw_f = false;


        if Tokenizer::is_potential_identifier_start(code.peek_char()) {
            while code.remaining() > 0 {
                let test = code.peek_char();

                match test {
                    Some('b') | Some('B') if !(saw_b || saw_u || saw_f) => {
                        saw_b = true;
                    },
                    Some('u') | Some('U') if !(saw_b || saw_u || saw_r || saw_f) => {
                        saw_u = true;
                    },
                    Some('r') | Some('R') if !(saw_r || saw_u ) => {
                        saw_r = true;
                    },
                    Some('f') | Some('F') if !(saw_f || saw_b || saw_u ) => {
                        saw_f = true;
                    },
                    _ => {
                        //We are likely not in a valid string prefix
                        break;
                    }
                }
                found.push(code.get_char().unwrap());

            }

            let mut test = code.get_char();
            match test {
                Some('"') | Some('\'') => {
                    let quote = test.unwrap();
                    found.push(quote);
                    return Tokenizer::attempt_string(quote, found, code, state);
                }
                _ => {}
            }

            let mut nonasci = false;

            while (Tokenizer::is_potential_identifier_char(test)) && code.remaining() > 0  {
                //TODO fix me
                // if (c >= 128) {
                //     nonasci = true;
                // }
                found.push(test.unwrap());

                test = code.get_char();
            }

            //c is not a potential identifier char!
            code.rewind();

            // if nonasci == true {
            //     return Err(TokError::BadCharacter(test));
            // }
            return Ok(Some( (TType::Name, found)));

        }

        return Ok(None);
    }

    fn attempt_number(&mut self, code: &mut CodeLine, state: &State) -> Result<Option<String>, TokError>
    {
        //The number regex is way too big so let's simplify things.

        let mut found: String = String::new();
        let hint = code.get_char();

        match hint {
                Some('0') => {
                    found.push('0');
                    match code.peek_char() {
                        Some('x') | Some('X') => {
                            code.get_char();
                            return self.fetch_hexidecimal(code, state);
                        },
                        Some('b') | Some('B') => {
                            code.get_char();
                            return self.fetch_binary(code, state);
                        },
                        Some('o') | Some('O') => {
                            code.get_char();
                            return self.fetch_octal(code, state);
                        }
                        Some('0'..='9') => {
                            let sym = code.get_char().unwrap();
                            found.push(sym);
                        }
                        Some('e') | Some('E') => {
                            found.push(code.get_char().unwrap());
                            return self.fetch_float_body(found, code, state);
                        }
                        Some('.') => {

                            return self.fetch_float_body(found, code, state);
                        }
                        _ => {
                            //got something non numeric
                            //lets break out.
                            return Ok(Some(found));
                        }
                    }
                },
                Some('1'..='9') => {
                    found.push(hint.unwrap());
                },
                _ => {
                    panic!("How did we get here? {:?}@{}", code.line, code.remaining());
                }
        }

        while code.remaining() > 0 {
            match code.peek_char() {
                None => {
                    break;
                }
                Some('0'..='9') => {
                    let sym = code.get_char().unwrap();
                    found.push(sym);
                },
                Some('e') | Some('E') | Some('.') => {
                    let sym = code.get_char().unwrap();
                    found.push(sym);
                    return self.fetch_float_body(found, code, state);
                }
                Some('_') => {
                    //skip over
                    let sym = code.get_char().unwrap();
                },
                _ => {
                    //unexpected char!
                    return Ok(Some(found));
                }
            }
        }


        return Ok(None);
    }

    fn attempt_floating_point(&mut self, code: &mut CodeLine, state: &State) -> Result<Option<String>, TokError> {
        let mut found: String = String::new();
        let point_sym = code.get_char().unwrap();
        assert_eq!(point_sym, '.', "How did we get here?");

        found.push(point_sym);

        match code.peek_char() {
            Some('0'..='9') => {
                let digit = code.get_char().unwrap();
                found.push(digit);
            },

            _ => {
                code.rewind(); //"push" the dot back onto unprocessed "stack"
                return Ok(None);
            }
        }

        return self.fetch_float_body(found, code, state);

    }

    fn fetch_float_body(&mut self, mut found: String, code: &mut CodeLine, state: &State ) -> Result<Option<String>, TokError> {
        let mut seen_e = found.contains('e') || found.contains('E');

        let mut seen_op = false;

        while code.remaining() > 0 {
            match code.peek_char() {
                Some('0'..='9') => {
                    let sym = code.get_char().unwrap();
                    found.push(sym);
                }
                Some('+') | Some('-') if seen_op == false => {
                    let sym = code.get_char().unwrap();
                    seen_op = true;
                    found.push(sym);
                },
                Some('+') | Some('-') => {
                    let sym = code.get_char().unwrap();
                    return Err(TokError::BadCharacter(sym));
                }
                Some('.')  => {
                    let sym = code.get_char().unwrap();
                    found.push(sym);
                },
                Some('e') | Some('E') if seen_e == false => {
                    let sym = code.get_char().unwrap();
                    found.push(sym);

                },
                Some('e') | Some('E') if seen_e == false => {
                    let sym = code.get_char().unwrap();
                    return Err(TokError::BadCharacter(sym));
                }

                Some('_') => {
                    //nop - skip ahead
                    code.get_char();
                }
                _ => {
                    return Ok(Some(found));
                }
            }

        }

        return Ok(Some(found));
    }

    fn attempt_indentation(state: &mut State, lineno: usize, line: &String)
        -> Result<Option<Vec<Token>>, TokError>
    {
        let mut product: Vec<Token> = Vec::new();

        //Ignore blank lines with comments
            if let Some(test) = BL_COMMENT.find(&line) {}
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

        if product.len() > 0 {
            return Ok(Some(product));
        }

        return Ok(None);


    }

    fn attempt_close_multiline_string(lineno: usize, state: &mut State, code: &mut CodeLine )
    -> Result<Option<Vec<Token>>, TokError>
    {
        let mut product: Vec<Token> = Vec::new();

        match state.string_type {
                    Some(StringType::TripleQuote) => {
                        if let Some((new_pos, found)) = code.return_match(TRIPLE_QUOTE_CLOSE.to_owned())
                        {
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
                            state.string_buffer = String::new();
                        }
                    },
                    Some(StringType::TripleApos) => {
                        if let Some((new_pos, found)) = code.return_match(TRIPLE_SINGLE_CLOSE.to_owned()) {
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
                            state.string_buffer = String::new();
                        }
                    },
                    Some(StringType::SingleApos) => {
                        if let Some((new_pos, found)) = code.return_match(SINGLE_APOS_CLOSE.to_owned()) {
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
                            state.string_buffer = String::new();
                        }
                    },
                    Some(StringType::SingleQuote) => {

                        if let Some((new_pos, found)) = code.return_match(SINGLE_QUOTE_CLOSE.to_owned()) {

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
                            state.string_buffer = String::new();
                        }

                    },
                    _ => {
                        println!("String continues under unexpected type: {:?}", state.string_type);
                    }
                }

        if state.string_continues == true {
            if let Some((_new_pos, found)) = code.return_match(Regex::new(r#"\A((\n|.)*)"#).expect("regex")) {
                state.string_buffer = format!("{}{}", state.string_buffer, found);
            }
        }

        if product.len() > 0 {
            return Ok(Some(product));
        }

        return Ok(None);


    }

    fn process_line(&mut self, state: &mut State, lineno: usize, line: String) -> Result<Vec<Token>, TokError> {
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
                match Tokenizer::attempt_indentation(state, lineno, &line) {
                    Ok(Some(indent_product)) => {
                        product.extend(indent_product);
                    },
                    Err(err_token) => { return Err(err_token); },
                    _ => {}
                }

            }
        }

        //reset this flag as its served its purpose
        state.line_continues = false;


        let mut code = CodeLine::new(line);
        //todo, string off the beginning whitespace if it exists for a minor speed boost

        while code.remaining() > 0 {
            let col_pos = code.position();

            //TODO should this be outside of the loop?
            if state.string_continues == true {
                //TODO check for string continuation type/state.type to use the correct regex
                match Tokenizer::attempt_close_multiline_string(lineno, state, &mut code) {
                    Ok(Some(multiline_option)) => {
                        product.extend(multiline_option);
                    }
                    Err(err_token) => {
                        return Err(err_token);
                    }
                    _ => {}
                }

            }
            else if let Some('"') = code.peek_char() {
                let sym = code.get_char().unwrap();
                match Tokenizer::attempt_string('"',
                                                       "\"".to_string(),
                                                       &mut code, state
                ) {
                    Ok(Some((token_type, found_str))) => {
                        assert_eq!(token_type, TType::String);
                        product.push(Token::quick(
                            token_type,
                            lineno, col_pos, col_pos+found_str.len(),
                            found_str
                        ));
                    },
                    _ => {
                        if state.string_continues == true {
                             state.string_start = Some(Position::t2((lineno, col_pos)));
                         } else {
                             println!("Failed to match string @ {}:{}", lineno, col_pos);
                         }
                    }
                }

            }
            else if let Some('\'') = code.peek_char() {
                let sym = code.get_char().unwrap();
                match Tokenizer::attempt_string('\'', "'".to_string(),
                &mut code, state) {
                    Ok(Some((token_type, found_str))) => {
                        assert_eq!(token_type, TType::String);
                        product.push(Token::quick(
                            token_type,
                            lineno, col_pos, col_pos+found_str.len(),
                            found_str
                        ));
                    },
                    _ => {
                        if state.string_continues == true {
                             state.string_start = Some(Position::t2((lineno, col_pos)));
                         } else {
                             println!("Failed to match @ {}:{}", lineno, col_pos);
                         }
                    }
                }
            }

            //Look for identifier/Name tokens
            else if Tokenizer::is_potential_identifier_start(code.peek_char()) == true {
                 match self.attempt_identifiers(&mut code, state) {
                     Ok(Some((token_type, found))) => {

                         if token_type == TType::Name && found == "async".to_string() {
                             product.push(
                                 Token::quick(TType::Async,
                                    lineno, col_pos, col_pos+found.len(),
                                     found
                                 )
                             );
                         }
                         else if token_type == TType::Name && found == "await".to_string() {
                             product.push(
                                 Token::quick(TType::Await,
                                    lineno, col_pos, col_pos+found.len(),
                                     found
                                 )
                             );
                         }

                         else if token_type == TType::Name || token_type == TType::String {
                             product.push(
                                 Token::quick(token_type,
                                    lineno, col_pos, col_pos+found.len(),
                                     found
                                 )
                             );
                         } else {
                             panic!("How did I get here? {:?}{:?}", token_type, found );
                         }

                     }
                     Err(err_token) => {
                         println!("Syntax error @ {}:{}", lineno, col_pos);
                         return Err(err_token);

                     }
                     _ => {

                         //Check to see if we have a continuing string
                         if state.string_continues == true {
                             state.string_start = Some(Position::t2((lineno, col_pos)));
                         } else {
                             println!("Failed to match @ {}:{}", lineno, col_pos);
                         }
                     }
                 }
                is_statement = true;
            }

            //Fetch numbers
            else if let Some('0'..='9') = code.peek_char() {
                match self.attempt_number(&mut code, &state) {
                    Ok(Some(found)) => {
                        product.push(Token::quick(TType::Number, lineno, col_pos, col_pos + found.len(), found));
                    }
                    Err(err_token) => {
                        return Err(err_token);
                    }
                    _ => {
                        //Assume failed or didn't match which will be problematic
                    }
                }
            }
            else if let Some('.') = code.peek_char() {
                match self.attempt_floating_point(&mut code, &state){
                    Ok(Some(found)) => {
                        product.push(Token::quick(
                            TType::Number,
                            lineno, col_pos, col_pos+found.len(),
                            found
                        ));
                    },
                    Err(err_token) => {
                        println!("Syntax error @ {}:{}", lineno, col_pos);
                        return Err(err_token);
                    },
                    _=> {
                        //Assume didn't match
                        //but we have a problem, this could be `.` or `...` operator tokens
                        if let Some((new_pos, found)) = code.return_match(OPERATOR_RE.to_owned()) {
                            product.push(
                                Token::quick(TType::Op,
                                    lineno, col_pos, new_pos,
                                    found
                                )
                            );
                        } else {
                            panic!("Syntax error (too many or too little .) on {} - \n\t{:?} - \n\t{:?} - \n\t{:?}\n", lineno, code.line, product, code.position());
                        }




                    }
                }
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
                        if (last_paren == '(' && current != ')')
                            || (last_paren == '[' && current != ']')
                            || (last_paren == '{' && current != '}') {
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
                //Try to consume the newline
                match code.peek_char() {
                    Some('\n') => {
                        code.get_char();
                        if product.len() > 0 && is_statement == true && state.paren_depth.len() == 0 {
                            product.push(Token::quick(TType::NL, lineno, col_pos, code.position()-1, "".to_string()));
                        }
                    },
                    _ => {}
                }
                //product.push(Token::quick(TType::Comment, lineno, col_pos, new_pos, found));
            }
            else {
                if let Some(sym) = code.get_char() {
                    if sym == ' ' {
                        //skipping white space
                        //Except if we're inside a triple quoted/multiline string!
                        if state.string_continues == true {
                            state.string_buffer = format!("{}{}", state.string_buffer, sym);
                        }
                    } else if sym == '\\' {
                        //Don't do anything, TODO how to signal a line continuation?
                        state.line_continues = true;
                        //abort processing for now, nothing matters after a \
                        return Ok(product);
                    } else if sym == '\n' {
                        if state.paren_depth.len() > 0 {
                            continue
                        } else if state.string_continues == true {
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
                        return Err(TokError::BadCharacter(sym));
                    }
                }
            }
        }

        return Ok(product);
    }
}
