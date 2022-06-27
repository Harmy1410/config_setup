use std::io::Read;
mod path_to_config;
mod symlink_arr;

fn main() -> std::io::Result<()> {
    // file for this
    let config = path_to_config::PathToConfig::new();

    let mut buf = String::new();
    std::fs::File::open(&config.path)?.read_to_string(&mut buf)?;

    // file for this
    symlink_arr::create_syms(&buf)?;

    Ok(())
}
