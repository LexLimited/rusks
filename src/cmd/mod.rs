mod edit;

use core::fmt;
use std::{
    io::{self, Read},
    os::{self, unix::process::CommandExt},
    process};

use clap::{Command, Arg, ArgMatches, arg, value_parser};

use crate::{
    storage::{RusksStorage, self},
    fs::{
        is_rusks_repository,
        init_rusks_repository,
        delete_rusks_repository,
        self,
        rusks_temp_file_relative_path}
};

use self::edit::edit;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    Generic,
    Reason { reason: String }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Generic => write!(f, "{}", "Generic"),
            Self::Reason { reason } => write!(f, "{}", reason)
        }
    }
}

/// Propms the user to confirm a command, then executes `f`.
/// If `f` was not executed, returns `None`,
/// otherwise returns the value returned from `f`
fn confirm_and_run<R>(question: &str, f: fn() -> R) -> Option<R> {
    println!("{} [Y/n]", question);
    
    let mut input = String::new();
    if let Err(_) = io::stdin().read_line(&mut input) {
        return None;
    }
    
    if input.to_lowercase() == "y\n" {
        return Some(f());
    }

    return None;
}

pub enum CMD {
    Init,
    Delete,
    New { name: String, options: Vec<String> },
    Remove { id: Option<u64>, name: Option<String> },
    Edit { id: Option<u64>, name: Option<String> },
    List { pattern: String },
}

/// A wrapper around `clap::Command`
impl CMD {
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
                Command::new("new").about("Add a new task")
                    .arg(Arg::new("name"))
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

    pub fn new() -> Result<Self, Error> {
        if let Some(sub) = Self::new_command().get_matches().subcommand() {
            match sub.0 {
                "init" => Ok(CMD::Init),
                "delete" => Ok(CMD::Delete),
                "new" => Self::create_cmd_new(sub.1),
                "remove" => Self::create_cmd_remove(sub.1),
                "edit" => Self::create_cmd_edit(sub.1),
                "list" => Self::create_cmd_list(sub.1),
                _ => Err(Error::Generic)
            }
        } else {
            Err(Error::Generic)
        }
    }

    fn create_cmd_new(m: &ArgMatches) -> Result<Self, Error> {
        match m.get_one::<String>("name") {
            Some(name) => {
                Ok(CMD::New{
                    name: name.to_owned(),
                    options: vec![]
                })
            },
            None => Err(Error::Generic)
        }
    }

    fn create_cmd_remove(m: &ArgMatches) -> Result<Self, Error> {
        todo!()
    }

    fn create_cmd_edit(m: &ArgMatches) -> Result<Self, Error> {
        match m.get_one::<u64>("id") {
            Some(id) => {
                Ok(CMD::Edit{
                    id: Some(id.clone()),
                    name: None
                })
            },
            None => Err(Error::Generic)
        }
    }

    fn create_cmd_list(m: &ArgMatches) -> Result<Self, Error> {
        Ok(CMD::List{
            pattern: match m.get_one::<String>("pattern") {
                Some(p) => p.to_owned(),
                None => String::from("")
            }
        })
    }

    pub fn display_help() -> Result<(), Error> {
        match Self::new_command().print_help() {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::Generic)
        }
    }

    pub fn exec(self: &Self) -> Result<(), Error> {
        if !is_rusks_repository() {
            if let CMD::Init{} = self {
                return match init_rusks_repository() {
                    Err(e) => {
                        println!("{}", e);
                        Err(Error::Generic)
                    }
                    Ok(_) => Ok(())
                }
            }

            println!("Not a rusks repository");
            return Err(Error::Generic);
        }

        match self {
            CMD::Delete => Self::exec_cmd_delete(),
            CMD::New { name, options } => Self::exec_cmd_new(name, options),
            CMD::Remove { id, name } => Self::exec_cmd_remove(id, &name),
            CMD::Edit { id, name } => Self::exec_cmd_edit(id, name),
            CMD::List { pattern } => Self::exec_cmd_list(pattern),
            _ => Err(Error::Generic),
        }
    }

    fn exec_cmd_delete() -> Result<(), Error> {
        let res = confirm_and_run("Delete rusks repository?", || {
            match delete_rusks_repository() {
                Ok(_) => Ok(()),
                Err(_) => Err(Error::Generic)
            }
        });

        if let None = res {
            return Ok(println!("Declined"));
        }

        match res {
            None => Ok(()),
            Some(e) => match e {
                Ok(_) => Ok(()),
                Err(_) => Err(Error::Generic)
            }
        }
    }

    fn exec_cmd_new(name: &String, options: &Vec<String>) -> Result<(), Error> {
        println!("Will add a task with name '{}' (also got {} options)", name, options.len());
        Ok(())
    }

    fn exec_cmd_remove(id: &Option<u64>, name: &Option<String>) -> Result<(), Error> {
        let panic_msg = "`remove` must specify id or name, but not both";

        if let Some(id) = id {
            if name.is_some() {
                panic!("{}", panic_msg)
            }
            println!("Will remove a task with id {}", id)
        }

        if let Some(name) = name {
            println!("Will remove a task with name {}", name);
        }
        
        Ok(())
    }

    fn exec_cmd_edit(id: &Option<u64>, name: &Option<String>) -> Result<(), Error> {
        let panic_msg = "`edit` must specify id or name, but not both";

        match id {
            Some(id) => {
                if name.is_some() {
                    panic!("{}", panic_msg);
                }
                let storage = RusksStorage::new();
                match storage {
                    Ok(storage) => {
                        let mut item = match storage.get_by_id(*id) {
                            Some(item) => item,
                            None => panic!("Item not found")
                        };
                        edit(*id, item.get_task_mut())
                    }
                    Err(_) => Err(Error::Generic)
                }
            },
            None => {
                panic!("edit by name is not yet supported");
            }
        }
    }

    fn exec_cmd_list(pattern: &String) -> Result<(), Error> {
        return match RusksStorage::new() {
            Ok(storage) => {
                for item in storage.get_all() {
                    println!("{}", item);
                }
                Ok(())
            },
            Err(_) => Err(Error::Generic)
        }
    }
}

