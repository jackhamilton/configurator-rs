use std::fs::OpenOptions;
use std::{fs, path::Path, io};
use toml::Table;

pub fn get_config<'a, Config: serde::Deserialize<'a> + serde::Serialize + Default>(program_name: String) -> Config {
    setup_or_load_config_file(program_name, Config::default())
}

fn setup_or_load_config_file<'a, Config: serde::Serialize + serde::Deserialize<'a>>(program_name: String, default_config: Config) -> Config {
    let dir_path_string = shellexpand::tilde(&format!("~/.config/{program_name}/")).into_owned().to_string();
    let dir_path = Path::new(&dir_path_string);
    let dir_exists = fs::metadata(dir_path).is_ok();
    let config_path_string = shellexpand::tilde(&format!("~/.config/{program_name}/config.toml")).into_owned().to_string();

    let config_path = Path::new(&config_path_string);
    let config_file_exists = fs::metadata(config_path).is_ok();
    if !dir_exists {
        if let Err(why) = fs::create_dir(dir_path) {
            println!("! {:?}", why.kind());
        }
    }
    if !config_file_exists {
        touch(config_path).unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });
        let default_config = toml::to_string(&default_config).expect("Failed to serialize default config.");
        fs::write(config_path, default_config).expect("Unable to write config file.")
    }
    let config_contents = fs::read_to_string(config_path).expect("Could not read config.toml!");
    let config = config_contents.parse::<Table>().expect("Could not parse config.toml!");
    Config::deserialize(config).expect("Failed to deserialize config.")
}

fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).truncate(false).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
