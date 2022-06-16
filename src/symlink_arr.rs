use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Symlink<'a> {
    pub from: &'a str,
    pub to: &'a str,
}

pub type SymlinkArr<'a> = Vec<Symlink<'a>>;
