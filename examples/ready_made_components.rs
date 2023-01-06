use csgo_gsi_builder::{config::Config, Builder, Components};

fn main() {
    let mut config_builder = Builder::with_config(Config {
        name: String::from("my_gsi_config_file"),
        data: Components::ALL.into(),
        ..Default::default()
    });
    config_builder
        .build()
        .install("C:\\Counter-Strike Global Offensive\\csgo\\cfg")
        .unwrap();
}
