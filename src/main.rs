extern crate pretty_env_logger;
#[macro_use] extern crate log;

mod lexer;
mod tokens;
mod parser;
mod ast;
mod walker;

use std::io::Read;
use std::path::PathBuf;
use clap::Parser;

use crate::lexer::{ Tokenizer, TConfig, cleaner};
use crate::parser::grammar::{python, TokVec};







#[derive(Parser, Debug)]
struct Args {
    #[clap(index=1, value_parser)]
    filename: Option<std::path::PathBuf>,
    #[clap(long, short, value_parser)]
    interpret: Option<String>,
    #[clap(long, short='S', default_value="false", value_parser)]
    show_tokens: bool,
    #[clap(long, short='C', default_value="false", value_parser)]
    check_code: bool,

}

fn py_check_code(filepath: PathBuf) {
    if filepath.is_dir() {
        for entry in filepath.read_dir().expect("directory") {
            if let Ok(test_path) = entry {
                if test_path.path().is_dir() {
                    py_check_code(test_path.path());
                } else {
                    py_run_file(test_path.path(), false, true);
                }
            }
        }
    } else {
        py_run_file(filepath, false, true);
    }
}




fn py_run_file(filename: PathBuf, show_tokens: bool, compile_only: bool)  {
    println!("Would read {:?}", filename);
    //Wow I need to find a better way to do this
    let display = filename.file_name().unwrap().to_str().unwrap().to_string();

    if compile_only == true && display.ends_with(".py") == false {
         println!("Will not compile/parse {} as it doesn't end with .py", display);
         return;
    }

    let mut file = std::fs::File::open(&filename).expect("Failed to open file");
    let mut buffer = String::new();

    println!("File opened, reading to string");
    match file.read_to_string(&mut buffer) {
        Err(why) => {
            println!("Couldn't read: {} because `{}`", display, why);
            return;
        },
        Ok(len) => println!("{} is {} bytes long", display, len),
    }


    println!("Cleaning");
    let lines = cleaner(buffer);
    let mut tokenizer = Tokenizer::new(TConfig{ skip_encoding: true, skip_endmarker: false } );
    println!("Tokenizing");
    let outcome = tokenizer.generate(&lines);
    println!("Tokenizer finished!\n");

    if let Ok(tokens) = outcome {
        if show_tokens == true {
            println!("I got {} of tokens", tokens.len());
            for token in tokens.iter() {
                println!("\t{:#?}", token);
            }
        }

        if compile_only == true {
            return;
        }


        let tvector = TokVec::from(tokens);
        let result = python::file(&tvector, &display.to_string().as_str());
        if let Ok(_ptree) = result {
            println!("Parsing succeeded! \n");
        }
        else if let Err(parse_err) = result {
            println!("Failed to parse: {:#?}", parse_err);
            println!("Line error @ {:#?}:{:#?}", parse_err.location.start_pos.line, parse_err.location.start_pos.column);
            let lineno = parse_err.location.start_pos.line;

            let line = lines.get(lineno-1).unwrap();
            println!("{:#?}", line);
            for token_ref in tvector.0 {
                if token_ref.start.line >= lineno-1 || token_ref.start.line == lineno {
                    println!("{:?} ({}, {}) {:?}", token_ref.r#type, token_ref.start.line, token_ref.start.col, token_ref.text);
                }
            }
        }
        else {
            println!("Failed to parse: {:#?}", result);

        }


    }
    else if let Err(issue) = outcome {
        println!("Failed to tokenize {:?} - token error", issue);
    }




}

fn py_intrepret_string(line: String, _show_tokens: bool) {
    //TODO implement repl

    println!("Would run {}", line);
}




fn main() {
    pretty_env_logger::init();

    let args = Args::parse();

    debug!("Got args: {:?}", args);
    //TODO - Check if file is being pushed in via stdin

    //Check for filename argument
    if let Some(filename) = args.filename {
        if args.check_code == true {
            py_check_code(filename);
        } else {
            py_run_file(filename, args.show_tokens, false);
        }

    }
    //Check for interpret
    else if let Some(interpret_line) = args.interpret {
        py_intrepret_string(interpret_line, args.show_tokens);
    }
    //else run interactive
    else {
        println!("Will do interactive console!");
    }

}
