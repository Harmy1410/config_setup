use serde_json::{Result, Value};
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut json = std::fs::File::open("/Users/harmeepatel/Developer/projects/rust/projects/config_setup/personal_config_symlinks.json")?;
    let mut buf = String::new();

    json.read_to_string(&mut buf)?;
    println!("{buf}");
    println!("{}", std::any::type_name<&str>());
    untyped_example(&buf)?;

    Ok(())
}

fn untyped_example(data: &String) -> Result<()> {
    let v: Value = serde_json::from_str(&data.to_string())?;

    println!("{}", v["one"]);

    Ok(())
}
