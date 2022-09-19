//
// use std::cmp::Ordering;
//
//
//
//
// // use super::{
// //     token::Token,
// //     error::TokError,
// //     ttype::TType,
// //     operators::OPERATOR_RE,
// //     // managed_line::ManagedLine,
// //     // module_lines::ModuleLines,
// //
// // };
//
// use once_cell::sync::Lazy;
// use regex::{Regex};
// use std::io::{Read};
//
// use log::{info};
// use super::position::Position;
//
//
// // use std::str::Chars;
//
// //Copied from LIBCST
// //TODO relocate to a common rgxs.rs file?
// const MAX_INDENT: usize = 100;
//
// static TRIPLE_QUOTE: Lazy<Regex> =
//     Lazy::new(|| Regex::new(r#"\A""""#).expect("regex"));
//
// static TRIPLE_QUOTE_AND_CONTENT: Lazy<Regex> =
//     Lazy::new(|| Regex::new(r#"\A"""[.\n]?"#).expect("regex"));
//
// static TRIPLE_QUOTE_AND_PRECONTENT: Lazy<Regex> =
//     Lazy::new(|| Regex::new(r#"\A.?""""#).expect("regex"));
//
//
// static CAPTURE_QUOTE_STRING: Lazy<Regex> =
//     Lazy::new(|| Regex::new(r#"\A(|Rb|br|Br|rF|F|R|r|rb|rf|B|u|RB|bR|f|b|FR|Rf|fr|Fr|rB|BR|RF|fR|U)?"[^\n"\\]*(?:\\.[^\n"\\]*)*""#).expect("regex"));
//
// static CAPTURE_APOS_STRING: Lazy<Regex> =
//     Lazy::new(|| Regex::new(r#"\A(|Rb|br|Br|rF|F|R|r|rb|rf|B|u|RB|bR|f|b|FR|Rf|fr|Fr|rB|BR|RF|fR|U)?'[^\n'\\]*(?:\\.[^\n'\\]*)*'"#).expect("regex"));
//
// static SINGLE_QUOTE_STRING: Lazy<Regex> =
//     Lazy::new(|| Regex::new(r#"\A""#).expect("regex"));
//
// static SINGLE_QUOTE_STRING_CONTENT: Lazy<Regex> =
//     Lazy::new(|| Regex::new(r#"\A"[.\n]?[^"]"#).expect("regex"));
//
// static SINGLE_QUOTE_STRING_PRECONTENT: Lazy<Regex> =
//     Lazy::new(|| Regex::new(r#"\A[^"]+""#).expect("regex"));
//
// static SINGLE_APOSTROPHE_STRING: Lazy<Regex> =
//     Lazy::new(|| Regex::new(r#"\A'"#).expect("regex"));
//
// static SINGLE_APOSTROPHE_PRECONTENT: Lazy<Regex> =
//     Lazy::new(|| Regex::new(r#"\A[^']?'"#).expect("regex"));
//
//
// static SPACE_TAB_FORMFEED_RE: Lazy<Regex> =
//     Lazy::new(|| Regex::new(r"\A[ \f\t]+").expect("regex"));
//
//
// static POTENTIAL_IDENTIFIER_TAIL_RE: Lazy<Regex> =
//     Lazy::new(|| Regex::new(r"\A(\w|[^\x00-\x7f])+").expect("regex"));
// static POSSIBLE_NAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A[a-zA-Z]{1}[\w\d]+").expect("regex"));
// static POSSIBLE_ONE_CHAR_NAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A[a-zA-Z]{1}").expect("regex"));
//
// static HEXNUMBER: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A0[xX](?:_?[0-9a-fA-F])+").expect("regex"));
//
// static BINNUMBER: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A0[bB](?:_?[01])+").expect("regex"));
//
// static OCTNUMBER: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A0[oO](?:_?[0-7])+").expect("regex"));
//
// static DECNUMBER: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A(?:0(?:_?0)*|[1-9](?:_?[0-9])*)").expect("regex"));
//
// static PointfloatStr: &str = r#"\A([0-9](?:_?[0-9])*\\.(?:[0-9](?:_?[0-9])*)?|\\.[0-9](?:_?[0-9])*)([eE][-+]?[0-9](?:_?[0-9])*)?"#;
//
// static POINTFLOAT: Lazy<Regex> = Lazy::new(|| Regex::new(PointfloatStr).expect("regex"));
// static POINTFLOAT1: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A[0-9](?:_?[0-9])*\.(?:[0-9](?:_?[0-9])*)?([eE][-+]?[0-9](?:_?[0-9])*)?").expect("regex"));
// static POINTFLOAT2: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A\.[0-9](?:_?[0-9])*([eE][-+]?[0-9](?:_?[0-9])*)?").expect("regex"));
//
// static EXPONENT: Lazy<Regex> = Lazy::new(|| Regex::new(r"[eE][-+]?[0-9](?:_?[0-9])*").expect("regex"));
//
// const NumberStr: &str = r"\A(([0-9](?:_?[0-9])*[jJ]|(([0-9](?:_?[0-9])*\\.(?:[0-9](?:_?[0-9])*)?|\\.[0-9](?:_?[0-9])*)([eE][-+]?[0-9](?:_?[0-9])*)?|[0-9](?:_?[0-9])*[eE][-+]?[0-9](?:_?[0-9])*)[jJ])|(([0-9](?:_?[0-9])*\\.(?:[0-9](?:_?[0-9])*)?|\\.[0-9](?:_?[0-9])*)([eE][-+]?[0-9](?:_?[0-9])*)?|[0-9](?:_?[0-9])*[eE][-+]?[0-9](?:_?[0-9])*)|(0[xX](?:_?[0-9a-fA-F])+|0[bB](?:_?[01])+|0[oO](?:_?[0-7])+|(?:0(?:_?0)*|[1-9](?:_?[0-9])*)))";
//
// static NUMBER: Lazy<Regex> = Lazy::new(||Regex::new(NumberStr).expect("regex"));
//
// #[derive(PartialEq, Debug)]
// enum StringType {
//     NONE,
//     SINGLE,
//     DOUBLE,
//     TRIPLE,
// }
//
//
// #[cfg(target_os = "macos")]
// static NLSYM: str = "\r";
//
// #[cfg(target_os = "linux")]
// static NLSYM: str = "\n";
//
// #[cfg(target_os = "windows")]
// static NLSYM: &str = "\r\n";
//
//
// ///Lowest tier tokenizer, handles tokenizing line
// ///
// pub struct Processor {
//     /**
//         number of elements is how far indented the code is
//         individual elements is the size of the identation.
//
//         i think it's insane to mix tabs and spaces.
//     */
//     indent_stack: Vec<usize>,
//     /**
//         (paren symbol, starting line no)
//      */
//     paren_stack: Vec<(char, usize)>,
//     /**
//         line to line state
//     */
//     last_line_was_blank: bool,
//
//     /**
//     Was the last line an open string or ( or something along those lines?
//     */
//
//     string_continues: bool,
//     string_type: StringType,
//     string_start: Position,
//     string_buffer_content: String,
//
//     // pub module: ModuleLines<'a>,
//
// }
//
// //
// // #[allow(non_snake_case)]
// // impl <'a> Processor<'a>  {
// //     pub fn initialize(lines: Vec<String>, module_name: Option<String>) -> Self {
// //
// //         let name = module_name.unwrap_or("__main__".to_string());
// //         Self {
// //             indent_stack: Vec::new(),
// //             paren_stack: Vec::new(),
// //             last_line_was_blank: false,
// //
// //             string_continues: false,
// //             string_type: StringType::NONE,
// //             string_start: Position::default(),
// //             string_buffer_content: "".to_string(),
// //             module: ModuleLines::Make(lines, name),
// //         }
// //     }
// //
// //     //string managers
// //
// //     fn continue_consuming_string(&mut self, flag: bool) {
// //         self.string_continues = flag;
// //     }
// //
// //     fn consuming_string(&self) -> bool {
// //         return self.string_continues;
// //     }
// //
// //     //misc helpers
// //
// //     fn has_more_lines(&self) -> bool {
// //         return self.module.has_lines();
// //     }
// //
// //     pub fn consume_file<P>(fname: P, module_name: Option<String>) -> Self
// //         where P: AsRef<std::path::Path>, {
// //
// //         let mut buffer: String = String::new();
// //         let _file = File::open(fname).unwrap().read_to_string(&mut buffer);
// //
// //         buffer.replace(NLSYM, "\n");
// //
// //         let lines : Vec<String> = buffer.split(&NLSYM).map(|l|format!("{}\n", l).to_string()).into_iter().collect();
// //
// //         return Processor::initialize(lines, module_name);
// //
// //     }
// //
// //     pub fn tokenize_file<P>(fname: P, module_name: Option<&str>, skip_encoding: bool) -> Vec<Token>
// //         where P: AsRef<std::path::Path>,    {
// //         let outcome = Processor::consume_file(fname, Some(module_name.unwrap().to_string())).run(skip_encoding);
// //         return outcome.expect("tokens").clone();
// //
// //     }
// //
// //     pub fn tokenize_str(input: &str, module_name: Option<String> ) -> Result<Vec<Token>, TokError> {
// //
// //         let mut engine = Processor::consume_string(input.to_string(), module_name);
// //         let retval = engine.run(true);
// //         return retval;
// //     }
// //
// //     pub fn consume_string(input: String, module_name: Option<String>) -> Self {
// //         let product = if input.contains("\r\n") {
// //             input.split("\r\n")
// //         } else {
// //             input.split("\n")
// //         };
// //
// //         let content = product.map(|l| format!("{}\n", l)).collect();
// //
// //         info!("Processing string into Vector {:?}", content);
// //         return Processor::initialize(content, module_name);
// //
// //     }
// //
// //     pub fn consume_vector(input: Vec<String>, module_name: Option<String>) -> Self {
// //         return Processor::initialize(input, module_name);
// //     }
// //
// //     pub fn run(&'a mut self, skip_encoding: bool) -> Result<Vec<Token>, TokError> {
// //
// //
// //         let mut body: Vec<Token> = Vec::new();
// //
// //         // For now, ALWAYS assume UTF-8 encoding for files.
// //         if skip_encoding == false {
// //             body.push(Token::Make(TType::Encoding, Position::m(0,0), Position::m(0,0), "utf-8"));
// //         }
// //
// //         let module_size = self.module.len();
// //
// //
// //
// //         debug!("Starting walk/iterate over module");
// //         while self.has_more_lines() == true {
// //
// //             let mut line = self.module.get().expect("Expected a line in module");
// //             debug!("Processing line: {:?}", line.text);
// //
// //             if self.string_continues == true {
// //                 debug!("inside of a string, consuming");
// //                 if self.string_type == StringType::TRIPLE {
// //                     if let Some(token) = self.process_triple_quote(&mut line) {
// //                         body.push(token);
// //                         self.continue_consuming_string(false);
// //                     }
// //                 }
// //
// //             }
// //             else if self.module.remaining() == 0 && line.len() == 1 {
// //                 // TODO verifiy line[0] == '\n'
// //                 if line.peek().expect("Missing last char!") == '\n' {
// //                     body.push(Token::quick(TType::EndMarker, module_size, 0, 0, ""));
// //                 }
// //
// //             }
// //             else if line.len() == 1 && line.peek().expect("last char") == '\n' {
// //                 //Blank lines don't exist and don't have NEWLINE or NL endings
// //                 continue;
// //             }
// //             else {
// //                 match self.process_line(&mut line) {
// //                     Ok(mut product) => {
// //                         // So.... yeah, not ideal but I needed a way to duplicate/copy all the elements
// //                         // of product so I can append them to body.
// //                         // TODO - Refactor so this is less stupid.
// //                         body.append(&mut product);
// ////                     },
//
// //                     Err(issue) => {
// //
// //                          return Err(issue.clone());
// //
// //                         // panic!("Tokenizer failure: {:?}", issue);
// //                         // TODO figure out why the borrow checker freaks out on this line
// //
// //                     }
// //                 }
// //
// //             }
// //
// //
// //         } // End While
// //
// //
// //
// //         if self.string_continues == true {
// //             //We are out of lines
// //             return Err(TokError::UnterminatedTripleQuotedString);
// //         }
// //
// //         if self.paren_stack.len() > 0 {
// //             let (hopefully_last, _lineno) = self.paren_stack.pop().unwrap();
// //             return Err(TokError::UnmatchedClosingParen(hopefully_last));
// //         }
// //
// //         if self.indent_stack.len() > 0 {
// //             while self.indent_stack.len() > 0 {
// //                 self.indent_stack.pop();
// //                 body.push(Token::quick(TType::Dedent, module_size+1, 0, 0, ""));
// //
// //             }
// //         }
// //
// //         if body.last().unwrap().r#type != TType::EndMarker {
// //             body.push(Token::quick(TType::EndMarker, module_size+1, 0, 0, ""));
// //         }
// //
// //
// //         return Ok(body);
// //     }
// //
// //     fn process_line(&'a mut self, line: &'a mut ManagedLine<'a>) -> Result<Vec<Token<'a>>, TokError> {
// //
// //
// //         let mut product: Vec<Token> = Vec::new();
// //         let mut has_statement = false;
// // //        let STRINGSRE: Regex = Regex::new(StringStr).expect("regex");
// //
// //         let lineno = line.lineno.saturating_add(1);
// //
// //         if self.string_continues == false {
// //         //Deal with empty lines first
// //             if line.text.len() == 1 || line.text == "\n" {
// //
// //                 if self.indent_stack.len() > 0 {
// //                     while self.indent_stack.len() > 0 {
// //                         self.indent_stack.pop();
// //                         product.push(Token::Make(TType::Dedent,  Position::m(0, lineno), Position::m(0, lineno), ""));
// //                     }
// //                 }
// //
// //                 product.push(Token::Make(TType::NL, Position::m(0,lineno), Position::m(0,lineno, ), "\n"));
// //
// //                 return Ok(product);
// //             }
// //
// //             //Consume the beginning of the line and handle indentations and dedentations
// //             let ws_match = SPACE_TAB_FORMFEED_RE.find(&line.text[..]);
// //             if let Some(whitespace) = ws_match {
// //                 let current_size = whitespace.end() - whitespace.start();
// //                 let last_size = self.indent_stack.last().unwrap_or(&0);
// //
// //                 match current_size.cmp(last_size) {
// //                     Ordering::Greater => {
// //                         //We are handing an indent
// //                         if self.indent_stack.len() + 1 > MAX_INDENT {
// //                             return Err(TokError::TooDeep);
// //                         }
// //                         self.indent_stack.push(current_size);
// //                         product.push(Token::Make(TType::Indent, Position::m(0, lineno), Position::m(current_size, lineno), whitespace.as_str()));
// //                     },
// //                     Ordering::Less => {
// //                         //We are handling 1 or more dedents
// //                         while self.indent_stack.len() > 0 {
// //                             let last_indent_size = self.indent_stack.pop().unwrap();
// //                             product.push(Token::Make(TType::Dedent, Position::m(0, lineno), Position::m(0, lineno), ""));
// //                             if last_indent_size == current_size {
// //                                 break;
// //                             }
// //                         }
// //
// //                     },
// //                     Ordering::Equal => {
// //                         //No operation
// //                     }
// //                 }
// //
// //             }
// //             else if self.indent_stack.len() > 0 {
// //                 //Handle edge case where dedent has gone all the way to zero
// //                 while self.indent_stack.len() > 0 {
// //                     self.indent_stack.pop();
// //                     product.push(Token::Make(TType::Dedent, Position::m(0, lineno), Position::m(0, lineno),""));
// //                 }
// //             }
// //
// //             //Skip whitespace
// //             let mut _ignore_me = line.test_and_return(&SPACE_TAB_FORMFEED_RE.to_owned());
// //
// //         }
// //
// //         while line.peek() != None {
// //
// //
// //             let index = line.get_idx();
// //
// //             //We're continuing to consume a string, like a `'` or `"` but it could also be a `"""`
// //             if self.consuming_string() == true {
// //
// //                 match self.string_type {
// //                     StringType::SINGLE => {
// //                         if let Some((current_idx, match_str)) = line.test_and_return(&SINGLE_APOSTROPHE_PRECONTENT){
// //                             self.string_buffer_content.push_str(match_str);
// //                             product.push(
// //                                 Token::Make(
// //                                     TType::String,
// //                                         self.string_start.clone(),
// //                                     Position::t((current_idx, lineno)),
// //                                  self.string_buffer_content.as_str()
// //                                 )
// //                             );
// //
// //                             self.continue_consuming_string(false);
// //
// //                         } else {
// //                             return Err(TokError::UnterminatedString);
// //                         }
// //                     },
// //                     StringType::DOUBLE => {
// //                         if let Some((current_idx, match_str)) = line.test_and_return(&SINGLE_QUOTE_STRING_PRECONTENT) {
// //                             self.string_buffer_content.push_str(match_str);
// //                             product.push(Token::Make(
// //                                 TType::String,
// //                                 self.string_start,
// //                                 Position::t((current_idx, lineno)),
// //                                 self.string_buffer_content.as_str()
// //                             ));
// //                             self.string_continues = false;
// //                         } else {
// //                             return Err(TokError::UnterminatedString);
// //                         }
// //                     },
// //                     StringType::TRIPLE => {
// //                         if let Some((current_idx, match_str)) = line.test_and_return(&TRIPLE_QUOTE_AND_PRECONTENT) {
// //                             self.string_buffer_content.push_str(match_str);
// //                             product.push(Token::Make(
// //                                 TType::String,
// //                                 self.string_start.clone(),
// //                                 Position::t((current_idx, lineno)),
// //                                 self.string_buffer_content.as_str()
// //                             ));
// //                             self.string_continues = false;
// //                             has_statement = true;
// //                         } else {
// //                             //Consume the whole line from current idx
// //                             self.string_buffer_content = format!("{}{}", self.string_buffer_content, line.return_all() );
// //                             return Ok(product);
// //                         }
// //                     },
// //                     _ => {
// //                         println!("How did i get here? {:?}", self.string_type);
// //                     }
// //                 }
// //
// //                 continue;
// //             }
// //             //Look for a comment and consume all after it.
// //             else if let Some((current_idx, retval)) = line.test_and_return(&Regex::new(r"\A#.*").expect("regex")) {
// //                 product.push(
// //                             Token::quick(TType::Comment, lineno, index, current_idx, retval)
// //                 );
// //             }
// //
// //             else if let Some((current_idx, retval)) = line.test_and_return(&POINTFLOAT) {
// //                 debug!("Matched {} with pf1", retval);
// //                 product.push(
// //                     Token::quick(TType::Number, lineno, index, current_idx, retval)
// //                 );
// //             }
// //
// //             //Check for Floating point #'s - version 1
// //             else if let Some((current_idx, retval)) = line.test_and_return(&POINTFLOAT1) {
// //                 debug!("Matched {} with pf1", retval);
// //                 product.push(
// //                     Token::quick(TType::Number, lineno, index, current_idx, retval)
// //                 );
// //             }
// //             //Check for Floating point #$'s - version 2
// //             else if let Some((current_idx, retval)) = line.test_and_return(&POINTFLOAT2) {
// //                 product.push(
// //                     Token::quick(TType::Number, lineno, index, current_idx, retval)
// //                 );
// //             }
// //
// //             //Last ditch effort to catch numbers
// //             else if let Some((new_idx, retval)) = line.test_and_return(&NUMBER) {
// //                 product.push(
// //                     Token::quick(TType::Number, lineno, index, new_idx, retval)
// //                 );
// //             }
// //             // look for numbers
// //             else if let Some((current_idx, retval)) = line.test_and_return(&Regex::new(r"\A\d+").expect("regex")) {
// //                 product.push(
// //                         Token::quick(TType::Number, lineno, index, current_idx, retval)
// //                     );
// //                 has_statement = true;
// //             }
// //             // Look for a operator
// //             else if let Some((current_idx, retval)) = line.test_and_return(&OPERATOR_RE.to_owned()) {
// //                 let char_retval = retval.chars().nth(0).unwrap();
// //
// //                 if retval.len() == 1 && retval.contains(&['[', '(']) {
// //                     self.paren_stack.push((char_retval, lineno));
// //
// //
// //                 } else if retval.contains(&[']', ')']) {
// //                     let latest = self.paren_stack.last();
// //                     match latest {
// //                         Some((verify_char, _ignore)) => {
// //                             if *verify_char == '(' && char_retval == ')' {
// //                                 self.paren_stack.pop();
// //                             } else if *verify_char == '[' && char_retval == ']' {
// //                                 self.paren_stack.pop();
// //                             } else {
// //                                 return Err(TokError::MismatchedClosingParen(*verify_char, char_retval));
// //                             }
// //                         },
// //                         None => {
// //                             return Err(TokError::UnmatchedClosingParen(char_retval));
// //                         }
// //                     }
// //                 }
// //
// //                 product.push(
// //                             Token::quick(TType::Op, lineno, index, current_idx, retval)
// //                     );
// //                 has_statement = true;
// //             }
// //             //Absorb  any spaces
// //             else if let Some((_current_idx, _retval)) = line.test_and_return(&SPACE_TAB_FORMFEED_RE.to_owned()) {
// //             // pass/ignore WS - TODO REFACTOR!
// //             }
// //             //Look for line continuation
// //             else if Some('\\') == line.peek() {
// //
// //                 println!("TODO, deal with line continutations!");
// //                 // self.string_continues = true;
// //                 let _ = line.get();
// //             }
// //
// //
// //             // Seek and then handle """ tokens
// //             else if let Some((current_idx, match_str)) = line.test_and_return(&TRIPLE_QUOTE.to_owned()) {
// //                 debug!("TQ3 matched on @ {},{}:{:?}", current_idx, lineno, match_str);
// //
// //                 self.string_continues = true;
// //                 self.string_type = StringType::TRIPLE;
// //                 self.string_buffer_content = format!("{}", match_str);
// //                 self.string_start = Position::m(index, lineno);
// //
// //                 if let Some((end_idx, end_match_str)) = line.test_and_return(&TRIPLE_QUOTE_AND_PRECONTENT) {
// //                     let str_content = format!(r#"""{}"#, end_match_str);
// //                     product.push(
// //                             Token::quick(TType::String, lineno, current_idx, end_idx, str_content.as_str())
// //                        );
// //                    has_statement = true;
// //                 } else {
// //                     // Consume rest of the line!
// //                     self.string_buffer_content = format!("{}{}", self.string_buffer_content, line.return_all()  );
// //                 }
// //             }
// //
// //
// //             else if let Some((current_idx, match_str)) = line.test_and_return(&CAPTURE_APOS_STRING) {
// //                 product.push(Token::Make(
// //                     TType::String,
// //                     Position::m(index, lineno),
// //                     Position::m(current_idx, lineno),
// //                     match_str
// //                 ));
// //
// //             }
// //             else if let Some((current_idx, match_str)) = line.test_and_return( &CAPTURE_QUOTE_STRING) {
// //                 product.push(Token::Make(
// //                     TType::String,
// //                     Position::m(index, lineno),
// //                     Position::m(current_idx, lineno),
// //                     match_str
// //                 ));
// //             }
// //             // like Regex says, look for non-quoted strings
// //             else if let Some((current_idx, retval)) = line.test_and_return(&POSSIBLE_NAME.to_owned()) {
// //                 product.push(
// //                          Token::quick(TType::Name, lineno, index, current_idx, retval)
// //                     );
// //                 has_statement = true;
// //             }
// //             else if let Some((current_idx, match_str)) = line.test_and_return(&POSSIBLE_ONE_CHAR_NAME) {
// //                 //TODO peak for " or '
// //
// //                 product.push(Token::Make(
// //                     TType::Name,
// //                     Position::m(index, lineno),
// //                     Position::m(current_idx, lineno),
// //                     match_str
// //                 ));
// //             }
// //             else {
// //                 let chr = line.get().unwrap();
// //
// //
// //                 if chr == '\n' {
// //                     let what = if has_statement == true {
// //                         TType::Newline
// //                     } else {
// //                         TType::NL
// //                     };
// //                     product.push(Token::Make(
// //                             what,
// //                             Position::m(index, lineno),
// //                             Position::m(index+1, lineno),
// //                             "\n"
// //                         ));
// //
// //                 } else {
// //                     println!("Did not capture: {:?} - #{}:{}", chr, lineno, line.idx);
// //
// //                     return Err(TokError::BadCharacter(chr) );
// //                 }
// //
// //
// //
// //             }
// //
// //         } // end while line peek
// //
// //
// //
// //
// //         Ok(product)
// //
// //     }
// //
// //     //Assumes that the python string has already started
// //     fn process_triple_quote(&mut self, line: &mut ManagedLine) -> Option<Token> {
// //
// //
// //         while line.peek() != None {
// //             if let Some((new_idx, match_str)) = line.test_and_return(&TRIPLE_QUOTE_AND_PRECONTENT) {
// //                 debug!("Captured closing 3Q and content {:?}", match_str);
// //
// //                 self.string_buffer_content = format!("{}{}", self.string_buffer_content, match_str);
// //
// //                 let str_token = Token::Make(TType::String,
// //                                             self.string_start,
// //                                             Position::m(new_idx.saturating_sub(match_str.len()), line.lineno),
// //                                             self.string_buffer_content.as_str()
// //                 );
// //
// //                 return Some(str_token);
// //             } else if let Some((new_idx, match_str)) = line.test_and_return(&TRIPLE_QUOTE) {
// //                 self.string_buffer_content = format!("{}{}", self.string_buffer_content, match_str);
// //
// //                 let str_token = Token::Make(TType::String,
// //                                             self.string_start,
// //                                             Position::m(new_idx.saturating_sub(match_str.len()), line.lineno),
// //                                             self.string_buffer_content.as_str()
// //                 );
// //                 return Some(str_token);
// //             } else {
// //                 self.string_buffer_content = format!("{}{}", self.string_buffer_content, line.get().unwrap());
// //             }
// //         }
// //
// //         return None;
// //
// //
// //
// //     }
// //
// // }
// //
// //
// // #[cfg(test)]
// // mod tests {
// //
// //     use crate::Processor;
// //     use crate::tokenizer::error::TokError;
// //     // use crate::tokenizer::module_lines::ModuleLines;
// //
// //
// //     use crate::tokenizer::position::Position;
// //     use crate::tokenizer::ttype::TType;
// //     use crate::tokenizer::token::Token;
// //
// //     macro_rules! test_token{
// //         ($token:expr, $ttype:expr, $content:expr)=>{
// //             assert_eq!($token.r#type, $ttype);
// //             assert_eq!($token.text, $content);
// //         }
// //     }
// //
// //     macro_rules! test_token_w_position{
// //         ($token:expr, $ttype:expr, $start:expr, $end:expr, $content:expr)=>{
// //
// //             assert_eq!($token.r#type, $ttype, "Testing for type with {:?} {:?} != {:?}", $token.text, $token.r#type, $ttype);
// //             assert_eq!($token.text, $content);
// //             assert_eq!($token.start, Position::t($start), "Testing for start with {:?} % {:?} : {:?} != {:?}", $token.text, $token.r#type, $token.start, $start);
// //             assert_eq!($token.end, Position::t($end), "Testing for end with {:?} % {:?} : {:?} != {:?}", $token.text, $token.r#type, $token.end, $end);
// //
// //         }
// //     }
// //
// //
// //     fn print_tokens(tokens: &Vec<Token>) {
// //         println!("Got {} tokens", tokens.len());
// //         for (idx, token) in tokens.iter().enumerate() {
// //             println!("{}: {:?}", idx, token);
// //         }
// //     }
// //
// //     #[test]
// //     fn rust_experiment() {
// //         let mut actual = "".to_string();
// //         actual.push('\n');
// //         assert_eq!(actual, "\n");
// //     }
// //
// //
// //     #[test]
// //     fn processor_works() {
// //         Processor::consume_string("Hello\nWorld".to_string(), Some("__test__".to_string()));
// //     }
// //
// //     #[test]
// //     fn processor_does_basic_dentation() {
// //         let tokens = Processor::consume_file("test_fixtures/basic_indent.py", Some("__test__".to_string())).run(false).expect("Tokens");
// //         assert!(tokens.len() > 1);
// //         print_tokens(&tokens);
// //     }
// //
// //     #[test]
// //     fn processor_does_adv_dentation() {
// //         let tokens = Processor::consume_file("test_fixtures/crazy_dents.py", Some("__test__".to_string())).run(false).expect("Expected vec<Tokens>");
// //         let mut indents = 0;
// //         let mut dedents = 0;
// //         for token in tokens.iter() {
// //             if token.r#type == TType::Indent {
// //                 indents += 1;
// //             } else if token.r#type == TType::Dedent {
// //                 dedents += 1
// //             }
// //         }
// //
// //         assert!(tokens.len() > 1);
// //         assert_eq!(indents, dedents);
// //     }
// //
// //     #[test]
// //     fn processor_correctly_handles_endmarker_vs_nl() {
// //         let mut engine = Processor::consume_file("test_fixtures/simple_string.py", Some("_simple_string_".to_string()));
// //         let tokens = engine.run(false).expect("Tokens");
// //         print_tokens(&tokens);
// //
// //         assert_eq!(tokens.len(), 4);
// //     }
// //
// //     #[test]
// //     fn processor_consumes_triple_strings_v2() {
// //         let data =
// //             r#"
// // """
// //     This is a test!
// // """
// // "#;
// //         let expected =
// //             r#""""
// //     This is a test!
// // """"#;
// //
// //
// //         let mut engine = Processor::consume_string(data.to_string(), Some("__test__".to_string()));
// //         let tokens = engine.run(false).expect("tokens");
// //
// //         print_tokens(&tokens);
// //
// //
// //         assert_eq!(tokens[1].r#type, TType::String);
// //         assert_eq!(tokens[1].text, expected);
// //     }
// //
// //     #[test]
// //     fn processor_properly_consumes_single_quote_strings_basic() {
// //         let mut engine = Processor::consume_file("test_fixtures/simple_string.py", Some("simple_string".to_string()));
// //         let tokens = engine.run(false).expect("Tokens");
// //         print_tokens(&tokens);
// //
// //         assert_eq!(tokens.len(), 4);
// //     }
// //
// //     #[test]
// //     fn processor_absorbs_multiline_triple_quoted_strings() {
// //         pretty_env_logger::init();
// //
// //
// //         println!("Loading multiline into processor");
// //
// //         let tokens = Processor::tokenize_file("test_fixtures/multiline_strings.py", Some("multiline"), true);
// //         print_tokens(&tokens);
// //
// //         assert_eq!(tokens.len(), 3);
// //
// //     }
// //
// //
// //     #[test]
// //     fn processor_consume_handles_names() {
// //         let mut processor = Processor::initialize(vec!["    def hello_world():".to_string()], Some("__test__".to_string()));
// //
// //         let mut line = processor.module.get().expect("Atleast one line");
// //
// //         let retval = processor.process_line(&mut line);
// //         let tokens = retval.unwrap();
// //
// //         print_tokens(&tokens);
// //
// //         assert_eq!(6, tokens.len());
// //         assert_eq!(tokens[0].r#type, TType::Indent);
// //         assert_eq!(tokens[1].r#type, TType::Name);
// //         assert_eq!(tokens[2].r#type, TType::Name);
// //         let test_types = vec!(TType::Indent, TType::Name, TType::Name, TType::Op, TType::Op, TType::Op);
// //         for (idx, test_type) in test_types.iter().enumerate() {
// //             assert_eq!(&tokens[idx].r#type, test_type);
// //         }
// //     }
// //
// //
// //     #[test]
// //     fn test_additive() {
// //
// //         let tokens = Processor::tokenize_file("test_fixtures/test_additive.py", Some("additive"), false);
// //         print_tokens(&tokens);
// //         test_token_w_position!(tokens[0], TType::Encoding, (0, 0), (0, 0), "utf-8" );
// //         test_token_w_position!(tokens[1], TType::Name, (0, 1), (1, 1), "x" );
// //         test_token_w_position!(tokens[2], TType::Op, (2, 1), (3, 1), "=" );
// //         test_token_w_position!(tokens[3], TType::Number, (4, 1), (5, 1), "1" );
// //         test_token_w_position!(tokens[4], TType::Op, (6, 1), (7, 1), "-" );
// //         test_token_w_position!(tokens[5], TType::Name, (8, 1), (9, 1), "y" );
// //         test_token_w_position!(tokens[6], TType::Op, (10, 1), (11, 1), "+" );
// //         test_token_w_position!(tokens[7], TType::Number, (12, 1), (14, 1), "15" );
// //         test_token_w_position!(tokens[8], TType::Op, (15, 1), (16, 1), "-" );
// //         test_token_w_position!(tokens[9], TType::Number, (17, 1), (18, 1), "1" );
// //         test_token_w_position!(tokens[10], TType::Op, (19, 1), (20, 1), "+" );
// //         test_token_w_position!(tokens[11], TType::Number, (21, 1), (26, 1), "0x124" );
// //         test_token_w_position!(tokens[12], TType::Op, (27, 1), (28, 1), "+" );
// //         test_token_w_position!(tokens[13], TType::Name, (29, 1), (30, 1), "z" );
// //         test_token_w_position!(tokens[14], TType::Op, (31, 1), (32, 1), "+" );
// //         test_token_w_position!(tokens[15], TType::Name, (33, 1), (34, 1), "a" );
// //         test_token_w_position!(tokens[16], TType::Op, (34, 1), (35, 1), "[" );
// //         test_token_w_position!(tokens[17], TType::Number, (35, 1), (36, 1), "5" );
// //         test_token_w_position!(tokens[18], TType::Op, (36, 1), (37, 1), "]" );
// //         test_token_w_position!(tokens[19], TType::Newline, (37, 1), (38, 1), "\n" );
// //         test_token_w_position!(tokens[20], TType::EndMarker, (0, 2), (0, 2), "" );
// //     }
// //
// //     #[test]
// //     fn test_async() {
// //         let tokens = Processor::tokenize_file("test_fixtures/test_async.py", Some("test_async"), false);
// //         test_token_w_position!(tokens[0], TType::Encoding, (0, 0), (0, 0), "utf-8" );
// //         test_token_w_position!(tokens[1], TType::Name, (0, 1), (5, 1), "async" );
// //         test_token_w_position!(tokens[2], TType::Op, (6, 1), (7, 1), "=" );
// //         test_token_w_position!(tokens[3], TType::Number, (8, 1), (9, 1), "1" );
// //         test_token_w_position!(tokens[4], TType::Newline, (9, 1), (10, 1), "\n" );
// //         test_token_w_position!(tokens[5], TType::Name, (0, 2), (1, 2), "a" );
// //         test_token_w_position!(tokens[6], TType::Op, (2, 2), (3, 2), "=" );
// //         test_token_w_position!(tokens[7], TType::Op, (4, 2), (5, 2), "(" );
// //         test_token_w_position!(tokens[8], TType::Name, (5, 2), (10, 2), "async" );
// //         test_token_w_position!(tokens[9], TType::Op, (11, 2), (12, 2), "=" );
// //         test_token_w_position!(tokens[10], TType::Number, (13, 2), (14, 2), "1" );
// //         test_token_w_position!(tokens[11], TType::Op, (14, 2), (15, 2), ")" );
// //         test_token_w_position!(tokens[12], TType::Newline, (15, 2), (16, 2), "\n" );
// //         test_token_w_position!(tokens[13], TType::Name, (0, 3), (5, 3), "async" );
// //         test_token_w_position!(tokens[14], TType::Op, (5, 3), (6, 3), "(" );
// //         test_token_w_position!(tokens[15], TType::Op, (6, 3), (7, 3), ")" );
// //         test_token_w_position!(tokens[16], TType::Newline, (7, 3), (8, 3), "\n" );
// //         test_token_w_position!(tokens[17], TType::Name, (0, 4), (5, 4), "class" );
// //         test_token_w_position!(tokens[18], TType::Name, (6, 4), (11, 4), "async" );
// //         test_token_w_position!(tokens[19], TType::Op, (11, 4), (12, 4), "(" );
// //         test_token_w_position!(tokens[20], TType::Name, (12, 4), (15, 4), "Bar" );
// //         test_token_w_position!(tokens[21], TType::Op, (15, 4), (16, 4), ")" );
// //         test_token_w_position!(tokens[22], TType::Op, (16, 4), (17, 4), ":" );
// //         test_token_w_position!(tokens[23], TType::Name, (17, 4), (21, 4), "pass" );
// //         test_token_w_position!(tokens[24], TType::Newline, (21, 4), (22, 4), "\n" );
// //         test_token_w_position!(tokens[25], TType::Name, (0, 5), (5, 5), "class" );
// //         test_token_w_position!(tokens[26], TType::Name, (6, 5), (11, 5), "async" );
// //         test_token_w_position!(tokens[27], TType::Op, (11, 5), (12, 5), ":" );
// //         test_token_w_position!(tokens[28], TType::Name, (12, 5), (16, 5), "pass" );
// //         test_token_w_position!(tokens[29], TType::Newline, (16, 5), (17, 5), "\n" );
// //         test_token_w_position!(tokens[30], TType::Name, (0, 6), (5, 6), "await" );
// //         test_token_w_position!(tokens[31], TType::Op, (6, 6), (7, 6), "=" );
// //         test_token_w_position!(tokens[32], TType::Number, (8, 6), (9, 6), "1" );
// //         test_token_w_position!(tokens[33], TType::Newline, (9, 6), (10, 6), "\n" );
// //         test_token_w_position!(tokens[34], TType::Name, (0, 7), (3, 7), "foo" );
// //         test_token_w_position!(tokens[35], TType::Op, (3, 7), (4, 7), "." );
// //         test_token_w_position!(tokens[36], TType::Name, (4, 7), (9, 7), "async" );
// //         test_token_w_position!(tokens[37], TType::Newline, (9, 7), (10, 7), "\n" );
// //         test_token_w_position!(tokens[38], TType::Name, (0, 8), (5, 8), "async" );
// //         test_token_w_position!(tokens[39], TType::Name, (6, 8), (9, 8), "for" );
// //         test_token_w_position!(tokens[40], TType::Name, (10, 8), (11, 8), "a" );
// //         test_token_w_position!(tokens[41], TType::Name, (12, 8), (14, 8), "in" );
// //         test_token_w_position!(tokens[42], TType::Name, (15, 8), (16, 8), "b" );
// //         test_token_w_position!(tokens[43], TType::Op, (16, 8), (17, 8), ":" );
// //         test_token_w_position!(tokens[44], TType::Name, (18, 8), (22, 8), "pass" );
// //         test_token_w_position!(tokens[45], TType::Newline, (22, 8), (23, 8), "\n" );
// //         test_token_w_position!(tokens[46], TType::Name, (0, 9), (5, 9), "async" );
// //         test_token_w_position!(tokens[47], TType::Name, (6, 9), (10, 9), "with" );
// //         test_token_w_position!(tokens[48], TType::Name, (11, 9), (12, 9), "a" );
// //         test_token_w_position!(tokens[49], TType::Name, (13, 9), (15, 9), "as" );
// //         test_token_w_position!(tokens[50], TType::Name, (16, 9), (17, 9), "b" );
// //         test_token_w_position!(tokens[51], TType::Op, (17, 9), (18, 9), ":" );
// //         test_token_w_position!(tokens[52], TType::Name, (19, 9), (23, 9), "pass" );
// //         test_token_w_position!(tokens[53], TType::Newline, (23, 9), (24, 9), "\n" );
// //         test_token_w_position!(tokens[54], TType::Name, (0, 10), (5, 10), "async" );
// //         test_token_w_position!(tokens[55], TType::Op, (5, 10), (6, 10), "." );
// //         test_token_w_position!(tokens[56], TType::Name, (6, 10), (9, 10), "foo" );
// //         test_token_w_position!(tokens[57], TType::Newline, (9, 10), (10, 10), "\n" );
// //         test_token_w_position!(tokens[58], TType::Name, (0, 11), (5, 11), "async" );
// //         test_token_w_position!(tokens[59], TType::Newline, (5, 11), (6, 11), "\n" );
// //         test_token_w_position!(tokens[60], TType::EndMarker, (0, 12), (0, 12), "" );
// //     }
// //
// //     #[test]
// //     fn test_comparison() {
// //         let tokens = Processor::tokenize_file("test_fixtures/test_comparison.py", Some("test_comparison"), false);
// //         test_token_w_position!(tokens[0], TType::Encoding, (0, 0), (0, 0), "utf-8" );
// //         test_token_w_position!(tokens[1], TType::Name, (0, 1), (2, 1), "if" );
// //         test_token_w_position!(tokens[2], TType::Number, (3, 1), (4, 1), "1" );
// //         test_token_w_position!(tokens[3], TType::Op, (5, 1), (6, 1), "<" );
// //         test_token_w_position!(tokens[4], TType::Number, (7, 1), (8, 1), "1" );
// //         test_token_w_position!(tokens[5], TType::Op, (9, 1), (10, 1), ">" );
// //         test_token_w_position!(tokens[6], TType::Number, (11, 1), (12, 1), "1" );
// //         test_token_w_position!(tokens[7], TType::Op, (13, 1), (15, 1), "==" );
// //         test_token_w_position!(tokens[8], TType::Number, (16, 1), (17, 1), "1" );
// //         test_token_w_position!(tokens[9], TType::Op, (18, 1), (20, 1), ">=" );
// //         test_token_w_position!(tokens[10], TType::Number, (21, 1), (22, 1), "5" );
// //         test_token_w_position!(tokens[11], TType::Op, (23, 1), (25, 1), "<=" );
// //         test_token_w_position!(tokens[12], TType::Number, (26, 1), (30, 1), "0x15" );
// //         test_token_w_position!(tokens[13], TType::Op, (31, 1), (33, 1), "<=" );
// //         test_token_w_position!(tokens[14], TType::Number, (34, 1), (38, 1), "0x12" );
// //         test_token_w_position!(tokens[15], TType::Op, (39, 1), (41, 1), "!=" );
// //         test_token_w_position!(tokens[16], TType::Number, (42, 1), (43, 1), "1" );
// //         test_token_w_position!(tokens[17], TType::Name, (44, 1), (47, 1), "and" );
// //         test_token_w_position!(tokens[18], TType::Number, (48, 1), (49, 1), "5" );
// //         test_token_w_position!(tokens[19], TType::Name, (50, 1), (52, 1), "in" );
// //         test_token_w_position!(tokens[20], TType::Number, (53, 1), (54, 1), "1" );
// //         test_token_w_position!(tokens[21], TType::Name, (55, 1), (58, 1), "not" );
// //         test_token_w_position!(tokens[22], TType::Name, (59, 1), (61, 1), "in" );
// //         test_token_w_position!(tokens[23], TType::Number, (62, 1), (63, 1), "1" );
// //         test_token_w_position!(tokens[24], TType::Name, (64, 1), (66, 1), "is" );
// //         test_token_w_position!(tokens[25], TType::Number, (67, 1), (68, 1), "1" );
// //         test_token_w_position!(tokens[26], TType::Name, (69, 1), (71, 1), "or" );
// //         test_token_w_position!(tokens[27], TType::Number, (72, 1), (73, 1), "5" );
// //         test_token_w_position!(tokens[28], TType::Name, (74, 1), (76, 1), "is" );
// //         test_token_w_position!(tokens[29], TType::Name, (77, 1), (80, 1), "not" );
// //         test_token_w_position!(tokens[30], TType::Number, (81, 1), (82, 1), "1" );
// //         test_token_w_position!(tokens[31], TType::Op, (82, 1), (83, 1), ":" );
// //         test_token_w_position!(tokens[32], TType::Newline, (83, 1), (84, 1), "\n" );
// //         test_token_w_position!(tokens[33], TType::Indent, (0, 2), (4, 2), "    " );
// //         test_token_w_position!(tokens[34], TType::Name, (4, 2), (8, 2), "pass" );
// //         test_token_w_position!(tokens[35], TType::Newline, (8, 2), (9, 2), "\n" );
// //         test_token_w_position!(tokens[36], TType::Dedent, (0, 3), (0, 3), "" );
// //         test_token_w_position!(tokens[37], TType::EndMarker, (0, 3), (0, 3), "" );
// //     }
// //
// //     #[test]
// //     fn test_float() {
// //         let tokens = Processor::tokenize_file("test_fixtures/test_float.py", Some("test_float"), false);
// //
// //         print_tokens(&tokens); //TODO make it a macro?
// //
// //         test_token_w_position!(tokens[0], TType::Encoding, (0, 0), (0, 0), "utf-8" );
// //         test_token_w_position!(tokens[1], TType::Name, (0, 1), (1, 1), "x" );
// //         test_token_w_position!(tokens[2], TType::Op, (2, 1), (3, 1), "=" );
// //         test_token_w_position!(tokens[3], TType::Number, (4, 1), (11, 1), "3.14159" );
// //         test_token_w_position!(tokens[4], TType::Newline, (11, 1), (12, 1), "\n" );
// //         test_token_w_position!(tokens[5], TType::Name, (0, 2), (1, 2), "x" );
// //         test_token_w_position!(tokens[6], TType::Op, (2, 2), (3, 2), "=" );
// //         test_token_w_position!(tokens[7], TType::Number, (4, 2), (11, 2), "314159." );
// //         test_token_w_position!(tokens[8], TType::Newline, (11, 2), (12, 2), "\n" );
// //         test_token_w_position!(tokens[9], TType::Name, (0, 3), (1, 3), "x" );
// //         test_token_w_position!(tokens[10], TType::Op, (2, 3), (3, 3), "=" );
// //         test_token_w_position!(tokens[11], TType::Number, (4, 3), (11, 3), ".314159" );
// //         test_token_w_position!(tokens[12], TType::Newline, (11, 3), (12, 3), "\n" );
// //         test_token_w_position!(tokens[13], TType::Name, (0, 4), (1, 4), "x" );
// //         test_token_w_position!(tokens[14], TType::Op, (2, 4), (3, 4), "=" );
// //         test_token_w_position!(tokens[15], TType::Number, (4, 4), (9, 4), "3e141" );
// //         test_token_w_position!(tokens[16], TType::Newline, (9, 4), (10, 4), "\n" );
// //         test_token_w_position!(tokens[17], TType::Name, (0, 5), (1, 5), "x" );
// //         test_token_w_position!(tokens[18], TType::Op, (2, 5), (3, 5), "=" );
// //         test_token_w_position!(tokens[19], TType::Number, (4, 5), (9, 5), "3E123" );
// //         test_token_w_position!(tokens[20], TType::Newline, (9, 5), (10, 5), "\n" );
// //         test_token_w_position!(tokens[21], TType::Name, (0, 6), (1, 6), "x" );
// //         test_token_w_position!(tokens[22], TType::Op, (1, 6), (2, 6), "+" );
// //         test_token_w_position!(tokens[23], TType::Name, (2, 6), (3, 6), "y" );
// //         test_token_w_position!(tokens[24], TType::Op, (4, 6), (5, 6), "=" );
// //         test_token_w_position!(tokens[25], TType::Number, (6, 6), (13, 6), "3e-1230" );
// //         test_token_w_position!(tokens[26], TType::Newline, (13, 6), (14, 6), "\n" );
// //         test_token_w_position!(tokens[27], TType::Name, (0, 7), (1, 7), "x" );
// //         test_token_w_position!(tokens[28], TType::Op, (2, 7), (3, 7), "=" );
// //         test_token_w_position!(tokens[29], TType::Number, (4, 7), (12, 7), "3.14e159" );
// //         test_token_w_position!(tokens[30], TType::Newline, (12, 7), (13, 7), "\n" );
// //         test_token_w_position!(tokens[31], TType::EndMarker, (0, 8), (0, 8), "" );
// //     }
// //
// //     #[test]
// //     fn test_function() {
// //         let tokens = Processor::tokenize_file("test_fixtures/test_function.py", Some("test_function"), false);
// //         test_token_w_position!(tokens[0], TType::Encoding, (0, 0), (0, 0), "utf-8" );
// //         test_token_w_position!(tokens[1], TType::Name, (0, 1), (3, 1), "def" );
// //         test_token_w_position!(tokens[2], TType::Name, (4, 1), (7, 1), "d22" );
// //         test_token_w_position!(tokens[3], TType::Op, (7, 1), (8, 1), "(" );
// //         test_token_w_position!(tokens[4], TType::Name, (8, 1), (9, 1), "a" );
// //         test_token_w_position!(tokens[5], TType::Op, (9, 1), (10, 1), "," );
// //         test_token_w_position!(tokens[6], TType::Name, (11, 1), (12, 1), "b" );
// //         test_token_w_position!(tokens[7], TType::Op, (12, 1), (13, 1), "," );
// //         test_token_w_position!(tokens[8], TType::Name, (14, 1), (15, 1), "c" );
// //         test_token_w_position!(tokens[9], TType::Op, (15, 1), (16, 1), "=" );
// //         test_token_w_position!(tokens[10], TType::Number, (16, 1), (17, 1), "2" );
// //         test_token_w_position!(tokens[11], TType::Op, (17, 1), (18, 1), "," );
// //         test_token_w_position!(tokens[12], TType::Name, (19, 1), (20, 1), "d" );
// //         test_token_w_position!(tokens[13], TType::Op, (20, 1), (21, 1), "=" );
// //         test_token_w_position!(tokens[14], TType::Number, (21, 1), (22, 1), "2" );
// //         test_token_w_position!(tokens[15], TType::Op, (22, 1), (23, 1), "," );
// //         test_token_w_position!(tokens[16], TType::Op, (24, 1), (25, 1), "*" );
// //         test_token_w_position!(tokens[17], TType::Name, (25, 1), (26, 1), "k" );
// //         test_token_w_position!(tokens[18], TType::Op, (26, 1), (27, 1), ")" );
// //         test_token_w_position!(tokens[19], TType::Op, (27, 1), (28, 1), ":" );
// //         test_token_w_position!(tokens[20], TType::Name, (29, 1), (33, 1), "pass" );
// //         test_token_w_position!(tokens[21], TType::Newline, (33, 1), (34, 1), "\n" );
// //         test_token_w_position!(tokens[22], TType::Name, (0, 2), (3, 2), "def" );
// //         test_token_w_position!(tokens[23], TType::Name, (4, 2), (9, 2), "d01v_" );
// //         test_token_w_position!(tokens[24], TType::Op, (9, 2), (10, 2), "(" );
// //         test_token_w_position!(tokens[25], TType::Name, (10, 2), (11, 2), "a" );
// //         test_token_w_position!(tokens[26], TType::Op, (11, 2), (12, 2), "=" );
// //         test_token_w_position!(tokens[27], TType::Number, (12, 2), (13, 2), "1" );
// //         test_token_w_position!(tokens[28], TType::Op, (13, 2), (14, 2), "," );
// //         test_token_w_position!(tokens[29], TType::Op, (15, 2), (16, 2), "*" );
// //         test_token_w_position!(tokens[30], TType::Name, (16, 2), (17, 2), "k" );
// //         test_token_w_position!(tokens[31], TType::Op, (17, 2), (18, 2), "," );
// //         test_token_w_position!(tokens[32], TType::Op, (19, 2), (21, 2), "**" );
// //         test_token_w_position!(tokens[33], TType::Name, (21, 2), (22, 2), "w" );
// //         test_token_w_position!(tokens[34], TType::Op, (22, 2), (23, 2), ")" );
// //         test_token_w_position!(tokens[35], TType::Op, (23, 2), (24, 2), ":" );
// //         test_token_w_position!(tokens[36], TType::Name, (25, 2), (29, 2), "pass" );
// //         test_token_w_position!(tokens[37], TType::Newline, (29, 2), (30, 2), "\n" );
// //         test_token_w_position!(tokens[38], TType::Name, (0, 3), (3, 3), "def" );
// //         test_token_w_position!(tokens[39], TType::Name, (4, 3), (7, 3), "d23" );
// //         test_token_w_position!(tokens[40], TType::Op, (7, 3), (8, 3), "(" );
// //         test_token_w_position!(tokens[41], TType::Name, (8, 3), (9, 3), "a" );
// //         test_token_w_position!(tokens[42], TType::Op, (9, 3), (10, 3), ":" );
// //         test_token_w_position!(tokens[43], TType::Name, (11, 3), (14, 3), "str" );
// //         test_token_w_position!(tokens[44], TType::Op, (14, 3), (15, 3), "," );
// //         test_token_w_position!(tokens[45], TType::Name, (16, 3), (17, 3), "b" );
// //         test_token_w_position!(tokens[46], TType::Op, (17, 3), (18, 3), ":" );
// //         test_token_w_position!(tokens[47], TType::Name, (19, 3), (22, 3), "int" );
// //         test_token_w_position!(tokens[48], TType::Op, (22, 3), (23, 3), "=" );
// //         test_token_w_position!(tokens[49], TType::Number, (23, 3), (24, 3), "3" );
// //         test_token_w_position!(tokens[50], TType::Op, (24, 3), (25, 3), ")" );
// //         test_token_w_position!(tokens[51], TType::Op, (26, 3), (28, 3), "->" );
// //         test_token_w_position!(tokens[52], TType::Name, (29, 3), (32, 3), "int" );
// //         test_token_w_position!(tokens[53], TType::Op, (32, 3), (33, 3), ":" );
// //         test_token_w_position!(tokens[54], TType::Name, (34, 3), (38, 3), "pass" );
// //         // test_token_w_position!(tokens[55], TType::Newline, (38, 3), (39, 3), "" );
// //         test_token_w_position!(tokens[56], TType::EndMarker, (0, 4), (0, 4), "" );
// //     }
// //
// //     #[test]
// //     fn test_int() {
// //         let tokens = Processor::tokenize_file("test_fixtures/test_int.py", Some("test_int"), false);
// //         test_token_w_position!(tokens[0], TType::Encoding, (0, 0), (0, 0), "utf-8" );
// //         test_token_w_position!(tokens[1], TType::Number, (0, 1), (4, 1), "0xff" );
// //         test_token_w_position!(tokens[2], TType::Op, (5, 1), (7, 1), "<=" );
// //         test_token_w_position!(tokens[3], TType::Number, (8, 1), (11, 1), "255" );
// //         test_token_w_position!(tokens[4], TType::Newline, (11, 1), (12, 1), "\n" );
// //         test_token_w_position!(tokens[5], TType::Number, (0, 2), (4, 2), "0b10" );
// //         test_token_w_position!(tokens[6], TType::Op, (5, 2), (7, 2), "<=" );
// //         test_token_w_position!(tokens[7], TType::Number, (8, 2), (11, 2), "255" );
// //         test_token_w_position!(tokens[8], TType::Newline, (11, 2), (12, 2), "\n" );
// //         test_token_w_position!(tokens[9], TType::Number, (0, 3), (5, 3), "0o123" );
// //         test_token_w_position!(tokens[10], TType::Op, (6, 3), (8, 3), "<=" );
// //         test_token_w_position!(tokens[11], TType::Number, (9, 3), (14, 3), "0O123" );
// //         test_token_w_position!(tokens[12], TType::Newline, (14, 3), (15, 3), "\n" );
// //         test_token_w_position!(tokens[13], TType::Number, (0, 4), (7, 4), "1234567" );
// //         test_token_w_position!(tokens[14], TType::Op, (8, 4), (9, 4), ">" );
// //         test_token_w_position!(tokens[15], TType::Op, (10, 4), (11, 4), "~" );
// //         test_token_w_position!(tokens[16], TType::Number, (11, 4), (15, 4), "0x15" );
// //         test_token_w_position!(tokens[17], TType::Newline, (15, 4), (16, 4), "\n" );
// //         test_token_w_position!(tokens[18], TType::Number, (0, 5), (7, 5), "2134568" );
// //         test_token_w_position!(tokens[19], TType::Op, (8, 5), (10, 5), "!=" );
// //         test_token_w_position!(tokens[20], TType::Number, (11, 5), (18, 5), "1231515" );
// //         test_token_w_position!(tokens[21], TType::Newline, (18, 5), (19, 5), "\n" );
// //         test_token_w_position!(tokens[22], TType::Op, (0, 6), (1, 6), "(" );
// //         test_token_w_position!(tokens[23], TType::Op, (1, 6), (2, 6), "-" );
// //         test_token_w_position!(tokens[24], TType::Number, (2, 6), (8, 6), "124561" );
// //         test_token_w_position!(tokens[25], TType::Op, (8, 6), (9, 6), "-" );
// //         test_token_w_position!(tokens[26], TType::Number, (9, 6), (10, 6), "1" );
// //         test_token_w_position!(tokens[27], TType::Op, (10, 6), (11, 6), ")" );
// //         test_token_w_position!(tokens[28], TType::Op, (12, 6), (13, 6), "&" );
// //         test_token_w_position!(tokens[29], TType::Number, (14, 6), (23, 6), "200000000" );
// //         test_token_w_position!(tokens[30], TType::Newline, (23, 6), (24, 6), "\n" );
// //         test_token_w_position!(tokens[31], TType::Number, (0, 7), (10, 7), "0xdeadbeef" );
// //         test_token_w_position!(tokens[32], TType::Op, (11, 7), (13, 7), "!=" );
// //         test_token_w_position!(tokens[33], TType::Op, (14, 7), (15, 7), "-" );
// //         test_token_w_position!(tokens[34], TType::Number, (15, 7), (16, 7), "1" );
// //         test_token_w_position!(tokens[35], TType::Newline, (16, 7), (17, 7), "\n" );
// //         test_token_w_position!(tokens[36], TType::Number, (0, 8), (10, 8), "0xdeadc0de" );
// //         test_token_w_position!(tokens[37], TType::Op, (11, 8), (12, 8), "&" );
// //         test_token_w_position!(tokens[38], TType::Number, (13, 8), (18, 8), "12345" );
// //         test_token_w_position!(tokens[39], TType::Newline, (18, 8), (19, 8), "\n" );
// //         test_token_w_position!(tokens[40], TType::Number, (0, 9), (4, 9), "0xFF" );
// //         test_token_w_position!(tokens[41], TType::Op, (5, 9), (6, 9), "&" );
// //         test_token_w_position!(tokens[42], TType::Number, (7, 9), (11, 9), "0x15" );
// //         test_token_w_position!(tokens[43], TType::Op, (12, 9), (13, 9), "|" );
// //         test_token_w_position!(tokens[44], TType::Number, (14, 9), (18, 9), "1234" );
// //         // test_token_w_position!(tokens[45], TType::Newline, (18, 9), (19, 9), "" );
// //         test_token_w_position!(tokens[46], TType::EndMarker, (0, 10), (0, 10), "" );
// //     }
// //
// //     #[test]
// //     fn test_long() {
// //         let tokens = Processor::tokenize_file("test_fixtures/test_long.py", Some("test_long"), false);
// //         test_token_w_position!(tokens[0], TType::Encoding, (0, 0), (0, 0), "utf-8" );
// //         test_token_w_position!(tokens[1], TType::Name, (0, 1), (1, 1), "x" );
// //         test_token_w_position!(tokens[2], TType::Op, (2, 1), (3, 1), "=" );
// //         test_token_w_position!(tokens[3], TType::Number, (4, 1), (5, 1), "0" );
// //         test_token_w_position!(tokens[4], TType::Newline, (5, 1), (6, 1), "\n" );
// //         test_token_w_position!(tokens[5], TType::Name, (0, 2), (1, 2), "x" );
// //         test_token_w_position!(tokens[6], TType::Op, (2, 2), (3, 2), "=" );
// //         test_token_w_position!(tokens[7], TType::Number, (4, 2), (17, 2), "0xfffffffffff" );
// //         test_token_w_position!(tokens[8], TType::Newline, (17, 2), (18, 2), "\n" );
// //         test_token_w_position!(tokens[9], TType::Name, (0, 3), (1, 3), "x" );
// //         test_token_w_position!(tokens[10], TType::Op, (2, 3), (3, 3), "=" );
// //         test_token_w_position!(tokens[11], TType::Number, (4, 3), (25, 3), "123141242151251616110" );
// //         test_token_w_position!(tokens[12], TType::Newline, (25, 3), (26, 3), "\n" );
// //         test_token_w_position!(tokens[13], TType::Name, (0, 4), (1, 4), "x" );
// //         test_token_w_position!(tokens[14], TType::Op, (2, 4), (3, 4), "=" );
// //         test_token_w_position!(tokens[15], TType::Op, (4, 4), (5, 4), "-" );
// //         test_token_w_position!(tokens[16], TType::Number, (5, 4), (22, 4), "15921590215012591" );
// //         test_token_w_position!(tokens[17], TType::Newline, (22, 4), (23, 4), "\n" );
// //         test_token_w_position!(tokens[18], TType::EndMarker, (0, 5), (0, 5), "" );
// //     }
// //
// //     #[test]
// //     fn test_method() {
// //         let tokens = Processor::tokenize_file("test_fixtures/test_method.py", Some("test_method"), false);
// //         test_token_w_position!(tokens[0], TType::Encoding, (0, 0), (0, 0), "utf-8" );
// //         test_token_w_position!(tokens[1], TType::Op, (0, 1), (1, 1), "@" );
// //         test_token_w_position!(tokens[2], TType::Name, (1, 1), (13, 1), "staticmethod" );
// //         test_token_w_position!(tokens[3], TType::Newline, (13, 1), (14, 1), "\n" );
// //         test_token_w_position!(tokens[4], TType::Name, (0, 2), (3, 2), "def" );
// //         test_token_w_position!(tokens[5], TType::Name, (4, 2), (7, 2), "foo" );
// //         test_token_w_position!(tokens[6], TType::Op, (7, 2), (8, 2), "(" );
// //         test_token_w_position!(tokens[7], TType::Name, (8, 2), (9, 2), "x" );
// //         test_token_w_position!(tokens[8], TType::Op, (9, 2), (10, 2), "," );
// //         test_token_w_position!(tokens[9], TType::Name, (10, 2), (11, 2), "y" );
// //         test_token_w_position!(tokens[10], TType::Op, (11, 2), (12, 2), ")" );
// //         test_token_w_position!(tokens[11], TType::Op, (12, 2), (13, 2), ":" );
// //         test_token_w_position!(tokens[12], TType::Newline, (13, 2), (14, 2), "\n" );
// //         test_token_w_position!(tokens[13], TType::Indent, (0, 3), (4, 3), "    " );
// //         test_token_w_position!(tokens[14], TType::Name, (4, 3), (8, 3), "pass" );
// //         // test_token_w_position!(tokens[15], TType::Newline, (8, 3), (9, 3), "" );
// //         test_token_w_position!(tokens[16], TType::Dedent, (0, 4), (0, 4), "" );
// //         test_token_w_position!(tokens[17], TType::EndMarker, (0, 4), (0, 4), "" );
// //     }
// //
// //     #[test]
// //     fn test_multiplicative() {
// //         let tokens = Processor::tokenize_file("test_fixtures/test_multiplicative.py", Some("test_multiplicative"), false );
// //         test_token_w_position!(tokens[0], TType::Encoding, (0, 0), (0, 0), "utf-8" );
// //         test_token_w_position!(tokens[1], TType::Name, (0, 1), (1, 1), "x" );
// //         test_token_w_position!(tokens[2], TType::Op, (2, 1), (3, 1), "=" );
// //         test_token_w_position!(tokens[3], TType::Number, (4, 1), (5, 1), "1" );
// //         test_token_w_position!(tokens[4], TType::Op, (5, 1), (7, 1), "//" );
// //         test_token_w_position!(tokens[5], TType::Number, (7, 1), (8, 1), "1" );
// //         test_token_w_position!(tokens[6], TType::Op, (8, 1), (9, 1), "*" );
// //         test_token_w_position!(tokens[7], TType::Number, (9, 1), (10, 1), "1" );
// //         test_token_w_position!(tokens[8], TType::Op, (10, 1), (11, 1), "/" );
// //         test_token_w_position!(tokens[9], TType::Number, (11, 1), (12, 1), "5" );
// //         test_token_w_position!(tokens[10], TType::Op, (12, 1), (13, 1), "*" );
// //         test_token_w_position!(tokens[11], TType::Number, (13, 1), (15, 1), "12" );
// //         test_token_w_position!(tokens[12], TType::Op, (15, 1), (16, 1), "%" );
// //         test_token_w_position!(tokens[13], TType::Number, (16, 1), (20, 1), "0x12" );
// //         test_token_w_position!(tokens[14], TType::Op, (20, 1), (21, 1), "@" );
// //         test_token_w_position!(tokens[15], TType::Number, (21, 1), (23, 1), "42" );
// //         // test_token_w_position!(tokens[16], TType::Newline, (23, 1), (24, 1), "" );
// //         test_token_w_position!(tokens[17], TType::EndMarker, (0, 2), (0, 2), "" );
// //     }
// //
// //     #[test]
// //     fn test_selector() {
// //         //import sys, time
// //         // x = sys.modules['time'].time()
// //
// //         let tokens = Processor::tokenize_file("test_fixtures/test_selector.py", Some("test_selector"), true);
// //
// //
// //         test_token!(tokens[0], TType::Name, "import");
// //         test_token!(tokens[1], TType::Name, "sys");
// //         test_token!(tokens[2], TType::Op, ",");
// //         test_token!(tokens[3], TType::Name, "time");
// //
// //
// //         assert_eq!(tokens.len(), 19);
// //     }
// //
// //     #[test]
// //     fn test_shift() {
// //         let tokens = Processor::tokenize_file("test_fixtures/test_shift.py", Some("test_shift"), false);
// //         test_token_w_position!(tokens[0], TType::Encoding, (0, 0), (0, 0), "utf-8" );
// //         test_token_w_position!(tokens[1], TType::Name, (0, 1), (1, 1), "x" );
// //         test_token_w_position!(tokens[2], TType::Op, (2, 1), (3, 1), "=" );
// //         test_token_w_position!(tokens[3], TType::Number, (4, 1), (5, 1), "1" );
// //         test_token_w_position!(tokens[4], TType::Op, (6, 1), (8, 1), "<<" );
// //         test_token_w_position!(tokens[5], TType::Number, (9, 1), (10, 1), "1" );
// //         test_token_w_position!(tokens[6], TType::Op, (11, 1), (13, 1), ">>" );
// //         test_token_w_position!(tokens[7], TType::Number, (14, 1), (15, 1), "5" );
// //         test_token_w_position!(tokens[8], TType::Newline, (15, 1), (16, 1), "\n" );
// //         test_token_w_position!(tokens[9], TType::EndMarker, (0, 2), (0, 2), "" );
// //     }
// //
// //     #[test]
// //     fn test_string() {
// //         let tokens = Processor::tokenize_file("test_fixtures/test_string.py", Some("test_string"), false);
// //         test_token_w_position!(tokens[0], TType::Encoding, (0, 0), (0, 0), "utf-8" );
// //         test_token_w_position!(tokens[1], TType::Name, (0, 1), (1, 1), "x" );
// //         test_token_w_position!(tokens[2], TType::Op, (2, 1), (3, 1), "=" );
// //         test_token_w_position!(tokens[3], TType::String, (4, 1), (6, 1), "''" );
// //         test_token_w_position!(tokens[4], TType::Op, (6, 1), (7, 1), ";" );
// //         test_token_w_position!(tokens[5], TType::Name, (8, 1), (9, 1), "y" );
// //         test_token_w_position!(tokens[6], TType::Op, (10, 1), (11, 1), "=" );
// //         test_token_w_position!(tokens[7], TType::String, (12, 1), (14, 1), r#""""# );
// //         test_token_w_position!(tokens[8], TType::Newline, (14, 1), (15, 1), "\n" );
// //         test_token_w_position!(tokens[9], TType::Name, (0, 2), (1, 2), "x" );
// //         test_token_w_position!(tokens[10], TType::Op, (2, 2), (3, 2), "=" );
// //         test_token_w_position!(tokens[11], TType::String, (4, 2), (7, 2), r#"'"'"# );
// //         test_token_w_position!(tokens[12], TType::Op, (7, 2), (8, 2), ";" );
// //         test_token_w_position!(tokens[13], TType::Name, (9, 2), (10, 2), "y" );
// //         test_token_w_position!(tokens[14], TType::Op, (11, 2), (12, 2), "=" );
// //         test_token_w_position!(tokens[15], TType::String, (13, 2), (16, 2), r#""'""# );
// //         test_token_w_position!(tokens[16], TType::Newline, (16, 2), (17, 2), "\n" );
// //         test_token_w_position!(tokens[17], TType::Name, (0, 3), (1, 3), "x" );
// //         test_token_w_position!(tokens[18], TType::Op, (2, 3), (3, 3), "=" );
// //         test_token_w_position!(tokens[19], TType::String, (4, 3), (38, 3), r#""it doesn't \"shrink\", does it\"""# );
// //         test_token_w_position!(tokens[20], TType::Newline, (38, 3), (39, 3), "\n" );
// //         test_token_w_position!(tokens[21], TType::Name, (0, 4), (1, 4), "x" );
// //         test_token_w_position!(tokens[22], TType::Op, (2, 4), (3, 4), "=" );
// //         test_token_w_position!(tokens[23], TType::String, (4, 4), (9, 4), "'abc'" );
// //         test_token_w_position!(tokens[24], TType::Op, (10, 4), (11, 4), "+" );
// //         test_token_w_position!(tokens[25], TType::String, (12, 4), (17, 4), "'ABC'" );
// //         test_token_w_position!(tokens[26], TType::Newline, (17, 4), (18, 4), "\n" );
// //         test_token_w_position!(tokens[27], TType::Name, (0, 5), (1, 5), "y" );
// //         test_token_w_position!(tokens[28], TType::Op, (2, 5), (3, 5), "=" );
// //         test_token_w_position!(tokens[29], TType::String, (4, 5), (9, 5), r#""ABC""# );
// //         test_token_w_position!(tokens[30], TType::Op, (10, 5), (11, 5), "+" );
// //         test_token_w_position!(tokens[31], TType::String, (12, 5), (17, 5), r#""ABC""# );
// //         test_token_w_position!(tokens[32], TType::Newline, (17, 5), (18, 5), "\n" );
// //         test_token_w_position!(tokens[33], TType::Name, (0, 6), (1, 6), "x" );
// //         test_token_w_position!(tokens[34], TType::Op, (2, 6), (3, 6), "=" );
// //         test_token_w_position!(tokens[35], TType::String, (4, 6), (10, 6), "r'abc'" );
// //         test_token_w_position!(tokens[36], TType::Op, (11, 6), (12, 6), "+" );
// //         test_token_w_position!(tokens[37], TType::String, (13, 6), (19, 6), "r'ABC'" );
// //         test_token_w_position!(tokens[38], TType::Op, (20, 6), (21, 6), "+" );
// //         test_token_w_position!(tokens[39], TType::String, (22, 6), (28, 6), "R'ABC'" );
// //         test_token_w_position!(tokens[40], TType::Op, (29, 6), (30, 6), "+" );
// //         test_token_w_position!(tokens[41], TType::String, (31, 6), (37, 6), "R'ABC'" );
// //         test_token_w_position!(tokens[42], TType::Newline, (37, 6), (38, 6), "\n" );
// //         test_token_w_position!(tokens[43], TType::Name, (0, 7), (1, 7), "y" );
// //         test_token_w_position!(tokens[44], TType::Op, (2, 7), (3, 7), "=" );
// //         test_token_w_position!(tokens[45], TType::String, (4, 7), (10, 7), r#"r"abc""# );
// //         test_token_w_position!(tokens[46], TType::Op, (11, 7), (12, 7), "+" );
// //         test_token_w_position!(tokens[47], TType::String, (13, 7), (19, 7), r#"r"ABC""# );
// //         test_token_w_position!(tokens[48], TType::Op, (20, 7), (21, 7), "+" );
// //         test_token_w_position!(tokens[49], TType::String, (22, 7), (28, 7), r#"R"ABC""# );
// //         test_token_w_position!(tokens[50], TType::Op, (29, 7), (30, 7), "+" );
// //         test_token_w_position!(tokens[51], TType::String, (31, 7), (37, 7), r#"R"ABC""# );
// //         test_token_w_position!(tokens[52], TType::Newline, (37, 7), (38, 7), "\n" );
// //         test_token_w_position!(tokens[53], TType::String, (0, 8), (6, 8), "u'abc'" );
// //         test_token_w_position!(tokens[54], TType::Op, (7, 8), (8, 8), "+" );
// //         test_token_w_position!(tokens[55], TType::String, (9, 8), (15, 8), "U'abc'" );
// //         test_token_w_position!(tokens[56], TType::Newline, (15, 8), (16, 8), "\n" );
// //         test_token_w_position!(tokens[57], TType::String, (0, 9), (6, 9), r#"u"abc""# );
// //         test_token_w_position!(tokens[58], TType::Op, (7, 9), (8, 9), "+" );
// //         test_token_w_position!(tokens[59], TType::String, (9, 9), (15, 9), r#"U"abc""# );
// //         test_token_w_position!(tokens[60], TType::Newline, (15, 9), (16, 9), "\n" );
// //         test_token_w_position!(tokens[61], TType::String, (0, 10), (6, 10), "b'abc'" );
// //         test_token_w_position!(tokens[62], TType::Op, (7, 10), (8, 10), "+" );
// //         test_token_w_position!(tokens[63], TType::String, (9, 10), (15, 10), "B'abc'" );
// //         test_token_w_position!(tokens[64], TType::Newline, (15, 10), (16, 10), "\n" );
// //         test_token_w_position!(tokens[65], TType::String, (0, 11), (7, 11), "br'abc'" );
// //         test_token_w_position!(tokens[66], TType::Op, (8, 11), (9, 11), "+" );
// //         test_token_w_position!(tokens[67], TType::String, (10, 11), (17, 11), "bR'abc'" );
// //         test_token_w_position!(tokens[68], TType::Op, (18, 11), (19, 11), "+" );
// //         test_token_w_position!(tokens[69], TType::String, (20, 11), (27, 11), "Br'abc'" );
// //         test_token_w_position!(tokens[70], TType::Op, (28, 11), (29, 11), "+" );
// //         test_token_w_position!(tokens[71], TType::String, (30, 11), (37, 11), "BR'abc'" );
// //         test_token_w_position!(tokens[72], TType::Newline, (37, 11), (38, 11), "\n" );
// //         test_token_w_position!(tokens[73], TType::String, (0, 12), (7, 12), r#"br"abc""# );
// //         test_token_w_position!(tokens[74], TType::Op, (8, 12), (9, 12), "+" );
// //         test_token_w_position!(tokens[75], TType::String, (10, 12), (17, 12), r#"bR"abc""# );
// //         test_token_w_position!(tokens[76], TType::Op, (18, 12), (19, 12), "+" );
// //         test_token_w_position!(tokens[77], TType::String, (20, 12), (27, 12), r#"Br"abc""# );
// //         test_token_w_position!(tokens[78], TType::Op, (28, 12), (29, 12), "+" );
// //         test_token_w_position!(tokens[79], TType::String, (30, 12), (37, 12), r#"BR"abc""# );
// //         test_token_w_position!(tokens[80], TType::Newline, (37, 12), (38, 12), "\n" );
// //         test_token_w_position!(tokens[81], TType::String, (0, 13), (7, 13), "rb'abc'" );
// //         test_token_w_position!(tokens[82], TType::Op, (8, 13), (9, 13), "+" );
// //         test_token_w_position!(tokens[83], TType::String, (10, 13), (17, 13), "rB'abc'" );
// //         test_token_w_position!(tokens[84], TType::Op, (18, 13), (19, 13), "+" );
// //         test_token_w_position!(tokens[85], TType::String, (20, 13), (27, 13), "Rb'abc'" );
// //         test_token_w_position!(tokens[86], TType::Op, (28, 13), (29, 13), "+" );
// //         test_token_w_position!(tokens[87], TType::String, (30, 13), (37, 13), "RB'abc'" );
// //         test_token_w_position!(tokens[88], TType::Newline, (37, 13), (38, 13), "\n" );
// //         test_token_w_position!(tokens[89], TType::String, (0, 14), (7, 14), r#"rb"abc""# );
// //         test_token_w_position!(tokens[90], TType::Op, (8, 14), (9, 14), "+" );
// //         test_token_w_position!(tokens[91], TType::String, (10, 14), (17, 14), r#"rB"abc""# );
// //         test_token_w_position!(tokens[92], TType::Op, (18, 14), (19, 14), "+" );
// //         test_token_w_position!(tokens[93], TType::String, (20, 14), (27, 14), r#"Rb"abc""# );
// //         test_token_w_position!(tokens[94], TType::Op, (28, 14), (29, 14), "+" );
// //         test_token_w_position!(tokens[95], TType::String, (30, 14), (37, 14), r#"RB"abc""# );
// //         test_token_w_position!(tokens[96], TType::Newline, (37, 14), (38, 14), "\n" );
// //         test_token_w_position!(tokens[97], TType::EndMarker, (0, 15), (0, 15), "" );
// //
// //     }
// //
// //     #[test]
// //     fn test_unary() {
// //         let tokens = Processor::tokenize_file("test_fixtures/test_unary.py", Some("test_unary"), false);
// //
// //         test_token_w_position!(tokens[0], TType::Encoding, (0, 0), (0, 0), "utf-8" );
// //         test_token_w_position!(tokens[1], TType::Op, (0, 1), (1, 1), "~" );
// //         test_token_w_position!(tokens[2], TType::Number, (1, 1), (2, 1), "1" );
// //         test_token_w_position!(tokens[3], TType::Op, (3, 1), (4, 1), "^" );
// //         test_token_w_position!(tokens[4], TType::Number, (5, 1), (6, 1), "1" );
// //         test_token_w_position!(tokens[5], TType::Op, (7, 1), (8, 1), "&" );
// //         test_token_w_position!(tokens[6], TType::Number, (9, 1), (10, 1), "1" );
// //         test_token_w_position!(tokens[7], TType::Op, (11, 1), (12, 1), "|" );
// //         test_token_w_position!(tokens[8], TType::Number, (12, 1), (13, 1), "1" );
// //         test_token_w_position!(tokens[9], TType::Op, (14, 1), (15, 1), "^" );
// //         test_token_w_position!(tokens[10], TType::Op, (16, 1), (17, 1), "-" );
// //         test_token_w_position!(tokens[11], TType::Number, (17, 1), (18, 1), "1" );
// //         test_token_w_position!(tokens[12], TType::Newline, (18, 1), (19, 1), "\n" );
// //         test_token_w_position!(tokens[13], TType::Op, (0, 2), (1, 2), "-" );
// //         test_token_w_position!(tokens[14], TType::Number, (1, 2), (2, 2), "1" );
// //         test_token_w_position!(tokens[15], TType::Op, (2, 2), (3, 2), "*" );
// //         test_token_w_position!(tokens[16], TType::Number, (3, 2), (4, 2), "1" );
// //         test_token_w_position!(tokens[17], TType::Op, (4, 2), (5, 2), "/" );
// //         test_token_w_position!(tokens[18], TType::Number, (5, 2), (6, 2), "1" );
// //         test_token_w_position!(tokens[19], TType::Op, (6, 2), (7, 2), "+" );
// //         test_token_w_position!(tokens[20], TType::Number, (7, 2), (8, 2), "1" );
// //         test_token_w_position!(tokens[21], TType::Op, (8, 2), (9, 2), "*" );
// //         test_token_w_position!(tokens[22], TType::Number, (9, 2), (10, 2), "1" );
// //         test_token_w_position!(tokens[23], TType::Op, (10, 2), (12, 2), "//" );
// //         test_token_w_position!(tokens[24], TType::Number, (12, 2), (13, 2), "1" );
// //         test_token_w_position!(tokens[25], TType::Op, (14, 2), (15, 2), "-" );
// //         test_token_w_position!(tokens[26], TType::Op, (16, 2), (17, 2), "-" );
// //         test_token_w_position!(tokens[27], TType::Op, (17, 2), (18, 2), "-" );
// //         test_token_w_position!(tokens[28], TType::Op, (18, 2), (19, 2), "-" );
// //         test_token_w_position!(tokens[29], TType::Number, (19, 2), (20, 2), "1" );
// //         test_token_w_position!(tokens[30], TType::Op, (20, 2), (22, 2), "**" );
// //         test_token_w_position!(tokens[31], TType::Number, (22, 2), (23, 2), "1" );
// //         // test_token_w_position!(tokens[32], TType::Newline, (23, 2), (24, 2), "" );
// //         test_token_w_position!(tokens[33], TType::EndMarker, (0, 3), (0, 3), "" );
// //     }
// //
// //     #[test]
// //     fn test_basic_operators() {
// //         let tokens = Processor::tokenize_file("test_fixtures/test_basic_operators.py", Some("test_basic_operators"), false);
// //
// //         test_token_w_position!(tokens[0], TType::Encoding, (0, 0), (0, 0), "utf-8" );
// //         test_token_w_position!(tokens[1], TType::Number, (0, 1), (1, 1), "1" );
// //         test_token_w_position!(tokens[2], TType::Op, (2, 1), (3, 1), "+" );
// //         test_token_w_position!(tokens[3], TType::Number, (4, 1), (5, 1), "1" );
// //         //TODO test_token_w_position!(tokens[4], TType::Newline, (5, 1), (6, 1), "" );
// //         test_token_w_position!(tokens[5], TType::EndMarker, (0, 2), (0, 2), "" );
// //
// //     }
// //
// //     #[test]
// //     fn test_valid_literals() {
// //         let VALID_UNDERSCORE_LITERALS: Vec<&str> = vec![
// //             "0_0_0",
// //             "4_2",
// //             "1_0000_0000",
// //             "0b1001_0100",
// //             "0xffff_ffff",
// //             "0o5_7_7",
// //             "1_00_00.5",
// //             "1_00_00.5e5",
// //             "1_00_00e5_1",
// //             "1e1_0",
// //             ".1_4",
// //             ".1_4e1",
// //             "0b_0",
// //             "0x_f",
// //             "0o_5",
// //             "1_00_00j",
// //             "1_00_00.5j",
// //             "1_00_00e5_1j",
// //             ".1_4j",
// //             "(1_2.5+3_3j)",
// //             "(.5_6j)",
// //         ];
// //
// //         for value in VALID_UNDERSCORE_LITERALS {
// //             if value.starts_with("(") {
// //                 continue;
// //             }
// //
// //             let result = Processor::tokenize_str(value, Some("ltierals".to_string())).expect("tokens");
// //             assert_eq!(result[0].r#type, TType::Number, "Got the wrong type when processing {:?}.  Got {:?}", value, result[0]);
// //         }
// //     }
// //
// //     #[test]
// //     fn try_syntax_errors() {
// //         let result = Processor::tokenize_str("(1+2]", Some("__main__".to_string()));
// //
// //         match result {
// //             Err(issue) => {
// //                 assert_eq!(issue, TokError::MismatchedClosingParen('(', ']'));
// //             },
// //             _ => {},
// //         }
// //
// //
// //         match Processor::tokenize_str("1_", Some("__main__".to_string())) {
// //             Err(issue) => {
// //                 assert_eq!(issue, TokError::BadCharacter('_'));
// //             },
// //             _ => {},
// //         }
// //
// //     }
// // }