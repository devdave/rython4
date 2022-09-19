mod lexer;
mod tokens;

use clap::Parser;



#[derive(Parser, Debug)]
struct Args {
    #[clap(index=1, value_parser)]
    filename: Option<std::path::PathBuf>,
    #[clap(long, short, value_parser)]
    interpret: Option<String>,
    #[clap(long, short='S', default_value="false", value_parser)]
    show_tokens: bool,


}

fn main() {
    let args = Args::parse();
    println!("Hello, world! {:?}", args);
}
