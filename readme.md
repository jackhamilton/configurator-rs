Makes adding a configuration file to a rust program trivial.

Available via cargo:

```zsh
cargo add toml-configurator
```

# Features
- Automatically manages file in ~/.config/\[program_name\]

If such a file doesn't exist, one will be created with the fields initialized to their default values. If the file exists, but some fields are not there, it won't be touched but missing fields will be read as their default values.

- Creates static lazy-locked property containing config values read at launch

Configuration properties can be accessed via CONFIG.\[field], for example CONFIG.example_field in the below snippet.

- Configs implement freezable-trait

This is another project of mine, all this means is that the given struct can be easily serialized and deserialized without messing with serde. Trait provides write_to_file() and from_file() methods, alongside to and from string methods. [https://github.com/jackhamilton/freezable] for more information.


``` rust
// This macro will create a file, ~/.config/program_name/config.toml with these properties set to their defaults.
// Changes to this file will be reflected in the value of the macro-created CONFIG static.
config_builder! {
    example_field: i32 = 67,
    another_field: String = "test".to_string()
}

fn main() {
    println!("{}" CONFIG.example_field); // Prints '67' if you haven't changed the file, or whatever int you have in there if you have.
}
```


~/.config/program_name/config.toml:
```toml
example_field = 67
another_field = "test"
```
