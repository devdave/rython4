mod lexer;
mod tokens;

use std::io::Read;
use std::path::PathBuf;
use clap::Parser;

use crate::lexer::{ Tokenizer, TConfig, cleaner};





#[derive(Parser, Debug)]
struct Args {
    #[clap(index=1, value_parser)]
    filename: Option<std::path::PathBuf>,
    #[clap(long, short, value_parser)]
    interpret: Option<String>,
    #[clap(long, short='S', default_value="false", value_parser)]
    show_tokens: bool,
}




fn py_run_file(filename: PathBuf, show_tokens: bool)  {
    println!("Would read {:?}", filename);
    let display = filename.display();
    let mut file = std::fs::File::open(&filename).expect("Failed to open file");
    let mut buffer = String::new();

    match file.read_to_string(&mut buffer) {
        Err(why) => panic!("Couldn't read: {} because `{}`", display, why),
        Ok(len) => println!("{} is {} bytes long", display, len),
    }


    let mut lines = cleaner(buffer);
    let mut tokenizer = Tokenizer::new(TConfig{ skip_encoding: false, skip_endmarker: false } );
    let outcome = tokenizer.generate(lines);
    if let Ok(tokens) = outcome {
        println!("I got {} of tokens", tokens.len());
        for token in tokens.iter() {
            println!("\t{:?}", token);
        }
    } else if let Err(issue) = outcome {
        panic!("Failed to process {:?} - token eror", issue);
    }




}

fn py_intrepret_string(line: String, show_tokens: bool) {
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
