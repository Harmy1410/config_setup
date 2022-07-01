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

fn remove_non_existing() {
    println!("non")
}

pub fn create_syms(buf: &String) {
    let mut exist_sym_count = 0;

    let mut to_remove: Vec<usize> = Vec::new();

    let arr: Symlinks = serde_json::from_str(buf).unwrap();
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

    let mut buf = String::new();
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

    println!(
        "Want to remove following objects from json config? (y/n): {:#?}",
        &to_remove.iter().map(|item| { arr[item] }).collect()
    );
    io::stdout().flush().unwrap();

    std::io::stdin().read_line(&mut buf).unwrap();
    let buf = buf[0..1].to_owned();
    if buf == "y" {
        remove_non_existing()
    } else {
        println!("Bye.");
    }

    if exist_sym_count == arr.len() {
        println!("Everything is fine!!");
    }
}
