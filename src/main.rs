use std::io::Read;
mod path_to_config;
mod symlink_arr;

fn main() -> std::io::Result<()> {
    let buf = String::new();
    let path = path_to_config::PathToConfig::new(&buf);
    let mut json_file = std::fs::File::open(&path.path)?;

    let mut buf = String::new();
    json_file.read_to_string(&mut buf)?;
    let arr: symlink_arr::SymlinkArr = serde_json::from_str(&buf)?;
    for i in arr.iter() {
        let hello = std::os::unix::fs::symlink(&i.from, &i.to);
        dbg!(&hello);
    }
    Ok(())
}

