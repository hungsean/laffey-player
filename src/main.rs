use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "laffey-player",
    about = "A lightweight music player",
    // override_usage = "laffey-player [OPTIONS] <FILE>\n       laffey-player -f <FILE>"
)]
struct Cli {
    /// Path to the music file
    #[arg(value_name = "FILE", conflicts_with = "file")]
    input: Option<String>,

    /// Path to the music file
    #[arg(short = 'f', long = "file", value_name = "FILE")]
    file: Option<String>,
}

fn main() {
    let cli: Cli = Cli::parse();

    let target: Option<String> = cli.file.or(cli.input);

    match target {
        Some(path) => {
            println!("Playing: {path}");
            // TODO: implement playback
        }
        None => {
            eprintln!("Error: no music file specified");
            eprintln!("Hint:  laffey-player <FILE>");
            eprintln!("       laffey-player --file <FILE>");
            eprintln!("       laffey-player --help   for more information");
            std::process::exit(1);
        }
    }
}
