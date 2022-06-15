use std::io::Read;
mod path_to_config;

fn main() -> std::io::Result<()> {
    let buf = String::new();
    let path = path_to_config::PathToConfig::new(&buf);
    let mut json_file = std::fs::File::open(&path.path)?;

    let mut buf = String::new();
    json_file.read_to_string(&mut buf)?;
    println!("{buf:#}");
    Ok(())
}
