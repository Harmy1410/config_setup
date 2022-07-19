use std::io::{self, Write};

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

fn replace_home(arr: Symlinks) -> Symlinks {
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
    arr
}

fn remove_non_existing(arr: &mut Symlinks, idx_remove: &Vec<usize>) {
    let mut c = 0;
    for i in idx_remove {
        let shifted_index = i - c;
        let _ = arr.remove(shifted_index);
        c += 1;
    }

    dbg!(arr);
}

pub fn create_syms(buf: &String) {
    let mut exist_sym_count = 0;
    let mut to_remove: Vec<usize> = Vec::new();

    let arr: Symlinks = serde_json::from_str(buf).unwrap();
    let mut arr: Symlinks = replace_home(arr);

    dbg!(&arr);
    for (id, sym) in arr.clone().iter().enumerate() {
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
                std::os::unix::fs::symlink(&sym.from, &sym.to).unwrap();
            }
        } else {
            println!("Source file '{}' not found!!!", sym.from);
            to_remove.push(id);
        }
    }

    print!(
        "Want to remove following objects from json config? (y/n): {:#?}",
        &to_remove
    );
    io::stdout().flush().unwrap();

    let mut rem_reply = String::new();
    std::io::stdin().read_line(&mut rem_reply).unwrap();
    let rem_reply = rem_reply.trim_end();

    if rem_reply == "y" {
        remove_non_existing(&mut arr, &to_remove);
    } else {
        println!("Bye.");
    }

    if exist_sym_count == arr.len() {
        println!("Everything is fine!!");
    }
}
