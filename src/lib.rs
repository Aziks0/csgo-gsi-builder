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
}
