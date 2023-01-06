use csgo_gsi_builder::{
    config::{Config, Data},
    Builder, Components,
};

fn main() {
    let components: &[Components] = &[Components::Provider, Components::PlayerId];
    let mut config_builder = Builder::with_config(Config {
        name: String::from("my_gsi_config_file"),
        data: Data::from(components),
        ..Default::default()
    });
    config_builder
        .build()
        .install("C:\\Counter-Strike Global Offensive\\csgo\\cfg")
        .unwrap();
}
