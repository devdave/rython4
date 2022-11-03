#[cfg(test)]
mod tests {
    use std::fs;



    use crate::parser::grammar::{python, TokenRef, TokVec};

    use crate::tokens::TType::{Op, Number};
    use std::rc::Rc;
    use crate::lexer::{Tokenizer, TConfig};
    // use super::{python, TokVec};
    use crate::ast::Module;

    use crate::ast::printer::print_module;
    // use crate::cleaner;
    // use crate::tokens::TType::String;


    #[test]
    fn basic() {
        let test = "1 + 2".to_string();
        let mut tokenizer = Tokenizer::new(TConfig{skip_encoding: true, skip_endmarker: false});
        let tokens = tokenizer.process_single_line(test).expect("tokens");

        for (pos, token) in tokens.clone().into_iter().enumerate() {
            println!("{pos}: {:?}", token);
        }

        let rctokens = tokens.into_iter().map(Rc::new).collect();
        let vec = TokVec(rctokens);

        let magic = python::expression_input(&vec);

        println!("{:?}", magic);

    }

    #[test]
    fn parse_operators() {
        let mut tokenizer = Tokenizer::new(TConfig { skip_encoding: true, skip_endmarker: false });
        let tokens = tokenizer.process_file("test_fixtures/operators.py").expect("tokens");

        for (pos, token) in tokens.clone().into_iter().enumerate() {
            println!("{pos}: {:?}", token);
        }

        let rctokens = tokens.into_iter().map(Rc::new).collect();
        let vec = TokVec(rctokens);



        let magic = python::file(&vec, "operators");


        if let Ok(module) = magic {
            print_module(module);
        } else {
            println!("{:?}", magic);
            assert!(false == true, "Parse error");
        }

    }

    #[test]
    fn parse_hello_world() {
        let tokens = Tokenizer::tokenize_file("test_fixtures/hello_world.py", TConfig{skip_encoding: true, skip_endmarker: false}).expect("tokens");

        let rctokens = tokens.into_iter().map(Rc::new).collect();
        let vec = TokVec(rctokens);

        let magic = python::file(&vec, "hello_world");
        let module: Module = magic.unwrap();

        assert_eq!(module.body.len(), 1);

        print_module(module);

    }

    fn attempt_parse_file<P>(filename:P)
    where P: AsRef<std::path::Path>
    {


        let display_str = filename.as_ref().display().to_string();
        let tokens = Tokenizer::tokenize_file(filename, TConfig{skip_encoding: true, skip_endmarker: false}).expect("Tokens");

        let vec = TokVec::from(tokens);

        let magic = python::file(&vec, &display_str.as_str());

        if let Ok(module) = magic {
            //Quiet this down
            //print_module(module);
            println!("Successfully parsed {}", display_str);

        } else if let Err(parse_loc) = magic {
            println!("Failed to parse: {:?} - because {:?}", display_str, parse_loc);
            for token_ref in vec.0.iter() {
                println!("Token: {:?}", token_ref);
            }
            panic!("Failed to parse!");

        }







    }

    #[test]
    fn parse_basic_class_fixture() {
        attempt_parse_file("test_fixtures/basic_class.py");
    }

    #[test]
    fn parse_basic_indent() {
        attempt_parse_file("test_fixtures/basic_indent.py");
    }

    #[test]
    fn parse_crazy_dents() {
        attempt_parse_file("test_fixtures/crazy_dents.py");
    }

    #[test]
    fn parse_multiline_strings() {
        attempt_parse_file("test_fixtures/multiline_strings.py");
    }

    #[test]
    fn parse_simple_typing() {
        attempt_parse_file("test_fixtures/simple_typing.py");
    }


    #[test]
    fn parse_all_python_fixtures() {
        //TODO refactor as this is a mess


        let paths = fs::read_dir("test_fixtures/").expect("paths");

        for test_path in paths {
            let path = test_path.expect("filepath").path();
            if path.is_file() {
                //skip invalid/error testing files
                if path.as_path().file_name().expect("fname").to_str().expect("fname string").starts_with("error") {
                    continue;
                }
                if path.as_path().to_str().unwrap().starts_with("error") {
                    continue;
                }

                if let Some(ext) = path.extension() {
                    if ext == "py" {
                        println!("Will parse: {:?}", path.display());
                        attempt_parse_file(path);
                    } else {
                        println!("Will not parse: {:?}", path.display());
                    }

                } else {
                    println!("Will not parse: failed to get extension {:?}", path.display());
                }

            } else {
                println!("Is not python file: {:?}", path.display());
            }

        }
    }

}
