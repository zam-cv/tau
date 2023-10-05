use crate::context::Details;
use anyhow::{anyhow, Result};
use colored::Colorize;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

// Variables
const WORKSPACE: &str = r"\{\{workspace\}\}";
const SRC: &str = r"\{\{src\}\}";
const ARG: &str = r"\{\{(.+?)\}\}";
const CASES: [&str; 3] = [WORKSPACE, SRC, ARG];

// Regular expression to search for variables
lazy_static! {
    static ref RE: Regex = {
        let pattern = CASES.iter().join("|");
        Regex::new(&pattern).unwrap()
    };
}

// Add the necessary variables to the command
pub fn replace_command(command: &String, details: &Details, args: &HashMap<&str, &String>) -> Result<String> {
    let mut result = Ok(());

    let replaced = RE.replace_all(command, |caps: &regex::Captures| {
        if let Some(m) = caps.get(0) {
            let value = match m.as_str() {
                "{{workspace}}" => details.workspace.to_str(),
                "{{src}}" => details.src.to_str(),
                arg if arg.starts_with("{{") => {
                    if let Some(name) = caps.get(1) {
                        let name = name.as_str();

                        if let Some(value) = args.get(name) {
                            return value.to_string();
                        } else {
                            // The argument requested in the command does not exist
                            result = Err(anyhow!(format!(
                                "{}\n{} {}",
                                "the argument is missing or does not exist",
                                "Command:".yellow().bold(),
                                command
                            )));
                        }
                    }

                    None
                }
                _ => None,
            };

            if let Some(value) = value.and_then(|s| Some(s.to_string())) {
                return value;
            }
        }

        String::new()
    });

    match result {
        Ok(_) => Ok(replaced.to_string()),
        Err(e) => Err(e),
    }
}
