use crate::{
    context::Config,
    utils::dir::rebuild_dir,
    exec::Commands
};
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
const COMMANDS: &str = "commands.json";
const CONFIG: &str = "config.json";
const TEMPLATES: &str = "templates";

pub struct Directory {
    pub root: PathBuf,
    pub config: PathBuf,
    pub templates: PathBuf,
    pub commands: PathBuf
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
            let commands = path.join(&COMMANDS);

            rebuild_dir(&PROJECT_DIR, &root)?;

            return Ok(Directory {
                root,
                config,
                templates,
                commands
            });
        }

        Err(anyhow!("Not found root directory"))
    }

    pub fn get_commands(&self) -> Result<Commands> {
        let commands = fs::read_to_string(&self.commands)?;
        Ok(serde_json::from_str(&commands)?)
    }

    pub fn get_config(&self) -> Result<Config> {
        let config = fs::read_to_string(&self.config)?;
        Ok(serde_json::from_str(&config)?)
    }

    pub fn display(&self) -> Result<()> {
        println!(
            "\nConfig: {}\nCommands: {}\nTemplates: {}",
            format!("\"{}\"", self.config.display()).yellow(),
            format!("\"{}\"", self.commands.display()).yellow(),
            format!("\"{}\"", self.templates.display()).yellow()
        );

        Ok(())
    }
}