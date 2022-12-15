//! CSGO Game State Integration builder

pub mod components;
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
}
