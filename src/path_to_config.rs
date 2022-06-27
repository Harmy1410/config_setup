#[derive(Debug)]
pub struct PathToConfig {
    pub path: String,
}
impl PathToConfig {
    pub fn new() -> PathToConfig {
        let show_sarcasm: bool = std::env::args().skip(1).len() > 1;
        if show_sarcasm {
            println!("no need to pass unnecessary stuff!!!\n");
        }

        let default_path_to_config =
            "/Users/harmeepatel/Developer/mmc/personal_config_symlinks.json";
        let provided_path = std::env::args().nth(1);
        if let Some(path) = provided_path {
            PathToConfig { path }
        } else {
            println!("Path not provided! Checking default path: {default_path_to_config}\n");
            PathToConfig {
                path: default_path_to_config.to_string(),
            }
        }
    }
}
