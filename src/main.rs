use std::io::Read;
mod symlink_arr;
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
        default_value = "/Developer/mmc/config_sym.json"
    )]
    path: String,
}

fn main() -> std::io::Result<()> {
    let mut args = Args::parse();

    let rel_path = args.path.to_owned();

    if let Ok(mut home) = std::env::var("HOME") {
        home.push_str(&args.path);
        args.path = home;
    } else {
        println!("Try setting $HOME variable.");
    }

    println!(
        "Path not provided! Checking default path: $HOME {}\n",
        &args.path
    );

    let mut buf = String::new();
    std::fs::File::open(&args.path)?.read_to_string(&mut buf)?;

    // file for this
    symlink_arr::create_syms(&buf, &args.path);

    Ok(())
}
