use toml::Table;
use freezable_trait::freezable;

#[derive(Clone, Debug)]
#[freezable]
struct Config {
    cache_file_location: String,
    term_env_var_name: String,
    nvim_plugin_env_var_name: String,
    nvim_theme_env_var_name: String,
    write_term_lua: bool,
    term_lua_path: String,
    theme: Vec<Theme>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cache_file_location: "~/.config/themester/.themecache".into(),
            term_env_var_name: "TERM_THEME".into(),
            nvim_theme_env_var_name: "NVIM_THEME".into(),
            nvim_plugin_env_var_name: "NVIM_THEME_PLUGIN".into(),
            write_term_lua: true,
            term_lua_path: "~/.config/wezterm/dynamic_theme.lua".into(),
            theme: vec![
                Theme::default()
            ]
        }
    }
}

#[derive(Clone, Debug)]
#[freezable]
struct Theme {
    term: String,
    nvim_plugin: String,
    nvim_themename: String,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            term: "plugin-name".into(),
            nvim_plugin: "nvim-plugin-name".into(),
            nvim_themename: "nvim-plugin-colorscheme".into()
        }
    }
}

#[test]
fn test_deser() {
    let config_str = include_str!("themester-conf.txt");
    let config = config_str.parse::<Table>().expect("Could not parse config.toml!");
    let conf = config.clone().try_into::<Config>().expect("Could not deserialize toml.");
    assert_ne!(toml::to_string(&conf).unwrap(), toml::to_string(&Config::default()).unwrap());
}
