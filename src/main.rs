#[derive(Debug)]
struct PathToConfig {
    path: String,
}
impl PathToConfig {
    fn new(path: &String) -> PathToConfig {
        let show_sarcasm: bool = std::env::args().skip(1).len() > 1;
        if show_sarcasm {
            println!("no need to pass unnecessary stuff!!!\n");
        }

        let default_path_to_config = String::from(path);
        // let default_path_to_config = String::from(prod_path);
        let provided_path = std::env::args().nth(1);
        if let Some(path) = provided_path {
            PathToConfig { path }
        } else {
            println!("Path not provided! Checking default path: {default_path_to_config}\n");
            PathToConfig {
                path: default_path_to_config,
            }
        }
    }
}

fn main() {
    let dev_path: String = String::from("/Desktop");
    // // const PROD_PATH: &str = "/Developer";
    // let buf = String::new();
    // let path = Path::new("./").to_str();

    let mmc = PathToConfig::new(&dev_path);
    println!("{:#?}", mmc.path);
    if let Some(mut path) = home::home_dir() {
        println!("{:?}", path.push(&mmc.path));
    }
}
