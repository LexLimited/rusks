use clap::{arg, value_parser, Arg, ArgMatches, Command};

use crate::{cmd::Cmd, error::Error};

pub struct CliParser;

impl CliParser {
    pub fn get_cmd() -> Result<Cmd, Error> {
        if let Some((cmd, matches)) = Self::parse_args().get_matches().subcommand() {
            match cmd {
                "init" => Ok(Cmd::Init),
                "delete" => Ok(Cmd::Delete),
                "status" => Ok(Cmd::Status),
                "add" => Self::parse_cmd_add(matches),
                "remove" => Self::parse_cmd_remove(matches),
                "edit" => Self::parse_cmd_edit(matches),
                "list" => Self::parse_cmd_list(matches),
                _ => Err(Error::Reason {
                    reason: "Not a valid command".to_string(),
                }),
            }
        } else {
            Err(Error::Reason {
                reason: "No command provided".to_string(),
            })
        }
    }

    fn parse_args() -> Command {
        Command::new("rusks")
            .about("Task management command")
            .subcommand(
                Command::new("init").about("Init a rusks repository in the current directory"),
            )
            .subcommand(Command::new("delete").about("Deletes rusks repository"))
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

    fn parse_cmd_add(matches: &ArgMatches) -> Result<Cmd, Error> {
        if let Some(title) = matches.get_one::<String>("title") {
            return Ok(Cmd::Add {
                title: title.clone(),
                options: vec![],
            });
        }

        Err(Error::Generic)
    }

    fn parse_cmd_remove(matches: &ArgMatches) -> Result<Cmd, Error> {
        match matches.get_one::<u64>("id") {
            Some(id) => Ok(Cmd::Remove {
                id: Some(*id),
                name: None,
            }),
            None => Err(Error::Reason {
                reason: "Failed to parse an `id` argument".to_string(),
            }),
        }
    }

    fn parse_cmd_edit(matches: &ArgMatches) -> Result<Cmd, Error> {
        if let Some(id) = matches.get_one::<u64>("id") {
            return Ok(Cmd::Edit {
                id: Some(*id),
                name: None,
            });
        }

        Err(Error::Generic)
    }

    fn parse_cmd_list(_: &ArgMatches) -> Result<Cmd, Error> {
        Ok(Cmd::List {
            pattern: String::new(),
        })
    }
}
