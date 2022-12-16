# CSGO Game State Integration builder

CSGO Game State Integration configuration file builder and installer

## Exemples

You can use one of the ready made components:

```rust
use csgo_gsi_builder::{config::Config, Components, Builder};

let mut config_builder = Builder::with_config(Config {
    name: String::from("my_gsi_config_file"),
    data: Components::ALL.into(),
    ..Default::default()
});
config_builder.build().install("C:\\Counter-Strike Global Offensive\\csgo\\cfg").unwrap()
```

Or create your own set of components:

```rust
use csgo_gsi_builder::{config::{Config, Data} Components, Builder};

let components: &[Components] = &[Components::Provider, Components::PlayerId];
let mut config_builder = Builder::with_config(Config {
    data: Data::from(components),
    ..Default::default()
});
config_builder.build().install("C:\\Counter-Strike Global Offensive\\csgo\\cfg").unwrap()
```

## auto-install support

You can enable the `auto_install` feature to install automatically the
config into CSGO's cfg folder

```rust
use csgo_gsi_builder::{config::Config, Components, Builder};

let mut config_builder = Builder::with_config(Config {
    name: String::from("my_gsi_config_file"),
    data: Components::ALL.into(),
    ..Default::default()
});
config_builder.build().auto_install().unwrap()
```
