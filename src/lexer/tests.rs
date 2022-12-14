
#[cfg(test)]
mod test {

    use ntest::timeout;

    use crate::lexer::tokenizer::{TConfig, Tokenizer};
    use crate::tokens::{TType, Position, TokError};

    macro_rules! test_token_w_position {
    ($token:expr, $ttype:expr, $start:expr, $end:expr, $content:expr)=>{

        assert_eq!($token.text, $content, "Testing for text/content with {:?} != {:?} for a/{:?} e/{:?}", $token.text, $content, $token.r#type, $ttype);
        assert_eq!($token.r#type, $ttype, "Testing for type with {:?} {:?} != {:?}", $token.text, $token.r#type, $ttype);
        assert_eq!($token.start, Position::t2($start), "Testing for start with {:?} % {:?} : {:?} != {:?}", $token.text, $token.r#type, $token.start, $start);
        assert_eq!($token.end, Position::t2($end), "Testing for end with {:?} % {:?} : {:?} != {:?}", $token.text, $token.r#type, $token.end, $end);

    }
}


    #[test]
    fn test_float() {
        let tokens = Tokenizer::tokenize_file("test_fixtures/test_float.py", TConfig { skip_encoding: true, skip_endmarker: false }).expect("tokens");

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
        test_token_w_position!(tokens[24], TType::Name, (8, 0), (8, 1), "x" );
        test_token_w_position!(tokens[25], TType::Op, (8, 2), (8, 3), "=" );
        test_token_w_position!(tokens[26], TType::Number, (8, 4), (8, 7), "0.0" );
        test_token_w_position!(tokens[27], TType::NL, (8, 7), (8, 7), "" );
    }

    #[test]
    fn test_aifc_issue1() {
        let tokens = Tokenizer::tokenize_file("test_fixtures/test_aifc_issue1.py", TConfig::default()).expect("tokens");

        test_token_w_position!(tokens[0], TType::Name, (1, 0), (1, 1), "f" );
        test_token_w_position!(tokens[1], TType::Op, (1, 2), (1, 3), "=" );
        test_token_w_position!(tokens[2], TType::Number, (1, 4), (1, 7), "0.0" );
        test_token_w_position!(tokens[3], TType::NL, (1, 7), (1, 7), "" );

    }

    #[test]
    #[ignore]
    fn test_issue1_asyncio_base_events() {
        let tokens = Tokenizer::tokenize_file("test_fixtures/test_issue1_asyncio_base_events.py", TConfig::default()).expect("tokens");

        test_token_w_position!(tokens[0], TType::Async, (1, 0), (1, 5), "async" );
        test_token_w_position!(tokens[1], TType::Name, (1, 6), (1, 9), "def" );
        test_token_w_position!(tokens[2], TType::Name, (1, 10), (1, 33), "_sock_sendfile_fallback" );
        test_token_w_position!(tokens[3], TType::Op, (1, 33), (1, 34), "(" );
        test_token_w_position!(tokens[4], TType::Name, (1, 34), (1, 38), "self" );
        test_token_w_position!(tokens[5], TType::Op, (1, 38), (1, 39), "," );
        test_token_w_position!(tokens[6], TType::Name, (1, 40), (1, 44), "sock" );
        test_token_w_position!(tokens[7], TType::Op, (1, 44), (1, 45), "," );
        test_token_w_position!(tokens[8], TType::Name, (1, 46), (1, 50), "file" );
        test_token_w_position!(tokens[9], TType::Op, (1, 50), (1, 51), "," );
        test_token_w_position!(tokens[10], TType::Name, (1, 52), (1, 58), "offset" );
        test_token_w_position!(tokens[11], TType::Op, (1, 58), (1, 59), "," );
        test_token_w_position!(tokens[12], TType::Name, (1, 60), (1, 65), "count" );
        test_token_w_position!(tokens[13], TType::Op, (1, 65), (1, 66), ")" );
        test_token_w_position!(tokens[14], TType::Op, (1, 66), (1, 67), ":" );
        test_token_w_position!(tokens[15], TType::NL, (1, 67), (1, 67), "" );
        test_token_w_position!(tokens[16], TType::Indent, (2, 0), (2, 0), "" );
        test_token_w_position!(tokens[17], TType::Name, (2, 4), (2, 6), "if" );
        test_token_w_position!(tokens[18], TType::Name, (2, 7), (2, 13), "offset" );
        test_token_w_position!(tokens[19], TType::Op, (2, 13), (2, 14), ":" );
        test_token_w_position!(tokens[20], TType::NL, (2, 14), (2, 14), "" );
        test_token_w_position!(tokens[21], TType::Indent, (3, 0), (3, 0), "" );
        test_token_w_position!(tokens[22], TType::Name, (3, 8), (3, 12), "file" );
        test_token_w_position!(tokens[23], TType::Op, (3, 12), (3, 13), "." );
        test_token_w_position!(tokens[24], TType::Name, (3, 13), (3, 17), "seek" );
        test_token_w_position!(tokens[25], TType::Op, (3, 17), (3, 18), "(" );
        test_token_w_position!(tokens[26], TType::Name, (3, 18), (3, 24), "offset" );
        test_token_w_position!(tokens[27], TType::Op, (3, 24), (3, 25), ")" );
        test_token_w_position!(tokens[28], TType::NL, (3, 25), (3, 25), "" );
        test_token_w_position!(tokens[29], TType::Dedent, (4, 0), (4, 0), "" );
        test_token_w_position!(tokens[30], TType::Name, (4, 4), (4, 13), "blocksize" );
        test_token_w_position!(tokens[31], TType::Op, (4, 14), (4, 15), "=" );
        test_token_w_position!(tokens[32], TType::Op, (4, 16), (4, 17), "(" );
        test_token_w_position!(tokens[33], TType::Name, (5, 8), (5, 11), "min" );
        test_token_w_position!(tokens[34], TType::Op, (5, 11), (5, 12), "(" );
        test_token_w_position!(tokens[35], TType::Name, (5, 12), (5, 17), "count" );
        test_token_w_position!(tokens[36], TType::Op, (5, 17), (5, 18), "," );
        test_token_w_position!(tokens[37], TType::Name, (5, 19), (5, 28), "constants" );
        test_token_w_position!(tokens[38], TType::Op, (5, 28), (5, 29), "." );
        test_token_w_position!(tokens[39], TType::Name, (5, 29), (5, 62), "SENDFILE_FALLBACK_READBUFFER_SIZE" );
        test_token_w_position!(tokens[40], TType::Op, (5, 62), (5, 63), ")" );
        test_token_w_position!(tokens[41], TType::Name, (6, 8), (6, 10), "if" );
        test_token_w_position!(tokens[42], TType::Name, (6, 11), (6, 16), "count" );
        test_token_w_position!(tokens[43], TType::Name, (6, 17), (6, 21), "else" );
        test_token_w_position!(tokens[44], TType::Name, (6, 22), (6, 31), "constants" );
        test_token_w_position!(tokens[45], TType::Op, (6, 31), (6, 32), "." );
        test_token_w_position!(tokens[46], TType::Name, (6, 32), (6, 65), "SENDFILE_FALLBACK_READBUFFER_SIZE" );
        test_token_w_position!(tokens[47], TType::Op, (7, 4), (7, 5), ")" );
        test_token_w_position!(tokens[48], TType::NL, (7, 5), (7, 5), "" );
        test_token_w_position!(tokens[49], TType::Name, (8, 4), (8, 7), "buf" );
        test_token_w_position!(tokens[50], TType::Op, (8, 8), (8, 9), "=" );
        test_token_w_position!(tokens[51], TType::Name, (8, 10), (8, 19), "bytearray" );
        test_token_w_position!(tokens[52], TType::Op, (8, 19), (8, 20), "(" );
        test_token_w_position!(tokens[53], TType::Name, (8, 20), (8, 29), "blocksize" );
        test_token_w_position!(tokens[54], TType::Op, (8, 29), (8, 30), ")" );
        test_token_w_position!(tokens[55], TType::NL, (8, 30), (8, 30), "" );
        test_token_w_position!(tokens[56], TType::Name, (9, 4), (9, 14), "total_sent" );
        test_token_w_position!(tokens[57], TType::Op, (9, 15), (9, 16), "=" );
        test_token_w_position!(tokens[58], TType::Number, (9, 17), (9, 18), "0" );
        test_token_w_position!(tokens[59], TType::NL, (9, 18), (9, 18), "" );
        test_token_w_position!(tokens[60], TType::Name, (10, 4), (10, 7), "try" );
        test_token_w_position!(tokens[61], TType::Op, (10, 7), (10, 8), ":" );
        test_token_w_position!(tokens[62], TType::NL, (10, 8), (10, 8), "" );
        test_token_w_position!(tokens[63], TType::Indent, (11, 0), (11, 0), "" );
        test_token_w_position!(tokens[64], TType::Name, (11, 8), (11, 13), "while" );
        test_token_w_position!(tokens[65], TType::Name, (11, 14), (11, 18), "True" );
        test_token_w_position!(tokens[66], TType::Op, (11, 18), (11, 19), ":" );
        test_token_w_position!(tokens[67], TType::NL, (11, 19), (11, 19), "" );
        test_token_w_position!(tokens[68], TType::Indent, (12, 0), (12, 0), "" );
        test_token_w_position!(tokens[69], TType::Name, (12, 12), (12, 14), "if" );
        test_token_w_position!(tokens[70], TType::Name, (12, 15), (12, 20), "count" );
        test_token_w_position!(tokens[71], TType::Op, (12, 20), (12, 21), ":" );
        test_token_w_position!(tokens[72], TType::NL, (12, 21), (12, 21), "" );
        test_token_w_position!(tokens[73], TType::Indent, (13, 0), (13, 0), "" );
        test_token_w_position!(tokens[74], TType::Name, (13, 16), (13, 25), "blocksize" );
        test_token_w_position!(tokens[75], TType::Op, (13, 26), (13, 27), "=" );
        test_token_w_position!(tokens[76], TType::Name, (13, 28), (13, 31), "min" );
        test_token_w_position!(tokens[77], TType::Op, (13, 31), (13, 32), "(" );
        test_token_w_position!(tokens[78], TType::Name, (13, 32), (13, 37), "count" );
        test_token_w_position!(tokens[79], TType::Op, (13, 38), (13, 39), "-" );
        test_token_w_position!(tokens[80], TType::Name, (13, 40), (13, 50), "total_sent" );
        test_token_w_position!(tokens[81], TType::Op, (13, 50), (13, 51), "," );
        test_token_w_position!(tokens[82], TType::Name, (13, 52), (13, 61), "blocksize" );
        test_token_w_position!(tokens[83], TType::Op, (13, 61), (13, 62), ")" );
        test_token_w_position!(tokens[84], TType::NL, (13, 62), (13, 62), "" );
        test_token_w_position!(tokens[85], TType::Name, (14, 16), (14, 18), "if" );
        test_token_w_position!(tokens[86], TType::Name, (14, 19), (14, 28), "blocksize" );
        test_token_w_position!(tokens[87], TType::Op, (14, 29), (14, 31), "<=" );
        test_token_w_position!(tokens[88], TType::Number, (14, 32), (14, 33), "0" );
        test_token_w_position!(tokens[89], TType::Op, (14, 33), (14, 34), ":" );
        test_token_w_position!(tokens[90], TType::NL, (14, 34), (14, 34), "" );
        test_token_w_position!(tokens[91], TType::Indent, (15, 0), (15, 0), "" );
        test_token_w_position!(tokens[92], TType::Name, (15, 20), (15, 25), "break" );
        test_token_w_position!(tokens[93], TType::NL, (15, 25), (15, 25), "" );
        test_token_w_position!(tokens[94], TType::Dedent, (16, 0), (16, 0), "" );
        test_token_w_position!(tokens[95], TType::Dedent, (16, 0), (16, 0), "" );
        test_token_w_position!(tokens[96], TType::Name, (16, 12), (16, 16), "view" );
        test_token_w_position!(tokens[97], TType::Op, (16, 17), (16, 18), "=" );
        test_token_w_position!(tokens[98], TType::Name, (16, 19), (16, 29), "memoryview" );
        test_token_w_position!(tokens[99], TType::Op, (16, 29), (16, 30), "(" );
        test_token_w_position!(tokens[100], TType::Name, (16, 30), (16, 33), "buf" );
        test_token_w_position!(tokens[101], TType::Op, (16, 33), (16, 34), ")" );
        test_token_w_position!(tokens[102], TType::Op, (16, 34), (16, 35), "[" );
        test_token_w_position!(tokens[103], TType::Op, (16, 35), (16, 36), ":" );
        test_token_w_position!(tokens[104], TType::Name, (16, 36), (16, 45), "blocksize" );
        test_token_w_position!(tokens[105], TType::Op, (16, 45), (16, 46), "]" );
        test_token_w_position!(tokens[106], TType::NL, (16, 46), (16, 46), "" );
        test_token_w_position!(tokens[107], TType::Name, (17, 12), (17, 16), "read" );
        test_token_w_position!(tokens[108], TType::Op, (17, 17), (17, 18), "=" );
        test_token_w_position!(tokens[109], TType::Await, (17, 19), (17, 24), "await" );
        test_token_w_position!(tokens[110], TType::Name, (17, 25), (17, 29), "self" );
        test_token_w_position!(tokens[111], TType::Op, (17, 29), (17, 30), "." );
        test_token_w_position!(tokens[112], TType::Name, (17, 30), (17, 45), "run_in_executor" );
        test_token_w_position!(tokens[113], TType::Op, (17, 45), (17, 46), "(" );
        test_token_w_position!(tokens[114], TType::Name, (17, 46), (17, 50), "None" );
        test_token_w_position!(tokens[115], TType::Op, (17, 50), (17, 51), "," );
        test_token_w_position!(tokens[116], TType::Name, (17, 52), (17, 56), "file" );
        test_token_w_position!(tokens[117], TType::Op, (17, 56), (17, 57), "." );
        test_token_w_position!(tokens[118], TType::Name, (17, 57), (17, 65), "readinto" );
        test_token_w_position!(tokens[119], TType::Op, (17, 65), (17, 66), "," );
        test_token_w_position!(tokens[120], TType::Name, (17, 67), (17, 71), "view" );
        test_token_w_position!(tokens[121], TType::Op, (17, 71), (17, 72), ")" );
        test_token_w_position!(tokens[122], TType::NL, (17, 72), (17, 72), "" );
        test_token_w_position!(tokens[123], TType::Name, (18, 12), (18, 14), "if" );
        test_token_w_position!(tokens[124], TType::Name, (18, 15), (18, 18), "not" );
        test_token_w_position!(tokens[125], TType::Name, (18, 19), (18, 23), "read" );
        test_token_w_position!(tokens[126], TType::Op, (18, 23), (18, 24), ":" );
        test_token_w_position!(tokens[127], TType::NL, (18, 24), (18, 24), "" );
        test_token_w_position!(tokens[128], TType::Indent, (19, 0), (19, 0), "" );
        test_token_w_position!(tokens[129], TType::Name, (19, 16), (19, 21), "break" );
        test_token_w_position!(tokens[130], TType::NL, (19, 23), (19, 28), "" );
        test_token_w_position!(tokens[131], TType::Dedent, (20, 0), (20, 0), "" );
        test_token_w_position!(tokens[132], TType::Await, (20, 12), (20, 17), "await" );
        test_token_w_position!(tokens[133], TType::Name, (20, 18), (20, 22), "self" );
        test_token_w_position!(tokens[134], TType::Op, (20, 22), (20, 23), "." );
        test_token_w_position!(tokens[135], TType::Name, (20, 23), (20, 35), "sock_sendall" );
        test_token_w_position!(tokens[136], TType::Op, (20, 35), (20, 36), "(" );
        test_token_w_position!(tokens[137], TType::Name, (20, 36), (20, 40), "sock" );
        test_token_w_position!(tokens[138], TType::Op, (20, 40), (20, 41), "," );
        test_token_w_position!(tokens[139], TType::Name, (20, 42), (20, 46), "view" );
        test_token_w_position!(tokens[140], TType::Op, (20, 46), (20, 47), "[" );
        test_token_w_position!(tokens[141], TType::Op, (20, 47), (20, 48), ":" );
        test_token_w_position!(tokens[142], TType::Name, (20, 48), (20, 52), "read" );
        test_token_w_position!(tokens[143], TType::Op, (20, 52), (20, 53), "]" );
        test_token_w_position!(tokens[144], TType::Op, (20, 53), (20, 54), ")" );
        test_token_w_position!(tokens[145], TType::NL, (20, 54), (20, 54), "" );
        test_token_w_position!(tokens[146], TType::Name, (21, 12), (21, 22), "total_sent" );
        test_token_w_position!(tokens[147], TType::Op, (21, 23), (21, 25), "+=" );
        test_token_w_position!(tokens[148], TType::Name, (21, 26), (21, 30), "read" );
        test_token_w_position!(tokens[149], TType::NL, (21, 30), (21, 30), "" );
        test_token_w_position!(tokens[150], TType::Dedent, (22, 0), (22, 0), "" );
        test_token_w_position!(tokens[151], TType::Name, (22, 8), (22, 14), "return" );
        test_token_w_position!(tokens[152], TType::Name, (22, 15), (22, 25), "total_sent" );
        test_token_w_position!(tokens[153], TType::NL, (22, 25), (22, 25), "" );
        test_token_w_position!(tokens[154], TType::Dedent, (23, 0), (23, 0), "" );
        test_token_w_position!(tokens[155], TType::Name, (23, 4), (23, 11), "finally" );
        test_token_w_position!(tokens[156], TType::Op, (23, 11), (23, 12), ":" );
        test_token_w_position!(tokens[157], TType::NL, (23, 12), (23, 12), "" );
        test_token_w_position!(tokens[158], TType::Indent, (24, 0), (24, 0), "" );
        test_token_w_position!(tokens[159], TType::Name, (24, 8), (24, 10), "if" );
        test_token_w_position!(tokens[160], TType::Name, (24, 11), (24, 21), "total_sent" );
        test_token_w_position!(tokens[161], TType::Op, (24, 22), (24, 23), ">" );
        test_token_w_position!(tokens[162], TType::Number, (24, 24), (24, 25), "0" );
        test_token_w_position!(tokens[163], TType::Name, (24, 26), (24, 29), "and" );
        test_token_w_position!(tokens[164], TType::Name, (24, 30), (24, 37), "hasattr" );
        test_token_w_position!(tokens[165], TType::Op, (24, 37), (24, 38), "(" );
        test_token_w_position!(tokens[166], TType::Name, (24, 38), (24, 42), "file" );
        test_token_w_position!(tokens[167], TType::Op, (24, 42), (24, 43), "," );
        test_token_w_position!(tokens[168], TType::String, (24, 44), (24, 50), "'seek'" );
        test_token_w_position!(tokens[169], TType::Op, (24, 50), (24, 51), ")" );
        test_token_w_position!(tokens[170], TType::Op, (24, 51), (24, 52), ":" );
        test_token_w_position!(tokens[171], TType::NL, (24, 52), (24, 52), "" );
        test_token_w_position!(tokens[172], TType::Indent, (25, 0), (25, 0), "" );
        test_token_w_position!(tokens[173], TType::Name, (25, 12), (25, 16), "file" );
        test_token_w_position!(tokens[174], TType::Op, (25, 16), (25, 17), "." );
        test_token_w_position!(tokens[175], TType::Name, (25, 17), (25, 21), "seek" );
        test_token_w_position!(tokens[176], TType::Op, (25, 21), (25, 22), "(" );
        test_token_w_position!(tokens[177], TType::Name, (25, 22), (25, 28), "offset" );
        test_token_w_position!(tokens[178], TType::Op, (25, 29), (25, 30), "+" );
        test_token_w_position!(tokens[179], TType::Name, (25, 31), (25, 41), "total_sent" );
        test_token_w_position!(tokens[180], TType::Op, (25, 41), (25, 42), ")" );
        test_token_w_position!(tokens[181], TType::NL, (25, 42), (25, 42), "" );
        test_token_w_position!(tokens[182], TType::Dedent, (25, 0), (25, 0), "" );
        test_token_w_position!(tokens[183], TType::Dedent, (25, 0), (25, 0), "" );
        test_token_w_position!(tokens[184], TType::Dedent, (25, 0), (25, 0), "" );

    }

    #[test]
    fn test_float_scientific() {
        let test1: String = "x = 3e141\n".to_string();
        let test2: String = "x = 3E123\n".to_string();

        let mut tokenizer = Tokenizer::new(TConfig { skip_endmarker: true, skip_encoding: true });
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
            TConfig { skip_encoding: true, skip_endmarker: false }).expect("tokens");

        for (idx, token) in tokens.iter().enumerate() {
            println!("{}: {:?}", idx, token);
        }
    }

    #[test]
    fn test_additive() {
        let mut tokenizer = Tokenizer::new(TConfig { skip_encoding: true, skip_endmarker: false });
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
        let mut tokenizer = Tokenizer::new(TConfig { skip_encoding: true, skip_endmarker: false });
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
                                              TConfig { skip_encoding: true, skip_endmarker: false }).expect("tokens");

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
        let mut tokenizer = Tokenizer::new(TConfig { skip_encoding: true, skip_endmarker: false });
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
        let tokens = Tokenizer::tokenize_file("test_fixtures/test_long.py",
                                              TConfig { skip_encoding: true, skip_endmarker: false }).expect("tokens");


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
                                              TConfig { skip_encoding: true, skip_endmarker: false }).expect("tokens");

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
        let tokens = Tokenizer::tokenize_file("test_fixtures/test_method.py", TConfig { skip_encoding: true, skip_endmarker: false }).expect("tokens");

        test_token_w_position!(tokens[0], TType::Op, (1, 0), (1, 1), "@" );
        test_token_w_position!(tokens[1], TType::Name, (1, 1), (1, 13), "staticmethod" );
        test_token_w_position!(tokens[2], TType::NL, (1, 13), (1, 13), "" );
        test_token_w_position!(tokens[3], TType::Name, (2, 0), (2, 3), "def" );
        test_token_w_position!(tokens[4], TType::Name, (2, 4), (2, 7), "foo" );
        test_token_w_position!(tokens[5], TType::Op, (2, 7), (2, 8), "(" );
        test_token_w_position!(tokens[6], TType::Name, (2, 8), (2, 9), "x" );
        test_token_w_position!(tokens[7], TType::Op, (2, 9), (2, 10), "," );
        test_token_w_position!(tokens[8], TType::Name, (2, 10), (2, 11), "y" );
        test_token_w_position!(tokens[9], TType::Op, (2, 11), (2, 12), ")" );
        test_token_w_position!(tokens[10], TType::Op, (2, 12), (2, 13), ":" );
        test_token_w_position!(tokens[11], TType::NL, (2, 13), (2, 13), "" );
        test_token_w_position!(tokens[12], TType::Indent, (3, 0), (3, 0), "" );
        test_token_w_position!(tokens[13], TType::Name, (3, 4), (3, 8), "pass" );
        test_token_w_position!(tokens[14], TType::NL, (3, 8), (3, 8), "" );
        test_token_w_position!(tokens[15], TType::Dedent, (3, 0), (3, 0), "" );
    }

    #[test]
    fn test_multilplicative() {
        let mut tokenizer = Tokenizer::new(TConfig { skip_encoding: true, skip_endmarker: false });
        let tokens = tokenizer.process_file("test_fixtures/test_multiplicative.py").expect("tokens");

        test_token_w_position!(tokens[0], TType::Name, (1, 0), (1, 1), "x" );
        test_token_w_position!(tokens[1], TType::Op, (1, 2), (1, 3), "=" );
        test_token_w_position!(tokens[2], TType::Number, (1, 4), (1, 5), "1" );
        test_token_w_position!(tokens[3], TType::Op, (1, 5), (1, 7), "//" );
        test_token_w_position!(tokens[4], TType::Number, (1, 7), (1, 8), "1" );
        test_token_w_position!(tokens[5], TType::Op, (1, 8), (1, 9), "*" );
        test_token_w_position!(tokens[6], TType::Number, (1, 9), (1, 10), "1" );
        test_token_w_position!(tokens[7], TType::Op, (1, 10), (1, 11), "/" );
        test_token_w_position!(tokens[8], TType::Number, (1, 11), (1, 12), "5" );
        test_token_w_position!(tokens[9], TType::Op, (1, 12), (1, 13), "*" );
        test_token_w_position!(tokens[10], TType::Number, (1, 13), (1, 15), "12" );
        test_token_w_position!(tokens[11], TType::Op, (1, 15), (1, 16), "%" );
        test_token_w_position!(tokens[12], TType::Number, (1, 16), (1, 20), "0x12" );
        test_token_w_position!(tokens[13], TType::Op, (1, 20), (1, 21), "@" );
        test_token_w_position!(tokens[14], TType::Number, (1, 21), (1, 23), "42" );
        test_token_w_position!(tokens[15], TType::NL, (1, 23), (1, 23), "" );
    }

    #[test]
    fn test_selector() {
        //import sys, time
        // x = sys.modules['time'].time()

        let mut tokenizer = Tokenizer::new(TConfig { skip_encoding: true, skip_endmarker: false });
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
        let mut tokenizer = Tokenizer::new(TConfig { skip_endmarker: false, skip_encoding: true });
        let tokens = tokenizer.process_file("test_fixtures/test_shift.py").expect("tokens");

        test_token_w_position!(tokens[0], TType::Name, (1, 0), (1, 1), "x" );
        test_token_w_position!(tokens[1], TType::Op, (1, 2), (1, 3), "=" );
        test_token_w_position!(tokens[2], TType::Number, (1, 4), (1, 5), "1" );
        test_token_w_position!(tokens[3], TType::Op, (1, 6), (1, 8), "<<" );
        test_token_w_position!(tokens[4], TType::Number, (1, 9), (1, 10), "1" );
        test_token_w_position!(tokens[5], TType::Op, (1, 11), (1, 13), ">>" );
        test_token_w_position!(tokens[6], TType::Number, (1, 14), (1, 15), "5" );
        test_token_w_position!(tokens[7], TType::NL, (1, 15), (1, 15), "" );
    }

    #[test]
    fn test_string_cases() {
        let data = vec![
            r#""Hello""#,
            r#"'World'"#,
            r#""""Big string""""#,
        ];

        let mut tokenizer = Tokenizer::new(TConfig::default());

        for test_data in data {
            println!("Trying {:?}", test_data);
            let tokens = tokenizer.process_single_line(test_data.to_string()).expect("tokens");

            assert_eq!(tokens[0].r#type, TType::String);
            assert_eq!(tokens[0].text, test_data.to_string());
        }
    }

    #[test]
    fn test_string() {
        let mut tokenizer = Tokenizer::new(TConfig { skip_encoding: true, skip_endmarker: false });
        let tokens = tokenizer.process_file("test_fixtures/test_string.py").expect("tokens");

        test_token_w_position!(tokens[0], TType::Name, (1, 0), (1, 1), "x" );
        test_token_w_position!(tokens[1], TType::Op, (1, 2), (1, 3), "=" );
        test_token_w_position!(tokens[2], TType::String, (1, 4), (1, 6), "''" );
        test_token_w_position!(tokens[3], TType::Op, (1, 6), (1, 7), ";" );
        test_token_w_position!(tokens[4], TType::Name, (1, 8), (1, 9), "y" );
        test_token_w_position!(tokens[5], TType::Op, (1, 10), (1, 11), "=" );
        test_token_w_position!(tokens[6], TType::String, (1, 12), (1, 14), "\"\"" );
        test_token_w_position!(tokens[7], TType::NL, (1, 14), (1, 14), "" );
        test_token_w_position!(tokens[8], TType::Name, (2, 0), (2, 1), "x" );
        test_token_w_position!(tokens[9], TType::Op, (2, 2), (2, 3), "=" );
        test_token_w_position!(tokens[10], TType::String, (2, 4), (2, 7), "'\"'" );
        test_token_w_position!(tokens[11], TType::Op, (2, 7), (2, 8), ";" );
        test_token_w_position!(tokens[12], TType::Name, (2, 9), (2, 10), "y" );
        test_token_w_position!(tokens[13], TType::Op, (2, 11), (2, 12), "=" );
        test_token_w_position!(tokens[14], TType::String, (2, 13), (2, 16), r#""'""# );
        test_token_w_position!(tokens[15], TType::NL, (2, 16), (2, 16), "" );
        test_token_w_position!(tokens[16], TType::Name, (3, 0), (3, 1), "x" );
        test_token_w_position!(tokens[17], TType::Op, (3, 2), (3, 3), "=" );
        test_token_w_position!(tokens[18], TType::String, (3, 4), (3, 38), r#""it doesn't \"shrink\", does it\"""# );
        test_token_w_position!(tokens[19], TType::NL, (3, 38), (3, 38), "" );
        test_token_w_position!(tokens[20], TType::Name, (4, 0), (4, 1), "x" );
        test_token_w_position!(tokens[21], TType::Op, (4, 2), (4, 3), "=" );
        test_token_w_position!(tokens[22], TType::String, (4, 4), (4, 9), "'abc'" );
        test_token_w_position!(tokens[23], TType::Op, (4, 10), (4, 11), "+" );
        test_token_w_position!(tokens[24], TType::String, (4, 12), (4, 17), "'ABC'" );
        test_token_w_position!(tokens[25], TType::NL, (4, 17), (4, 17), "" );
        test_token_w_position!(tokens[26], TType::Name, (5, 0), (5, 1), "y" );
        test_token_w_position!(tokens[27], TType::Op, (5, 2), (5, 3), "=" );
        test_token_w_position!(tokens[28], TType::String, (5, 4), (5, 9), "\"ABC\"" );
        test_token_w_position!(tokens[29], TType::Op, (5, 10), (5, 11), "+" );
        test_token_w_position!(tokens[30], TType::String, (5, 12), (5, 17), "\"ABC\"" );
        test_token_w_position!(tokens[31], TType::NL, (5, 17), (5, 17), "" );
        test_token_w_position!(tokens[32], TType::Name, (6, 0), (6, 1), "x" );
        test_token_w_position!(tokens[33], TType::Op, (6, 2), (6, 3), "=" );
        test_token_w_position!(tokens[34], TType::String, (6, 4), (6, 10), "r'abc'" );
        test_token_w_position!(tokens[35], TType::Op, (6, 11), (6, 12), "+" );
        test_token_w_position!(tokens[36], TType::String, (6, 13), (6, 19), "r'ABC'" );
        test_token_w_position!(tokens[37], TType::Op, (6, 20), (6, 21), "+" );
        test_token_w_position!(tokens[38], TType::String, (6, 22), (6, 28), "R'ABC'" );
        test_token_w_position!(tokens[39], TType::Op, (6, 29), (6, 30), "+" );
        test_token_w_position!(tokens[40], TType::String, (6, 31), (6, 37), "R'ABC'" );
        test_token_w_position!(tokens[41], TType::NL, (6, 37), (6, 37), "" );
        test_token_w_position!(tokens[42], TType::Name, (7, 0), (7, 1), "y" );
        test_token_w_position!(tokens[43], TType::Op, (7, 2), (7, 3), "=" );
        test_token_w_position!(tokens[44], TType::String, (7, 4), (7, 10), "r\"abc\"" );
        test_token_w_position!(tokens[45], TType::Op, (7, 11), (7, 12), "+" );
        test_token_w_position!(tokens[46], TType::String, (7, 13), (7, 19), "r\"ABC\"" );
        test_token_w_position!(tokens[47], TType::Op, (7, 20), (7, 21), "+" );
        test_token_w_position!(tokens[48], TType::String, (7, 22), (7, 28), "R\"ABC\"" );
        test_token_w_position!(tokens[49], TType::Op, (7, 29), (7, 30), "+" );
        test_token_w_position!(tokens[50], TType::String, (7, 31), (7, 37), "R\"ABC\"" );
        test_token_w_position!(tokens[51], TType::NL, (7, 37), (7, 37), "" );
        test_token_w_position!(tokens[52], TType::String, (8, 0), (8, 6), "u'abc'" );
        test_token_w_position!(tokens[53], TType::Op, (8, 7), (8, 8), "+" );
        test_token_w_position!(tokens[54], TType::String, (8, 9), (8, 15), "U'abc'" );
        test_token_w_position!(tokens[55], TType::NL, (8, 15), (8, 15), "" );
        test_token_w_position!(tokens[56], TType::String, (9, 0), (9, 6), "u\"abc\"" );
        test_token_w_position!(tokens[57], TType::Op, (9, 7), (9, 8), "+" );
        test_token_w_position!(tokens[58], TType::String, (9, 9), (9, 15), "U\"abc\"" );
        test_token_w_position!(tokens[59], TType::NL, (9, 15), (9, 15), "" );
        test_token_w_position!(tokens[60], TType::String, (10, 0), (10, 6), "b'abc'" );
        test_token_w_position!(tokens[61], TType::Op, (10, 7), (10, 8), "+" );
        test_token_w_position!(tokens[62], TType::String, (10, 9), (10, 15), "B'abc'" );
        test_token_w_position!(tokens[63], TType::NL, (10, 15), (10, 15), "" );
        test_token_w_position!(tokens[64], TType::String, (11, 0), (11, 7), "br'abc'" );
        test_token_w_position!(tokens[65], TType::Op, (11, 8), (11, 9), "+" );
        test_token_w_position!(tokens[66], TType::String, (11, 10), (11, 17), "bR'abc'" );
        test_token_w_position!(tokens[67], TType::Op, (11, 18), (11, 19), "+" );
        test_token_w_position!(tokens[68], TType::String, (11, 20), (11, 27), "Br'abc'" );
        test_token_w_position!(tokens[69], TType::Op, (11, 28), (11, 29), "+" );
        test_token_w_position!(tokens[70], TType::String, (11, 30), (11, 37), "BR'abc'" );
        test_token_w_position!(tokens[71], TType::NL, (11, 37), (11, 37), "" );
        test_token_w_position!(tokens[72], TType::String, (12, 0), (12, 7), "br\"abc\"" );
        test_token_w_position!(tokens[73], TType::Op, (12, 8), (12, 9), "+" );
        test_token_w_position!(tokens[74], TType::String, (12, 10), (12, 17), "bR\"abc\"" );
        test_token_w_position!(tokens[75], TType::Op, (12, 18), (12, 19), "+" );
        test_token_w_position!(tokens[76], TType::String, (12, 20), (12, 27), "Br\"abc\"" );
        test_token_w_position!(tokens[77], TType::Op, (12, 28), (12, 29), "+" );
        test_token_w_position!(tokens[78], TType::String, (12, 30), (12, 37), "BR\"abc\"" );
        test_token_w_position!(tokens[79], TType::NL, (12, 37), (12, 37), "" );
        test_token_w_position!(tokens[80], TType::String, (13, 0), (13, 7), "rb'abc'" );
        test_token_w_position!(tokens[81], TType::Op, (13, 8), (13, 9), "+" );
        test_token_w_position!(tokens[82], TType::String, (13, 10), (13, 17), "rB'abc'" );
        test_token_w_position!(tokens[83], TType::Op, (13, 18), (13, 19), "+" );
        test_token_w_position!(tokens[84], TType::String, (13, 20), (13, 27), "Rb'abc'" );
        test_token_w_position!(tokens[85], TType::Op, (13, 28), (13, 29), "+" );
        test_token_w_position!(tokens[86], TType::String, (13, 30), (13, 37), "RB'abc'" );
        test_token_w_position!(tokens[87], TType::NL, (13, 37), (13, 37), "" );
        test_token_w_position!(tokens[88], TType::String, (14, 0), (14, 7), "rb\"abc\"" );
        test_token_w_position!(tokens[89], TType::Op, (14, 8), (14, 9), "+" );
        test_token_w_position!(tokens[90], TType::String, (14, 10), (14, 17), "rB\"abc\"" );
        test_token_w_position!(tokens[91], TType::Op, (14, 18), (14, 19), "+" );
        test_token_w_position!(tokens[92], TType::String, (14, 20), (14, 27), "Rb\"abc\"" );
        test_token_w_position!(tokens[93], TType::Op, (14, 28), (14, 29), "+" );
        test_token_w_position!(tokens[94], TType::String, (14, 30), (14, 37), "RB\"abc\"" );
        test_token_w_position!(tokens[95], TType::NL, (14, 37), (14, 37), "" );
    }


    #[test]
    fn test_unary() {
        let mut tokenizer = Tokenizer::new(TConfig::default());
        let tokens = tokenizer.process_file("test_fixtures/test_unary.py").expect("tokens");

        test_token_w_position!(tokens[0], TType::Op, (1, 0), (1, 1), "~" );
        test_token_w_position!(tokens[1], TType::Number, (1, 1), (1, 2), "1" );
        test_token_w_position!(tokens[2], TType::Op, (1, 3), (1, 4), "^" );
        test_token_w_position!(tokens[3], TType::Number, (1, 5), (1, 6), "1" );
        test_token_w_position!(tokens[4], TType::Op, (1, 7), (1, 8), "&" );
        test_token_w_position!(tokens[5], TType::Number, (1, 9), (1, 10), "1" );
        test_token_w_position!(tokens[6], TType::Op, (1, 11), (1, 12), "|" );
        test_token_w_position!(tokens[7], TType::Number, (1, 12), (1, 13), "1" );
        test_token_w_position!(tokens[8], TType::Op, (1, 14), (1, 15), "^" );
        test_token_w_position!(tokens[9], TType::Op, (1, 16), (1, 17), "-" );
        test_token_w_position!(tokens[10], TType::Number, (1, 17), (1, 18), "1" );
        test_token_w_position!(tokens[11], TType::NL, (1, 18), (1, 18), "" );
        test_token_w_position!(tokens[12], TType::Op, (2, 0), (2, 1), "-" );
        test_token_w_position!(tokens[13], TType::Number, (2, 1), (2, 2), "1" );
        test_token_w_position!(tokens[14], TType::Op, (2, 2), (2, 3), "*" );
        test_token_w_position!(tokens[15], TType::Number, (2, 3), (2, 4), "1" );
        test_token_w_position!(tokens[16], TType::Op, (2, 4), (2, 5), "/" );
        test_token_w_position!(tokens[17], TType::Number, (2, 5), (2, 6), "1" );
        test_token_w_position!(tokens[18], TType::Op, (2, 6), (2, 7), "+" );
        test_token_w_position!(tokens[19], TType::Number, (2, 7), (2, 8), "1" );
        test_token_w_position!(tokens[20], TType::Op, (2, 8), (2, 9), "*" );
        test_token_w_position!(tokens[21], TType::Number, (2, 9), (2, 10), "1" );
        test_token_w_position!(tokens[22], TType::Op, (2, 10), (2, 12), "//" );
        test_token_w_position!(tokens[23], TType::Number, (2, 12), (2, 13), "1" );
        test_token_w_position!(tokens[24], TType::Op, (2, 14), (2, 15), "-" );
        test_token_w_position!(tokens[25], TType::Op, (2, 16), (2, 17), "-" );
        test_token_w_position!(tokens[26], TType::Op, (2, 17), (2, 18), "-" );
        test_token_w_position!(tokens[27], TType::Op, (2, 18), (2, 19), "-" );
        test_token_w_position!(tokens[28], TType::Number, (2, 19), (2, 20), "1" );
        test_token_w_position!(tokens[29], TType::Op, (2, 20), (2, 22), "**" );
        test_token_w_position!(tokens[30], TType::Number, (2, 22), (2, 23), "1" );
        test_token_w_position!(tokens[31], TType::NL, (2, 23), (2, 23), "" );
    }

    #[test]
    fn test_basic_operators() {
        let mut tokenizer = Tokenizer::new(TConfig { skip_encoding: true, skip_endmarker: false });
        let tokens = tokenizer.process_file("test_fixtures/test_basic_operators.py").expect("tokens");

        test_token_w_position!(tokens[0], TType::Number, (1, 0), (1, 1), "1" );
        test_token_w_position!(tokens[1], TType::Op, (1, 2), (1, 3), "+" );
        test_token_w_position!(tokens[2], TType::Number, (1, 4), (1, 5), "1" );
        test_token_w_position!(tokens[3], TType::NL, (1, 5), (1, 5), "" );
    }

    #[test]
    #[timeout(200)]
    fn test_valid_literals() {
        let valid_underscore_literals: Vec<&str> = vec![
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

        let mut tokenizer = Tokenizer::new(TConfig { skip_endmarker: true, skip_encoding: true });


        for value in valid_underscore_literals {
            if value.starts_with("(") {
                continue;
            }

            println!("Testing {:?}", value);
            let result = tokenizer.process_single_line(value.to_string()).expect("tokens");
            assert_eq!(result[0].r#type, TType::Number, "Got the wrong type when processing {:?}.  Got {:?}", value, result[0]);
        }
    }

    #[test]
    #[timeout(100)]
    fn test_troublesome_literal() {
        let mut tokenizer = Tokenizer::new(TConfig { skip_endmarker: true, skip_encoding: true });
        let result = tokenizer.process_single_line("1_00_00e5_1".to_string()).expect("tokens");
        println!("Processed {:#?}", result);
    }

    #[test]
    fn test_multiline_strings() {
        let mut tokenizer = Tokenizer::new(TConfig { skip_encoding: true, skip_endmarker: true });
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
        let tokens = Tokenizer::tokenize_file("test_fixtures/test_function.py", TConfig { skip_encoding: true, skip_endmarker: false }).expect("tokens");

        for (lno, token) in tokens.iter().enumerate() {
            println!("{}: {:?}", lno, token);
        }
        test_token_w_position!(tokens[0], TType::Name, (1, 0), (1, 3), "def" );
        test_token_w_position!(tokens[1], TType::Name, (1, 4), (1, 7), "d22" );
        test_token_w_position!(tokens[2], TType::Op, (1, 7), (1, 8), "(" );
        test_token_w_position!(tokens[3], TType::Name, (1, 8), (1, 9), "a" );
        test_token_w_position!(tokens[4], TType::Op, (1, 9), (1, 10), "," );
        test_token_w_position!(tokens[5], TType::Name, (1, 11), (1, 12), "b" );
        test_token_w_position!(tokens[6], TType::Op, (1, 12), (1, 13), "," );
        test_token_w_position!(tokens[7], TType::Name, (1, 14), (1, 15), "c" );
        test_token_w_position!(tokens[8], TType::Op, (1, 15), (1, 16), "=" );
        test_token_w_position!(tokens[9], TType::Number, (1, 16), (1, 17), "2" );
        test_token_w_position!(tokens[10], TType::Op, (1, 17), (1, 18), "," );
        test_token_w_position!(tokens[11], TType::Name, (1, 19), (1, 20), "d" );
        test_token_w_position!(tokens[12], TType::Op, (1, 20), (1, 21), "=" );
        test_token_w_position!(tokens[13], TType::Number, (1, 21), (1, 22), "2" );
        test_token_w_position!(tokens[14], TType::Op, (1, 22), (1, 23), "," );
        test_token_w_position!(tokens[15], TType::Op, (1, 24), (1, 25), "*" );
        test_token_w_position!(tokens[16], TType::Name, (1, 25), (1, 26), "k" );
        test_token_w_position!(tokens[17], TType::Op, (1, 26), (1, 27), ")" );
        test_token_w_position!(tokens[18], TType::Op, (1, 27), (1, 28), ":" );
        test_token_w_position!(tokens[19], TType::Name, (1, 29), (1, 33), "pass" );
        test_token_w_position!(tokens[20], TType::NL, (1, 33), (1, 33), "" );
        test_token_w_position!(tokens[21], TType::Name, (2, 0), (2, 3), "def" );
        test_token_w_position!(tokens[22], TType::Name, (2, 4), (2, 9), "d01v_" );
        test_token_w_position!(tokens[23], TType::Op, (2, 9), (2, 10), "(" );
        test_token_w_position!(tokens[24], TType::Name, (2, 10), (2, 11), "a" );
        test_token_w_position!(tokens[25], TType::Op, (2, 11), (2, 12), "=" );
        test_token_w_position!(tokens[26], TType::Number, (2, 12), (2, 13), "1" );
        test_token_w_position!(tokens[27], TType::Op, (2, 13), (2, 14), "," );
        test_token_w_position!(tokens[28], TType::Op, (2, 15), (2, 16), "*" );
        test_token_w_position!(tokens[29], TType::Name, (2, 16), (2, 17), "k" );
        test_token_w_position!(tokens[30], TType::Op, (2, 17), (2, 18), "," );
        test_token_w_position!(tokens[31], TType::Op, (2, 19), (2, 21), "**" );
        test_token_w_position!(tokens[32], TType::Name, (2, 21), (2, 22), "w" );
        test_token_w_position!(tokens[33], TType::Op, (2, 22), (2, 23), ")" );
        test_token_w_position!(tokens[34], TType::Op, (2, 23), (2, 24), ":" );
        test_token_w_position!(tokens[35], TType::Name, (2, 25), (2, 29), "pass" );
        test_token_w_position!(tokens[36], TType::NL, (2, 29), (2, 29), "" );
        test_token_w_position!(tokens[37], TType::Name, (3, 0), (3, 3), "def" );
        test_token_w_position!(tokens[38], TType::Name, (3, 4), (3, 7), "d23" );
        test_token_w_position!(tokens[39], TType::Op, (3, 7), (3, 8), "(" );
        test_token_w_position!(tokens[40], TType::Name, (3, 8), (3, 9), "a" );
        test_token_w_position!(tokens[41], TType::Op, (3, 9), (3, 10), ":" );
        test_token_w_position!(tokens[42], TType::Name, (3, 11), (3, 14), "str" );
        test_token_w_position!(tokens[43], TType::Op, (3, 14), (3, 15), "," );
        test_token_w_position!(tokens[44], TType::Name, (3, 16), (3, 17), "b" );
        test_token_w_position!(tokens[45], TType::Op, (3, 17), (3, 18), ":" );
        test_token_w_position!(tokens[46], TType::Name, (3, 19), (3, 22), "int" );
        test_token_w_position!(tokens[47], TType::Op, (3, 22), (3, 23), "=" );
        test_token_w_position!(tokens[48], TType::Number, (3, 23), (3, 24), "3" );
        test_token_w_position!(tokens[49], TType::Op, (3, 24), (3, 25), ")" );
        test_token_w_position!(tokens[50], TType::Op, (3, 26), (3, 28), "->" );
        test_token_w_position!(tokens[51], TType::Name, (3, 29), (3, 32), "int" );
        test_token_w_position!(tokens[52], TType::Op, (3, 32), (3, 33), ":" );
        test_token_w_position!(tokens[53], TType::Name, (3, 34), (3, 38), "pass" );
        test_token_w_position!(tokens[54], TType::NL, (3, 38), (3, 38), "" );
    }

    #[test]
    fn test_basic_class() {
        let tokens = Tokenizer::tokenize_file(
            "test_fixtures/basic_class.py",
            TConfig { skip_encoding: true, skip_endmarker: false }
        ).expect("tokens");

        test_token_w_position!(tokens[0], TType::Name, (1, 0), (1, 5), "class" );
        test_token_w_position!(tokens[1], TType::Name, (1, 6), (1, 11), "Basic" );
        test_token_w_position!(tokens[2], TType::Op, (1, 11), (1, 12), ":" );
        test_token_w_position!(tokens[3], TType::NL, (1, 12), (1, 12), "" );
        test_token_w_position!(tokens[4], TType::Indent, (2, 0), (2, 0), "" );
        // test_token_w_position!(tokens[5], TType::String, (2, 4), (4, 7), """"
        //     A basic class definition
        //     """" );
        test_token_w_position!(tokens[6], TType::NL, (4, 7), (4, 7), "" );
        test_token_w_position!(tokens[7], TType::Name, (5, 4), (5, 7), "def" );
        test_token_w_position!(tokens[8], TType::Name, (5, 8), (5, 16), "__init__" );
        test_token_w_position!(tokens[9], TType::Op, (5, 16), (5, 17), "(" );
        test_token_w_position!(tokens[10], TType::Name, (5, 17), (5, 21), "self" );
        test_token_w_position!(tokens[11], TType::Op, (5, 21), (5, 22), ")" );
        test_token_w_position!(tokens[12], TType::Op, (5, 22), (5, 23), ":" );
        test_token_w_position!(tokens[13], TType::NL, (5, 23), (5, 23), "" );
        test_token_w_position!(tokens[14], TType::Indent, (6, 0), (6, 0), "" );
        // test_token_w_position!(tokens[15], TType::String, (6, 8), (8, 11), """"
        //         A basic init
        //         """" );
        test_token_w_position!(tokens[16], TType::NL, (8, 11), (8, 11), "" );
        test_token_w_position!(tokens[17], TType::Name, (9, 8), (9, 12), "self" );
        test_token_w_position!(tokens[18], TType::Op, (9, 12), (9, 13), "." );
        test_token_w_position!(tokens[19], TType::Name, (9, 13), (9, 14), "c" );
        test_token_w_position!(tokens[20], TType::Op, (9, 15), (9, 16), "=" );
        test_token_w_position!(tokens[21], TType::Number, (9, 17), (9, 18), "0" );
        test_token_w_position!(tokens[22], TType::NL, (9, 18), (9, 18), "" );
        test_token_w_position!(tokens[23], TType::Dedent, (11, 0), (11, 0), "" );
        test_token_w_position!(tokens[24], TType::Name, (11, 4), (11, 7), "def" );
        test_token_w_position!(tokens[25], TType::Name, (11, 8), (11, 11), "add" );
        test_token_w_position!(tokens[26], TType::Op, (11, 11), (11, 12), "(" );
        test_token_w_position!(tokens[27], TType::Name, (11, 12), (11, 16), "self" );
        test_token_w_position!(tokens[28], TType::Op, (11, 16), (11, 17), "," );
        test_token_w_position!(tokens[29], TType::Name, (11, 18), (11, 19), "a" );
        test_token_w_position!(tokens[30], TType::Op, (11, 19), (11, 20), "," );
        test_token_w_position!(tokens[31], TType::Name, (11, 21), (11, 22), "b" );
        test_token_w_position!(tokens[32], TType::Op, (11, 22), (11, 23), ")" );
        test_token_w_position!(tokens[33], TType::Op, (11, 23), (11, 24), ":" );
        test_token_w_position!(tokens[34], TType::NL, (11, 24), (11, 24), "" );
        test_token_w_position!(tokens[35], TType::Indent, (12, 0), (12, 0), "" );
        test_token_w_position!(tokens[36], TType::Name, (12, 8), (12, 12), "self" );
        test_token_w_position!(tokens[37], TType::Op, (12, 12), (12, 13), "." );
        test_token_w_position!(tokens[38], TType::Name, (12, 13), (12, 14), "c" );
        test_token_w_position!(tokens[39], TType::Op, (12, 15), (12, 16), "=" );
        test_token_w_position!(tokens[40], TType::Name, (12, 17), (12, 18), "a" );
        test_token_w_position!(tokens[41], TType::Op, (12, 19), (12, 20), "+" );
        test_token_w_position!(tokens[42], TType::Name, (12, 21), (12, 22), "b" );
        test_token_w_position!(tokens[43], TType::NL, (12, 22), (12, 22), "" );
        test_token_w_position!(tokens[44], TType::Dedent, (14, 0), (14, 0), "" );
        test_token_w_position!(tokens[45], TType::Name, (14, 4), (14, 7), "def" );
        test_token_w_position!(tokens[46], TType::Name, (14, 8), (14, 11), "get" );
        test_token_w_position!(tokens[47], TType::Op, (14, 11), (14, 12), "(" );
        test_token_w_position!(tokens[48], TType::Name, (14, 12), (14, 16), "self" );
        test_token_w_position!(tokens[49], TType::Op, (14, 16), (14, 17), ")" );
        test_token_w_position!(tokens[50], TType::Op, (14, 17), (14, 18), ":" );
        test_token_w_position!(tokens[51], TType::NL, (14, 18), (14, 18), "" );
        test_token_w_position!(tokens[52], TType::Indent, (15, 0), (15, 0), "" );
        test_token_w_position!(tokens[53], TType::Name, (15, 8), (15, 14), "return" );
        test_token_w_position!(tokens[54], TType::Name, (15, 15), (15, 19), "self" );
        test_token_w_position!(tokens[55], TType::Op, (15, 19), (15, 20), "." );
        test_token_w_position!(tokens[56], TType::Name, (15, 20), (15, 21), "c" );
        test_token_w_position!(tokens[57], TType::NL, (15, 21), (15, 21), "" );
        test_token_w_position!(tokens[58], TType::Dedent, (15, 0), (15, 0), "" );
        test_token_w_position!(tokens[59], TType::Dedent, (15, 0), (15, 0), "" );

    }

    #[test]
    fn test_basic_indent() {
        let tokens = Tokenizer::tokenize_file(
            "test_fixtures/basic_indent.py",
            TConfig { skip_encoding: true, skip_endmarker: true }).expect("tokens");

        test_token_w_position!(tokens[0], TType::Name, (1, 0), (1, 3), "def" );
        test_token_w_position!(tokens[1], TType::Name, (1, 4), (1, 8), "test" );
        test_token_w_position!(tokens[2], TType::Op, (1, 8), (1, 9), "(" );
        test_token_w_position!(tokens[3], TType::Op, (1, 9), (1, 10), ")" );
        test_token_w_position!(tokens[4], TType::Op, (1, 10), (1, 11), ":" );
        test_token_w_position!(tokens[5], TType::NL, (1, 11), (1, 11), "" );
        test_token_w_position!(tokens[6], TType::Indent, (2, 0), (2, 0), "" );
        test_token_w_position!(tokens[7], TType::Name, (2, 4), (2, 9), "print" );
        test_token_w_position!(tokens[8], TType::Op, (2, 9), (2, 10), "(" );
        test_token_w_position!(tokens[9], TType::String, (2, 10), (2, 23), "\"Hello world\"" );
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
        test_token_w_position!(tokens[22], TType::String, (5, 10), (5, 19), "\"block 2\"" );
        test_token_w_position!(tokens[23], TType::Op, (5, 19), (5, 20), ")" );
        test_token_w_position!(tokens[24], TType::NL, (5, 20), (5, 20), "" );
        test_token_w_position!(tokens[25], TType::Dedent, (5, 0), (5, 0), "" );
    }

    #[test]
    fn test_crazy_dents() {
        let tokens = Tokenizer::tokenize_file(
            "test_fixtures/crazy_dents.py",
            TConfig { skip_encoding: true, skip_endmarker: true }).expect("tokens");


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
            TConfig { skip_encoding: true, skip_endmarker: false }
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
    fn test_attempt_identifiers() {
        let valid_names = vec![
            "hello_world",
            "a",
            "_",
            "__",
            "rtest",
        ];

        let mut tokenizer = Tokenizer::new(TConfig::default());

        for valid_name in valid_names {
            let tokens = tokenizer.process_single_line(valid_name.to_string()).expect("tokens");

            assert_eq!(tokens[0].r#type, TType::Name);
            assert_eq!(tokens[0].text, valid_name.to_string());
        }
    }

    #[test]
    fn test_issue_difflib_raw_string_block() {
        let tokens = Tokenizer::tokenize_file("test_fixtures/test_issue_difflib_raw_string_block.py", TConfig::default()).expect("tokens");

        for (idx, token) in tokens.iter().enumerate(){
            println!("{}:{:?}", idx, token.r#type);
        }

        test_token_w_position!(tokens[0], TType::Name, (1, 0), (1, 5), "class" );
        test_token_w_position!(tokens[1], TType::Name, (1, 6), (1, 12), "Differ" );
        test_token_w_position!(tokens[2], TType::Op, (1, 12), (1, 13), ":" );
        test_token_w_position!(tokens[3], TType::NL, (1, 13), (1, 13), "" );
        test_token_w_position!(tokens[4], TType::Indent, (2, 0), (2, 0), "" );
        assert_eq!(tokens[5].r#type, TType::String);
        test_token_w_position!(tokens[6], TType::NL, (85, 7), (85, 7), "" );
        test_token_w_position!(tokens[7], TType::Dedent, (85, 0), (85, 0), "" );
    }

    #[test]
    fn test_issue_multiline_single_quote_string() {

        let tokens = Tokenizer::tokenize_file("test_fixtures/test_issue_multiline_single_quote_string.py", TConfig::default()).expect("tokens");

        let str1 =
r#""This is a multiline string with a continuation here, \
    which really fucks things up for me""#;

        test_token_w_position!(tokens[0], TType::Name, (1, 0), (1, 3), "foo" );
        test_token_w_position!(tokens[1], TType::Op, (1, 3), (1, 4), "(" );
        test_token_w_position!(tokens[2], TType::String, (1, 4), (2, 40), str1 );
        test_token_w_position!(tokens[3], TType::Op, (2, 40), (2, 41), ")" );
        test_token_w_position!(tokens[4], TType::NL, (2, 41), (2, 41), "" );

    }

    #[test]
    #[ignore]
    fn test_issue_test_smptd() {
        let tokens = Tokenizer::tokenize_file("test_fixtures/test_issue_test_smptpd.py", TConfig::default()).expect("tokens");

        for (idx, token) in tokens.iter().enumerate() {
            println!("{}:{:?}@{:?}:{:?}({:?})", idx, token.r#type, token.start.line, token.start.col, token.text );
        }

        test_token_w_position!(tokens[0], TType::Name, (1, 0), (1, 3), "def" );
        test_token_w_position!(tokens[1], TType::Name, (1, 4), (1, 18), "test_utf8_data" );
        test_token_w_position!(tokens[2], TType::Op, (1, 18), (1, 19), "(" );
        test_token_w_position!(tokens[3], TType::Name, (1, 19), (1, 23), "self" );
        test_token_w_position!(tokens[4], TType::Op, (1, 23), (1, 24), ")" );
        test_token_w_position!(tokens[5], TType::Op, (1, 24), (1, 25), ":" );
        test_token_w_position!(tokens[6], TType::NL, (1, 25), (1, 25), "" );
        test_token_w_position!(tokens[7], TType::Indent, (2, 0), (2, 0), "" );
        test_token_w_position!(tokens[8], TType::Name, (2, 4), (2, 8), "self" );
        test_token_w_position!(tokens[9], TType::Op, (2, 8), (2, 9), "." );
        test_token_w_position!(tokens[10], TType::Name, (2, 9), (2, 19), "write_line" );
        test_token_w_position!(tokens[11], TType::Op, (2, 19), (2, 20), "(" );
        //oh geeze... token.end position column is off by 4 according to my tokenizer against cPython's tokenize.c
        test_token_w_position!(tokens[12], TType::String, (3, 8), (3, 64), "'MAIL From: nai\u{308}ve@exampl?? BODY=8BITMIME SMTPUTF8'" );
        test_token_w_position!(tokens[13], TType::Op, (3, 64), (3, 65), "." );
        test_token_w_position!(tokens[14], TType::Name, (3, 65), (3, 71), "encode" );
        test_token_w_position!(tokens[15], TType::Op, (3, 71), (3, 72), "(" );
        test_token_w_position!(tokens[16], TType::String, (3, 72), (3, 79), "'utf-8'" );
        test_token_w_position!(tokens[17], TType::Op, (3, 79), (3, 80), ")" );
        test_token_w_position!(tokens[18], TType::Op, (3, 80), (3, 81), ")" );
        test_token_w_position!(tokens[19], TType::NL, (3, 81), (3, 81), "" );
        test_token_w_position!(tokens[20], TType::Name, (4, 4), (4, 8), "self" );
        test_token_w_position!(tokens[21], TType::Op, (4, 8), (4, 9), "." );
        test_token_w_position!(tokens[22], TType::Name, (4, 9), (4, 20), "assertEqual" );
        test_token_w_position!(tokens[23], TType::Op, (4, 20), (4, 21), "(" );
        test_token_w_position!(tokens[24], TType::Name, (4, 21), (4, 25), "self" );
        test_token_w_position!(tokens[25], TType::Op, (4, 25), (4, 26), "." );
        test_token_w_position!(tokens[26], TType::Name, (4, 26), (4, 33), "channel" );
        test_token_w_position!(tokens[27], TType::Op, (4, 33), (4, 34), "." );
        test_token_w_position!(tokens[28], TType::Name, (4, 34), (4, 40), "socket" );
        test_token_w_position!(tokens[29], TType::Op, (4, 40), (4, 41), "." );
        test_token_w_position!(tokens[30], TType::Name, (4, 41), (4, 45), "last" );
        test_token_w_position!(tokens[31], TType::Op, (4, 45), (4, 46), "[" );
        test_token_w_position!(tokens[32], TType::Number, (4, 46), (4, 47), "0" );
        test_token_w_position!(tokens[33], TType::Op, (4, 47), (4, 48), ":" );
        test_token_w_position!(tokens[34], TType::Number, (4, 48), (4, 49), "3" );
        test_token_w_position!(tokens[35], TType::Op, (4, 49), (4, 50), "]" );
        test_token_w_position!(tokens[36], TType::Op, (4, 50), (4, 51), "," );
        test_token_w_position!(tokens[37], TType::String, (4, 52), (4, 58), "b'250'" );
        test_token_w_position!(tokens[38], TType::Op, (4, 58), (4, 59), ")" );
        test_token_w_position!(tokens[39], TType::NL, (4, 59), (4, 59), "" );
        test_token_w_position!(tokens[40], TType::Dedent, (4, 0), (4, 0), "" );

    }

    #[test]
    fn test_multiline_concurrent_strings() {
        let tokens = Tokenizer::tokenize_file("test_fixtures/test_multiline_concurrent_strings.py", TConfig::default()).expect("tokens");

        let str1 =
r#""""Hello world this is  the first
    multiline string""""#;

        let str2 =
r#""""
    This is the second multiline""""#;

        test_token_w_position!(tokens[0], TType::Name, (1, 0), (1, 3), "def" );
        test_token_w_position!(tokens[1], TType::Name, (1, 4), (1, 7), "foo" );
        test_token_w_position!(tokens[2], TType::Op, (1, 7), (1, 8), "(" );
        test_token_w_position!(tokens[3], TType::Name, (1, 8), (1, 13), "first" );
        test_token_w_position!(tokens[4], TType::Op, (1, 13), (1, 14), "," );
        test_token_w_position!(tokens[5], TType::Name, (1, 15), (1, 21), "second" );
        test_token_w_position!(tokens[6], TType::Op, (1, 21), (1, 22), ")" );
        test_token_w_position!(tokens[7], TType::Op, (1, 22), (1, 23), ":" );
        test_token_w_position!(tokens[8], TType::NL, (1, 23), (1, 23), "" );
        test_token_w_position!(tokens[9], TType::Indent, (2, 0), (2, 0), "" );
        test_token_w_position!(tokens[10], TType::Name, (2, 4), (2, 9), "print" );
        test_token_w_position!(tokens[11], TType::Op, (2, 9), (2, 10), "(" );
        test_token_w_position!(tokens[12], TType::Name, (2, 10), (2, 13), "len" );
        test_token_w_position!(tokens[13], TType::Op, (2, 13), (2, 14), "(" );
        test_token_w_position!(tokens[14], TType::Name, (2, 14), (2, 19), "first" );
        test_token_w_position!(tokens[15], TType::Op, (2, 19), (2, 20), ")" );
        test_token_w_position!(tokens[16], TType::Op, (2, 20), (2, 21), ")" );
        test_token_w_position!(tokens[17], TType::NL, (2, 21), (2, 21), "" );
        test_token_w_position!(tokens[18], TType::Name, (3, 4), (3, 9), "print" );
        test_token_w_position!(tokens[19], TType::Op, (3, 9), (3, 10), "(" );
        test_token_w_position!(tokens[20], TType::Name, (3, 10), (3, 13), "len" );
        test_token_w_position!(tokens[21], TType::Op, (3, 13), (3, 14), "(" );
        test_token_w_position!(tokens[22], TType::Name, (3, 14), (3, 20), "second" );
        test_token_w_position!(tokens[23], TType::Op, (3, 20), (3, 21), ")" );
        test_token_w_position!(tokens[24], TType::Op, (3, 21), (3, 22), ")" );
        test_token_w_position!(tokens[25], TType::NL, (3, 22), (3, 22), "" );
        test_token_w_position!(tokens[26], TType::Dedent, (6, 0), (6, 0), "" );
        test_token_w_position!(tokens[27], TType::Name, (6, 0), (6, 3), "foo" );
        test_token_w_position!(tokens[28], TType::Op, (6, 3), (6, 4), "(" );
        test_token_w_position!(tokens[29], TType::String, (7, 4), (8, 23), str1  );
        test_token_w_position!(tokens[30], TType::Op, (8, 23), (8, 24), "," );
        test_token_w_position!(tokens[31], TType::String, (8, 25), (9, 35), str2 );
        test_token_w_position!(tokens[32], TType::Op, (10, 0), (10, 1), ")" );
        test_token_w_position!(tokens[33], TType::NL, (10, 1), (10, 1), "" );

    }


    #[test]
    fn test_escaped_chars_in_string() {
        let mut tokenizer = Tokenizer::new(TConfig::default());
        let escaped1 = r#""Hello World\" that's an escaped quote!""#;

        let tokens = tokenizer.process_single_line(escaped1.to_string()).expect("tokens");
        assert_eq!(tokens[0].r#type, TType::String);
        assert_eq!(tokens[0].text, escaped1);
    }

    #[test]
    fn test_unterminated_string() {
        let mut tokenizer = Tokenizer::new(TConfig::default());
        let escaped1 = r#""It's important to"#;

        let err = tokenizer.process_single_line(escaped1.to_string()).unwrap_err();
        assert_eq!(err, TokError::UnterminatedString);

        let escaped2 = r#"'Hello world this should crash"#;

        let err = tokenizer.process_single_line(escaped2.to_string()).unwrap_err();
        assert_eq!(err, TokError::UnterminatedString);

        let escaped3 = r#"'This is
        broken'"#;
        let err = tokenizer.process_single_line(escaped3.to_string()).unwrap_err();
        assert_eq!(err, TokError::UnterminatedString);

    }

    fn test_fstring_basic() {
        let tokens = Tokenizer::tokenize_file("test_fixtures/fstring_simple.py", TConfig::default()).expect("tokens");

        assert_eq!(tokens[0].r#type, TType::FStringStart);
        assert_eq!(tokens[1].r#type, TType::FStringString);
        assert_eq!(tokens[2].r#type, TType::FStringEnd);



    }
}