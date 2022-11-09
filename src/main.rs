
mod lexer;
mod tokens;
mod parser;
mod ast;

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
    #[clap(long, short='C', value_parser)]
    check_code: Option<std::path::PathBuf>,

}




fn py_run_file(filename: PathBuf, show_tokens: bool)  {
    println!("Would read {:?}", filename);
    //Wow I need to find a better way to do this
    let display = filename.file_name().unwrap().to_str().unwrap().to_string();

    let mut file = std::fs::File::open(&filename).expect("Failed to open file");
    let mut buffer = String::new();

    println!("File opened, reading to string");
    match file.read_to_string(&mut buffer) {
        Err(why) => panic!("Couldn't read: {} because `{}`", display, why),
        Ok(len) => println!("{} is {} bytes long", display, len),
    }


    println!("Cleaning");
    let lines = cleaner(buffer);
    let mut tokenizer = Tokenizer::new(TConfig{ skip_encoding: true, skip_endmarker: false } );
    println!("Tokenizing");
    let outcome = tokenizer.generate(lines);
    println!("Tokenized!");

    if let Ok(tokens) = outcome {
        if show_tokens == true {
            println!("I got {} of tokens", tokens.len());
            for token in tokens.iter() {
                println!("\t{:#?}", token);
            }
        }


        let tvector = TokVec::from(tokens);
        let result = python::file(&tvector, &display.to_string().as_str());
        if let Ok(ptree) = result {
            println!("Parsing succeeded! \n");
        } else {
            println!("Failed to parse: {:#?}", result);
        }


    } else if let Err(issue) = outcome {
        panic!("Failed to process {:?} - token eror", issue);
    }




}

fn py_intrepret_string(line: String, _show_tokens: bool) {
    //TODO implement repl

    println!("Would run {}", line);
}




fn main() {
    let args = Args::parse();
    println!("Hello, world! {:?}", args);
    //TODO - Check if file is being pushed in via stdin
    //Check for filename argument
    if let Some(filename) = args.filename {
        py_run_file(filename, args.show_tokens);
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
