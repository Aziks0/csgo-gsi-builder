//! CSGO Game State Integration builder

mod components;
pub use components::Components;

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
    pub fn with_config(config: Config) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }

    pub fn build(&mut self) -> &mut Self {
        self.output = vdf_serde::to_string(&self.config).unwrap();
        self
    }

    pub fn output(&self) -> String {
        self.output.clone()
    }

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
