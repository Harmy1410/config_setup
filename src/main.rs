#[derive(Debug)]
struct PathToConfig {
    path: String,
}

const DEV_PATH: &str = "/Desktop";
// const PROD_PATH: &str = "/Developer";

fn main() {
    let mmc = PathToConfig::new();
    println!("{:#?}", mmc.path);
    if let Some(path) = home::home_dir() {
        println!("{}", path.display());
    }
}

impl PathToConfig {
    fn new() -> PathToConfig {
        let show_sarcasm: bool = std::env::args().skip(1).len() > 1;
        if show_sarcasm {
            println!("no need to pass unnecessary stuff!!!\n");
        }

        let default_path_to_config = String::from(DEV_PATH);
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
