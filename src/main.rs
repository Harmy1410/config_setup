use std::io::Read;
mod handler;
use ansi_term::Color;
use clap::Parser;

const ORANGE: ansi_term::Color = Color::RGB(245, 90, 66);

/// Program to setup all config files with a json file.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to config file
    #[clap(
        short,
        long,
        value_parser,
        default_value = "~/Developer/mmc/config_sym.json"
    )]
    path: String,

    /// Insert into json
    #[clap(short, long, value_parser, default_value_t = false)]
    write: bool,

    /// Remove Non-existing links from json
    #[clap(short, long, value_parser, default_value_t = false)]
    remove: bool,
}

fn main() -> std::io::Result<()> {
    let mut args = Args::parse();

    if let Ok(home) = std::env::var("HOME") {
        if args.path.contains("~") || args.path.contains("$HOME") {
            args.path = args.path.replace("~", &home);
        }
    } else {
        println!("Try setting $HOME variable.");
    }

    println!(
        "{}{}\n",
        Color::Yellow
            .bold()
            .paint("Path not provided! Checking default path: "),
        Color::Yellow.bold().paint(&args.path)
    );

    if args.write {
        let mut to = String::new();
        let mut from = String::new();
        println!("{}", ORANGE.bold().paint("Link FROM: "));
        let _ = std::io::stdin().read_line(&mut from);
        println!("{}", ORANGE.bold().paint("Link TO: "));
        let _ = std::io::stdin().read_line(&mut to);
        handler::write(&to, &from);
    }

    let mut buf = String::new();
    std::fs::File::open(&args.path)?.read_to_string(&mut buf)?;
    handler::create_syms(&buf, &args.path, &args.remove);

    Ok(())
}
