use crate::directory::Directory;
use crate::utils::{
    dir::{self, compare_dir},
    replace::replace_command,
};
use anyhow::{anyhow, Result};
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};
use dirs;
use fs_extra::dir::copy;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{BufWriter, Write},
    time::Instant,
    {
        collections::{BTreeMap, HashMap, HashSet},
        env,
        path::PathBuf,
    },
};
use subprocess;

type TemplateName = String;

// Output type
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Output {
    Optional,
    Required,
    None,
}

// Command argument
#[derive(Serialize, Deserialize, Clone)]
pub struct Arg {
    pub name: String,
    pub description: Option<String>,
}

// Project commands
#[derive(Serialize, Deserialize, Clone)]
pub struct CommandProject {
    pub tasks: Vec<Task>,
    pub args: Option<Vec<Arg>>,
    pub description: Option<String>,
}

// Task to execute
#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub name: String,
    pub command: String,
}

// Project options
#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectConfig {
    pub optional_files: Vec<String>,
    pub commands: HashMap<String, CommandProject>,
    pub routes: HashSet<PathBuf>,
}

// Config
#[derive(Serialize, Deserialize)]
pub struct Config(pub BTreeMap<TemplateName, ProjectConfig>);

// Target
pub struct Details {
    pub workspace: PathBuf,
    pub src: PathBuf,
}

// Program context
pub struct Context {
    pub details: Details,
    pub project_config: ProjectConfig,
    pub template_name: String,
}

impl Details {
    // Get project details
    pub fn from(workspace: &PathBuf) -> Details {
        let src = workspace.join("src");

        Details {
            workspace: workspace.clone(),
            src,
        }
    }
}

impl Config {
    // Update settings
    pub fn update(&self, directory: &Directory) -> Result<()> {
        let mut writer = BufWriter::new(fs::File::from(
            fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&directory.config)?,
        ));

        // Save the modification
        writer.write_all(serde_json::to_string_pretty(&self)?.as_bytes())?;

        Ok(())
    }

    // Add a project path to the configuration
    pub fn add_route(&mut self, project_path: PathBuf, template_name: String) -> Result<Context> {
        if let Some(mut project_config) = self.0.remove(&template_name) {
            let details = Details::from(&project_path);
            project_config.routes.insert(project_path);

            return Ok(Context {
                details,
                project_config,
                template_name,
            });
        }

        Err(anyhow!("Project not found"))
    }

    // Show template names
    pub fn display(&self, directory: &Directory) -> Result<()> {
        println!();

        for (name, _) in &self.0 {
            let template_path = directory.templates.join(name);
            let size: f32 = fs_extra::dir::get_size(&template_path)? as f32 / 1024.0;

            println!("{:>12} {:>10.3} KB", name.bold().cyan(), size);
        }

        Ok(())
    }
}

impl Context {
    // Create a new context
    pub fn new(
        project_name: &String,
        template_name: Option<&String>,
        directory: &Directory,
        config: &mut Config,
    ) -> Result<Context> {
        let mut project_path = env::current_dir()?;

        if project_name == "." {
            // If there is content, the project cannot be created
            if project_path.read_dir()?.count() > 0 {
                return Err(anyhow!("The directory is not empty"));
            }
        } else {
            project_path = project_path.join(project_name);

            // Verify that the project does not exist
            if project_path.exists() {
                return Err(anyhow!("Project already exists"));
            }
        };

        let template_name = match template_name {
            Some(value) => value.clone(),
            None => {
                let names = config.0.keys().collect::<Vec<&String>>();
                let mut option = 0;

                if names.len() > 1 {
                    // The user is left to decide which template to use
                    option = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("Select a template")
                        .default(0)
                        .items(&names)
                        .interact()?;
                } else if names.len() == 0 {
                    return Err(anyhow!("Templates not found"));
                }

                names[option].clone()
            }
        };

        let template_path = directory.templates.join(&template_name);

        // Verify that the template exists
        if !template_path.exists() {
            return Err(anyhow!("Template not found"));
        }

        let mut options = fs_extra::dir::CopyOptions::new();
        options.content_only = true;

        let _ = copy(&template_path, &project_path, &options);

        if let Some(mut project_config) = config.0.remove(&template_name) {
            println!(
                "   {} {} ({})",
                "New project created:".bold().green(),
                project_name,
                project_path.display()
            );

            let details = Details::from(&project_path);
            project_config.routes.insert(project_path);

            return Ok(Context {
                details,
                project_config,
                template_name,
            });
        }

        Err(anyhow!("Template not found in config"))
    }

    // Create a new context from an existing project
    pub fn this(directory: &Directory, config: &mut Config) -> Result<Context> {
        let project_path = env::current_dir()?;

        if let Some(home) = dirs::home_dir() {
            let mut template_name = None;
            let mut current_path = project_path.clone();

            dir::up(&home, &project_path, &mut |path| {
                current_path = path;

                loop {
                    for (name, project_config) in &mut config.0 {
                        if let Some(_) = project_config.routes.get(&current_path) {
                            let template_path = directory.templates.join(&name);

                            match compare_dir(
                                &template_path,
                                &current_path,
                                &project_config.optional_files,
                            ) {
                                Ok(true) => {
                                    template_name = Some(name.clone());
                                    return Some(());
                                }
                                Ok(false) => {
                                    // If the project was found but the structure is not the same
                                    project_config.routes.remove(&current_path);
                                    continue;
                                }
                                Err(_) => {}
                            }

                            return None;
                        }
                    }

                    break;
                }

                None
            });

            if let Some(template_name) = template_name {
                if let Some(mut project_config) = config.0.remove(&template_name) {
                    let details = Details::from(&current_path);
                    project_config.routes.insert(current_path);

                    return Ok(Context {
                        details,
                        project_config,
                        template_name,
                    });
                }
            }

            let mut coincidences = Vec::new();

            let project_path = dir::up(&home, &current_path, &mut |path| {
                for (name, project_config) in &config.0 {
                    let template_path = directory.templates.join(name);

                    // Verify that the template exists
                    if !template_path.exists() {
                        return Some(Err(anyhow!("Template not found")));
                    }

                    if let Ok(true) =
                        compare_dir(&template_path, &path, &project_config.optional_files)
                    {
                        coincidences.push(name.clone());
                    }
                }

                if coincidences.len() != 0 {
                    return Some(Ok(path));
                }

                None
            });

            if coincidences.len() == 0 {
                return Err(anyhow!("Project not found"));
            }

            if let Some(project_path) = project_path {
                let project_path = project_path?;
                let mut option = 0;

                if coincidences.len() > 1 {
                    // The user is left to decide which template to use
                    option = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("Select a template")
                        .default(0)
                        .items(&coincidences)
                        .interact()?;
                }

                return config.add_route(project_path, coincidences[option].clone());
            }
        } else {
            return Err(anyhow!("Home directory not found"));
        }

        Err(anyhow!("Project not found"))
    }

    // Run a command
    pub fn exec(&self, command: &str, time: &bool, args: &HashMap<&str, &String>) -> Result<()> {
        let command = command.to_lowercase();

        if let Some(command_project) = self.project_config.commands.get(&command.to_string()) {
            for task in command_project.tasks.iter() {
                println!("\n{}\n", task.name.bold().cyan());
                let command = replace_command(&task.command, &self.details, &args)?;
                let start = Instant::now();
                let result = subprocess::Exec::shell(&command)
                    .cwd(&self.details.workspace)
                    .stdout(subprocess::Redirection::Pipe)
                    .stderr(subprocess::Redirection::Merge)
                    .capture()?;
                let end = Instant::now();
                let duration = end.duration_since(start);
                println!("{}", result.stdout_str().trim());

                if *time {
                    println!("\n{}: {} ms", "Time".bold().yellow(), duration.as_millis());
                }
            }

            return Ok(());
        }

        Err(anyhow!("Command not found"))
    }
}
