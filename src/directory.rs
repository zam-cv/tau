use crate::context::Config;
use crate::utils::dir::rebuild_dir;
use anyhow::{anyhow, Result};
use directories;
use include_dir::{include_dir, Dir};
use std::{fs, path::PathBuf};
use colored::Colorize;

// Program Directories
const PROJECT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/public/");

// General information
const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "tau";
const APPLICATION: &str = "dev";

// Program Files and Folders
const CONFIG: &str = "config.json";
const TEMPLATES: &str = "templates";

pub struct Directory {
    pub root: PathBuf,
    pub config: PathBuf,
    pub templates: PathBuf,
}

impl Directory {
    pub fn new() -> Result<Directory> {
        let dir = directories::ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION);

        if let Some(dir) = dir {
            let path = dir.config_dir();

            // If the folder does not exist, it is created
            if !path.exists() {
                fs::create_dir_all(&path)?;
            }

            let root = path.to_path_buf();
            let config = path.join(&CONFIG);
            let templates = path.join(&TEMPLATES);

            rebuild_dir(&PROJECT_DIR, &root)?;

            return Ok(Directory {
                root,
                config,
                templates,
            });
        }

        Err(anyhow!("Not found root directory"))
    }

    pub fn get_config(&self) -> Result<Config> {
        let config = fs::read_to_string(&self.config)?;

        if let Ok(config) = serde_json::from_str(&config) {
            return Ok(config);
        }

        Err(anyhow!("Error parsing config file"))
    }

    pub fn display(&self) -> Result<()> {
        println!(
            "\nConfig: {}\nTemplates: {}",
            format!("\"{}\"", self.config.display()).yellow(),
            format!("\"{}\"", self.templates.display()).yellow()
        );

        Ok(())
    }
}