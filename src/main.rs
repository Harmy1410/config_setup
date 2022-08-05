use std::io::Read;
mod symlink_arr;
use ansi_term::Color;
use clap::Parser;

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
        "Path not provided! Checking default path: {:#?}\n",
        &args.path
    );

    if args.write {
        let mut to = String::new();
        let mut from = String::new();
        println!("{}", Color::RGB(245, 90, 66).bold().paint("Link FROM: "));
        let _ = std::io::stdin().read_line(&mut from);
        println!("{}", Color::RGB(245, 90, 66).bold().paint("Link TO: "));
        let _ = std::io::stdin().read_line(&mut to);
        symlink_arr::write(&to, &from);
    }

    let mut buf = String::new();
    std::fs::File::open(&args.path)?.read_to_string(&mut buf)?;
    symlink_arr::create_syms(&buf, &args.path);

    Ok(())
}
