use std::process::id;
use crate::lexer::tokenizer::{TConfig, Tokenizer};
use crate::tokens::{TType, Position};

macro_rules! test_token_w_position{
    ($token:expr, $ttype:expr, $start:expr, $end:expr, $content:expr)=>{

        assert_eq!($token.text, $content, "Testing for text/content with {:?} != {:?} for a/{:?} e/{:?}", $token.text, $content, $token.r#type, $ttype);
        assert_eq!($token.r#type, $ttype, "Testing for type with {:?} {:?} != {:?}", $token.text, $token.r#type, $ttype);
        assert_eq!($token.start, Position::t2($start), "Testing for start with {:?} % {:?} : {:?} != {:?}", $token.text, $token.r#type, $token.start, $start);
        assert_eq!($token.end, Position::t2($end), "Testing for end with {:?} % {:?} : {:?} != {:?}", $token.text, $token.r#type, $token.end, $end);

    }
}

macro_rules! test_token{
    ($token:expr, $ttype:expr, $content:expr)=>{
        assert_eq!($token.r#type, $ttype);
        assert_eq!($token.text, $content);
    }
}

#[test]
fn test_float() {

    let tokens = Tokenizer::tokenize_file("test_fixtures/test_float.py", TConfig{skip_encoding: true, skip_endmarker: false}).expect("tokens");


    test_token_w_position!(tokens[0], TType::Name, (1, 0), (1, 1), "x" );
    test_token_w_position!(tokens[1], TType::Op, (1, 2), (1, 3), "=" );
    test_token_w_position!(tokens[2], TType::Number, (1, 4), (1, 11), "3.14159" );
    test_token_w_position!(tokens[3], TType::NL, (1, 11), (1, 11), "" );
    test_token_w_position!(tokens[4], TType::Name, (2, 0), (2, 1), "x" );
    test_token_w_position!(tokens[5], TType::Op, (2, 2), (2, 3), "=" );
    test_token_w_position!(tokens[6], TType::Number, (2, 4), (2, 11), "314159." );
    test_token_w_position!(tokens[7], TType::NL, (2, 11), (2, 11), "" );
    test_token_w_position!(tokens[8], TType::Name, (3, 0), (3, 1), "x" );
    test_token_w_position!(tokens[9], TType::Op, (3, 2), (3, 3), "=" );
    test_token_w_position!(tokens[10], TType::Number, (3, 4), (3, 11), ".314159" );
    test_token_w_position!(tokens[11], TType::NL, (3, 11), (3, 11), "" );
    test_token_w_position!(tokens[12], TType::Name, (4, 0), (4, 1), "x" );
    test_token_w_position!(tokens[13], TType::Op, (4, 2), (4, 3), "=" );
    test_token_w_position!(tokens[14], TType::Number, (4, 4), (4, 9), "3e141" );
    test_token_w_position!(tokens[15], TType::NL, (4, 9), (4, 9), "" );
    test_token_w_position!(tokens[16], TType::Name, (5, 0), (5, 1), "x" );
    test_token_w_position!(tokens[17], TType::Op, (5, 2), (5, 3), "=" );
    test_token_w_position!(tokens[18], TType::Number, (5, 4), (5, 9), "3E123" );
    test_token_w_position!(tokens[19], TType::NL, (5, 9), (5, 9), "" );
    test_token_w_position!(tokens[20], TType::Name, (7, 0), (7, 1), "x" );
    test_token_w_position!(tokens[21], TType::Op, (7, 2), (7, 3), "=" );
    test_token_w_position!(tokens[22], TType::Number, (7, 4), (7, 12), "3.14e159" );
    test_token_w_position!(tokens[23], TType::NL, (7, 12), (7, 12), "" );

}

#[test]
fn test_float_scientific () {
    let test1: String = "x = 3e141\n".to_string();
    let test2: String = "x = 3E123\n".to_string();

    let mut tokenizer = Tokenizer::new(TConfig{skip_endmarker: true, skip_encoding: true });
    let tokens1 = tokenizer.process_single_line(test1).expect("tokens");

    assert_eq!(tokens1.len(), 4);

    assert_eq!(tokens1[2].text, "3e141");
    assert_eq!(tokens1[2].r#type, TType::Number);

    let tokens2 = tokenizer.process_single_line(test2).expect("tokens");

    assert_eq!(tokens2.len(), 4);

    assert_eq!(tokens2[2].text, "3E123");
    assert_eq!(tokens2[2].r#type, TType::Number);

}

#[test]
fn test_integer_literals() {
    let tokens = Tokenizer::tokenize_file(
        "test_fixtures/test_integer_literals.py",
        TConfig{skip_encoding: true, skip_endmarker: false}).expect("tokens");

    for (idx, token) in tokens.iter().enumerate() {
        println!("{}: {:?}", idx, token);
    }

}

#[test]
fn test_additive() {
    let mut tokenizer = Tokenizer::new(TConfig{skip_encoding: true, skip_endmarker: false});
    let tokens = tokenizer.process_file("test_fixtures/test_additive.py").expect("tokens");

    test_token_w_position!(tokens[0], TType::Name, (1, 0), (1, 1), "x" );
test_token_w_position!(tokens[1], TType::Op, (1, 2), (1, 3), "=" );
test_token_w_position!(tokens[2], TType::Number, (1, 4), (1, 5), "1" );
test_token_w_position!(tokens[3], TType::Op, (1, 6), (1, 7), "-" );
test_token_w_position!(tokens[4], TType::Name, (1, 8), (1, 9), "y" );
test_token_w_position!(tokens[5], TType::Op, (1, 10), (1, 11), "+" );
test_token_w_position!(tokens[6], TType::Number, (1, 12), (1, 14), "15" );
test_token_w_position!(tokens[7], TType::Op, (1, 15), (1, 16), "-" );
test_token_w_position!(tokens[8], TType::Number, (1, 17), (1, 18), "1" );
test_token_w_position!(tokens[9], TType::Op, (1, 19), (1, 20), "+" );
test_token_w_position!(tokens[10], TType::Number, (1, 21), (1, 26), "0x124" );
test_token_w_position!(tokens[11], TType::Op, (1, 27), (1, 28), "+" );
test_token_w_position!(tokens[12], TType::Name, (1, 29), (1, 30), "z" );
test_token_w_position!(tokens[13], TType::Op, (1, 31), (1, 32), "+" );
test_token_w_position!(tokens[14], TType::Name, (1, 33), (1, 34), "a" );
test_token_w_position!(tokens[15], TType::Op, (1, 34), (1, 35), "[" );
test_token_w_position!(tokens[16], TType::Number, (1, 35), (1, 36), "5" );
test_token_w_position!(tokens[17], TType::Op, (1, 36), (1, 37), "]" );
test_token_w_position!(tokens[18], TType::NL, (1, 37), (1, 37), "" );

}

#[test]
fn test_async_as_name() {
    let mut tokenizer = Tokenizer::new(TConfig{skip_encoding: true, skip_endmarker: false});
    let tokens = tokenizer.process_file("test_fixtures/error_test_async.py").expect("tokens");

    test_token_w_position!(tokens[0], TType::Async, (1, 0), (1, 5), "async" );
test_token_w_position!(tokens[1], TType::Op, (1, 6), (1, 7), "=" );
test_token_w_position!(tokens[2], TType::Number, (1, 8), (1, 9), "1" );
test_token_w_position!(tokens[3], TType::NL, (1, 9), (1, 9), "" );
test_token_w_position!(tokens[4], TType::Name, (2, 0), (2, 1), "a" );
test_token_w_position!(tokens[5], TType::Op, (2, 2), (2, 3), "=" );
test_token_w_position!(tokens[6], TType::Op, (2, 4), (2, 5), "(" );
test_token_w_position!(tokens[7], TType::Async, (2, 5), (2, 10), "async" );
test_token_w_position!(tokens[8], TType::Op, (2, 11), (2, 12), "=" );
test_token_w_position!(tokens[9], TType::Number, (2, 13), (2, 14), "1" );
test_token_w_position!(tokens[10], TType::Op, (2, 14), (2, 15), ")" );
test_token_w_position!(tokens[11], TType::NL, (2, 15), (2, 15), "" );
test_token_w_position!(tokens[12], TType::Async, (3, 0), (3, 5), "async" );
test_token_w_position!(tokens[13], TType::Op, (3, 5), (3, 6), "(" );
test_token_w_position!(tokens[14], TType::Op, (3, 6), (3, 7), ")" );
test_token_w_position!(tokens[15], TType::NL, (3, 7), (3, 7), "" );
test_token_w_position!(tokens[16], TType::Name, (4, 0), (4, 5), "class" );
test_token_w_position!(tokens[17], TType::Async, (4, 6), (4, 11), "async" );
test_token_w_position!(tokens[18], TType::Op, (4, 11), (4, 12), "(" );
test_token_w_position!(tokens[19], TType::Name, (4, 12), (4, 15), "Bar" );
test_token_w_position!(tokens[20], TType::Op, (4, 15), (4, 16), ")" );
test_token_w_position!(tokens[21], TType::Op, (4, 16), (4, 17), ":" );
test_token_w_position!(tokens[22], TType::Name, (4, 17), (4, 21), "pass" );
test_token_w_position!(tokens[23], TType::NL, (4, 21), (4, 21), "" );
test_token_w_position!(tokens[24], TType::Name, (5, 0), (5, 5), "class" );
test_token_w_position!(tokens[25], TType::Async, (5, 6), (5, 11), "async" );
test_token_w_position!(tokens[26], TType::Op, (5, 11), (5, 12), ":" );
test_token_w_position!(tokens[27], TType::Name, (5, 12), (5, 16), "pass" );
test_token_w_position!(tokens[28], TType::NL, (5, 16), (5, 16), "" );
test_token_w_position!(tokens[29], TType::Await, (6, 0), (6, 5), "await" );
test_token_w_position!(tokens[30], TType::Op, (6, 6), (6, 7), "=" );
test_token_w_position!(tokens[31], TType::Number, (6, 8), (6, 9), "1" );
test_token_w_position!(tokens[32], TType::NL, (6, 9), (6, 9), "" );
test_token_w_position!(tokens[33], TType::Name, (7, 0), (7, 3), "foo" );
test_token_w_position!(tokens[34], TType::Op, (7, 3), (7, 4), "." );
test_token_w_position!(tokens[35], TType::Async, (7, 4), (7, 9), "async" );
test_token_w_position!(tokens[36], TType::NL, (7, 9), (7, 9), "" );
test_token_w_position!(tokens[37], TType::Async, (8, 0), (8, 5), "async" );
test_token_w_position!(tokens[38], TType::Name, (8, 6), (8, 9), "for" );
test_token_w_position!(tokens[39], TType::Name, (8, 10), (8, 11), "a" );
test_token_w_position!(tokens[40], TType::Name, (8, 12), (8, 14), "in" );
test_token_w_position!(tokens[41], TType::Name, (8, 15), (8, 16), "b" );
test_token_w_position!(tokens[42], TType::Op, (8, 16), (8, 17), ":" );
test_token_w_position!(tokens[43], TType::Name, (8, 18), (8, 22), "pass" );
test_token_w_position!(tokens[44], TType::NL, (8, 22), (8, 22), "" );
test_token_w_position!(tokens[45], TType::Async, (9, 0), (9, 5), "async" );
test_token_w_position!(tokens[46], TType::Name, (9, 6), (9, 10), "with" );
test_token_w_position!(tokens[47], TType::Name, (9, 11), (9, 12), "a" );
test_token_w_position!(tokens[48], TType::Name, (9, 13), (9, 15), "as" );
test_token_w_position!(tokens[49], TType::Name, (9, 16), (9, 17), "b" );
test_token_w_position!(tokens[50], TType::Op, (9, 17), (9, 18), ":" );
test_token_w_position!(tokens[51], TType::Name, (9, 19), (9, 23), "pass" );
test_token_w_position!(tokens[52], TType::NL, (9, 23), (9, 23), "" );
test_token_w_position!(tokens[53], TType::Async, (10, 0), (10, 5), "async" );
test_token_w_position!(tokens[54], TType::Op, (10, 5), (10, 6), "." );
test_token_w_position!(tokens[55], TType::Name, (10, 6), (10, 9), "foo" );
test_token_w_position!(tokens[56], TType::NL, (10, 9), (10, 9), "" );
test_token_w_position!(tokens[57], TType::Async, (11, 0), (11, 5), "async" );
test_token_w_position!(tokens[58], TType::NL, (11, 5), (11, 5), "" );
}

#[test]
fn test_comparison() {

    let tokens = Tokenizer::tokenize_file("test_fixtures/test_comparison.py",
    TConfig{skip_encoding: true, skip_endmarker: false}).expect("tokens");

test_token_w_position!(tokens[0], TType::Name, (1, 0), (1, 2), "if" );
test_token_w_position!(tokens[1], TType::Number, (1, 3), (1, 4), "1" );
test_token_w_position!(tokens[2], TType::Op, (1, 5), (1, 6), "<" );
test_token_w_position!(tokens[3], TType::Number, (1, 7), (1, 8), "1" );
test_token_w_position!(tokens[4], TType::Op, (1, 9), (1, 10), ">" );
test_token_w_position!(tokens[5], TType::Number, (1, 11), (1, 12), "1" );
test_token_w_position!(tokens[6], TType::Op, (1, 13), (1, 15), "==" );
test_token_w_position!(tokens[7], TType::Number, (1, 16), (1, 17), "1" );
test_token_w_position!(tokens[8], TType::Op, (1, 18), (1, 20), ">=" );
test_token_w_position!(tokens[9], TType::Number, (1, 21), (1, 22), "5" );
test_token_w_position!(tokens[10], TType::Op, (1, 23), (1, 25), "<=" );
test_token_w_position!(tokens[11], TType::Number, (1, 26), (1, 30), "0x15" );
test_token_w_position!(tokens[12], TType::Op, (1, 31), (1, 33), "<=" );
test_token_w_position!(tokens[13], TType::Number, (1, 34), (1, 38), "0x12" );
test_token_w_position!(tokens[14], TType::Op, (1, 39), (1, 41), "!=" );
test_token_w_position!(tokens[15], TType::Number, (1, 42), (1, 43), "1" );
test_token_w_position!(tokens[16], TType::Name, (1, 44), (1, 47), "and" );
test_token_w_position!(tokens[17], TType::Number, (1, 48), (1, 49), "5" );
test_token_w_position!(tokens[18], TType::Name, (1, 50), (1, 52), "in" );
test_token_w_position!(tokens[19], TType::Number, (1, 53), (1, 54), "1" );
test_token_w_position!(tokens[20], TType::Name, (1, 55), (1, 58), "not" );
test_token_w_position!(tokens[21], TType::Name, (1, 59), (1, 61), "in" );
test_token_w_position!(tokens[22], TType::Number, (1, 62), (1, 63), "1" );
test_token_w_position!(tokens[23], TType::Name, (1, 64), (1, 66), "is" );
test_token_w_position!(tokens[24], TType::Number, (1, 67), (1, 68), "1" );
test_token_w_position!(tokens[25], TType::Name, (1, 69), (1, 71), "or" );
test_token_w_position!(tokens[26], TType::Number, (1, 72), (1, 73), "5" );
test_token_w_position!(tokens[27], TType::Name, (1, 74), (1, 76), "is" );
test_token_w_position!(tokens[28], TType::Name, (1, 77), (1, 80), "not" );
test_token_w_position!(tokens[29], TType::Number, (1, 81), (1, 82), "1" );
test_token_w_position!(tokens[30], TType::Op, (1, 82), (1, 83), ":" );
test_token_w_position!(tokens[31], TType::NL, (1, 83), (1, 83), "" );
test_token_w_position!(tokens[32], TType::Indent, (2, 0), (2, 0), "" );
test_token_w_position!(tokens[33], TType::Name, (2, 4), (2, 8), "pass" );
test_token_w_position!(tokens[34], TType::NL, (2, 8), (2, 8), "" );
test_token_w_position!(tokens[35], TType::Dedent, (2, 0), (2, 0), "" );


}

#[test]
fn test_int() {

    let mut tokenizer = Tokenizer::new(TConfig{skip_encoding: true, skip_endmarker: false});
    let tokens = tokenizer.process_file("test_fixtures/test_int.py").expect("tokens");

    test_token_w_position!(tokens[0], TType::Number, (1, 0), (1, 4), "0xff" );
test_token_w_position!(tokens[1], TType::Op, (1, 5), (1, 7), "<=" );
test_token_w_position!(tokens[2], TType::Number, (1, 8), (1, 11), "255" );
test_token_w_position!(tokens[3], TType::NL, (1, 11), (1, 11), "" );
test_token_w_position!(tokens[4], TType::Number, (2, 0), (2, 4), "0b10" );
test_token_w_position!(tokens[5], TType::Op, (2, 5), (2, 7), "<=" );
test_token_w_position!(tokens[6], TType::Number, (2, 8), (2, 11), "255" );
test_token_w_position!(tokens[7], TType::NL, (2, 11), (2, 11), "" );
test_token_w_position!(tokens[8], TType::Number, (3, 0), (3, 5), "0o123" );
test_token_w_position!(tokens[9], TType::Op, (3, 6), (3, 8), "<=" );
test_token_w_position!(tokens[10], TType::Number, (3, 9), (3, 14), "0o123" );
test_token_w_position!(tokens[11], TType::NL, (3, 14), (3, 14), "" );
test_token_w_position!(tokens[12], TType::Number, (4, 0), (4, 7), "1234567" );
test_token_w_position!(tokens[13], TType::Op, (4, 8), (4, 9), ">" );
test_token_w_position!(tokens[14], TType::Op, (4, 10), (4, 11), "~" );
test_token_w_position!(tokens[15], TType::Number, (4, 11), (4, 15), "0x15" );
test_token_w_position!(tokens[16], TType::NL, (4, 15), (4, 15), "" );
test_token_w_position!(tokens[17], TType::Number, (5, 0), (5, 7), "2134568" );
test_token_w_position!(tokens[18], TType::Op, (5, 8), (5, 10), "!=" );
test_token_w_position!(tokens[19], TType::Number, (5, 11), (5, 18), "1231515" );
test_token_w_position!(tokens[20], TType::NL, (5, 18), (5, 18), "" );
test_token_w_position!(tokens[21], TType::Op, (6, 0), (6, 1), "(" );
test_token_w_position!(tokens[22], TType::Op, (6, 1), (6, 2), "-" );
test_token_w_position!(tokens[23], TType::Number, (6, 2), (6, 8), "124561" );
test_token_w_position!(tokens[24], TType::Op, (6, 8), (6, 9), "-" );
test_token_w_position!(tokens[25], TType::Number, (6, 9), (6, 10), "1" );
test_token_w_position!(tokens[26], TType::Op, (6, 10), (6, 11), ")" );
test_token_w_position!(tokens[27], TType::Op, (6, 12), (6, 13), "&" );
test_token_w_position!(tokens[28], TType::Number, (6, 14), (6, 23), "200000000" );
test_token_w_position!(tokens[29], TType::NL, (6, 23), (6, 23), "" );
test_token_w_position!(tokens[30], TType::Number, (7, 0), (7, 10), "0xdeadbeef" );
test_token_w_position!(tokens[31], TType::Op, (7, 11), (7, 13), "!=" );
test_token_w_position!(tokens[32], TType::Op, (7, 14), (7, 15), "-" );
test_token_w_position!(tokens[33], TType::Number, (7, 15), (7, 16), "1" );
test_token_w_position!(tokens[34], TType::NL, (7, 16), (7, 16), "" );
test_token_w_position!(tokens[35], TType::Number, (8, 0), (8, 10), "0xdeadc0de" );
test_token_w_position!(tokens[36], TType::Op, (8, 11), (8, 12), "&" );
test_token_w_position!(tokens[37], TType::Number, (8, 13), (8, 18), "12345" );
test_token_w_position!(tokens[38], TType::NL, (8, 18), (8, 18), "" );
test_token_w_position!(tokens[39], TType::Number, (9, 0), (9, 4), "0xFF" );
test_token_w_position!(tokens[40], TType::Op, (9, 5), (9, 6), "&" );
test_token_w_position!(tokens[41], TType::Number, (9, 7), (9, 11), "0x15" );
test_token_w_position!(tokens[42], TType::Op, (9, 12), (9, 13), "|" );
test_token_w_position!(tokens[43], TType::Number, (9, 14), (9, 18), "1234" );
test_token_w_position!(tokens[44], TType::NL, (9, 18), (9, 18), "" );

}

#[test]
fn test_long() {

    let tokens = Tokenizer::tokenize_file("test_fixtures/test_long.py" ,
                                          TConfig{skip_encoding: true, skip_endmarker: false}).expect("tokens");


    for (idx, token) in tokens.iter().enumerate() {
        println!("{}:{:?}", idx, token);
    }


    test_token_w_position!(tokens[0], TType::Name, (1, 0), (1, 1), "x" );
    test_token_w_position!(tokens[1], TType::Op, (1, 2), (1, 3), "=" );
    test_token_w_position!(tokens[2], TType::Number, (1, 4), (1, 5), "0" );
    test_token_w_position!(tokens[3], TType::NL, (1, 5), (1, 5), "" );
    test_token_w_position!(tokens[4], TType::Name, (2, 0), (2, 1), "x" );
    test_token_w_position!(tokens[5], TType::Op, (2, 2), (2, 3), "=" );
    test_token_w_position!(tokens[6], TType::Number, (2, 4), (2, 17), "0xfffffffffff" );
    test_token_w_position!(tokens[7], TType::NL, (2, 17), (2, 17), "" );
    test_token_w_position!(tokens[8], TType::Name, (3, 0), (3, 1), "x" );
    test_token_w_position!(tokens[9], TType::Op, (3, 2), (3, 3), "=" );
    test_token_w_position!(tokens[10], TType::Number, (3, 4), (3, 25), "123141242151251616110" );
    test_token_w_position!(tokens[11], TType::NL, (3, 25), (3, 25), "" );
    test_token_w_position!(tokens[12], TType::Name, (4, 0), (4, 1), "x" );
    test_token_w_position!(tokens[13], TType::Op, (4, 2), (4, 3), "=" );
    test_token_w_position!(tokens[14], TType::Op, (4, 4), (4, 5), "-" );
    test_token_w_position!(tokens[15], TType::Number, (4, 5), (4, 22), "15921590215012591" );
    test_token_w_position!(tokens[16], TType::NL, (4, 22), (4, 22), "" );

}

#[test]
fn test_numbers_mixed() {
    let tokens = Tokenizer::tokenize_file("test_fixtures/test_numbers_mixed.py",
    TConfig{skip_encoding: true, skip_endmarker: false}).expect("tokens");

    test_token_w_position!(tokens[0], TType::Name, (1, 0), (1, 1), "x" );
    test_token_w_position!(tokens[1], TType::Op, (1, 2), (1, 3), "=" );
    test_token_w_position!(tokens[2], TType::Number, (1, 4), (1, 5), "0" );
    test_token_w_position!(tokens[3], TType::NL, (1, 5), (1, 5), "" );
    test_token_w_position!(tokens[4], TType::Name, (3, 0), (3, 2), "if" );
    test_token_w_position!(tokens[5], TType::Name, (3, 3), (3, 4), "x" );
    test_token_w_position!(tokens[6], TType::Op, (3, 5), (3, 6), ">" );
    test_token_w_position!(tokens[7], TType::Number, (3, 7), (3, 8), "3" );
    test_token_w_position!(tokens[8], TType::Op, (3, 8), (3, 9), ":" );
    test_token_w_position!(tokens[9], TType::NL, (3, 9), (3, 9), "" );
    test_token_w_position!(tokens[10], TType::Indent, (4, 0), (4, 0), "" );
    test_token_w_position!(tokens[11], TType::Name, (4, 4), (4, 9), "print" );
    test_token_w_position!(tokens[12], TType::Op, (4, 9), (4, 10), "(" );
    test_token_w_position!(tokens[13], TType::Number, (4, 10), (4, 13), "456" );
    test_token_w_position!(tokens[14], TType::Op, (4, 13), (4, 14), ")" );
    test_token_w_position!(tokens[15], TType::NL, (4, 14), (4, 14), "" );
    test_token_w_position!(tokens[16], TType::Dedent, (7, 0), (7, 0), "" );
    test_token_w_position!(tokens[17], TType::Name, (7, 0), (7, 3), "def" );
    test_token_w_position!(tokens[18], TType::Name, (7, 4), (7, 7), "foo" );
    test_token_w_position!(tokens[19], TType::Op, (7, 7), (7, 8), "(" );
    test_token_w_position!(tokens[20], TType::Name, (7, 8), (7, 11), "bar" );
    test_token_w_position!(tokens[21], TType::Op, (7, 11), (7, 12), "," );
    test_token_w_position!(tokens[22], TType::Name, (7, 13), (7, 16), "kwt" );
    test_token_w_position!(tokens[23], TType::Op, (7, 16), (7, 17), "=" );
    test_token_w_position!(tokens[24], TType::Name, (7, 17), (7, 21), "None" );
    test_token_w_position!(tokens[25], TType::Op, (7, 21), (7, 22), ")" );
    test_token_w_position!(tokens[26], TType::Op, (7, 22), (7, 23), ":" );
    test_token_w_position!(tokens[27], TType::NL, (7, 23), (7, 23), "" );
    test_token_w_position!(tokens[28], TType::Indent, (8, 0), (8, 0), "" );
    test_token_w_position!(tokens[29], TType::Name, (8, 4), (8, 8), "pass" );
    test_token_w_position!(tokens[30], TType::NL, (8, 8), (8, 8), "" );
    test_token_w_position!(tokens[31], TType::Dedent, (11, 0), (11, 0), "" );
    test_token_w_position!(tokens[32], TType::Name, (11, 0), (11, 3), "foo" );
    test_token_w_position!(tokens[33], TType::Op, (11, 3), (11, 4), "(" );
    test_token_w_position!(tokens[34], TType::Number, (11, 4), (11, 5), "3" );
    test_token_w_position!(tokens[35], TType::Op, (11, 5), (11, 6), "," );
    test_token_w_position!(tokens[36], TType::Name, (11, 7), (11, 10), "kwt" );
    test_token_w_position!(tokens[37], TType::Op, (11, 10), (11, 11), "=" );
    test_token_w_position!(tokens[38], TType::Number, (11, 11), (11, 12), "5" );
    test_token_w_position!(tokens[39], TType::Op, (11, 12), (11, 13), ")" );
    test_token_w_position!(tokens[40], TType::NL, (11, 13), (11, 13), "" );

}

#[test]
fn test_method() {
    let mut tokenizer = Tokenizer::new(TConfig{skip_endmarker: false, skip_encoding: false});
    let tokens = tokenizer.process_file("test_fixtures/test_method.py").expect("tokens");

    test_token_w_position!(tokens[0], TType::Encoding, (0, 0), (0, 0), "utf-8" );
    test_token_w_position!(tokens[1], TType::Op, (0, 1), (1, 1), "@" );
    test_token_w_position!(tokens[2], TType::Name, (1, 1), (13, 1), "staticmethod" );
    test_token_w_position!(tokens[3], TType::Newline, (13, 1), (14, 1), "\n" );
    test_token_w_position!(tokens[4], TType::Name, (0, 2), (3, 2), "def" );
    test_token_w_position!(tokens[5], TType::Name, (4, 2), (7, 2), "foo" );
    test_token_w_position!(tokens[6], TType::Op, (7, 2), (8, 2), "(" );
    test_token_w_position!(tokens[7], TType::Name, (8, 2), (9, 2), "x" );
    test_token_w_position!(tokens[8], TType::Op, (9, 2), (10, 2), "," );
    test_token_w_position!(tokens[9], TType::Name, (10, 2), (11, 2), "y" );
    test_token_w_position!(tokens[10], TType::Op, (11, 2), (12, 2), ")" );
    test_token_w_position!(tokens[11], TType::Op, (12, 2), (13, 2), ":" );
    test_token_w_position!(tokens[12], TType::Newline, (13, 2), (14, 2), "\n" );
    test_token_w_position!(tokens[13], TType::Indent, (0, 3), (4, 3), "    " );
    test_token_w_position!(tokens[14], TType::Name, (4, 3), (8, 3), "pass" );
    // test_token_w_position!(tokens[15], TType::Newline, (8, 3), (9, 3), "" );
    test_token_w_position!(tokens[16], TType::Dedent, (0, 4), (0, 4), "" );
    test_token_w_position!(tokens[17], TType::EndMarker, (0, 4), (0, 4), "" );


}

#[test]
fn test_multilplicative() {

    let mut tokenizer = Tokenizer::new(TConfig{skip_encoding: false, skip_endmarker: false});
    let tokens = tokenizer.process_file("test_fixtures/test_multiplicative.py").expect("tokens");

    test_token_w_position!(tokens[0], TType::Encoding, (0, 0), (0, 0), "utf-8" );
    test_token_w_position!(tokens[1], TType::Name, (0, 1), (1, 1), "x" );
    test_token_w_position!(tokens[2], TType::Op, (2, 1), (3, 1), "=" );
    test_token_w_position!(tokens[3], TType::Number, (4, 1), (5, 1), "1" );
    test_token_w_position!(tokens[4], TType::Op, (5, 1), (7, 1), "//" );
    test_token_w_position!(tokens[5], TType::Number, (7, 1), (8, 1), "1" );
    test_token_w_position!(tokens[6], TType::Op, (8, 1), (9, 1), "*" );
    test_token_w_position!(tokens[7], TType::Number, (9, 1), (10, 1), "1" );
    test_token_w_position!(tokens[8], TType::Op, (10, 1), (11, 1), "/" );
    test_token_w_position!(tokens[9], TType::Number, (11, 1), (12, 1), "5" );
    test_token_w_position!(tokens[10], TType::Op, (12, 1), (13, 1), "*" );
    test_token_w_position!(tokens[11], TType::Number, (13, 1), (15, 1), "12" );
    test_token_w_position!(tokens[12], TType::Op, (15, 1), (16, 1), "%" );
    test_token_w_position!(tokens[13], TType::Number, (16, 1), (20, 1), "0x12" );
    test_token_w_position!(tokens[14], TType::Op, (20, 1), (21, 1), "@" );
    test_token_w_position!(tokens[15], TType::Number, (21, 1), (23, 1), "42" );
    // test_token_w_position!(tokens[16], TType::Newline, (23, 1), (24, 1), "" );
    test_token_w_position!(tokens[17], TType::EndMarker, (0, 2), (0, 2), "" );
}

#[test]
fn test_selector() {
    //import sys, time
    // x = sys.modules['time'].time()

    let mut tokenizer = Tokenizer::new(TConfig{skip_encoding: true, skip_endmarker: false});
    let tokens = tokenizer.process_file("test_fixtures/test_selector.py").expect("tokens");

    for (lineno, token) in tokens.iter().enumerate() {
        println!("{}: {:?}", lineno, token);
    }

    test_token_w_position!(tokens[0], TType::Name, (1, 0), (1, 6), "import" );
    test_token_w_position!(tokens[1], TType::Name, (1, 7), (1, 10), "sys" );
    test_token_w_position!(tokens[2], TType::Op, (1, 10), (1, 11), "," );
    test_token_w_position!(tokens[3], TType::Name, (1, 12), (1, 16), "time" );
    test_token_w_position!(tokens[4], TType::NL, (1, 16), (1, 16), "" );
    test_token_w_position!(tokens[5], TType::Name, (2, 0), (2, 1), "x" );
    test_token_w_position!(tokens[6], TType::Op, (2, 2), (2, 3), "=" );
    test_token_w_position!(tokens[7], TType::Name, (2, 4), (2, 7), "sys" );
    test_token_w_position!(tokens[8], TType::Op, (2, 7), (2, 8), "." );
    test_token_w_position!(tokens[9], TType::Name, (2, 8), (2, 15), "modules" );
    test_token_w_position!(tokens[10], TType::Op, (2, 15), (2, 16), "[" );
    test_token_w_position!(tokens[11], TType::String, (2, 16), (2, 22), "'time'" );
    test_token_w_position!(tokens[12], TType::Op, (2, 22), (2, 23), "]" );
    test_token_w_position!(tokens[13], TType::Op, (2, 23), (2, 24), "." );
    test_token_w_position!(tokens[14], TType::Name, (2, 24), (2, 28), "time" );
    test_token_w_position!(tokens[15], TType::Op, (2, 28), (2, 29), "(" );
    test_token_w_position!(tokens[16], TType::Op, (2, 29), (2, 30), ")" );
    test_token_w_position!(tokens[17], TType::NL, (2, 30), (2, 30), "" );
}

#[test]
fn test_shift() {

    let mut tokenizer = Tokenizer::new(TConfig{skip_endmarker: false, skip_encoding: false});
    let tokens = tokenizer.process_file("test_fixtures/test_shift.py").expect("tokens");

    test_token_w_position!(tokens[0], TType::Encoding, (0, 0), (0, 0), "utf-8" );
    test_token_w_position!(tokens[1], TType::Name, (0, 1), (1, 1), "x" );
    test_token_w_position!(tokens[2], TType::Op, (2, 1), (3, 1), "=" );
    test_token_w_position!(tokens[3], TType::Number, (4, 1), (5, 1), "1" );
    test_token_w_position!(tokens[4], TType::Op, (6, 1), (8, 1), "<<" );
    test_token_w_position!(tokens[5], TType::Number, (9, 1), (10, 1), "1" );
    test_token_w_position!(tokens[6], TType::Op, (11, 1), (13, 1), ">>" );
    test_token_w_position!(tokens[7], TType::Number, (14, 1), (15, 1), "5" );
    test_token_w_position!(tokens[8], TType::Newline, (15, 1), (16, 1), "\n" );
    test_token_w_position!(tokens[9], TType::EndMarker, (0, 2), (0, 2), "" );
}

#[test]
fn test_string() {

    let mut tokenizer = Tokenizer::new(TConfig{skip_encoding: false, skip_endmarker: false});
    let tokens = tokenizer.process_file("test_fixtures/test_string.py").expect("tokens");



    test_token_w_position!(tokens[0], TType::Encoding, (0, 0), (0, 0), "utf-8" );
    test_token_w_position!(tokens[1], TType::Name, (0, 1), (1, 1), "x" );
    test_token_w_position!(tokens[2], TType::Op, (2, 1), (3, 1), "=" );
    test_token_w_position!(tokens[3], TType::String, (4, 1), (6, 1), "''" );
    test_token_w_position!(tokens[4], TType::Op, (6, 1), (7, 1), ";" );
    test_token_w_position!(tokens[5], TType::Name, (8, 1), (9, 1), "y" );
    test_token_w_position!(tokens[6], TType::Op, (10, 1), (11, 1), "=" );
    test_token_w_position!(tokens[7], TType::String, (12, 1), (14, 1), r#""""# );
    test_token_w_position!(tokens[8], TType::Newline, (14, 1), (15, 1), "\n" );
    test_token_w_position!(tokens[9], TType::Name, (0, 2), (1, 2), "x" );
    test_token_w_position!(tokens[10], TType::Op, (2, 2), (3, 2), "=" );
    test_token_w_position!(tokens[11], TType::String, (4, 2), (7, 2), r#"'"'"# );
    test_token_w_position!(tokens[12], TType::Op, (7, 2), (8, 2), ";" );
    test_token_w_position!(tokens[13], TType::Name, (9, 2), (10, 2), "y" );
    test_token_w_position!(tokens[14], TType::Op, (11, 2), (12, 2), "=" );
    test_token_w_position!(tokens[15], TType::String, (13, 2), (16, 2), r#""'""# );
    test_token_w_position!(tokens[16], TType::Newline, (16, 2), (17, 2), "\n" );
    test_token_w_position!(tokens[17], TType::Name, (0, 3), (1, 3), "x" );
    test_token_w_position!(tokens[18], TType::Op, (2, 3), (3, 3), "=" );
    test_token_w_position!(tokens[19], TType::String, (4, 3), (38, 3), r#""it doesn't \"shrink\", does it\"""# );
    test_token_w_position!(tokens[20], TType::Newline, (38, 3), (39, 3), "\n" );
    test_token_w_position!(tokens[21], TType::Name, (0, 4), (1, 4), "x" );
    test_token_w_position!(tokens[22], TType::Op, (2, 4), (3, 4), "=" );
    test_token_w_position!(tokens[23], TType::String, (4, 4), (9, 4), "'abc'" );
    test_token_w_position!(tokens[24], TType::Op, (10, 4), (11, 4), "+" );
    test_token_w_position!(tokens[25], TType::String, (12, 4), (17, 4), "'ABC'" );
    test_token_w_position!(tokens[26], TType::Newline, (17, 4), (18, 4), "\n" );
    test_token_w_position!(tokens[27], TType::Name, (0, 5), (1, 5), "y" );
    test_token_w_position!(tokens[28], TType::Op, (2, 5), (3, 5), "=" );
    test_token_w_position!(tokens[29], TType::String, (4, 5), (9, 5), r#""ABC""# );
    test_token_w_position!(tokens[30], TType::Op, (10, 5), (11, 5), "+" );
    test_token_w_position!(tokens[31], TType::String, (12, 5), (17, 5), r#""ABC""# );
    test_token_w_position!(tokens[32], TType::Newline, (17, 5), (18, 5), "\n" );
    test_token_w_position!(tokens[33], TType::Name, (0, 6), (1, 6), "x" );
    test_token_w_position!(tokens[34], TType::Op, (2, 6), (3, 6), "=" );
    test_token_w_position!(tokens[35], TType::String, (4, 6), (10, 6), "r'abc'" );
    test_token_w_position!(tokens[36], TType::Op, (11, 6), (12, 6), "+" );
    test_token_w_position!(tokens[37], TType::String, (13, 6), (19, 6), "r'ABC'" );
    test_token_w_position!(tokens[38], TType::Op, (20, 6), (21, 6), "+" );
    test_token_w_position!(tokens[39], TType::String, (22, 6), (28, 6), "R'ABC'" );
    test_token_w_position!(tokens[40], TType::Op, (29, 6), (30, 6), "+" );
    test_token_w_position!(tokens[41], TType::String, (31, 6), (37, 6), "R'ABC'" );
    test_token_w_position!(tokens[42], TType::Newline, (37, 6), (38, 6), "\n" );
    test_token_w_position!(tokens[43], TType::Name, (0, 7), (1, 7), "y" );
    test_token_w_position!(tokens[44], TType::Op, (2, 7), (3, 7), "=" );
    test_token_w_position!(tokens[45], TType::String, (4, 7), (10, 7), r#"r"abc""# );
    test_token_w_position!(tokens[46], TType::Op, (11, 7), (12, 7), "+" );
    test_token_w_position!(tokens[47], TType::String, (13, 7), (19, 7), r#"r"ABC""# );
    test_token_w_position!(tokens[48], TType::Op, (20, 7), (21, 7), "+" );
    test_token_w_position!(tokens[49], TType::String, (22, 7), (28, 7), r#"R"ABC""# );
    test_token_w_position!(tokens[50], TType::Op, (29, 7), (30, 7), "+" );
    test_token_w_position!(tokens[51], TType::String, (31, 7), (37, 7), r#"R"ABC""# );
    test_token_w_position!(tokens[52], TType::Newline, (37, 7), (38, 7), "\n" );
    test_token_w_position!(tokens[53], TType::String, (0, 8), (6, 8), "u'abc'" );
    test_token_w_position!(tokens[54], TType::Op, (7, 8), (8, 8), "+" );
    test_token_w_position!(tokens[55], TType::String, (9, 8), (15, 8), "U'abc'" );
    test_token_w_position!(tokens[56], TType::Newline, (15, 8), (16, 8), "\n" );
    test_token_w_position!(tokens[57], TType::String, (0, 9), (6, 9), r#"u"abc""# );
    test_token_w_position!(tokens[58], TType::Op, (7, 9), (8, 9), "+" );
    test_token_w_position!(tokens[59], TType::String, (9, 9), (15, 9), r#"U"abc""# );
    test_token_w_position!(tokens[60], TType::Newline, (15, 9), (16, 9), "\n" );
    test_token_w_position!(tokens[61], TType::String, (0, 10), (6, 10), "b'abc'" );
    test_token_w_position!(tokens[62], TType::Op, (7, 10), (8, 10), "+" );
    test_token_w_position!(tokens[63], TType::String, (9, 10), (15, 10), "B'abc'" );
    test_token_w_position!(tokens[64], TType::Newline, (15, 10), (16, 10), "\n" );
    test_token_w_position!(tokens[65], TType::String, (0, 11), (7, 11), "br'abc'" );
    test_token_w_position!(tokens[66], TType::Op, (8, 11), (9, 11), "+" );
    test_token_w_position!(tokens[67], TType::String, (10, 11), (17, 11), "bR'abc'" );
    test_token_w_position!(tokens[68], TType::Op, (18, 11), (19, 11), "+" );
    test_token_w_position!(tokens[69], TType::String, (20, 11), (27, 11), "Br'abc'" );
    test_token_w_position!(tokens[70], TType::Op, (28, 11), (29, 11), "+" );
    test_token_w_position!(tokens[71], TType::String, (30, 11), (37, 11), "BR'abc'" );
    test_token_w_position!(tokens[72], TType::Newline, (37, 11), (38, 11), "\n" );
    test_token_w_position!(tokens[73], TType::String, (0, 12), (7, 12), r#"br"abc""# );
    test_token_w_position!(tokens[74], TType::Op, (8, 12), (9, 12), "+" );
    test_token_w_position!(tokens[75], TType::String, (10, 12), (17, 12), r#"bR"abc""# );
    test_token_w_position!(tokens[76], TType::Op, (18, 12), (19, 12), "+" );
    test_token_w_position!(tokens[77], TType::String, (20, 12), (27, 12), r#"Br"abc""# );
    test_token_w_position!(tokens[78], TType::Op, (28, 12), (29, 12), "+" );
    test_token_w_position!(tokens[79], TType::String, (30, 12), (37, 12), r#"BR"abc""# );
    test_token_w_position!(tokens[80], TType::Newline, (37, 12), (38, 12), "\n" );
    test_token_w_position!(tokens[81], TType::String, (0, 13), (7, 13), "rb'abc'" );
    test_token_w_position!(tokens[82], TType::Op, (8, 13), (9, 13), "+" );
    test_token_w_position!(tokens[83], TType::String, (10, 13), (17, 13), "rB'abc'" );
    test_token_w_position!(tokens[84], TType::Op, (18, 13), (19, 13), "+" );
    test_token_w_position!(tokens[85], TType::String, (20, 13), (27, 13), "Rb'abc'" );
    test_token_w_position!(tokens[86], TType::Op, (28, 13), (29, 13), "+" );
    test_token_w_position!(tokens[87], TType::String, (30, 13), (37, 13), "RB'abc'" );
    test_token_w_position!(tokens[88], TType::Newline, (37, 13), (38, 13), "\n" );
    test_token_w_position!(tokens[89], TType::String, (0, 14), (7, 14), r#"rb"abc""# );
    test_token_w_position!(tokens[90], TType::Op, (8, 14), (9, 14), "+" );
    test_token_w_position!(tokens[91], TType::String, (10, 14), (17, 14), r#"rB"abc""# );
    test_token_w_position!(tokens[92], TType::Op, (18, 14), (19, 14), "+" );
    test_token_w_position!(tokens[93], TType::String, (20, 14), (27, 14), r#"Rb"abc""# );
    test_token_w_position!(tokens[94], TType::Op, (28, 14), (29, 14), "+" );
    test_token_w_position!(tokens[95], TType::String, (30, 14), (37, 14), r#"RB"abc""# );
    test_token_w_position!(tokens[96], TType::Newline, (37, 14), (38, 14), "\n" );
    test_token_w_position!(tokens[97], TType::EndMarker, (0, 15), (0, 15), "" );

}


#[test]
fn test_unary() {
    let mut tokenizer = Tokenizer::new(TConfig{skip_endmarker: false, skip_encoding: false});
    let tokens = tokenizer.process_file("test_fixtures/test_unary.py").expect("tokens");

    test_token_w_position!(tokens[0], TType::Encoding, (0, 0), (0, 0), "utf-8" );
    test_token_w_position!(tokens[1], TType::Op, (0, 1), (1, 1), "~" );
    test_token_w_position!(tokens[2], TType::Number, (1, 1), (2, 1), "1" );
    test_token_w_position!(tokens[3], TType::Op, (3, 1), (4, 1), "^" );
    test_token_w_position!(tokens[4], TType::Number, (5, 1), (6, 1), "1" );
    test_token_w_position!(tokens[5], TType::Op, (7, 1), (8, 1), "&" );
    test_token_w_position!(tokens[6], TType::Number, (9, 1), (10, 1), "1" );
    test_token_w_position!(tokens[7], TType::Op, (11, 1), (12, 1), "|" );
    test_token_w_position!(tokens[8], TType::Number, (12, 1), (13, 1), "1" );
    test_token_w_position!(tokens[9], TType::Op, (14, 1), (15, 1), "^" );
    test_token_w_position!(tokens[10], TType::Op, (16, 1), (17, 1), "-" );
    test_token_w_position!(tokens[11], TType::Number, (17, 1), (18, 1), "1" );
    test_token_w_position!(tokens[12], TType::Newline, (18, 1), (19, 1), "\n" );
    test_token_w_position!(tokens[13], TType::Op, (0, 2), (1, 2), "-" );
    test_token_w_position!(tokens[14], TType::Number, (1, 2), (2, 2), "1" );
    test_token_w_position!(tokens[15], TType::Op, (2, 2), (3, 2), "*" );
    test_token_w_position!(tokens[16], TType::Number, (3, 2), (4, 2), "1" );
    test_token_w_position!(tokens[17], TType::Op, (4, 2), (5, 2), "/" );
    test_token_w_position!(tokens[18], TType::Number, (5, 2), (6, 2), "1" );
    test_token_w_position!(tokens[19], TType::Op, (6, 2), (7, 2), "+" );
    test_token_w_position!(tokens[20], TType::Number, (7, 2), (8, 2), "1" );
    test_token_w_position!(tokens[21], TType::Op, (8, 2), (9, 2), "*" );
    test_token_w_position!(tokens[22], TType::Number, (9, 2), (10, 2), "1" );
    test_token_w_position!(tokens[23], TType::Op, (10, 2), (12, 2), "//" );
    test_token_w_position!(tokens[24], TType::Number, (12, 2), (13, 2), "1" );
    test_token_w_position!(tokens[25], TType::Op, (14, 2), (15, 2), "-" );
    test_token_w_position!(tokens[26], TType::Op, (16, 2), (17, 2), "-" );
    test_token_w_position!(tokens[27], TType::Op, (17, 2), (18, 2), "-" );
    test_token_w_position!(tokens[28], TType::Op, (18, 2), (19, 2), "-" );
    test_token_w_position!(tokens[29], TType::Number, (19, 2), (20, 2), "1" );
    test_token_w_position!(tokens[30], TType::Op, (20, 2), (22, 2), "**" );
    test_token_w_position!(tokens[31], TType::Number, (22, 2), (23, 2), "1" );
    // test_token_w_position!(tokens[32], TType::Newline, (23, 2), (24, 2), "" );
    test_token_w_position!(tokens[33], TType::EndMarker, (0, 3), (0, 3), "" );
}

#[test]
fn test_basic_operators() {

    let mut tokenizer = Tokenizer::new(TConfig{skip_encoding: true, skip_endmarker: false});
    let tokens = tokenizer.process_file("test_fixtures/test_basic_operators.py").expect("tokens");

    test_token_w_position!(tokens[0], TType::Number, (1, 0), (1, 1), "1" );
    test_token_w_position!(tokens[1], TType::Op, (1, 2), (1, 3), "+" );
    test_token_w_position!(tokens[2], TType::Number, (1, 4), (1, 5), "1" );
    test_token_w_position!(tokens[3], TType::NL, (1, 5), (1, 5), "" );


}

#[test]
fn test_valid_literals() {
    let ValidUnderscoreLiterals: Vec<&str> = vec![
        "0_0_0",
        "4_2",
        "1_0000_0000",
        "0b1001_0100",
        "0xffff_ffff",
        "0o5_7_7",
        "1_00_00.5",
        "1_00_00.5e5",
        "1_00_00e5_1",
        "1e1_0",
        ".1_4",
        ".1_4e1",
        "0b_0",
        "0x_f",
        "0o_5",
        "1_00_00j",
        "1_00_00.5j",
        "1_00_00e5_1j",
        ".1_4j",
        "(1_2.5+3_3j)",
        "(.5_6j)",
    ];

    let mut tokenizer = Tokenizer::new(TConfig{skip_endmarker: true, skip_encoding: true});


    for value in ValidUnderscoreLiterals {
        if value.starts_with("(") {
            continue;
        }

        let result = tokenizer.process_single_line(value.to_string()).expect("tokens");
        assert_eq!(result[0].r#type, TType::Number, "Got the wrong type when processing {:?}.  Got {:?}", value, result[0]);
    }
}


#[test]
fn test_multiline_strings() {

    let mut tokenizer = Tokenizer::new(TConfig{ skip_encoding: true, skip_endmarker: true});
    let tokens = tokenizer.process_file("test_fixtures/multiline_strings.py").expect("tokens");

    for (lno, token) in tokens.iter().enumerate() {
        println!("{}: {:?}", lno, token);
    }

    let str1 =
r#""""
    Hello World
""""#.to_string();

    let str2 =
r#""""This is a test
to see if it works""""#.to_string();



    test_token_w_position!(tokens[0], TType::String, (1, 0), (1, 41), "\"\"\"This is a triple quote on one line!\"\"\"" );
    test_token_w_position!(tokens[1], TType::NL, (1, 41), (1, 41), "" );
    test_token_w_position!(tokens[2], TType::String, (3, 0), (5, 3), str1);
    test_token_w_position!(tokens[3], TType::NL, (5, 3), (5, 3), "" );
    test_token_w_position!(tokens[4], TType::String, (7, 0), (8, 21), str2);
    test_token_w_position!(tokens[5], TType::NL, (8, 21), (8, 21), "" );


}

// #[test]
// fn test_correct_newlines() {
//     let mut tokenizer = Tokenizer::new(TConfig::default());
//     let tokens = tokenizer.process_file("test_fixtures/test_correct_newlines.py").expect("tokens");
//
//     for (lno, token) in tokens.iter().enumerate() {
//         println!("{}: {:?}", lno, token);
//     }
//
// }

#[test]
fn test_function() {
    let mut tokenizer = Tokenizer::new(TConfig::default());
    let tokens = tokenizer.process_file("test_fixtures/test_function.py").expect("tokens");

    for (lno, token) in tokens.iter().enumerate() {
        println!("{}: {:?}", lno, token);
    }

    test_token_w_position!(tokens[0], TType::Encoding, (0, 0), (0, 0), "utf-8" );
    test_token_w_position!(tokens[1], TType::Name, (0, 1), (3, 1), "def" );
    test_token_w_position!(tokens[2], TType::Name, (4, 1), (7, 1), "d22" );
    test_token_w_position!(tokens[3], TType::Op, (7, 1), (8, 1), "(" );
    test_token_w_position!(tokens[4], TType::Name, (8, 1), (9, 1), "a" );
    test_token_w_position!(tokens[5], TType::Op, (9, 1), (10, 1), "," );
    test_token_w_position!(tokens[6], TType::Name, (11, 1), (12, 1), "b" );
    test_token_w_position!(tokens[7], TType::Op, (12, 1), (13, 1), "," );
    test_token_w_position!(tokens[8], TType::Name, (14, 1), (15, 1), "c" );
    test_token_w_position!(tokens[9], TType::Op, (15, 1), (16, 1), "=" );
    test_token_w_position!(tokens[10], TType::Number, (16, 1), (17, 1), "2" );
    test_token_w_position!(tokens[11], TType::Op, (17, 1), (18, 1), "," );
    test_token_w_position!(tokens[12], TType::Name, (19, 1), (20, 1), "d" );
    test_token_w_position!(tokens[13], TType::Op, (20, 1), (21, 1), "=" );
    test_token_w_position!(tokens[14], TType::Number, (21, 1), (22, 1), "2" );
    test_token_w_position!(tokens[15], TType::Op, (22, 1), (23, 1), "," );
    test_token_w_position!(tokens[16], TType::Op, (24, 1), (25, 1), "*" );
    test_token_w_position!(tokens[17], TType::Name, (25, 1), (26, 1), "k" );
    test_token_w_position!(tokens[18], TType::Op, (26, 1), (27, 1), ")" );
    test_token_w_position!(tokens[19], TType::Op, (27, 1), (28, 1), ":" );
    test_token_w_position!(tokens[20], TType::Name, (29, 1), (33, 1), "pass" );
    test_token_w_position!(tokens[21], TType::Newline, (33, 1), (34, 1), "\n" );
    test_token_w_position!(tokens[22], TType::Name, (0, 2), (3, 2), "def" );
    test_token_w_position!(tokens[23], TType::Name, (4, 2), (9, 2), "d01v_" );
    test_token_w_position!(tokens[24], TType::Op, (9, 2), (10, 2), "(" );
    test_token_w_position!(tokens[25], TType::Name, (10, 2), (11, 2), "a" );
    test_token_w_position!(tokens[26], TType::Op, (11, 2), (12, 2), "=" );
    test_token_w_position!(tokens[27], TType::Number, (12, 2), (13, 2), "1" );
    test_token_w_position!(tokens[28], TType::Op, (13, 2), (14, 2), "," );
    test_token_w_position!(tokens[29], TType::Op, (15, 2), (16, 2), "*" );
    test_token_w_position!(tokens[30], TType::Name, (16, 2), (17, 2), "k" );
    test_token_w_position!(tokens[31], TType::Op, (17, 2), (18, 2), "," );
    test_token_w_position!(tokens[32], TType::Op, (19, 2), (21, 2), "**" );
    test_token_w_position!(tokens[33], TType::Name, (21, 2), (22, 2), "w" );
    test_token_w_position!(tokens[34], TType::Op, (22, 2), (23, 2), ")" );
    test_token_w_position!(tokens[35], TType::Op, (23, 2), (24, 2), ":" );
    test_token_w_position!(tokens[36], TType::Name, (25, 2), (29, 2), "pass" );
    test_token_w_position!(tokens[37], TType::Newline, (29, 2), (30, 2), "\n" );
    test_token_w_position!(tokens[38], TType::Name, (0, 3), (3, 3), "def" );
    test_token_w_position!(tokens[39], TType::Name, (4, 3), (7, 3), "d23" );
    test_token_w_position!(tokens[40], TType::Op, (7, 3), (8, 3), "(" );
    test_token_w_position!(tokens[41], TType::Name, (8, 3), (9, 3), "a" );
    test_token_w_position!(tokens[42], TType::Op, (9, 3), (10, 3), ":" );
    test_token_w_position!(tokens[43], TType::Name, (11, 3), (14, 3), "str" );
    test_token_w_position!(tokens[44], TType::Op, (14, 3), (15, 3), "," );
    test_token_w_position!(tokens[45], TType::Name, (16, 3), (17, 3), "b" );
    test_token_w_position!(tokens[46], TType::Op, (17, 3), (18, 3), ":" );
    test_token_w_position!(tokens[47], TType::Name, (19, 3), (22, 3), "int" );
    test_token_w_position!(tokens[48], TType::Op, (22, 3), (23, 3), "=" );
    test_token_w_position!(tokens[49], TType::Number, (23, 3), (24, 3), "3" );
    test_token_w_position!(tokens[50], TType::Op, (24, 3), (25, 3), ")" );
    test_token_w_position!(tokens[51], TType::Op, (26, 3), (28, 3), "->" );
    test_token_w_position!(tokens[52], TType::Name, (29, 3), (32, 3), "int" );
    test_token_w_position!(tokens[53], TType::Op, (32, 3), (33, 3), ":" );
    test_token_w_position!(tokens[54], TType::Name, (34, 3), (38, 3), "pass" );
    test_token_w_position!(tokens[55], TType::Newline, (38, 3), (39, 3), "\n" );
    test_token_w_position!(tokens[56], TType::EndMarker, (0, 4), (0, 4), "" );

}

#[test]
fn test_basic_class() {
    let tokens = Tokenizer::tokenize_file(
        "test_fixtures/basic_class.py",
        TConfig{skip_encoding: true, skip_endmarker: false}
    ).expect("tokens");


}

#[test]
fn test_basic_indent() {

    let tokens = Tokenizer::tokenize_file(
        "test_fixtures/basic_indent.py",
        TConfig{ skip_encoding: true, skip_endmarker: true}).expect("tokens");

    test_token_w_position!(tokens[0], TType::Name, (1, 0), (1, 3), "def" );
    test_token_w_position!(tokens[1], TType::Name, (1, 4), (1, 8), "test" );
    test_token_w_position!(tokens[2], TType::Op, (1, 8), (1, 9), "(" );
    test_token_w_position!(tokens[3], TType::Op, (1, 9), (1, 10), ")" );
    test_token_w_position!(tokens[4], TType::Op, (1, 10), (1, 11), ":" );
    test_token_w_position!(tokens[5], TType::NL, (1, 11), (1, 11), "" );
    test_token_w_position!(tokens[6], TType::Indent, (2, 0), (2, 0), "" );
    test_token_w_position!(tokens[7], TType::Name, (2, 4), (2, 9), "print" );
    test_token_w_position!(tokens[8], TType::Op, (2, 9), (2, 10), "(" );
    test_token_w_position!(tokens[9], TType::String, (2, 10), (2, 23), "Hello world" );
    test_token_w_position!(tokens[10], TType::Op, (2, 23), (2, 24), ")" );
    test_token_w_position!(tokens[11], TType::NL, (2, 24), (2, 24), "" );
    test_token_w_position!(tokens[12], TType::Dedent, (4, 0), (4, 0), "" );
    test_token_w_position!(tokens[13], TType::Name, (4, 0), (4, 3), "def" );
    test_token_w_position!(tokens[14], TType::Name, (4, 4), (4, 7), "foo" );
    test_token_w_position!(tokens[15], TType::Op, (4, 7), (4, 8), "(" );
    test_token_w_position!(tokens[16], TType::Op, (4, 8), (4, 9), ")" );
    test_token_w_position!(tokens[17], TType::Op, (4, 9), (4, 10), ":" );
    test_token_w_position!(tokens[18], TType::NL, (4, 10), (4, 10), "" );
    test_token_w_position!(tokens[19], TType::Indent, (5, 0), (5, 0), "" );
    test_token_w_position!(tokens[20], TType::Name, (5, 4), (5, 9), "print" );
    test_token_w_position!(tokens[21], TType::Op, (5, 9), (5, 10), "(" );
    test_token_w_position!(tokens[22], TType::String, (5, 10), (5, 19), "block 2" );
    test_token_w_position!(tokens[23], TType::Op, (5, 19), (5, 20), ")" );
    test_token_w_position!(tokens[24], TType::NL, (5, 20), (5, 20), "" );
    test_token_w_position!(tokens[25], TType::Dedent, (5, 0), (5, 0), "" );


}

#[test]
fn test_crazy_dents() {
    let tokens = Tokenizer::tokenize_file(
        "test_fixtures/crazy_dents.py",
        TConfig{ skip_encoding: true, skip_endmarker: true}).expect("tokens");



    test_token_w_position!(tokens[0], TType::Name, (3, 0), (3, 3), "def" );
test_token_w_position!(tokens[1], TType::Name, (3, 4), (3, 11), "toptier" );
test_token_w_position!(tokens[2], TType::Op, (3, 11), (3, 12), "(" );
test_token_w_position!(tokens[3], TType::Op, (3, 12), (3, 13), ")" );
test_token_w_position!(tokens[4], TType::Op, (3, 13), (3, 14), ":" );
test_token_w_position!(tokens[5], TType::NL, (3, 14), (3, 14), "" );
test_token_w_position!(tokens[6], TType::Indent, (4, 0), (4, 0), "" );
test_token_w_position!(tokens[7], TType::Name, (4, 4), (4, 15), "still_depth" );
test_token_w_position!(tokens[8], TType::Op, (4, 16), (4, 17), "=" );
test_token_w_position!(tokens[9], TType::Name, (4, 18), (4, 22), "True" );
test_token_w_position!(tokens[10], TType::NL, (4, 22), (4, 22), "" );
test_token_w_position!(tokens[11], TType::Name, (5, 4), (5, 7), "def" );
test_token_w_position!(tokens[12], TType::Name, (5, 8), (5, 15), "midtier" );
test_token_w_position!(tokens[13], TType::Op, (5, 15), (5, 16), "(" );
test_token_w_position!(tokens[14], TType::Name, (5, 16), (5, 27), "still_depth" );
test_token_w_position!(tokens[15], TType::Op, (5, 27), (5, 28), ")" );
test_token_w_position!(tokens[16], TType::Op, (5, 28), (5, 29), ":" );
test_token_w_position!(tokens[17], TType::NL, (5, 29), (5, 29), "" );
test_token_w_position!(tokens[18], TType::Indent, (6, 0), (6, 0), "" );
test_token_w_position!(tokens[19], TType::Name, (6, 8), (6, 23), "more_to_go_true" );
test_token_w_position!(tokens[20], TType::Op, (6, 24), (6, 25), "=" );
test_token_w_position!(tokens[21], TType::Name, (6, 26), (6, 30), "True" );
test_token_w_position!(tokens[22], TType::NL, (6, 30), (6, 30), "" );
test_token_w_position!(tokens[23], TType::Name, (7, 8), (7, 11), "def" );
test_token_w_position!(tokens[24], TType::Name, (7, 12), (7, 23), "bottom_tier" );
test_token_w_position!(tokens[25], TType::Op, (7, 23), (7, 24), "(" );
test_token_w_position!(tokens[26], TType::Op, (7, 24), (7, 25), ")" );
test_token_w_position!(tokens[27], TType::Op, (7, 25), (7, 26), ":" );
test_token_w_position!(tokens[28], TType::NL, (7, 26), (7, 26), "" );
test_token_w_position!(tokens[29], TType::Indent, (8, 0), (8, 0), "" );
test_token_w_position!(tokens[30], TType::Name, (8, 12), (8, 14), "if" );
test_token_w_position!(tokens[31], TType::Name, (8, 15), (8, 26), "still_depth" );
test_token_w_position!(tokens[32], TType::Op, (8, 26), (8, 27), ":" );
test_token_w_position!(tokens[33], TType::NL, (8, 27), (8, 27), "" );
test_token_w_position!(tokens[34], TType::Indent, (9, 0), (9, 0), "" );
test_token_w_position!(tokens[35], TType::Name, (9, 16), (9, 27), "still_depth" );
test_token_w_position!(tokens[36], TType::Op, (9, 28), (9, 29), "=" );
test_token_w_position!(tokens[37], TType::Name, (9, 30), (9, 35), "False" );
test_token_w_position!(tokens[38], TType::NL, (9, 35), (9, 35), "" );
test_token_w_position!(tokens[39], TType::Name, (10, 16), (10, 22), "return" );
test_token_w_position!(tokens[40], TType::Name, (10, 23), (10, 27), "True" );
test_token_w_position!(tokens[41], TType::NL, (10, 27), (10, 27), "" );
test_token_w_position!(tokens[42], TType::Dedent, (12, 0), (12, 0), "" );
test_token_w_position!(tokens[43], TType::Name, (12, 12), (12, 16), "else" );
test_token_w_position!(tokens[44], TType::Op, (12, 16), (12, 17), ":" );
test_token_w_position!(tokens[45], TType::NL, (12, 17), (12, 17), "" );
test_token_w_position!(tokens[46], TType::Indent, (13, 0), (13, 0), "" );
test_token_w_position!(tokens[47], TType::Name, (13, 16), (13, 18), "if" );
test_token_w_position!(tokens[48], TType::Name, (13, 19), (13, 34), "more_to_go_true" );
test_token_w_position!(tokens[49], TType::Op, (13, 34), (13, 35), ":" );
test_token_w_position!(tokens[50], TType::NL, (13, 35), (13, 35), "" );
test_token_w_position!(tokens[51], TType::Indent, (14, 0), (14, 0), "" );
test_token_w_position!(tokens[52], TType::Name, (14, 20), (14, 35), "more_to_go_true" );
test_token_w_position!(tokens[53], TType::Op, (14, 36), (14, 37), "=" );
test_token_w_position!(tokens[54], TType::Name, (14, 38), (14, 43), "False" );
test_token_w_position!(tokens[55], TType::NL, (14, 43), (14, 43), "" );
test_token_w_position!(tokens[56], TType::Name, (15, 20), (15, 26), "return" );
test_token_w_position!(tokens[57], TType::Name, (15, 27), (15, 32), "False" );
test_token_w_position!(tokens[58], TType::NL, (15, 32), (15, 32), "" );
test_token_w_position!(tokens[59], TType::Dedent, (18, 0), (18, 0), "" );
test_token_w_position!(tokens[60], TType::Dedent, (18, 0), (18, 0), "" );
test_token_w_position!(tokens[61], TType::Dedent, (18, 0), (18, 0), "" );
test_token_w_position!(tokens[62], TType::Dedent, (18, 0), (18, 0), "" );
test_token_w_position!(tokens[63], TType::Dedent, (18, 0), (18, 0), "" );
test_token_w_position!(tokens[64], TType::Name, (18, 0), (18, 4), "from" );
test_token_w_position!(tokens[65], TType::Name, (18, 5), (18, 8), "foo" );
test_token_w_position!(tokens[66], TType::Name, (18, 9), (18, 15), "import" );
test_token_w_position!(tokens[67], TType::Op, (18, 16), (18, 17), "(" );
test_token_w_position!(tokens[68], TType::Name, (18, 17), (18, 23), "thing1" );
test_token_w_position!(tokens[69], TType::Op, (18, 23), (18, 24), "," );
test_token_w_position!(tokens[70], TType::Name, (19, 17), (19, 33), "thing2_no_indent" );
test_token_w_position!(tokens[71], TType::Op, (19, 33), (19, 34), "," );
test_token_w_position!(tokens[72], TType::Name, (20, 17), (20, 23), "thing3" );
test_token_w_position!(tokens[73], TType::Op, (20, 23), (20, 24), ")" );
test_token_w_position!(tokens[74], TType::NL, (20, 24), (20, 24), "" );

}


#[test]
fn test_match() {
    let tokens = Tokenizer::tokenize_file(
        "test_fixtures/test_match.py",
        TConfig{skip_encoding: true, skip_endmarker: false}
    ).expect("tokens");

    for (idx, token) in tokens.iter().enumerate() {
        println!("{}: {:?}", idx, token);
    }

    test_token_w_position!(tokens[0], TType::Name, (1, 0), (1, 1), "x" );
    test_token_w_position!(tokens[1], TType::Op, (1, 2), (1, 3), "=" );
    test_token_w_position!(tokens[2], TType::Number, (1, 4), (1, 7), "123" );
    test_token_w_position!(tokens[3], TType::NL, (1, 7), (1, 7), "" );
    test_token_w_position!(tokens[4], TType::Name, (3, 0), (3, 5), "match" );
    test_token_w_position!(tokens[5], TType::Name, (3, 6), (3, 7), "x" );
    test_token_w_position!(tokens[6], TType::Op, (3, 7), (3, 8), ":" );
    test_token_w_position!(tokens[7], TType::NL, (3, 8), (3, 8), "" );
    test_token_w_position!(tokens[8], TType::Indent, (4, 0), (4, 0), "" );
    test_token_w_position!(tokens[9], TType::Name, (4, 4), (4, 8), "case" );
    test_token_w_position!(tokens[10], TType::Name, (4, 9), (4, 10), "x" );
    test_token_w_position!(tokens[11], TType::Name, (4, 11), (4, 13), "if" );
    test_token_w_position!(tokens[12], TType::Name, (4, 14), (4, 15), "x" );
    test_token_w_position!(tokens[13], TType::Op, (4, 16), (4, 17), ">" );
    test_token_w_position!(tokens[14], TType::Number, (4, 18), (4, 19), "1" );
    test_token_w_position!(tokens[15], TType::Op, (4, 19), (4, 20), ":" );
    test_token_w_position!(tokens[16], TType::NL, (4, 20), (4, 20), "" );
    test_token_w_position!(tokens[17], TType::Indent, (5, 0), (5, 0), "" );
    test_token_w_position!(tokens[18], TType::Name, (5, 8), (5, 13), "print" );
    test_token_w_position!(tokens[19], TType::Op, (5, 13), (5, 14), "(" );
    test_token_w_position!(tokens[20], TType::String, (5, 14), (5, 27), "\"Hello World\"" );
    test_token_w_position!(tokens[21], TType::Op, (5, 27), (5, 28), ")" );
    test_token_w_position!(tokens[22], TType::NL, (5, 28), (5, 28), "" );
    test_token_w_position!(tokens[23], TType::Dedent, (6, 0), (6, 0), "" );
    test_token_w_position!(tokens[24], TType::Name, (6, 4), (6, 8), "case" );
    test_token_w_position!(tokens[25], TType::Name, (6, 9), (6, 10), "x" );
    test_token_w_position!(tokens[26], TType::Name, (6, 11), (6, 13), "if" );
    test_token_w_position!(tokens[27], TType::Name, (6, 14), (6, 15), "x" );
    test_token_w_position!(tokens[28], TType::Op, (6, 16), (6, 17), "<" );
    test_token_w_position!(tokens[29], TType::Number, (6, 18), (6, 19), "0" );
    test_token_w_position!(tokens[30], TType::Op, (6, 19), (6, 20), ":" );
    test_token_w_position!(tokens[31], TType::NL, (6, 20), (6, 20), "" );
    test_token_w_position!(tokens[32], TType::Indent, (7, 0), (7, 0), "" );
    test_token_w_position!(tokens[33], TType::Name, (7, 8), (7, 13), "print" );
    test_token_w_position!(tokens[34], TType::Op, (7, 13), (7, 14), "(" );
    test_token_w_position!(tokens[35], TType::String, (7, 14), (7, 24), "\"Good bye\"" );
    test_token_w_position!(tokens[36], TType::Op, (7, 24), (7, 25), ")" );
    test_token_w_position!(tokens[37], TType::NL, (7, 25), (7, 25), "" );
    test_token_w_position!(tokens[38], TType::Dedent, (9, 0), (9, 0), "" );
    test_token_w_position!(tokens[39], TType::Dedent, (9, 0), (9, 0), "" );
    test_token_w_position!(tokens[40], TType::Name, (9, 0), (9, 1), "x" );
    test_token_w_position!(tokens[41], TType::Op, (9, 2), (9, 3), "=" );
    test_token_w_position!(tokens[42], TType::String, (9, 4), (9, 15), "\"new block\"" );
    test_token_w_position!(tokens[43], TType::NL, (9, 15), (9, 15), "" );
    test_token_w_position!(tokens[44], TType::Name, (10, 0), (10, 5), "match" );
    test_token_w_position!(tokens[45], TType::Op, (10, 6), (10, 7), "=" );
    test_token_w_position!(tokens[46], TType::String, (10, 8), (10, 21), "\"softkeyword\"" );
    test_token_w_position!(tokens[47], TType::NL, (10, 21), (10, 21), "" );

}


#[test]
fn test_and_profile_tokenizing_stdlib_astpy(){

    return; //Disable for now!
    let tokens = Tokenizer::tokenize_file("PyLib/ast.py",
                                          TConfig{skip_encoding: true,
                                              skip_endmarker: false})
        .expect("tokens");




}
