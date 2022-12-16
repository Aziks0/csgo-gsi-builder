//! CSGO Game State Integration builder
//!
//! CSGO Game State Integration configuration file builder and installer
//!
//! # Exemples
//!
//! You can use one of the ready made components:
//!
//! ```no_run
//! use csgo_gsi_builder::{config::Config, Components, Builder};
//!
//! let mut config_builder = Builder::with_config(Config {
//!     name: String::from("my_gsi_config_file"),
//!     data: Components::ALL.into(),
//!     ..Default::default()
//! });
//! config_builder.build().install("C:\\Counter-Strike Global Offensive\\csgo\\cfg").unwrap()
//! ```
//!
//! Or create your own set of components:
//!
//! ```no_run
//! use csgo_gsi_builder::{config::{Config, Data} Components, Builder};
//!
//! let components: &[Components] = &[Components::Provider, Components::PlayerId];
//! let mut config_builder = Builder::with_config(Config {
//!     data: Data::from(components),
//!     ..Default::default()
//! });
//! config_builder.build().install("C:\\Counter-Strike Global Offensive\\csgo\\cfg").unwrap()
//! ```
//!
//! # auto-install support
//!
//! You can enable the `auto_install` feature to install automatically the
//! config into CSGO's cfg folder
//!
//! ```no_run
//! use csgo_gsi_builder::{config::Config, Components, Builder};
//!
//! let mut config_builder = Builder::with_config(Config {
//!     name: String::from("my_gsi_config_file"),
//!     data: Components::ALL.into(),
//!     ..Default::default()
//! });
//! config_builder.build().auto_install().unwrap()
//! ```

mod components;
pub use components::Components; // re-export to avoid repetition

pub mod config;

use std::fs;
use std::path::PathBuf;

use crate::config::Config;

#[derive(Default)]
pub struct Builder {
    config: Config,
    output: String,
}

impl Builder {
    /// Creates a config file builder based on a [`Config`].
    pub fn with_config(config: Config) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }

    /// Serializes the [`Config`] to a string ready to be written in a cfg file.
    pub fn build(&mut self) -> &mut Self {
        self.output = vdf_serde::to_string(&self.config).unwrap();
        self
    }

    /// Gets the serialized [`Config`].
    ///
    /// This method must only be called after `build`.
    pub fn output(&self) -> String {
        self.output.clone()
    }

    /// Write the serialized [`Config`] to a cfg file at the path passed in
    /// argument.
    pub fn install<P: Into<PathBuf>>(
        &self,
        folder_path: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        assert!(
            !self.output.is_empty(),
            "config should be build before installing"
        );

        let mut path: PathBuf = folder_path.into();

        let folder_path = path.as_path();
        let metadata = fs::metadata(folder_path)?;
        if !metadata.is_dir() {
            return Err(format!("{} is not a directory", folder_path.to_string_lossy()).into());
        }

        path.push(&format!("gamestate_integration_{}.cfg", self.config.name));
        fs::write(path, self.output.as_bytes())?;

        Ok(())
    }

    /// Automatically find the CSGO install directory and write the serialized.
    /// [`Config`] to a cfg file
    #[cfg(feature = "auto_install")]
    pub fn auto_install(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut steam_dir =
            steamlocate::SteamDir::locate().ok_or("could not locate steam install directory")?;
        match steam_dir.app(&730) {
            None => Err("could not locate CSGO install directory".into()),
            Some(csgo) => {
                let mut csgo_path = csgo.path.clone();
                csgo_path.push("csgo");
                csgo_path.push("cfg");

                self.install(csgo_path)
            }
        }
    }
}
