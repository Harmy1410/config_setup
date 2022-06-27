use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Symlink<'a> {
    from: &'a str,
    to: &'a str,
}

type Symlinks<'a> = Vec<Symlink<'a>>;

pub fn create_syms(buf: &String) -> std::io::Result<()> {
    let arr: Symlinks = serde_json::from_str(buf)?;
    let mut sym_link_count = 0;

    for i in arr.iter() {
        let metadata: Result<std::fs::Metadata, &'static str> =
            match std::fs::symlink_metadata(i.to) {
                Ok(metadata) => Ok(metadata),
                Err(_) => Err("Path not correct"),
            };
        if let Ok(metadata) = metadata {
            if metadata.file_type().is_symlink() {
                sym_link_count += 1;
            }
        } else {
            println!("Creating link from: \t{}\n\t\tto: \t{}", i.from, i.to);
            // std::os::unix::fs::symlink(i.from, i.to)?;
        }
    }

    if sym_link_count == arr.len() {
        println!("Everything is fine!!");
    }

    Ok(())
}
