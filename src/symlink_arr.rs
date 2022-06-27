use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Symlink {
    from: String,
    to: String,
}

type Symlinks = Vec<Symlink>;

pub fn create_syms(buf: &String) -> std::io::Result<()> {
    let arr: Symlinks = serde_json::from_str(buf)?;
    let mut exist_sym_count = 0;

    for i in arr.iter() {
        let mut to_home = match std::env::var("HOME") {
            Ok(home_user) => home_user,
            Err(_) => String::from("Set $HOME."),
        };

        let mut from_home = to_home.clone();
        to_home.push_str(&i.to);
        let to = to_home;
        from_home.push_str(&i.from);
        let from = from_home;

        let to_meta = match std::fs::symlink_metadata(&to) {
            Ok(metadata) => Ok(metadata),
            Err(_) => Err(()),
        };
        let from_meta = match std::fs::symlink_metadata(&from) {
            Ok(metadata) => Ok(metadata),
            Err(_) => Err(()),
        };

        if from_meta.is_ok() {
            if let Ok(meta) = to_meta {
                if meta.file_type().is_symlink() {
                    exist_sym_count += 1;
                }
            } else {
                println!("Creating link from: \t{}\n\t\tto: \t{}", i.from, i.to);
                // std::os::unix::fs::symlink(i.from, i.to)?;
            }
        } else {
            println!("Source file '{}' not found!!!", &from);
        }
    }

    if exist_sym_count == arr.len() {
        println!("Everything is fine!!");
    }

    Ok(())
}
