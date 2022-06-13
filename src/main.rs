use serde_json::{Result, Value};
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut json = std::fs::File::open("/Users/harmeepatel/Developer/projects/rust/projects/config_setup/personal_config_symlinks.json")?;
    let mut buf = String::new();

    json.read_to_string(&mut buf)?;
    println!("{buf}");
    untyped_example(&buf)?;

    Ok(())
}

fn untyped_example(data: &String) -> Result<()> {
    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(&data.to_string())?;

    // Access parts of the data by indexing with square brackets.
    println!("{}", v["one"]);

    Ok(())
}
