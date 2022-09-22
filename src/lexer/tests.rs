


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

}