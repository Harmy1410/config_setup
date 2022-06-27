#[warn(unused_must_use)]
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Symlink {
    pub from: String,
    pub to: String,
}

impl Symlink {
    fn new() -> Symlink {
        Symlink {
            to: String::new(),
            from: String::new(),
        }
    }
}

type Symlinks = Vec<Symlink>;

pub fn create_syms(buf: &String) -> std::io::Result<()> {
    let arr: Symlinks = serde_json::from_str(buf)?;
    let mut exist_sym_count = 0;
    let arr: Symlinks = arr
        .iter()
        .map(|sym| {
            let env_home = vec!["$HOME", "~"];
            let home = match std::env::var("HOME") {
                Ok(home_user) => home_user,
                Err(_) => String::from("Set $HOME."),
            };
            let mut temp = Symlink::new();

            for i in env_home {
                if sym.to.contains(i) || sym.from.contains(i) {
                    temp.to = sym.to.replace(i, &home);
                    temp.from = sym.from.replace(i, &home);
                }
            }
            temp
        })
        .collect();

    for sym in &arr {
        let to_meta = match std::fs::symlink_metadata(&sym.to) {
            Ok(metadata) => Ok(metadata),
            Err(_) => Err(()),
        };
        let from_meta = match std::fs::symlink_metadata(&sym.from) {
            Ok(metadata) => Ok(metadata),
            Err(_) => Err(()),
        };

        if from_meta.is_ok() {
            if let Ok(meta) = to_meta {
                if meta.file_type().is_symlink() {
                    exist_sym_count += 1;
                }
            } else {
                println!("Creating link from: \t{}\n\t\tto: \t{}", sym.from, sym.to);
                std::os::unix::fs::symlink(&sym.from, &sym.to)?;
            }
        } else {
            println!("Source file '{}' not found!!!", sym.from);
            let mut conf_to_remove: console::Term = '';
            println!("Want to remove it from json config? (y/n): ",);
            conf_to_remove.read_char()?;
            dbg!(conf_to_remove);
        }
    }

    if exist_sym_count == arr.len() {
        println!("Everything is fine!!");
    }

    Ok(())
}
