use std::io::{self, Write};

#[warn(unused_must_use)]
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Symlink {
    pub from: String,
    pub to: String,
}

enum Remove {
    Yes,
    No,
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

fn replace_home(arr: &mut Symlinks, remove: Remove) -> Symlinks {
    let new_arr: Symlinks = match remove {
        Remove::Yes => arr
            .iter()
            .map(|sym| {
                let home = match std::env::var("HOME") {
                    Ok(home_user) => home_user,
                    Err(_) => String::from("Set $HOME."),
                };

                let env_home = vec![&home];
                let mut temp = Symlink::new();

                let home = String::from("~");
                for i in env_home {
                    if sym.to.contains(i) || sym.from.contains(i) {
                        temp.to = sym.to.replace(i, &home);
                        temp.from = sym.from.replace(i, &home);
                    }
                }
                temp
            })
            .collect(),
        Remove::No => arr
            .iter()
            .map(|sym| {
                let home = match std::env::var("HOME") {
                    Ok(home_user) => home_user,
                    Err(_) => String::from("Set $HOME."),
                };

                let env_home = vec!["$HOME", "~"];
                let mut temp = Symlink::new();

                for i in env_home {
                    if sym.to.contains(i) || sym.from.contains(i) {
                        temp.to = sym.to.replace(i, &home);
                        temp.from = sym.from.replace(i, &home);
                    }
                }
                temp
            })
            .collect(),
    };
    new_arr
}

fn remove_non_existing(
    arr: &mut Symlinks,
    idx_remove: &Vec<usize>,
    config_path: &String,
) -> std::io::Result<()> {
    let mut c = 0;
    for i in idx_remove {
        let shifted_index = i - c;
        let _ = arr.remove(shifted_index);
        c += 1;
    }
    let arr: Symlinks = replace_home(arr, Remove::Yes);

    // serde_json::to_writer(&std::fs::File::create(config_path)?, &arr).unwrap();

    println!("arr from remove_non_existing: {:#?}", &arr);
    Ok(())
}

pub fn create_syms(buf: &String, path: &String) {
    let mut exist_sym_count = 0;
    let mut to_remove: Vec<usize> = Vec::new();

    let mut arr: Symlinks = serde_json::from_str(buf).unwrap();
    let mut arr: Symlinks = replace_home(&mut arr, Remove::No);

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

    dbg!(&to_remove);
    if to_remove.len() > 0 {
        print!("Want to remove following objects from json config? (y/n): ");
        io::stdout().flush().unwrap();

        let mut rem_reply = String::new();
        std::io::stdin().read_line(&mut rem_reply).unwrap();
        let rem_reply = rem_reply.trim_end();

        if rem_reply == "y" {
            // dbg!(&arr, &path);
            if remove_non_existing(&mut arr, &to_remove, &path).is_ok() {
                println!("Removed.");
                return;
            } else {
                println!("Something went wrong!!!");
            }
        } else {
            println!("Bye.");
        }
    }

    if exist_sym_count == arr.len() {
        println!("Everything is fine!!");
    }
}
