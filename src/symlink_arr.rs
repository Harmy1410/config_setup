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

    for i in arr.iter() {
        if let Ok(metadata) = std::fs::symlink_metadata(i.to) {
            dbg!(metadata.file_type().is_symlink());
        };
        // let hello = std::os::unix::fs::symlink(&i.from, &i.to);
    }

    Ok(())
}
