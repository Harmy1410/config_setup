#[derive(Debug)]
pub struct PathToConfig {
    pub path: String,
}
impl PathToConfig {
    pub fn new() -> PathToConfig {
        let show_sarcasm: bool = std::env::args().skip(1).len() > 1;
        if show_sarcasm {
            println!("No need to pass unnecessary stuff.😏\n");
        }

        let provided_path = std::env::args().nth(1);
        if let Some(path) = provided_path {
            PathToConfig { path }
        } else {
            let mut abs_path_to_config = String::new();
            let rel_path_to_config = String::from("/Developer/mmc/personal_config_symlinks.json");

            if let Ok(mut home) = std::env::var("HOME") {
                home.push_str(&rel_path_to_config);
                abs_path_to_config = home;
            } else {
                println!("Try setting $HOME variable.");
            }

            println!(
                "Path not provided! Checking default path: $HOME{}\n",
                rel_path_to_config
            );

            PathToConfig {
                path: abs_path_to_config.to_string(),
            }
        }
    }
}
