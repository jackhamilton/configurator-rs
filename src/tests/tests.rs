use toml::Table;
use freezable_trait::freezable;
use configurator_macros::config_builder;

config_builder! {
    cache_file_location: String = "~/.config/themester/.themecache".into(),
    term_env_var_name: String = "TERM_THEME".into(),
    nvim_plugin_env_var_name: String = "NVIM_THEME".into(),
    nvim_theme_env_var_name: String = "NVIM_THEME_PLUGIN".into(),
    write_term_lua: bool = true,
    term_lua_path: String = "~/.config/wezterm/dynamic_theme.lua".into(),
    theme: Vec<Theme> = vec![
        Theme::default()
    ],
}

#[derive(Debug)]
#[freezable]
struct Theme {
    term: String,
    nvim_plugin: Option<String>,
    nvim_themename: String,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            term: "plugin-name".into(),
            nvim_plugin: None,
            nvim_themename: "nvim-plugin-colorscheme".into(),
            _unknown_fields: [].into(),
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
