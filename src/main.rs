use clap::Parser;

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(short = 'f', long = "file")]
    file: String,
}

fn main() {
    let args: Args = Args::parse();
    println!("file: {}", args.file);
}
