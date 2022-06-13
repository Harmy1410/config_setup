#[derive(Debug)]
struct PathToConfig {
    path: String,
}

const DEV_PATH: &str = "~/Desktop";
// const PROD_PATH: &str = "~/Developer";

fn main() -> std::io::Result<()> {
    let mmc = PathToConfig::new();
    // let mmc = std::path::Path::new(&mmc.path);
    // let a = std::env::set_current_dir(&mmc);
    let file_path = std::path::PathBuf::from(&mmc.path).join("hello.txt");
    let a = std::fs::write(&file_path, "Hello")?;
    println!("{:#?}", file_path);
    println!("{:#?}", a);

    for entry in std::fs::read_dir(&mmc.path)? {
        let dir = match entry {
            Ok(entry) => println!("{entry:?}"),
            Err(e) => println!("{e:?}"),
        };
        println!("{:#?}", dir);
    }

    // println!("{:#?}", mmc.display());

    Ok(())
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
