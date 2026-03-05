use anyhow::{anyhow, Result};
use clap::{arg, value_parser, Arg, ArgMatches, Command};

use crate::{
    cmd::{AddCommand, Cmd},
    core::task::TaskId,
};

pub struct CliParser;

impl CliParser {
    pub fn get_cmd() -> Result<Cmd> {
        if let Some((cmd, matches)) = Self::parse_args().get_matches().subcommand() {
            match cmd {
                "version" => Ok(Cmd::Version),
                "init" => Ok(Cmd::Init),
                "deinit" => Ok(Cmd::Deinit),
                "status" => Ok(Cmd::Status),
                "add" => Self::parse_cmd_add(matches),
                "remove" => Self::parse_cmd_remove(matches),
                "edit" => Self::parse_cmd_edit(matches),
                "list" => Self::parse_cmd_list(matches),
                _ => Err(anyhow!("Not a valid command name")),
            }
        } else {
            Err(anyhow!("No command provided"))
        }
    }

    fn parse_args() -> Command {
        Command::new("rusks")
            .about("Task management command")
            .subcommand(Command::new("version").about("Rusks version info"))
            .subcommand(
                Command::new("init").about("Init a rusks repository in the current directory"),
            )
            .subcommand(Command::new("deinit").about("Deinit rusks repository"))
            .subcommand(Command::new("status").about("IDK -- git has it, so does rusks"))
            .subcommand(
                Command::new("add")
                    .about("Adds a new task")
                    .arg(Arg::new("title").required(true))
                    .arg(arg!(-m --message "Adds a message to the task")),
            )
            .subcommand(
                Command::new("remove").about("Remove a task").arg(
                    Arg::new("id")
                        .value_parser(value_parser!(u64))
                        .required(true),
                ), //    .arg(Arg::new("name"))
            )
            .subcommand(
                Command::new("edit").about("Edit an existing command").arg(
                    Arg::new("id")
                        .value_parser(value_parser!(u64))
                        .required(true),
                ), // .arg(Arg::new("name"))
            )
            .subcommand(
                Command::new("list")
                    .about("List tasks")
                    .arg(Arg::new("pattern").required(false))
                    .arg(arg!(-A --all "Lists more information about the task")),
            )
            .subcommand_required(true)
    }

    fn parse_cmd_add(matches: &ArgMatches) -> Result<Cmd> {
        if let Some(title) = matches.get_one::<String>("title") {
            return Ok(Cmd::Add(AddCommand {
                title: title.clone(),
                options: vec![],
            }));
        }

        Err(anyhow!("No `title` option found"))
    }

    fn parse_cmd_remove(matches: &ArgMatches) -> Result<Cmd> {
        match matches.get_one::<u64>("id") {
            Some(id) => Ok(Cmd::Remove(TaskId::new(*id))),
            None => Err(anyhow!("Failed to parse an `id` argument")),
        }
    }

    fn parse_cmd_edit(matches: &ArgMatches) -> Result<Cmd> {
        if let Some(id) = matches.get_one::<u64>("id") {
            return Ok(Cmd::Edit(TaskId::new(*id)));
        }

        Err(anyhow!("No `id` option found"))
    }

    fn parse_cmd_list(_: &ArgMatches) -> Result<Cmd> {
        Ok(Cmd::List {
            pattern: String::new(),
        })
    }
}
