


#[cfg(test)]
mod test {
    use crate::lexer::tokenizer::{TConfig, Tokenizer};
    use crate::tokens::{TType, Position};

    macro_rules! test_token_w_position{
        ($token:expr, $ttype:expr, $start:expr, $end:expr, $content:expr)=>{

            assert_eq!($token.text, $content, "Testing for text/content with {:?} != {:?} for a/{:?} e/{:?}", $token.text, $content, $token.r#type, $ttype);
            assert_eq!($token.r#type, $ttype, "Testing for type with {:?} {:?} != {:?}", $token.text, $token.r#type, $ttype);
            assert_eq!($token.start, Position::t($start), "Testing for start with {:?} % {:?} : {:?} != {:?}", $token.text, $token.r#type, $token.start, $start);
            assert_eq!($token.end, Position::t($end), "Testing for end with {:?} % {:?} : {:?} != {:?}", $token.text, $token.r#type, $token.end, $end);

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


        let mut tokenizer = Tokenizer::new(TConfig{skip_endmarker: false, skip_encoding: false });
        let tokens = tokenizer.process_file("test_fixtures/test_float.py").expect("tokens");

        for token in tokens.iter() {
            println!("{:?}", token);
        }

        test_token_w_position!(tokens[0], TType::Encoding, (0, 0), (0, 0), "utf-8" );
        test_token_w_position!(tokens[1], TType::Name, (0, 1), (1, 1), "x" );
        test_token_w_position!(tokens[2], TType::Op, (2, 1), (3, 1), "=" );
        test_token_w_position!(tokens[3], TType::Number, (4, 1), (11, 1), "3.14159" );
        test_token_w_position!(tokens[4], TType::Newline, (11, 1), (12, 1), "\n" );
        test_token_w_position!(tokens[5], TType::Name, (0, 2), (1, 2), "x" );
        test_token_w_position!(tokens[6], TType::Op, (2, 2), (3, 2), "=" );
        test_token_w_position!(tokens[7], TType::Number, (4, 2), (11, 2), "314159." );
        test_token_w_position!(tokens[8], TType::Newline, (11, 2), (12, 2), "\n" );
        test_token_w_position!(tokens[9], TType::Name, (0, 3), (1, 3), "x" );
        test_token_w_position!(tokens[10], TType::Op, (2, 3), (3, 3), "=" );
        test_token_w_position!(tokens[11], TType::Number, (4, 3), (11, 3), ".314159" );
        test_token_w_position!(tokens[12], TType::Newline, (11, 3), (12, 3), "\n" );
        test_token_w_position!(tokens[13], TType::Name, (0, 4), (1, 4), "x" );
        test_token_w_position!(tokens[14], TType::Op, (2, 4), (3, 4), "=" );
        test_token_w_position!(tokens[15], TType::Number, (4, 4), (9, 4), "3e141" );
        test_token_w_position!(tokens[16], TType::Newline, (9, 4), (10, 4), "\n" );
        test_token_w_position!(tokens[17], TType::Name, (0, 5), (1, 5), "x" );
        test_token_w_position!(tokens[18], TType::Op, (2, 5), (3, 5), "=" );
        test_token_w_position!(tokens[19], TType::Number, (4, 5), (9, 5), "3E123" );
        test_token_w_position!(tokens[20], TType::Newline, (9, 5), (10, 5), "\n" );
        test_token_w_position!(tokens[21], TType::Name, (0, 6), (1, 6), "x" );
        test_token_w_position!(tokens[22], TType::Op, (1, 6), (2, 6), "+" );
        test_token_w_position!(tokens[23], TType::Name, (2, 6), (3, 6), "y" );
        test_token_w_position!(tokens[24], TType::Op, (4, 6), (5, 6), "=" );
        test_token_w_position!(tokens[25], TType::Number, (6, 6), (13, 6), "3e-1230" );
        test_token_w_position!(tokens[26], TType::Newline, (13, 6), (14, 6), "\n" );
        test_token_w_position!(tokens[27], TType::Name, (0, 7), (1, 7), "x" );
        test_token_w_position!(tokens[28], TType::Op, (2, 7), (3, 7), "=" );
        test_token_w_position!(tokens[29], TType::Number, (4, 7), (12, 7), "3.14e159" );
        test_token_w_position!(tokens[30], TType::Newline, (12, 7), (13, 7), "\n" );
        test_token_w_position!(tokens[31], TType::EndMarker, (0, 8), (0, 8), "" );

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
    fn test_additive() {
        let mut tokenizer = Tokenizer::new(TConfig{skip_encoding: false, skip_endmarker: false});
        let tokens = tokenizer.process_file("test_fixtures/test_additive.py").expect("tokens");

        test_token_w_position!(tokens[0], TType::Encoding, (0, 0), (0, 0), "utf-8" );
        test_token_w_position!(tokens[1], TType::Name, (0, 1), (1, 1), "x" );
        test_token_w_position!(tokens[2], TType::Op, (2, 1), (3, 1), "=" );
        test_token_w_position!(tokens[3], TType::Number, (4, 1), (5, 1), "1" );
        test_token_w_position!(tokens[4], TType::Op, (6, 1), (7, 1), "-" );
        test_token_w_position!(tokens[5], TType::Name, (8, 1), (9, 1), "y" );
        test_token_w_position!(tokens[6], TType::Op, (10, 1), (11, 1), "+" );
        test_token_w_position!(tokens[7], TType::Number, (12, 1), (14, 1), "15" );
        test_token_w_position!(tokens[8], TType::Op, (15, 1), (16, 1), "-" );
        test_token_w_position!(tokens[9], TType::Number, (17, 1), (18, 1), "1" );
        test_token_w_position!(tokens[10], TType::Op, (19, 1), (20, 1), "+" );
        test_token_w_position!(tokens[11], TType::Number, (21, 1), (26, 1), "0x124" );
        test_token_w_position!(tokens[12], TType::Op, (27, 1), (28, 1), "+" );
        test_token_w_position!(tokens[13], TType::Name, (29, 1), (30, 1), "z" );
        test_token_w_position!(tokens[14], TType::Op, (31, 1), (32, 1), "+" );
        test_token_w_position!(tokens[15], TType::Name, (33, 1), (34, 1), "a" );
        test_token_w_position!(tokens[16], TType::Op, (34, 1), (35, 1), "[" );
        test_token_w_position!(tokens[17], TType::Number, (35, 1), (36, 1), "5" );
        test_token_w_position!(tokens[18], TType::Op, (36, 1), (37, 1), "]" );
        test_token_w_position!(tokens[19], TType::Newline, (37, 1), (38, 1), "\n" );
        test_token_w_position!(tokens[20], TType::EndMarker, (0, 2), (0, 2), "" );

    }

    #[test]
    fn test_async_as_name() {
        let mut tokenizer = Tokenizer::new(TConfig{skip_encoding: false, skip_endmarker: false});
        let tokens = tokenizer.process_file("test_fixtures/test_async.py").expect("tokens");

        test_token_w_position!(tokens[0], TType::Encoding, (0, 0), (0, 0), "utf-8" );
        test_token_w_position!(tokens[1], TType::Name, (0, 1), (5, 1), "async" );
        test_token_w_position!(tokens[2], TType::Op, (6, 1), (7, 1), "=" );
        test_token_w_position!(tokens[3], TType::Number, (8, 1), (9, 1), "1" );
        test_token_w_position!(tokens[4], TType::Newline, (9, 1), (10, 1), "\n" );
        test_token_w_position!(tokens[5], TType::Name, (0, 2), (1, 2), "a" );
        test_token_w_position!(tokens[6], TType::Op, (2, 2), (3, 2), "=" );
        test_token_w_position!(tokens[7], TType::Op, (4, 2), (5, 2), "(" );
        test_token_w_position!(tokens[8], TType::Name, (5, 2), (10, 2), "async" );
        test_token_w_position!(tokens[9], TType::Op, (11, 2), (12, 2), "=" );
        test_token_w_position!(tokens[10], TType::Number, (13, 2), (14, 2), "1" );
        test_token_w_position!(tokens[11], TType::Op, (14, 2), (15, 2), ")" );
        test_token_w_position!(tokens[12], TType::Newline, (15, 2), (16, 2), "\n" );
        test_token_w_position!(tokens[13], TType::Name, (0, 3), (5, 3), "async" );
        test_token_w_position!(tokens[14], TType::Op, (5, 3), (6, 3), "(" );
        test_token_w_position!(tokens[15], TType::Op, (6, 3), (7, 3), ")" );
        test_token_w_position!(tokens[16], TType::Newline, (7, 3), (8, 3), "\n" );
        test_token_w_position!(tokens[17], TType::Name, (0, 4), (5, 4), "class" );
        test_token_w_position!(tokens[18], TType::Name, (6, 4), (11, 4), "async" );
        test_token_w_position!(tokens[19], TType::Op, (11, 4), (12, 4), "(" );
        test_token_w_position!(tokens[20], TType::Name, (12, 4), (15, 4), "Bar" );
        test_token_w_position!(tokens[21], TType::Op, (15, 4), (16, 4), ")" );
        test_token_w_position!(tokens[22], TType::Op, (16, 4), (17, 4), ":" );
        test_token_w_position!(tokens[23], TType::Name, (17, 4), (21, 4), "pass" );
        test_token_w_position!(tokens[24], TType::Newline, (21, 4), (22, 4), "\n" );
        test_token_w_position!(tokens[25], TType::Name, (0, 5), (5, 5), "class" );
        test_token_w_position!(tokens[26], TType::Name, (6, 5), (11, 5), "async" );
        test_token_w_position!(tokens[27], TType::Op, (11, 5), (12, 5), ":" );
        test_token_w_position!(tokens[28], TType::Name, (12, 5), (16, 5), "pass" );
        test_token_w_position!(tokens[29], TType::Newline, (16, 5), (17, 5), "\n" );
        test_token_w_position!(tokens[30], TType::Name, (0, 6), (5, 6), "await" );
        test_token_w_position!(tokens[31], TType::Op, (6, 6), (7, 6), "=" );
        test_token_w_position!(tokens[32], TType::Number, (8, 6), (9, 6), "1" );
        test_token_w_position!(tokens[33], TType::Newline, (9, 6), (10, 6), "\n" );
        test_token_w_position!(tokens[34], TType::Name, (0, 7), (3, 7), "foo" );
        test_token_w_position!(tokens[35], TType::Op, (3, 7), (4, 7), "." );
        test_token_w_position!(tokens[36], TType::Name, (4, 7), (9, 7), "async" );
        test_token_w_position!(tokens[37], TType::Newline, (9, 7), (10, 7), "\n" );
        test_token_w_position!(tokens[38], TType::Name, (0, 8), (5, 8), "async" );
        test_token_w_position!(tokens[39], TType::Name, (6, 8), (9, 8), "for" );
        test_token_w_position!(tokens[40], TType::Name, (10, 8), (11, 8), "a" );
        test_token_w_position!(tokens[41], TType::Name, (12, 8), (14, 8), "in" );
        test_token_w_position!(tokens[42], TType::Name, (15, 8), (16, 8), "b" );
        test_token_w_position!(tokens[43], TType::Op, (16, 8), (17, 8), ":" );
        test_token_w_position!(tokens[44], TType::Name, (18, 8), (22, 8), "pass" );
        test_token_w_position!(tokens[45], TType::Newline, (22, 8), (23, 8), "\n" );
        test_token_w_position!(tokens[46], TType::Name, (0, 9), (5, 9), "async" );
        test_token_w_position!(tokens[47], TType::Name, (6, 9), (10, 9), "with" );
        test_token_w_position!(tokens[48], TType::Name, (11, 9), (12, 9), "a" );
        test_token_w_position!(tokens[49], TType::Name, (13, 9), (15, 9), "as" );
        test_token_w_position!(tokens[50], TType::Name, (16, 9), (17, 9), "b" );
        test_token_w_position!(tokens[51], TType::Op, (17, 9), (18, 9), ":" );
        test_token_w_position!(tokens[52], TType::Name, (19, 9), (23, 9), "pass" );
        test_token_w_position!(tokens[53], TType::Newline, (23, 9), (24, 9), "\n" );
        test_token_w_position!(tokens[54], TType::Name, (0, 10), (5, 10), "async" );
        test_token_w_position!(tokens[55], TType::Op, (5, 10), (6, 10), "." );
        test_token_w_position!(tokens[56], TType::Name, (6, 10), (9, 10), "foo" );
        test_token_w_position!(tokens[57], TType::Newline, (9, 10), (10, 10), "\n" );
        test_token_w_position!(tokens[58], TType::Name, (0, 11), (5, 11), "async" );
        test_token_w_position!(tokens[59], TType::Newline, (5, 11), (6, 11), "\n" );
        test_token_w_position!(tokens[60], TType::EndMarker, (0, 12), (0, 12), "" );
    }

    #[test]
    fn test_comparison() {

        let mut tokenizer = Tokenizer::new(TConfig{skip_encoding: false, skip_endmarker: false });
        let tokens = tokenizer.process_file("test_fixtures/test_comparison.py").expect("tokens");



        test_token_w_position!(tokens[0], TType::Encoding, (0, 0), (0, 0), "utf-8" );
        test_token_w_position!(tokens[1], TType::Name, (0, 1), (2, 1), "if" );
        test_token_w_position!(tokens[2], TType::Number, (3, 1), (4, 1), "1" );
        test_token_w_position!(tokens[3], TType::Op, (5, 1), (6, 1), "<" );
        test_token_w_position!(tokens[4], TType::Number, (7, 1), (8, 1), "1" );
        test_token_w_position!(tokens[5], TType::Op, (9, 1), (10, 1), ">" );
        test_token_w_position!(tokens[6], TType::Number, (11, 1), (12, 1), "1" );
        test_token_w_position!(tokens[7], TType::Op, (13, 1), (15, 1), "==" );
        test_token_w_position!(tokens[8], TType::Number, (16, 1), (17, 1), "1" );
        test_token_w_position!(tokens[9], TType::Op, (18, 1), (20, 1), ">=" );
        test_token_w_position!(tokens[10], TType::Number, (21, 1), (22, 1), "5" );
        test_token_w_position!(tokens[11], TType::Op, (23, 1), (25, 1), "<=" );
        test_token_w_position!(tokens[12], TType::Number, (26, 1), (30, 1), "0x15" );
        test_token_w_position!(tokens[13], TType::Op, (31, 1), (33, 1), "<=" );
        test_token_w_position!(tokens[14], TType::Number, (34, 1), (38, 1), "0x12" );
        test_token_w_position!(tokens[15], TType::Op, (39, 1), (41, 1), "!=" );
        test_token_w_position!(tokens[16], TType::Number, (42, 1), (43, 1), "1" );
        test_token_w_position!(tokens[17], TType::Name, (44, 1), (47, 1), "and" );
        test_token_w_position!(tokens[18], TType::Number, (48, 1), (49, 1), "5" );
        test_token_w_position!(tokens[19], TType::Name, (50, 1), (52, 1), "in" );
        test_token_w_position!(tokens[20], TType::Number, (53, 1), (54, 1), "1" );
        test_token_w_position!(tokens[21], TType::Name, (55, 1), (58, 1), "not" );
        test_token_w_position!(tokens[22], TType::Name, (59, 1), (61, 1), "in" );
        test_token_w_position!(tokens[23], TType::Number, (62, 1), (63, 1), "1" );
        test_token_w_position!(tokens[24], TType::Name, (64, 1), (66, 1), "is" );
        test_token_w_position!(tokens[25], TType::Number, (67, 1), (68, 1), "1" );
        test_token_w_position!(tokens[26], TType::Name, (69, 1), (71, 1), "or" );
        test_token_w_position!(tokens[27], TType::Number, (72, 1), (73, 1), "5" );
        test_token_w_position!(tokens[28], TType::Name, (74, 1), (76, 1), "is" );
        test_token_w_position!(tokens[29], TType::Name, (77, 1), (80, 1), "not" );
        test_token_w_position!(tokens[30], TType::Number, (81, 1), (82, 1), "1" );
        test_token_w_position!(tokens[31], TType::Op, (82, 1), (83, 1), ":" );
        test_token_w_position!(tokens[32], TType::Newline, (83, 1), (84, 1), "\n" );
        test_token_w_position!(tokens[33], TType::Indent, (0, 2), (4, 2), "    " );
        test_token_w_position!(tokens[34], TType::Name, (4, 2), (8, 2), "pass" );
        test_token_w_position!(tokens[35], TType::Newline, (8, 2), (9, 2), "\n" );
        test_token_w_position!(tokens[36], TType::Dedent, (0, 3), (0, 3), "" );
        test_token_w_position!(tokens[37], TType::EndMarker, (0, 3), (0, 3), "" );
    }

    #[test]
    fn test_int() {

        let mut tokenizer = Tokenizer::new(TConfig{skip_encoding: false, skip_endmarker: false});
        let tokens = tokenizer.process_file("test_fixtures/test_int.py").expect("tokens");

        test_token_w_position!(tokens[0], TType::Encoding, (0, 0), (0, 0), "utf-8" );
        test_token_w_position!(tokens[1], TType::Number, (0, 1), (4, 1), "0xff" );
        test_token_w_position!(tokens[2], TType::Op, (5, 1), (7, 1), "<=" );
        test_token_w_position!(tokens[3], TType::Number, (8, 1), (11, 1), "255" );
        test_token_w_position!(tokens[4], TType::Newline, (11, 1), (12, 1), "\n" );
        test_token_w_position!(tokens[5], TType::Number, (0, 2), (4, 2), "0b10" );
        test_token_w_position!(tokens[6], TType::Op, (5, 2), (7, 2), "<=" );
        test_token_w_position!(tokens[7], TType::Number, (8, 2), (11, 2), "255" );
        test_token_w_position!(tokens[8], TType::Newline, (11, 2), (12, 2), "\n" );
        test_token_w_position!(tokens[9], TType::Number, (0, 3), (5, 3), "0o123" );
        test_token_w_position!(tokens[10], TType::Op, (6, 3), (8, 3), "<=" );
        test_token_w_position!(tokens[11], TType::Number, (9, 3), (14, 3), "0O123" );
        test_token_w_position!(tokens[12], TType::Newline, (14, 3), (15, 3), "\n" );
        test_token_w_position!(tokens[13], TType::Number, (0, 4), (7, 4), "1234567" );
        test_token_w_position!(tokens[14], TType::Op, (8, 4), (9, 4), ">" );
        test_token_w_position!(tokens[15], TType::Op, (10, 4), (11, 4), "~" );
        test_token_w_position!(tokens[16], TType::Number, (11, 4), (15, 4), "0x15" );
        test_token_w_position!(tokens[17], TType::Newline, (15, 4), (16, 4), "\n" );
        test_token_w_position!(tokens[18], TType::Number, (0, 5), (7, 5), "2134568" );
        test_token_w_position!(tokens[19], TType::Op, (8, 5), (10, 5), "!=" );
        test_token_w_position!(tokens[20], TType::Number, (11, 5), (18, 5), "1231515" );
        test_token_w_position!(tokens[21], TType::Newline, (18, 5), (19, 5), "\n" );
        test_token_w_position!(tokens[22], TType::Op, (0, 6), (1, 6), "(" );
        test_token_w_position!(tokens[23], TType::Op, (1, 6), (2, 6), "-" );
        test_token_w_position!(tokens[24], TType::Number, (2, 6), (8, 6), "124561" );
        test_token_w_position!(tokens[25], TType::Op, (8, 6), (9, 6), "-" );
        test_token_w_position!(tokens[26], TType::Number, (9, 6), (10, 6), "1" );
        test_token_w_position!(tokens[27], TType::Op, (10, 6), (11, 6), ")" );
        test_token_w_position!(tokens[28], TType::Op, (12, 6), (13, 6), "&" );
        test_token_w_position!(tokens[29], TType::Number, (14, 6), (23, 6), "200000000" );
        test_token_w_position!(tokens[30], TType::Newline, (23, 6), (24, 6), "\n" );
        test_token_w_position!(tokens[31], TType::Number, (0, 7), (10, 7), "0xdeadbeef" );
        test_token_w_position!(tokens[32], TType::Op, (11, 7), (13, 7), "!=" );
        test_token_w_position!(tokens[33], TType::Op, (14, 7), (15, 7), "-" );
        test_token_w_position!(tokens[34], TType::Number, (15, 7), (16, 7), "1" );
        test_token_w_position!(tokens[35], TType::Newline, (16, 7), (17, 7), "\n" );
        test_token_w_position!(tokens[36], TType::Number, (0, 8), (10, 8), "0xdeadc0de" );
        test_token_w_position!(tokens[37], TType::Op, (11, 8), (12, 8), "&" );
        test_token_w_position!(tokens[38], TType::Number, (13, 8), (18, 8), "12345" );
        test_token_w_position!(tokens[39], TType::Newline, (18, 8), (19, 8), "\n" );
        test_token_w_position!(tokens[40], TType::Number, (0, 9), (4, 9), "0xFF" );
        test_token_w_position!(tokens[41], TType::Op, (5, 9), (6, 9), "&" );
        test_token_w_position!(tokens[42], TType::Number, (7, 9), (11, 9), "0x15" );
        test_token_w_position!(tokens[43], TType::Op, (12, 9), (13, 9), "|" );
        test_token_w_position!(tokens[44], TType::Number, (14, 9), (18, 9), "1234" );
        // test_token_w_position!(tokens[45], TType::Newline, (18, 9), (19, 9), "" );
        test_token_w_position!(tokens[46], TType::EndMarker, (0, 10), (0, 10), "" );

    }

    #[test]
    fn test_long() {

        let mut tokenizer = Tokenizer::new(TConfig{skip_encoding: false, skip_endmarker: false});
        let tokens = tokenizer.process_file("test_fixtures/test_long.py").expect("tokens");

        test_token_w_position!(tokens[0], TType::Encoding, (0, 0), (0, 0), "utf-8" );
        test_token_w_position!(tokens[1], TType::Name, (0, 1), (1, 1), "x" );
        test_token_w_position!(tokens[2], TType::Op, (2, 1), (3, 1), "=" );
        test_token_w_position!(tokens[3], TType::Number, (4, 1), (5, 1), "0" );
        test_token_w_position!(tokens[4], TType::Newline, (5, 1), (6, 1), "\n" );
        test_token_w_position!(tokens[5], TType::Name, (0, 2), (1, 2), "x" );
        test_token_w_position!(tokens[6], TType::Op, (2, 2), (3, 2), "=" );
        test_token_w_position!(tokens[7], TType::Number, (4, 2), (17, 2), "0xfffffffffff" );
        test_token_w_position!(tokens[8], TType::Newline, (17, 2), (18, 2), "\n" );
        test_token_w_position!(tokens[9], TType::Name, (0, 3), (1, 3), "x" );
        test_token_w_position!(tokens[10], TType::Op, (2, 3), (3, 3), "=" );
        test_token_w_position!(tokens[11], TType::Number, (4, 3), (25, 3), "123141242151251616110" );
        test_token_w_position!(tokens[12], TType::Newline, (25, 3), (26, 3), "\n" );
        test_token_w_position!(tokens[13], TType::Name, (0, 4), (1, 4), "x" );
        test_token_w_position!(tokens[14], TType::Op, (2, 4), (3, 4), "=" );
        test_token_w_position!(tokens[15], TType::Op, (4, 4), (5, 4), "-" );
        test_token_w_position!(tokens[16], TType::Number, (5, 4), (22, 4), "15921590215012591" );
        test_token_w_position!(tokens[17], TType::Newline, (22, 4), (23, 4), "\n" );
        test_token_w_position!(tokens[18], TType::EndMarker, (0, 5), (0, 5), "" );
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

        let mut tokenizer = Tokenizer::new(TConfig{skip_encoding: true, skip_endmarker: true});
        let tokens = tokenizer.process_file("test_fixtures/test_selector.py").expect("tokens");

        for (lineno, token) in tokens.iter().enumerate() {
            println!("{}: {:?}", lineno, token);
        }

        test_token!(tokens[0], TType::Name, "import");
        test_token!(tokens[1], TType::Name, "sys");
        test_token!(tokens[2], TType::Op, ",");
        test_token!(tokens[3], TType::Name, "time");

        test_token!(tokens[11], TType::String, "'time'");

        assert_eq!(tokens.len(), 18);
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

        let mut tokenizer = Tokenizer::new(TConfig{skip_encoding: false, skip_endmarker: false});
        let tokens = tokenizer.process_file("test_fixtures/test_basic_operators.py").expect("tokens");

        test_token_w_position!(tokens[0], TType::Encoding, (0, 0), (0, 0), "utf-8" );
        test_token_w_position!(tokens[1], TType::Number, (0, 1), (1, 1), "1" );
        test_token_w_position!(tokens[2], TType::Op, (2, 1), (3, 1), "+" );
        test_token_w_position!(tokens[3], TType::Number, (4, 1), (5, 1), "1" );
        //TODO test_token_w_position!(tokens[4], TType::Newline, (5, 1), (6, 1), "" );
        test_token_w_position!(tokens[5], TType::EndMarker, (0, 2), (0, 2), "" );

    }

    #[test]
    fn test_valid_literals() {
        let VALID_UNDERSCORE_LITERALS: Vec<&str> = vec![
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


        for value in VALID_UNDERSCORE_LITERALS {
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


        test_token_w_position!(tokens[0], TType::String, (0, 2), (3, 4), str1);
        // test_token_w_position!(tokens[1], TType::Newline, (3, 4), (3, 4), "\n" );
        test_token_w_position!(tokens[2], TType::String, (0, 6), (21, 7), str2 );
        // test_token_w_position!(tokens[3], TType::Newline, (21, 7), (21, 7), "\n" );

        // test_token_w_position!(tokens[3], TType::String, (0,2), (3,4), str1);
        // test_token_w_position!(tokens[7], TType::String, (0,6), (21,7), str2);



    }

    #[test]
    fn test_correct_newlines() {
        let mut tokenizer = Tokenizer::new(TConfig::default());
        let tokens = tokenizer.process_file("test_fixtures/test_correct_newlines.py").expect("tokens");

        for (lno, token) in tokens.iter().enumerate() {
            println!("{}: {:?}", lno, token);
        }

    }

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
        let mut tokenizer = Tokenizer::new(TConfig::default());
        let tokens = tokenizer.process_file("test_fixtures/basic_class.py").expect("tokens");

        for (lno, token) in tokens.iter().enumerate() {
            println!("{}: {:?}", lno, token);
        }

        test_token_w_position!(tokens[0], TType::Encoding, (0, 0), (0, 0), "utf-8" );
        test_token_w_position!(tokens[1], TType::Name, (0, 1), (5, 1), "class" );
        test_token_w_position!(tokens[2], TType::Name, (6, 1), (11, 1), "Basic" );
        test_token_w_position!(tokens[3], TType::Op, (11, 1), (12, 1), ":" );
        test_token_w_position!(tokens[4], TType::NL, (12, 1), (13, 1), "\n" );
        test_token_w_position!(tokens[5], TType::Indent, (0, 2), (4, 2), "    " );
        // test_token_w_position!(tokens[6], TType::String, (4, 2), (7, 4), """"
        //     A basic class definition
        //     """" );
        test_token_w_position!(tokens[7], TType::NL, (7, 4), (8, 4), "\n" );
        test_token_w_position!(tokens[8], TType::Name, (4, 5), (7, 5), "def" );
        test_token_w_position!(tokens[9], TType::Name, (8, 5), (16, 5), "__init__" );
        test_token_w_position!(tokens[10], TType::Op, (16, 5), (17, 5), "(" );
        test_token_w_position!(tokens[11], TType::Name, (17, 5), (21, 5), "self" );
        test_token_w_position!(tokens[12], TType::Op, (21, 5), (22, 5), ")" );
        test_token_w_position!(tokens[13], TType::Op, (22, 5), (23, 5), ":" );
        // test_token_w_position!(tokens[14], TType::Newline, (23, 5), (24, 5), "\n" );
        test_token_w_position!(tokens[15], TType::Indent, (0, 6), (8, 6), "        " );
        // test_token_w_position!(tokens[16], TType::String, (8, 6), (11, 8), """"
        //         A basic init
        //         """" );
        // test_token_w_position!(tokens[17], TType::Newline, (11, 8), (12, 8), "\n" );
        test_token_w_position!(tokens[18], TType::Name, (8, 9), (12, 9), "self" );
        test_token_w_position!(tokens[19], TType::Op, (12, 9), (13, 9), "." );
        test_token_w_position!(tokens[20], TType::Name, (13, 9), (14, 9), "c" );
        test_token_w_position!(tokens[21], TType::Op, (15, 9), (16, 9), "=" );
        test_token_w_position!(tokens[22], TType::Number, (17, 9), (18, 9), "0" );
        // test_token_w_position!(tokens[23], TType::Newline, (18, 9), (19, 9), "\n" );
        // test_token_w_position!(tokens[24], TType::NL, (0, 10), (1, 10), "\n" );
        test_token_w_position!(tokens[25], TType::Dedent, (4, 11), (4, 11), "" );
        test_token_w_position!(tokens[26], TType::Name, (4, 11), (7, 11), "def" );
        test_token_w_position!(tokens[27], TType::Name, (8, 11), (11, 11), "add" );
        test_token_w_position!(tokens[28], TType::Op, (11, 11), (12, 11), "(" );
        test_token_w_position!(tokens[29], TType::Name, (12, 11), (16, 11), "self" );
        test_token_w_position!(tokens[30], TType::Op, (16, 11), (17, 11), "," );
        test_token_w_position!(tokens[31], TType::Name, (18, 11), (19, 11), "a" );
        test_token_w_position!(tokens[32], TType::Op, (19, 11), (20, 11), "," );
        test_token_w_position!(tokens[33], TType::Name, (21, 11), (22, 11), "b" );
        test_token_w_position!(tokens[34], TType::Op, (22, 11), (23, 11), ")" );
        test_token_w_position!(tokens[35], TType::Op, (23, 11), (24, 11), ":" );
        // test_token_w_position!(tokens[36], TType::Newline, (24, 11), (25, 11), "\n" );
        test_token_w_position!(tokens[37], TType::Indent, (0, 12), (8, 12), "        " );
        test_token_w_position!(tokens[38], TType::Name, (8, 12), (12, 12), "self" );
        test_token_w_position!(tokens[39], TType::Op, (12, 12), (13, 12), "." );
        test_token_w_position!(tokens[40], TType::Name, (13, 12), (14, 12), "c" );
        test_token_w_position!(tokens[41], TType::Op, (15, 12), (16, 12), "=" );
        test_token_w_position!(tokens[42], TType::Name, (17, 12), (18, 12), "a" );
        test_token_w_position!(tokens[43], TType::Op, (19, 12), (20, 12), "+" );
        test_token_w_position!(tokens[44], TType::Name, (21, 12), (22, 12), "b" );
        // test_token_w_position!(tokens[45], TType::Newline, (22, 12), (23, 12), "\n" );
        test_token_w_position!(tokens[46], TType::NL, (0, 13), (1, 13), "\n" );
        test_token_w_position!(tokens[47], TType::Dedent, (4, 14), (4, 14), "" );
        test_token_w_position!(tokens[48], TType::Name, (4, 14), (7, 14), "def" );
        test_token_w_position!(tokens[49], TType::Name, (8, 14), (11, 14), "get" );
        test_token_w_position!(tokens[50], TType::Op, (11, 14), (12, 14), "(" );
        test_token_w_position!(tokens[51], TType::Name, (12, 14), (16, 14), "self" );
        test_token_w_position!(tokens[52], TType::Op, (16, 14), (17, 14), ")" );
        test_token_w_position!(tokens[53], TType::Op, (17, 14), (18, 14), ":" );
        // test_token_w_position!(tokens[54], TType::Newline, (18, 14), (19, 14), "\n" );
        test_token_w_position!(tokens[55], TType::Indent, (0, 15), (8, 15), "        " );
        test_token_w_position!(tokens[56], TType::Name, (8, 15), (14, 15), "return" );
        test_token_w_position!(tokens[57], TType::Name, (15, 15), (19, 15), "self" );
        test_token_w_position!(tokens[58], TType::Op, (19, 15), (20, 15), "." );
        test_token_w_position!(tokens[59], TType::Name, (20, 15), (21, 15), "c" );
        // test_token_w_position!(tokens[60], TType::Newline, (21, 15), (22, 15), "\n" );
        test_token_w_position!(tokens[61], TType::Dedent, (0, 16), (0, 16), "" );
        test_token_w_position!(tokens[62], TType::Dedent, (0, 16), (0, 16), "" );
        //test_token_w_position!(tokens[63], TType::EndMarker, (0, 16), (0, 16), "" );
    }

    #[test]
    fn test_basic_indent() {

        let tokens = Tokenizer::tokenize_file(
            "test_fixtures/basic_indent.py",
            TConfig{ skip_encoding: false, skip_endmarker: true}).expect("tokens");

        for (lno, token) in tokens.iter().enumerate() {
            println!("{}: {:?}", lno, token);
        }

        // test_token_w_position!(tokens[0], TType::Encoding, (0, 0), (0, 0), "utf-8" );
        test_token_w_position!(tokens[1], TType::Name, (0, 1), (3, 1), "def" );
        test_token_w_position!(tokens[2], TType::Name, (4, 1), (8, 1), "test" );
        test_token_w_position!(tokens[3], TType::Op, (8, 1), (9, 1), "(" );
        test_token_w_position!(tokens[4], TType::Op, (9, 1), (10, 1), ")" );
        test_token_w_position!(tokens[5], TType::Op, (10, 1), (11, 1), ":" );
        // test_token_w_position!(tokens[6], TType::Newline, (11, 1), (12, 1), "\n" );
        test_token_w_position!(tokens[7], TType::Indent, (0, 2), (4, 2), "    " );
        test_token_w_position!(tokens[8], TType::Name, (4, 2), (9, 2), "print" );
        test_token_w_position!(tokens[9], TType::Op, (9, 2), (10, 2), "(" );
        test_token_w_position!(tokens[10], TType::String, (10, 2), (23, 2), "\"Hello world\"" );
        test_token_w_position!(tokens[11], TType::Op, (23, 2), (24, 2), ")" );
        // test_token_w_position!(tokens[12], TType::Newline, (24, 2), (25, 2), "\n" );
        // test_token_w_position!(tokens[13], TType::NL, (0, 3), (1, 3), "\n" );
        test_token_w_position!(tokens[14], TType::Dedent, (0, 4), (0, 4), "" );
        test_token_w_position!(tokens[15], TType::Name, (0, 4), (3, 4), "def" );
        test_token_w_position!(tokens[16], TType::Name, (4, 4), (7, 4), "foo" );
        test_token_w_position!(tokens[17], TType::Op, (7, 4), (8, 4), "(" );
        test_token_w_position!(tokens[18], TType::Op, (8, 4), (9, 4), ")" );
        test_token_w_position!(tokens[19], TType::Op, (9, 4), (10, 4), ":" );
        // test_token_w_position!(tokens[20], TType::Newline, (10, 4), (11, 4), "\n" );
        test_token_w_position!(tokens[21], TType::Indent, (0, 5), (4, 5), "    " );
        test_token_w_position!(tokens[22], TType::Name, (4, 5), (9, 5), "print" );
        test_token_w_position!(tokens[23], TType::Op, (9, 5), (10, 5), "(" );
        test_token_w_position!(tokens[24], TType::String, (10, 5), (19, 5), "\"block 2\"" );
        test_token_w_position!(tokens[25], TType::Op, (19, 5), (20, 5), ")" );
        // test_token_w_position!(tokens[26], TType::Newline, (20, 5), (21, 5), "\n" );
        // test_token_w_position!(tokens[27], TType::NL, (0, 6), (1, 6), "\n" );
        test_token_w_position!(tokens[28], TType::Dedent, (0, 7), (0, 7), "" );
        // test_token_w_position!(tokens[29], TType::EndMarker, (0, 7), (0, 7), "" );
    }

    #[test]
    fn test_crazy_dents() {
        let tokens = Tokenizer::tokenize_file(
            "test_fixtures/crazy_dents.py",
            TConfig{ skip_encoding: true, skip_endmarker: true}).expect("tokens");

        test_token_w_position!(tokens[0], TType::Name, (0, 3), (3, 3), "def" );
        test_token_w_position!(tokens[1], TType::Name, (4, 3), (11, 3), "toptier" );
        test_token_w_position!(tokens[2], TType::Op, (11, 3), (12, 3), "(" );
        test_token_w_position!(tokens[3], TType::Op, (12, 3), (13, 3), ")" );
        test_token_w_position!(tokens[4], TType::Op, (13, 3), (14, 3), ":" );
        // test_token_w_position!(tokens[5], TType::NL, (14, 3), (14, 3), "\n" );
        test_token_w_position!(tokens[6], TType::Indent, (0, 4), (0, 4), "    " );
        test_token_w_position!(tokens[7], TType::Name, (4, 4), (15, 4), "still_depth" );
        test_token_w_position!(tokens[8], TType::Op, (16, 4), (17, 4), "=" );
        test_token_w_position!(tokens[9], TType::Name, (18, 4), (22, 4), "True" );
        // test_token_w_position!(tokens[10], TType::NL, (22, 4), (22, 4), "\n" );
        test_token_w_position!(tokens[11], TType::Name, (4, 5), (7, 5), "def" );
        test_token_w_position!(tokens[12], TType::Name, (8, 5), (15, 5), "midtier" );
        test_token_w_position!(tokens[13], TType::Op, (15, 5), (16, 5), "(" );
        test_token_w_position!(tokens[14], TType::Name, (16, 5), (27, 5), "still_depth" );
        test_token_w_position!(tokens[15], TType::Op, (27, 5), (28, 5), ")" );
        test_token_w_position!(tokens[16], TType::Op, (28, 5), (29, 5), ":" );
        // test_token_w_position!(tokens[17], TType::NL, (29, 5), (29, 5), "\n" );
        test_token_w_position!(tokens[18], TType::Indent, (0, 6), (0, 6), "    " );
        test_token_w_position!(tokens[19], TType::Name, (8, 6), (23, 6), "more_to_go_true" );
        test_token_w_position!(tokens[20], TType::Op, (24, 6), (25, 6), "=" );
        test_token_w_position!(tokens[21], TType::Name, (26, 6), (30, 6), "True" );
        // test_token_w_position!(tokens[22], TType::NL, (30, 6), (30, 6), "\n" );
        test_token_w_position!(tokens[23], TType::Name, (8, 7), (11, 7), "def" );
        test_token_w_position!(tokens[24], TType::Name, (12, 7), (23, 7), "bottom_tier" );
        test_token_w_position!(tokens[25], TType::Op, (23, 7), (24, 7), "(" );
        test_token_w_position!(tokens[26], TType::Op, (24, 7), (25, 7), ")" );
        test_token_w_position!(tokens[27], TType::Op, (25, 7), (26, 7), ":" );
        // test_token_w_position!(tokens[28], TType::NL, (26, 7), (26, 7), "\n" );
        test_token_w_position!(tokens[29], TType::Indent, (0, 8), (0, 8), "    " );
        test_token_w_position!(tokens[30], TType::Name, (12, 8), (14, 8), "if" );
        test_token_w_position!(tokens[31], TType::Name, (15, 8), (26, 8), "still_depth" );
        test_token_w_position!(tokens[32], TType::Op, (26, 8), (27, 8), ":" );
        // test_token_w_position!(tokens[33], TType::NL, (27, 8), (27, 8), "\n" );
        test_token_w_position!(tokens[34], TType::Indent, (0, 9), (0, 9), "    " );
        test_token_w_position!(tokens[35], TType::Name, (16, 9), (27, 9), "still_depth" );
        test_token_w_position!(tokens[36], TType::Op, (28, 9), (29, 9), "=" );
        test_token_w_position!(tokens[37], TType::Name, (30, 9), (35, 9), "False" );
        // test_token_w_position!(tokens[38], TType::NL, (35, 9), (35, 9), "\n" );
        test_token_w_position!(tokens[39], TType::Name, (16, 10), (22, 10), "return" );
        test_token_w_position!(tokens[40], TType::Name, (23, 10), (27, 10), "True" );
        // test_token_w_position!(tokens[41], TType::NL, (27, 10), (27, 10), "\n" );
        test_token_w_position!(tokens[42], TType::Dedent, (0, 11), (0, 11), "" );
        test_token_w_position!(tokens[43], TType::Name, (12, 11), (16, 11), "else" );
        test_token_w_position!(tokens[44], TType::Op, (16, 11), (17, 11), ":" );
        // test_token_w_position!(tokens[45], TType::NL, (17, 11), (17, 11), "\n" );
        test_token_w_position!(tokens[46], TType::Indent, (0, 12), (0, 12), "    " );
        test_token_w_position!(tokens[47], TType::Name, (16, 12), (18, 12), "if" );
        test_token_w_position!(tokens[48], TType::Name, (19, 12), (34, 12), "more_to_go_true" );
        test_token_w_position!(tokens[49], TType::Op, (34, 12), (35, 12), ":" );
        // test_token_w_position!(tokens[50], TType::NL, (35, 12), (35, 12), "\n" );
        test_token_w_position!(tokens[51], TType::Indent, (0, 13), (0, 13), "    " );
        test_token_w_position!(tokens[52], TType::Name, (20, 13), (35, 13), "more_to_go_true" );
        test_token_w_position!(tokens[53], TType::Op, (36, 13), (37, 13), "=" );
        test_token_w_position!(tokens[54], TType::Name, (38, 13), (43, 13), "False" );
        // test_token_w_position!(tokens[55], TType::NL, (43, 13), (43, 13), "\n" );
        test_token_w_position!(tokens[56], TType::Name, (20, 14), (26, 14), "return" );
        test_token_w_position!(tokens[57], TType::Name, (27, 14), (32, 14), "False" );
        // test_token_w_position!(tokens[58], TType::NL, (32, 14), (32, 14), "\n" );
        test_token_w_position!(tokens[59], TType::Dedent, (0, 14), (0, 14), "" );
        test_token_w_position!(tokens[60], TType::Dedent, (0, 14), (0, 14), "" );
        test_token_w_position!(tokens[61], TType::Dedent, (0, 14), (0, 14), "" );
        test_token_w_position!(tokens[62], TType::Dedent, (0, 14), (0, 14), "" );
        test_token_w_position!(tokens[63], TType::Dedent, (0, 14), (0, 14), "" );

    }

}