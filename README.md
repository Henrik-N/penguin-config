# Penguin Config

Penguin-config simplifies the creation of config files using [serde](https://github.com/serde-rs/serde) and [serde_json](https://github.com/serde-rs/json).


## Dependencies
#### Cargo.toml
```toml
[dependencies]
serde = "1.0"
penguin-config = "0.1.1"
```

## Usage

##### window_config.json
```json
{ "width": 640, "height": 400 }
```

### Using derive macros
##### Generating a config file
```rust
use penguin_config::*;

// the PenguinConfigGenerate macro will use the std::ops::Default implementation of the struct
#[derive(Serialize, PenguinConfigGenerate, Default)]
#[penguin_config(path = "window_config.json")]
struct WindowConfig {
    width: u32,
    height: u32,
}

fn generate_config() {
    WindowConfig::generate_penguin_config_file();
}
```

##### Reading a config file
```rust
use penguin_config::*;

#[derive(Deserialize, PenguinConfigFile)]
#[penguin_config(path = "window_config.json")]
struct WindowConfig {
    width: u32,
    height: u32,
}

fn read_config() {
    let config = WindowConfig::read_config();

    assert_eq!(config.width, 640);
    assert_eq!(config.height, 400);
}
```


### Using traits
##### Generating a config file
```rust
use penguin_config::*;

#[derive(Serialize)]
struct WindowConfig {
    width: u32,
    height: u32,
}
impl PenguinConfigGenerate for WindowConfig {
    fn generate_penguin_config_file() {
        let config = WindowConfig { width: 640, height: 400 };
        Serializer::file_path("window_config.json").serialize(&config);
    }
}
```

##### Reading a config file
```rust
use penguin_config::*;

#[derive(Deserialize)]
struct WindowConfig {
    width: u32,
    height: u32,
}
impl PenguinConfig for WindowConfig {
    fn read_config() -> Self {
        let config: Self = Deserializer::file_path("window_config.json").deserialize();
        config
    }
}
```

## License
Licensed under [MIT](https://github.com/Henrik-N/penguin-config/blob/master/LICENSE). Do whatever you want with it :)
