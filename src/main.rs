use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(name = "setup_config", author = "harmeepatel", version = "2.0.0", about = "CLI tool to put files in place.", long_about = None)]
struct CLI {
    /// Name of the person to greet
    #[arg(short = 'p', long, default_value_t = String::from("~/Developer/mmc/config_sym.json"))]
    path: String,

    /// Number of times to greet
    #[arg(short = 'w', long, default_value_t = 0)]
    write: u8,
}

fn main() {
    let args = CLI::parse();

    println!("Hello {}!", &args.path)
}
