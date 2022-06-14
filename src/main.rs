use serde_json::Value;
use std::io::Read;

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
    let data: Value = serde_json::from_str(data).unwrap();
    for i in data.as_array() {
        println!("{:?}", i);
    }
}
