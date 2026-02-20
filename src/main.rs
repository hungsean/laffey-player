use clap::Parser;
mod utils;
use crate::utils::is_file_valid;
use std::path::Path;


#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(short = 'f', long = "file")]
    file: String,
}

fn main() {
    let args: Args = Args::parse();
    println!("file: {}", args.file);
    match is_file_valid(Path::new(&args.file)) {
        Ok(()) => println!("成功"),
        Err(e) => println!("{}", e)
    }
    
}
