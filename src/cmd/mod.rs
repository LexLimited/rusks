mod creation;
mod execution;

use clap::{Command, Arg, arg, value_parser};

use crate::error::Error;

use self::creation::{edit::create_edit, new::create_new, list::create_list};

pub enum CMD {
    Init,
    Delete,
    Status,
    New { title: String, options: Vec<String> },
    Remove { id: Option<u64>, name: Option<String> },
    Edit { id: Option<u64>, name: Option<String> },
    List { pattern: String },
}

/// A wrapper around `clap::Command`
impl CMD {
    pub fn new() -> Result<Self, Error> {
        if let Some((cmd, matches)) = Self::new_command().get_matches().subcommand() {
            match cmd {
                "init" => Ok(CMD::Init),
                "delete" => Ok(CMD::Delete),
                "status" => Ok(CMD::Status),
                "new" => create_new(matches),
                "remove" => todo!("implement create cmd remove"),
                "edit" => create_edit(matches),
                "list" => create_list(matches),
                _ => Err(Error::Generic)
            }
        } else {
            Err(Error::Generic)
        }
    }
    
    /// Creates a `clap::Command` object
    fn new_command() -> Command {
        Command::new("rusks")
            .about("Task management command")
            .subcommand(
                Command::new("init").about("Init a rusks repository in the current directory")
            )
            .subcommand(
                Command::new("delete").about("Deletes rusks repository")
            )
            .subcommand(
                Command::new("status").about("IDK -- git has it, so does rusks")
            )
            .subcommand(
                Command::new("new").about("Add a new task")
                    .arg(Arg::new("title").required(true))
                    .arg(
                        arg!(-m --message "Adds a message to the task")
                    )
            )
            .subcommand(
                Command::new("remove").about("Remove a task")
                    .arg(Arg::new("id"))
                //    .arg(Arg::new("name"))
            )
            .subcommand(
                Command::new("edit").about("Edit an existing command")
                .arg(Arg::new("id")
                    .value_parser(value_parser!(u64))
                    .required(true))
                // .arg(Arg::new("name"))
            )
            .subcommand(
                Command::new("list").about("List tasks")
                    .arg(Arg::new("pattern").required(false))
                    .arg(
                        arg!(-A --all "Lists more information about the task")
                    )
            )
    }
}

