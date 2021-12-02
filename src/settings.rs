use anyhow::Result;
use config::{Config, File};
use serde::Deserialize;
use std::path::{Path, PathBuf};

use crate::stack::Stack;

const DEFAULT_SETTINGS_FILE: &str = "default";

#[derive(Clone, Debug, Deserialize)]
pub struct Logging {
    pub log_dir: PathBuf,
    pub verbosity: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    pub logging: Logging,
}

impl Settings {
    pub fn new(settings_dir: &Path, stack: Option<Stack>) -> Result<Self> {
        let mut s = Config::default();

        let settings_dir_str = settings_dir.to_str().unwrap();

        // Load default settings first
        s.merge(
            File::with_name(&format!("{}/{}", settings_dir_str, DEFAULT_SETTINGS_FILE))
                .required(true),
        )
        .map_err(|e| {
            anyhow::Error::new(e).context(format!(
                "Cannot read default settings from dir '{:?}'",
                settings_dir
            ))
        })?;

        // Apply stack settings, if a stack is defined
        if let Some(x) = stack {
            s.merge(File::with_name(&format!(
                "{}/{}",
                settings_dir_str,
                &x.to_string()
            )))
            .map_err(|e| {
                anyhow::Error::new(e).context(format!(
                    "Cannot read stack '{}' settings from dir '{:?}'",
                    x, settings_dir
                ))
            })?;
        }

        s.try_into().map_err(|e| {
            let context = "Cannot deserialize settings";
            anyhow::Error::new(e).context(context)
        })
    }
}
