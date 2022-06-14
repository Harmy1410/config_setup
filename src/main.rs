use serde::{Deserialize, Serialize};
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
struct ToFrom {
    from: String,
    to: String,
}
type Json = Vec<ToFrom>;

fn main() {
    let path = "/Users/harmeepatel/Developer/projects/rust/projects/config_setup/personal_config_symlinks.json";
    let mut buf = String::new();
    let mut json_file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(err) => panic!("{}", err),
    };

    match json_file.read_to_string(&mut buf) {
        Ok(_) => parse_json(&buf.to_string()),
        Err(err) => panic!("{}", err),
    }
}

fn parse_json(data: &str) {
    let data: Json = match serde_json::from_str(data) {
        Ok(data) => data,
        Err(err) => panic!("err in serde_json::from_str: {}", err),
    };

    println!("{:#?}", data);
}
