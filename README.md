# Penguin Config

Penguin-config simplifies the creation of config files using [serde](https://github.com/serde-rs/serde) and [serde_json](https://github.com/serde-rs/json).


## Dependencies
#### Cargo.toml
```toml
[dependencies]
serde = { version = "1.0", features = ["serde_derive"] }
serde_json = { version = "1.0" }
penguin-config = { git = "https://github.com/Henrik-N/penguin-config", features = ["penguin-config-derive"] }
```

## Usage

#### window_config.json
```json
{
    "width": 640,
    "height": 400,
}
```

#### read_window_config.rs
```rust
use penguin_config::*;

#[derive(Deserialize, PenguinConfigFile)]
#[penguin_config(path = "window_config.json")]
struct WindowConfig {
    width: u32,
    height: u32,
}

fn read_window_config() {
    let config: WindowConfig = WindowConfig::read_config();
    
    assert_eq!(config.width, 640);
    assert_eq!(config.height, 400);
}
```
