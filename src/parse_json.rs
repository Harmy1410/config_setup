use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct ToFrom {
    from: String,
    to: String,
}
// pub type Json = Vec<ToFrom>;
//
// fn parse_json(data: &str) {
//     let data: Json = match serde_json::from_str(data) {
//         Ok(data) => data,
//         Err(err) => panic!("err in serde_json::from_str: {}", err),
//     };
//
//     println!("{:#?}", data);
// }
