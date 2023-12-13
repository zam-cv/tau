use anyhow::{anyhow, Result};
use clap::{Arg, Command};
use colored::Colorize;
use context::Context;
use directory::Directory;
use std::collections::{HashMap, HashSet};
use utils::string::{persistent_str, persistent_str_optional};

mod context;
mod directory;
mod exec;
mod utils;

fn app() -> Result<()> {
    let mut main = Command::new("Tau")
        .version("0.1.0")
        .author("zam")
        .subcommand(
            Command::new("new")
                .about("Create a new project")
                .arg(
                    Arg::new("project_name")
                        .help("The name of the project")
                        .value_parser(clap::value_parser!(String))
                        .required(true),
                )
                .arg(Arg::new("template_name").help("The project to use")),
        )
        .subcommand(Command::new("path").about("Shows the resource paths used by tau"))
        .subcommand(Command::new("list").about("Shows available templates"))
        .subcommand(
            Command::new("exec").about("Executes a command").arg(
                Arg::new("name")
                    .help("The name of the command")
                    .value_parser(clap::value_parser!(String)),
            ),
        );

    let directory = Directory::new()?;
    let mut config = directory.get_config()?;
    let context = Context::this(&directory, &mut config);
    let mut commands: HashSet<&str> = HashSet::new();

    if let Ok(context) = &context {
        for (name, command_project) in context.project_config.commands.iter() {
            // No need to free memory
            let name: &'static str = persistent_str(name.clone());
            let description = persistent_str_optional(command_project.description.clone());

            let mut command = Command::new(name)
                .arg(
                    Arg::new("time")
                        .help("Show the time of the command")
                        .num_args(0)
                        .long("time")
                        .short('t'),
                )
                .about(&description);

            if let Some(args) = &command_project.args {
                for arg in args {
                    let name: &'static str = persistent_str(arg.name.clone());
                    let description = persistent_str_optional(arg.description.clone());

                    command = command.arg(
                        Arg::new(name)
                            .help(&description)
                            .long(name)
                            .value_parser(clap::value_parser!(String))
                            .required(true),
                    );
                }
            }

            main = main.subcommand(command);
            commands.insert(name);
        }
    }

    if let Some(subcommand) = main.get_matches().subcommand() {
        match subcommand {
            ("new", args) => {
                let project_name: &String = args
                    .get_one("project_name")
                    .expect("project_name is required");

                let template_name: Option<&String> = args.get_one("template_name");

                if let Ok(context) = context {
                    config
                        .0
                        .insert(context.template_name, context.project_config);
                }

                let context = Context::new(project_name, template_name, &directory, &mut config)?;
                config
                    .0
                    .insert(context.template_name, context.project_config);
                config.update(&directory)?;
            }
            ("path", _) => directory.display()?,
            ("list", _) => config.display(&directory)?,
            ("exec", args) => {
                let commands = directory.get_commands()?;
                let name: Option<&String> = args.get_one("name");

                if let Some(name) = name {
                    if let Some(commands) = commands.0.get(name) {
                        for group in commands {
                            println!("");
                            println!(">> {}", group.name.bold());

                            if let Some(description) = &group.description {
                                println!("{}", format!("// {}", description).dimmed());
                            }

                            println!("");
                            for command in &group.commands {
                                println!("{}", command.bold().cyan());
                            }
                        }
                    } else {
                        return Err(anyhow!("Command not found"));
                    }
                } else {
                    println!("");
                    for (name, commands) in commands.0.iter() {
                        println!(
                            "{} {}",
                            name.bold(),
                            format!("[{}]", commands.len()).bold().cyan()
                        );
                    }
                }
            }
            (name, args) => {
                let context = context?;

                if commands.contains(name) {
                    let time: bool = args.get_flag("time");

                    let mut arguments: HashMap<&str, &String> = HashMap::new();

                    if let Some(command_project) = context.project_config.commands.get(name) {
                        if let Some(command_project_args) = &command_project.args {
                            for arg in command_project_args {
                                let name: &'static str =
                                    Box::leak(arg.name.clone().into_boxed_str());

                                if let Some(value) = args.get_one(name) {
                                    arguments.insert(name, value);
                                }
                            }
                        }
                    }

                    context.exec(name, &time, &arguments)?;
                } else {
                    return Err(anyhow!("Command not found"));
                }

                config
                    .0
                    .insert(context.template_name, context.project_config);
                config.update(&directory)?;
            }
        };
    };

    Ok(())
}

fn main() {
    if let Err(e) = app() {
        eprintln!("{} {}", "Error:".bold().red(), e);
    }
}
