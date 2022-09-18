mod lexer;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[clap(long, value_parser)]
    filename: Option<std::path::PathBuf>,

}

fn main() {
    println!("Hello, world!");
}
