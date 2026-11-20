use configurator_macros::config_builder;
use freezable_trait::freezable;
use toml::Table;

config_builder! {
    cache_file_location: String = "~/.config/themester/.themecache",
    write_term_lua: bool,
    term_lua_path: String = "~/.config/wezterm/dynamic_theme.lua",
    theme: Vec<Theme> = vec![
        Theme::default()
    ],
    environment {
        term_env_var_name: String = "TERM_THEME",
        nvim_plugin_env_var_name: String = "NVIM_THEME",
        nvim_theme_env_var_name: String = "NVIM_THEME_PLUGIN",
    },
}

#[derive(Debug)]
#[freezable]
struct Theme {
    #[default("plugin-name")]
    term: String,

    nvim_plugin: Option<String>,

    #[default("nvim-plugin-colorscheme")]
    nvim_themename: String,
}

#[test]
fn test_deser() {
    let config_str = include_str!("themester-conf.txt");
    let config = config_str
        .parse::<Table>()
        .expect("Could not parse config.toml!");
    let conf = config
        .clone()
        .try_into::<Config>()
        .expect("Could not deserialize toml.");
    assert_ne!(
        toml::to_string(&conf).unwrap(),
        toml::to_string(&Config::default()).unwrap()
    );
}
