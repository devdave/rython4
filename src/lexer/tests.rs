


#[cfg(test)]
mod test {
    use crate::lexer::tokenizer::{TConfig, Tokenizer};
    use crate::tokens::{TType, Position};

    macro_rules! test_token_w_position{
        ($token:expr, $ttype:expr, $start:expr, $end:expr, $content:expr)=>{

            assert_eq!($token.text, $content, "Testing for text/content with {:?} != {:?}", $token.text, $content);
            assert_eq!($token.r#type, $ttype, "Testing for type with {:?} {:?} != {:?}", $token.text, $token.r#type, $ttype);
            assert_eq!($token.start, Position::t($start), "Testing for start with {:?} % {:?} : {:?} != {:?}", $token.text, $token.r#type, $token.start, $start);
            assert_eq!($token.end, Position::t($end), "Testing for end with {:?} % {:?} : {:?} != {:?}", $token.text, $token.r#type, $token.end, $end);

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

}